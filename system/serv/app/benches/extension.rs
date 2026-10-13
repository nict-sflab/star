mod common;

use std::os::raw::{c_int, c_uchar, c_uint};
use std::ptr;
use std::slice;
use std::sync::Arc;
use std::time::{Duration, Instant};

use common::scale_out::{OPS_PER_SERVER, SERVER_COUNTS, ScaleOutServers, run_worker};
use common::star::{KeyedEnclaves, MAX_COUNT_PER_CREDENTIAL, register_user};
use common::tls::{self, CertificateKind, MemoryStream};
use criterion::{Criterion, Throughput, black_box, criterion_group};
use foreign_types::ForeignTypeRef;
use openssl::error::ErrorStack;
use openssl::ssl::{
    ClientHelloResponse, ExtensionContext, HandshakeError, Ssl, SslAcceptor, SslAlert,
    SslMethod, SslRef,
};
use rand_core::OsRng;
use star_app::filter::StarFilterErrorStatus;
use star_app::tls::STAR_TLS_EXTENSION_TYPE;

unsafe extern "C" {
    fn SSL_client_hello_get0_ext(
        s: *mut openssl_sys::SSL,
        type_: c_uint,
        out: *mut *const c_uchar,
        outlen: *mut usize,
    ) -> c_int;
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
        tls::configure_server(&mut acceptor, CertificateKind::File);

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

        let mut connector = tls::client_connector();
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

        let (client_hello, _client) = tls::prepare_client_hello(connector.build());

        let harness = Self {
            acceptor: acceptor.build(),
            client_hello,
        };
        assert!(harness.is_rejected());
        harness
    }

    fn is_rejected(&self) -> bool {
        match self.acceptor.accept(MemoryStream::new(black_box(self.client_hello.as_slice()))) {
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

fn run_scale_out_worker(case: &str, shard_index: u32, shard_count: u32, core_id: usize) {
    let case = match case {
        "openssl_missing_extension" => RejectCase::MissingExtension,
        "star_filter_replayed_token_openssl_parse" => RejectCase::ReplayedToken,
        "star_filter_invalid_credential_openssl_parse" => RejectCase::InvalidCredential,
        _ => panic!("unknown scale-out benchmark case"),
    };
    run_worker(
        core_id,
        || RejectHarness::new(case, shard_index, shard_count),
        |server, operations| {
            for _ in 0..operations {
                assert!(server.is_rejected());
            }
        },
    );
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
    let mut missing_extension = None;
    let mut replayed_token = None;
    let mut invalid_credential = None;

    let mut group = c.benchmark_group("client_hello_reject");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(4));
    group.throughput(Throughput::Elements(1));

    group.bench_function("openssl_missing_extension", |b| {
        let server = missing_extension
            .get_or_insert_with(|| RejectHarness::new(RejectCase::MissingExtension, 0, 1));
        b.iter_custom(|iters| server.measure(iters));
    });
    group.bench_function("star_filter_invalid_credential_openssl_parse", |b| {
        let server = invalid_credential
            .get_or_insert_with(|| RejectHarness::new(RejectCase::InvalidCredential, 0, 1));
        b.iter_custom(|iters| server.measure(iters));
    });
    group.bench_function("star_filter_replayed_token_openssl_parse", |b| {
        let server = replayed_token
            .get_or_insert_with(|| RejectHarness::new(RejectCase::ReplayedToken, 0, 1));
        b.iter_custom(|iters| server.measure(iters));
    });
    group.finish();

    let cases = [
        "openssl_missing_extension",
        "star_filter_replayed_token_openssl_parse",
        "star_filter_invalid_credential_openssl_parse",
    ];
    let mut group = c.benchmark_group("client_hello_reject_scale_out_throughput");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(4));

    for &server_count in SERVER_COUNTS {
        for &name in &cases {
            let mut servers = None;
            group.throughput(Throughput::Elements(
                u64::from(server_count) * OPS_PER_SERVER,
            ));
            group.bench_function(format!("{name}_{server_count}_servers"), |b| {
                let servers = servers.get_or_insert_with(|| {
                    ScaleOutServers::new(server_count, |shard_index, shard_count, core| {
                        vec![
                            name.to_string(),
                            shard_index.to_string(),
                            shard_count.to_string(),
                            core.to_string(),
                        ]
                    })
                });
                b.iter_custom(|batches| servers.measure(batches));
            });
            if let Some(servers) = servers {
                servers.finish();
            }
        }
    }
    group.finish();
}

criterion_group!(benches, bench_client_hello_reject);

fn main() {
    let mut args = std::env::args();
    if args.nth(1).as_deref() == Some("--scale-out-worker") {
        let case = args.next().expect("server case");
        let shard_index = args
            .next()
            .expect("server shard index")
            .parse()
            .expect("shard index");
        let shard_count = args
            .next()
            .expect("server shard count")
            .parse()
            .expect("shard count");
        let core_id = args
            .next()
            .expect("server CPU core")
            .parse()
            .expect("CPU core");
        run_scale_out_worker(&case, shard_index, shard_count, core_id);
        return;
    }
    benches();
    Criterion::default().configure_from_args().final_summary();
}
