use std::io::{self, Read, Write};
use std::os::raw::{c_int, c_uchar, c_uint};
use std::ptr;
use std::slice;
use std::sync::Arc;
use std::time::{Duration, Instant};

use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use foreign_types::ForeignTypeRef;
use openssl::error::ErrorStack;
use openssl::ssl::{
    ClientHelloResponse, ExtensionContext, HandshakeError, Ssl, SslAcceptor, SslAlert,
    SslConnector, SslFiletype, SslMethod, SslRef, SslVerifyMode,
};
use rand_core::OsRng;
use simple_filter_box::dashmap::DashMapFilterBox;
use star_app::enclave::{EnclaveRuntime, EnclaveRuntimes};
use star_app::filter::{StarFilter, StarFilterErrorStatus};
use star_app::tls::STAR_TLS_EXTENSION_TYPE;
use star_core::{client::User, secure_channel::client::SecureChannelClient};

const TLS_CERT_RELATIVE_PATH: &str = "examples/certs/server.crt";
const TLS_KEY_RELATIVE_PATH: &str = "examples/certs/server.key";
const MAX_COUNT_PER_CREDENTIAL: u64 = 1_000_000;
const FILTER_ENCLAVE_POOL_SIZE: u8 = 10;

type BenchFilter = StarFilter<DashMapFilterBox>;

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

impl ClientHelloRecorder {
    fn new() -> Self {
        Self { output: Vec::new() }
    }
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

#[derive(Debug)]
struct ClientHelloInput<'a> {
    input: &'a [u8],
    read_offset: usize,
}

impl<'a> ClientHelloInput<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            input,
            read_offset: 0,
        }
    }
}

impl Read for ClientHelloInput<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.read_offset >= self.input.len() {
            return Err(io::Error::from(io::ErrorKind::WouldBlock));
        }

        let remaining = &self.input[self.read_offset..];
        let len = remaining.len().min(buf.len());
        buf[..len].copy_from_slice(&remaining[..len]);
        self.read_offset += len;
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ClientHelloOutcome {
    PendingMoreInput,
    Rejected,
}

struct OpenSslRejectHarness {
    acceptor: SslAcceptor,
    client_hello: Vec<u8>,
}

struct StarFilterRejectHarness {
    acceptor: SslAcceptor,
    filter: Arc<BenchFilter>,
    token_request: Vec<u8>,
    client_hello: Vec<u8>,
}

impl OpenSslRejectHarness {
    fn missing_extension() -> Self {
        let mut builder = base_acceptor_builder();

        builder.set_client_hello_callback(move |ssl, alert| {
            if star_extension_from_client_hello(ssl).is_none() {
                return reject_client_hello(alert, SslAlert::DECODE_ERROR);
            }

            Ok(ClientHelloResponse::SUCCESS)
        });

        let connector = connector_with_star_extension(None);

        Self {
            acceptor: builder.build(),
            client_hello: record_client_hello(&connector),
        }
    }

    fn timed_rejections(&self, iters: u64) -> Duration {
        let mut total = Duration::ZERO;

        for _ in 0..iters {
            let start = Instant::now();
            let outcome =
                process_client_hello(&self.acceptor, black_box(self.client_hello.as_slice()));
            total += start.elapsed();
            assert_eq!(outcome, ClientHelloOutcome::Rejected);
        }

        total
    }
}

impl StarFilterRejectHarness {
    fn replayed_token() -> Self {
        let filter = Arc::new(BenchFilter::new(
            DashMapFilterBox::new(),
            vec![0],
            1,
            EnclaveRuntimes::from_env(FILTER_ENCLAVE_POOL_SIZE).expect("filter enclave pool"),
        ));
        let token_request = build_token_request();
        let connector = connector_with_star_extension(Some(token_request.clone()));
        let client_hello = record_client_hello(&connector);
        let mut builder = base_acceptor_builder();
        let token_index = Ssl::new_ex_index::<Vec<u8>>().expect("TLS token ex_data index");
        let callback_filter = Arc::clone(&filter);

        builder.set_client_hello_callback(move |ssl, alert| {
            let Some(ext_data) = star_extension_from_client_hello(ssl) else {
                return reject_client_hello(alert, SslAlert::DECODE_ERROR);
            };

            if ext_data.is_empty() {
                return reject_client_hello(alert, SslAlert::DECODE_ERROR);
            }

            Ok(ClientHelloResponse::SUCCESS)
        });

        builder
            .add_custom_ext(
                STAR_TLS_EXTENSION_TYPE,
                ExtensionContext::CLIENT_HELLO,
                |_ssl, _context, _cert| Ok(None::<Vec<u8>>),
                move |ssl, _context, ext_data, _cert| {
                    if ext_data.is_empty() {
                        return Err(SslAlert::DECODE_ERROR);
                    }

                    if let Some(existing) = ssl.ex_data(token_index) {
                        if existing.as_slice() == ext_data {
                            return Ok(());
                        }

                        return Err(SslAlert::ILLEGAL_PARAMETER);
                    }

                    if let Err(filter_err) = callback_filter.filter_client_hello(ext_data) {
                        return Err(alert_for_filter_error(&filter_err));
                    }

                    ssl.set_ex_data(token_index, ext_data.to_vec());
                    Ok(())
                },
            )
            .expect("configure STAR TLS extension");

        let harness = Self {
            acceptor: builder.build(),
            filter,
            token_request,
            client_hello,
        };

        let outcome = process_client_hello(&harness.acceptor, harness.client_hello.as_slice());
        assert_eq!(outcome, ClientHelloOutcome::PendingMoreInput);

        harness
    }

    fn timed_openssl_and_filter_rejections(&self, iters: u64) -> Duration {
        let mut total = Duration::ZERO;

        for _ in 0..iters {
            let start = Instant::now();
            let outcome =
                process_client_hello(&self.acceptor, black_box(self.client_hello.as_slice()));
            total += start.elapsed();
            assert_eq!(outcome, ClientHelloOutcome::Rejected);
        }

        total
    }

    fn timed_filter_only_rejections(&self, iters: u64) -> Duration {
        let mut total = Duration::ZERO;

        for _ in 0..iters {
            let start = Instant::now();
            let result = self
                .filter
                .filter_client_hello(black_box(self.token_request.as_slice()));
            total += start.elapsed();
            assert!(matches!(
                result,
                Err(StarFilterErrorStatus::DuplicatedToken)
            ));
        }

        total
    }
}

fn process_client_hello(acceptor: &SslAcceptor, client_hello: &[u8]) -> ClientHelloOutcome {
    match acceptor.accept(ClientHelloInput::new(client_hello)) {
        Ok(_) => ClientHelloOutcome::PendingMoreInput,
        Err(HandshakeError::WouldBlock(_)) => ClientHelloOutcome::PendingMoreInput,
        Err(HandshakeError::Failure(_)) => ClientHelloOutcome::Rejected,
        Err(HandshakeError::SetupFailure(err)) => panic!("TLS server setup failed: {err}"),
    }
}

fn record_client_hello(connector: &SslConnector) -> Vec<u8> {
    let recorder = ClientHelloRecorder::new();

    let client_hello = match connector.connect("localhost", recorder) {
        Ok(_) => panic!("client handshake unexpectedly completed without server input"),
        Err(HandshakeError::WouldBlock(mid)) | Err(HandshakeError::Failure(mid)) => {
            mid.get_ref().output.clone()
        }
        Err(HandshakeError::SetupFailure(err)) => panic!("TLS client setup failed: {err}"),
    };

    assert!(!client_hello.is_empty(), "recorded ClientHello is empty");
    client_hello
}

fn build_token_request() -> Vec<u8> {
    let mut issue_enclave = EnclaveRuntime::from_env().expect("issue enclave");
    let sender =
        SecureChannelClient::new(issue_enclave.public_key().expect("public key")).expect("sender");
    let mut user = User::new(MAX_COUNT_PER_CREDENTIAL, sender);
    let mut rng = OsRng;

    let (cred_request, shared_key) = user.request_credential(&mut rng);
    let cred_cipher = issue_enclave
        .issue_cred(&cred_request)
        .expect("issue credential");
    user.receive_credential(&cred_cipher, &shared_key);

    let period = u64::from_be_bytes(constant_time_utils::time::current_period());
    user.request_auth(0, period, &mut rng)
        .expect("token request")
}

fn base_acceptor_builder() -> openssl::ssl::SslAcceptorBuilder {
    let mut builder =
        SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("build TLS acceptor");
    builder
        .set_certificate_file(manifest_path(TLS_CERT_RELATIVE_PATH), SslFiletype::PEM)
        .expect("set TLS certificate");
    builder
        .set_private_key_file(manifest_path(TLS_KEY_RELATIVE_PATH), SslFiletype::PEM)
        .expect("set TLS private key");
    builder.check_private_key().expect("check TLS private key");
    builder.set_groups_list("X25519").expect("set TLS groups");
    builder
}

fn connector_with_star_extension(ext: Option<Vec<u8>>) -> SslConnector {
    let mut builder = SslConnector::builder(SslMethod::tls()).expect("build TLS connector");
    builder.set_verify(SslVerifyMode::NONE);
    builder.set_groups_list("X25519").expect("set TLS groups");

    if let Some(ext) = ext {
        builder
            .add_custom_ext(
                STAR_TLS_EXTENSION_TYPE,
                ExtensionContext::CLIENT_HELLO,
                move |_ssl, _context, _cert| Ok(Some(ext.clone())),
                |_ssl, _context, _ext_data, _cert| Ok(()),
            )
            .expect("configure client STAR extension");
    }

    builder.build()
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

    if found != 1 {
        return None;
    }

    if out_len == 0 {
        return Some(&[]);
    }

    if out.is_null() {
        return None;
    }

    Some(unsafe { slice::from_raw_parts(out as *const u8, out_len) })
}

fn alert_for_filter_error(status: &StarFilterErrorStatus) -> SslAlert {
    match status {
        StarFilterErrorStatus::InvalidToken => SslAlert::DECODE_ERROR,
        StarFilterErrorStatus::InvalidRoute => SslAlert::ILLEGAL_PARAMETER,
        StarFilterErrorStatus::DuplicatedToken => SslAlert::ILLEGAL_PARAMETER,
        StarFilterErrorStatus::Internal => SslAlert::DECODE_ERROR,
    }
}

fn manifest_path(relative_path: &str) -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), relative_path)
}

fn configure_group(group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>) {
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(4));
    group.throughput(Throughput::Elements(1));
}

fn bench_client_hello_reject_no_network(c: &mut Criterion) {
    let openssl = OpenSslRejectHarness::missing_extension();
    let star_filter = StarFilterRejectHarness::replayed_token();

    assert_eq!(
        process_client_hello(&openssl.acceptor, openssl.client_hello.as_slice()),
        ClientHelloOutcome::Rejected
    );
    assert_eq!(
        process_client_hello(&star_filter.acceptor, star_filter.client_hello.as_slice()),
        ClientHelloOutcome::Rejected
    );
    assert!(matches!(
        star_filter
            .filter
            .filter_client_hello(star_filter.token_request.as_slice()),
        Err(StarFilterErrorStatus::DuplicatedToken)
    ));

    let mut group = c.benchmark_group("client_hello_reject_no_network");
    configure_group(&mut group);

    group.bench_function("openssl_missing_extension", |b| {
        b.iter_custom(|iters| openssl.timed_rejections(iters));
    });

    group.bench_function("star_filter_replayed_token_openssl_parse", |b| {
        b.iter_custom(|iters| star_filter.timed_openssl_and_filter_rejections(iters));
    });

    group.bench_function("star_filter_replayed_token_filter_only", |b| {
        b.iter_custom(|iters| star_filter.timed_filter_only_rejections(iters));
    });

    group.finish();
}

criterion_group!(benches, bench_client_hello_reject_no_network);
criterion_main!(benches);
