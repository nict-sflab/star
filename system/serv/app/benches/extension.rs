mod common;

use std::io::{self, Read, Write};
use std::os::raw::{c_int, c_uchar, c_uint};
use std::ptr;
use std::slice;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, Instant};

use common::star::{KeyedEnclaves, MAX_COUNT_PER_CREDENTIAL, register_user};
use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use foreign_types::ForeignTypeRef;
use openssl::error::ErrorStack;
use openssl::ssl::{
    ClientHelloResponse, ExtensionContext, HandshakeError, Ssl, SslAcceptor, SslAlert,
    SslConnector, SslFiletype, SslMethod, SslRef, SslVerifyMode,
};
use rand_core::OsRng;
use star_app::filter::StarFilterErrorStatus;
use star_app::tls::STAR_TLS_EXTENSION_TYPE;

const TLS_CERT_RELATIVE_PATH: &str = "examples/certs/server.crt";
const TLS_KEY_RELATIVE_PATH: &str = "examples/certs/server.key";
const SCALE_OUT_SERVER_COUNTS: &[u32] = &[1, 2, 4];
const SCALE_OUT_OPS_PER_SERVER: u64 = 64;

unsafe extern "C" {
    fn SSL_client_hello_get0_ext(
        s: *mut openssl_sys::SSL,
        type_: c_uint,
        out: *mut *const c_uchar,
        outlen: *mut usize,
    ) -> c_int;
}

#[derive(Debug)]
struct ClientHelloRecorder {
    output: Vec<u8>,
}

impl Read for ClientHelloRecorder {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::from(io::ErrorKind::WouldBlock))
    }
}

impl Write for ClientHelloRecorder {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.output.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct ClientHelloInput<'a> {
    input: &'a [u8],
    offset: usize,
}

impl Read for ClientHelloInput<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.offset == self.input.len() {
            return Err(io::Error::from(io::ErrorKind::WouldBlock));
        }

        let input = &self.input[self.offset..];
        let len = input.len().min(buf.len());
        buf[..len].copy_from_slice(&input[..len]);
        self.offset += len;
        Ok(len)
    }
}

impl Write for ClientHelloInput<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum RejectCase {
    MissingExtension,
    ReplayedToken,
    InvalidCredential,
}

struct RejectHarness {
    acceptor: SslAcceptor,
    client_hello: Vec<u8>,
}

impl RejectHarness {
    fn new(case: RejectCase, shard_index: u32, shard_count: u32) -> Self {
        let mut acceptor =
            SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("build TLS acceptor");
        acceptor
            .set_certificate_file(
                common::manifest_path(TLS_CERT_RELATIVE_PATH),
                SslFiletype::PEM,
            )
            .expect("set TLS certificate");
        acceptor
            .set_private_key_file(
                common::manifest_path(TLS_KEY_RELATIVE_PATH),
                SslFiletype::PEM,
            )
            .expect("set TLS private key");
        acceptor.check_private_key().expect("check TLS private key");
        acceptor.set_groups_list("X25519").expect("set TLS groups");

        let extension = match case {
            RejectCase::MissingExtension => {
                acceptor.set_client_hello_callback(|ssl, alert| {
                    if star_extension_from_client_hello(ssl).is_none() {
                        return reject_client_hello(alert, SslAlert::DECODE_ERROR);
                    }
                    Ok(ClientHelloResponse::SUCCESS)
                });
                None
            }
            RejectCase::ReplayedToken | RejectCase::InvalidCredential => {
                let invalid_credential = matches!(case, RejectCase::InvalidCredential);
                let (filter, issue_enclave) =
                    KeyedEnclaves::new().into_filter(vec![shard_index], shard_count);
                let filter = Arc::new(filter);
                let mut rng = OsRng;
                let mut user = register_user(&issue_enclave, MAX_COUNT_PER_CREDENTIAL, &mut rng);
                let period = u64::from_be_bytes(constant_time_utils::time::current_period());
                let (counter, request, token) = (0..MAX_COUNT_PER_CREDENTIAL)
                    .find_map(|counter| {
                        let request = user
                            .request_auth(counter, period, &mut rng)
                            .expect("token request");
                        let (token, route) = filter
                            .issue_token_and_route_sync(&request)
                            .expect("issue token and route");
                        (route == shard_index).then_some((counter, request, token))
                    })
                    .expect("request routed to shard");

                let request = if invalid_credential {
                    user.credential.as_mut().expect("credential").code[0] ^= 0x01;
                    let request = user
                        .request_auth(counter, period, &mut rng)
                        .expect("invalid credential request");
                    assert!(matches!(
                        filter.filter_client_hello(&request),
                        Err(StarFilterErrorStatus::InvalidToken)
                    ));
                    request
                } else {
                    filter
                        .check_duplication(&token, shard_index)
                        .expect("seed duplicate token");
                    assert!(matches!(
                        filter.check_duplication(&token, shard_index),
                        Err(StarFilterErrorStatus::DuplicatedToken)
                    ));
                    request
                };

                acceptor.set_client_hello_callback(|ssl, alert| {
                    match star_extension_from_client_hello(ssl) {
                        Some(data) if !data.is_empty() => Ok(ClientHelloResponse::SUCCESS),
                        _ => reject_client_hello(alert, SslAlert::DECODE_ERROR),
                    }
                });

                let token_index = Ssl::new_ex_index::<Vec<u8>>().expect("TLS token ex_data index");
                acceptor
                    .add_custom_ext(
                        STAR_TLS_EXTENSION_TYPE,
                        ExtensionContext::CLIENT_HELLO,
                        |_ssl, _context, _cert| Ok(None::<Vec<u8>>),
                        move |ssl, _context, ext_data, _cert| {
                            if ext_data.is_empty() {
                                return Err(SslAlert::DECODE_ERROR);
                            }

                            if let Some(existing) = ssl.ex_data(token_index) {
                                return if existing.as_slice() == ext_data {
                                    Ok(())
                                } else {
                                    Err(SslAlert::ILLEGAL_PARAMETER)
                                };
                            }

                            filter
                                .filter_client_hello(ext_data)
                                .map_err(|error| match error {
                                    StarFilterErrorStatus::InvalidToken
                                    | StarFilterErrorStatus::Internal => SslAlert::DECODE_ERROR,
                                    StarFilterErrorStatus::InvalidRoute
                                    | StarFilterErrorStatus::DuplicatedToken => {
                                        SslAlert::ILLEGAL_PARAMETER
                                    }
                                })?;
                            ssl.set_ex_data(token_index, ext_data.to_vec());
                            Ok(())
                        },
                    )
                    .expect("configure STAR extension");

                Some(request)
            }
        };

        let mut connector = SslConnector::builder(SslMethod::tls()).expect("build TLS connector");
        connector.set_verify(SslVerifyMode::NONE);
        connector.set_groups_list("X25519").expect("set TLS groups");
        if let Some(extension) = extension {
            connector
                .add_custom_ext(
                    STAR_TLS_EXTENSION_TYPE,
                    ExtensionContext::CLIENT_HELLO,
                    move |_ssl, _context, _cert| Ok(Some(extension.clone())),
                    |_ssl, _context, _ext_data, _cert| Ok(()),
                )
                .expect("configure client STAR extension");
        }

        let recorder = ClientHelloRecorder { output: Vec::new() };
        let client_hello = match connector.build().connect("localhost", recorder) {
            Ok(_) => panic!("client handshake unexpectedly completed without server input"),
            Err(HandshakeError::WouldBlock(handshake))
            | Err(HandshakeError::Failure(handshake)) => handshake.get_ref().output.clone(),
            Err(HandshakeError::SetupFailure(error)) => {
                panic!("TLS client setup failed: {error}")
            }
        };
        assert!(!client_hello.is_empty(), "recorded ClientHello is empty");

        let harness = Self {
            acceptor: acceptor.build(),
            client_hello,
        };
        assert!(harness.is_rejected());
        harness
    }

    fn is_rejected(&self) -> bool {
        match self.acceptor.accept(ClientHelloInput {
            input: black_box(self.client_hello.as_slice()),
            offset: 0,
        }) {
            Err(HandshakeError::Failure(_)) => true,
            Ok(_) | Err(HandshakeError::WouldBlock(_)) => false,
            Err(HandshakeError::SetupFailure(error)) => {
                panic!("TLS server setup failed: {error}")
            }
        }
    }

    fn measure(&self, iters: u64) -> Duration {
        let mut total = Duration::ZERO;
        for _ in 0..iters {
            let start = Instant::now();
            let rejected = self.is_rejected();
            total += start.elapsed();
            assert!(rejected);
        }
        total
    }
}

struct ShardedServers {
    servers: Vec<RejectHarness>,
}

impl ShardedServers {
    fn new(case: RejectCase, server_count: u32) -> Self {
        let servers = (0..server_count)
            .map(|shard_index| RejectHarness::new(case, shard_index, server_count))
            .collect();
        Self { servers }
    }

    fn measure(&self, batches: u64) -> Duration {
        let operations = batches
            .checked_mul(SCALE_OUT_OPS_PER_SERVER)
            .expect("scale-out benchmark operation count overflow");
        let ready = Barrier::new(self.servers.len() + 1);
        let start = Barrier::new(self.servers.len() + 1);

        thread::scope(|scope| {
            let mut handles = Vec::with_capacity(self.servers.len());
            for server in &self.servers {
                let ready = &ready;
                let start = &start;
                handles.push(scope.spawn(move || {
                    ready.wait();
                    start.wait();
                    for _ in 0..operations {
                        assert!(server.is_rejected());
                    }
                }));
            }

            ready.wait();
            let timer = Instant::now();
            start.wait();
            for handle in handles {
                handle.join().expect("scale-out benchmark server");
            }
            timer.elapsed()
        })
    }
}

fn reject_client_hello(
    alert: &mut SslAlert,
    ssl_alert: SslAlert,
) -> Result<ClientHelloResponse, ErrorStack> {
    *alert = ssl_alert;
    Err(ErrorStack::get())
}

fn star_extension_from_client_hello<'a>(ssl: &'a SslRef) -> Option<&'a [u8]> {
    let mut out = ptr::null();
    let mut out_len = 0usize;
    let found = unsafe {
        SSL_client_hello_get0_ext(
            ssl.as_ptr(),
            STAR_TLS_EXTENSION_TYPE as c_uint,
            &mut out,
            &mut out_len,
        )
    };

    if found != 1 || (out.is_null() && out_len != 0) {
        return None;
    }
    if out_len == 0 {
        return Some(&[]);
    }
    Some(unsafe { slice::from_raw_parts(out, out_len) })
}

fn bench_client_hello_reject(c: &mut Criterion) {
    let missing_extension = RejectHarness::new(RejectCase::MissingExtension, 0, 1);
    let replayed_token = RejectHarness::new(RejectCase::ReplayedToken, 0, 1);
    let invalid_credential = RejectHarness::new(RejectCase::InvalidCredential, 0, 1);

    let mut group = c.benchmark_group("client_hello_reject");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(4));
    group.throughput(Throughput::Elements(1));

    group.bench_function("openssl_missing_extension", |b| {
        b.iter_custom(|iters| missing_extension.measure(iters));
    });
    group.bench_function("star_filter_invalid_credential_openssl_parse", |b| {
        b.iter_custom(|iters| invalid_credential.measure(iters));
    });
    group.bench_function("star_filter_replayed_token_openssl_parse", |b| {
        b.iter_custom(|iters| replayed_token.measure(iters));
    });
    group.finish();

    let cases = [
        (RejectCase::MissingExtension, "openssl_missing_extension"),
        (
            RejectCase::ReplayedToken,
            "star_filter_replayed_token_openssl_parse",
        ),
        (
            RejectCase::InvalidCredential,
            "star_filter_invalid_credential_openssl_parse",
        ),
    ];
    let mut group = c.benchmark_group("client_hello_reject_scale_out_throughput");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(4));

    for &server_count in SCALE_OUT_SERVER_COUNTS {
        for &(case, name) in &cases {
            let servers = ShardedServers::new(case, server_count);
            group.throughput(Throughput::Elements(
                u64::from(server_count) * SCALE_OUT_OPS_PER_SERVER,
            ));
            group.bench_function(format!("{name}_{server_count}_servers"), |b| {
                b.iter_custom(|batches| servers.measure(batches));
            });
        }
    }
    group.finish();
}

criterion_group!(benches, bench_client_hello_reject);
criterion_main!(benches);
