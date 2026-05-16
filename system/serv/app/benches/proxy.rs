mod common;

use common::latency_proxy::StarLatencyProxy;
use common::local_http::{reserve_local_addr, spawn_ok_server};
use common::retry::{StartupProbe, wait_until_ready};
use common::tls::StarTlsClient;
use constant_time_utils::time as ct_time;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use pingora::listeners::tls::TlsSettings;
use pingora::server::Server;
use pingora_proxy::http_proxy_service;
use rand_core::OsRng;
use simple_filter_box::dashmap::DashMapFilterBox;
use star_app::enclave::{EnclaveRuntime, EnclaveRuntimes};
use star_app::filter::StarFilter;
use star_app::proxy::StarProxy;
use star_app::tls_token::configure_tls_token_extension;
use star_core::{client::User, secure_channel::client::SecureChannelClient};
use std::net::ToSocketAddrs;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

const STARTUP_RETRIES: u32 = 100;
const STARTUP_RETRY_DELAY: Duration = Duration::from_millis(50);
const WARMUP_REQUESTS: usize = 128;
const TLS_CERT_RELATIVE_PATH: &str = "examples/certs/server.crt";
const TLS_KEY_RELATIVE_PATH: &str = "examples/certs/server.key";
const MAX_COUNT_PER_CREDENTIAL: u64 = 3;
const PROXY_TLS_READ_TIMEOUT: Duration = Duration::from_secs(2);
const LATENCY_RECV_TIMEOUT: Duration = Duration::from_secs(2);
const BENCH_GROUP_NAME: &str = "pingora_proxy_basic_latency_tls";
const BENCH_FUNCTION_NAME: &str = "single_thread_tls_process_cpu";

const PROXY_ONLY: bool = false;

type BenchFilter = StarFilter<DashMapFilterBox>;

fn build_star_proxy(upstream_addr: String) -> StarProxy<DashMapFilterBox> {
    let filter = BenchFilter::new(
        DashMapFilterBox::new(),
        vec![0],
        1,
        EnclaveRuntimes::from_env(10).expect("filter enclave pool"),
    );

    StarProxy {
        filter: Arc::new(filter),
        upstream_addr,
    }
}

struct TokenGenerator {
    issue_enclave: EnclaveRuntime,
    user: User,
    counter: u64,
    period: u64,
    rng: OsRng,
}

impl TokenGenerator {
    fn register_user(issue_enclave: &mut EnclaveRuntime, rng: &mut OsRng) -> User {
        let sender = SecureChannelClient::new(issue_enclave.public_key().expect("public key"))
            .expect("secure channel");

        let mut user = User::new(MAX_COUNT_PER_CREDENTIAL, sender);
        let (cred_request, shared_key) = user.request_credential(rng);

        let cred_cipher = issue_enclave
            .issue_cred(&cred_request)
            .expect("issue credential");

        user.receive_credential(&cred_cipher, &shared_key);
        user
    }

    fn new() -> Self {
        let mut issue_enclave = EnclaveRuntime::from_env().expect("issue enclave");
        let mut rng = OsRng;
        let user = Self::register_user(&mut issue_enclave, &mut rng);

        Self {
            issue_enclave,
            user,
            counter: 0,
            period: u64::from_be_bytes(ct_time::current_period()),
            rng,
        }
    }

    fn rotate_credential(&mut self) {
        self.user = Self::register_user(&mut self.issue_enclave, &mut self.rng);
        self.counter = 0;
    }

    fn refresh_period(&mut self) {
        let current_period = u64::from_be_bytes(ct_time::current_period());

        if current_period != self.period {
            self.period = current_period;
            self.rotate_credential();
        }
    }

    fn next_request(&mut self) -> Vec<u8> {
        self.refresh_period();

        if self.counter >= MAX_COUNT_PER_CREDENTIAL {
            self.rotate_credential();
        }

        let token_request = self
            .user
            .request_auth(self.counter, self.period, &mut self.rng)
            .expect("build auth request");

        self.counter += 1;
        token_request
    }
}

fn cert_path() -> String {
    common::manifest_path(TLS_CERT_RELATIVE_PATH)
}

fn key_path() -> String {
    common::manifest_path(TLS_KEY_RELATIVE_PATH)
}

fn spawn_upstream(bind_addr: &str) -> thread::JoinHandle<()> {
    let (_, handle) = spawn_ok_server(bind_addr, "upstream");
    handle
}

fn spawn_proxy(
    proxy_addr: String,
    upstream_addr: String,
    cert_pem_path: String,
    key_pem_path: String,
    latency_tx: mpsc::Sender<u64>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut server = Server::new(None).expect("create pingora server");
        server.bootstrap();

        let inner = build_star_proxy(upstream_addr);

        let mut proxy_svc = http_proxy_service(
            &server.configuration,
            StarLatencyProxy {
                inner: StarProxy {
                    filter: Arc::clone(&inner.filter),
                    upstream_addr: inner.upstream_addr.clone(),
                },
                latency_tx,
                bypass_request_filter: PROXY_ONLY,
            },
        );

        proxy_svc.threads = Some(1);

        let mut tls_settings = TlsSettings::intermediate(&cert_pem_path, &key_pem_path)
            .expect("build pingora TLS settings");

        tls_settings
            .set_groups_list("X25519")
            .expect("set server TLS group");

        if !PROXY_ONLY {
            configure_tls_token_extension(&mut tls_settings, inner.filter)
                .expect("configure STAR TLS extension");
        }

        proxy_svc.add_tls_with_settings(&proxy_addr, None, tls_settings);

        server.add_service(proxy_svc);
        server.run_forever();
    })
}

struct BenchHarness {
    proxy_addr: std::net::SocketAddr,
    star_tls_client: StarTlsClient,
    latency_rx: mpsc::Receiver<u64>,
    token_generator: TokenGenerator,
    _upstream_handle: thread::JoinHandle<()>,
    _proxy_handle: thread::JoinHandle<()>,
}

impl BenchHarness {
    fn start() -> Self {
        let cert_pem_path = cert_path();
        let key_pem_path = key_path();

        let upstream_addr = reserve_local_addr();
        let proxy_addr = reserve_local_addr();

        let upstream_handle = spawn_upstream(&upstream_addr);

        let (latency_tx, latency_rx) = mpsc::channel();

        let proxy_handle = spawn_proxy(
            proxy_addr.clone(),
            upstream_addr,
            cert_pem_path,
            key_pem_path,
            latency_tx,
        );

        let proxy_addr = proxy_addr
            .to_socket_addrs()
            .expect("resolve proxy address")
            .next()
            .expect("proxy address should resolve");

        let mut harness = Self {
            proxy_addr,
            star_tls_client: StarTlsClient::new().expect("build STAR TLS client"),
            latency_rx,
            token_generator: TokenGenerator::new(),
            _upstream_handle: upstream_handle,
            _proxy_handle: proxy_handle,
        };

        harness.wait_until_ready();
        harness.warmup(WARMUP_REQUESTS);
        harness.drain_latencies();

        harness
    }

    fn send_request(&mut self, token_request: &[u8]) {
        let (status, body) = self
            .star_tls_client
            .get(self.proxy_addr, token_request, PROXY_TLS_READ_TIMEOUT)
            .expect("request through proxy");

        assert_eq!(status, 200);
        assert_eq!(body.trim(), "OK");
    }

    fn one_internal_latency_ns(&mut self) -> u64 {
        let token_request = self.token_generator.next_request();

        self.send_request(&token_request);

        self.latency_rx
            .recv_timeout(LATENCY_RECV_TIMEOUT)
            .expect("wait internal proxy latency")
    }

    fn wait_until_ready(&mut self) {
        wait_until_ready(
            "proxy benchmark service",
            STARTUP_RETRIES,
            STARTUP_RETRY_DELAY,
            || {
                let token_request = self.token_generator.next_request();

                match self.star_tls_client.get(
                    self.proxy_addr,
                    &token_request,
                    PROXY_TLS_READ_TIMEOUT,
                ) {
                    Ok((200, body)) if body.trim() == "OK" => {
                        let _ = self.latency_rx.recv_timeout(LATENCY_RECV_TIMEOUT);
                        StartupProbe::Ready
                    }
                    Ok((status, _)) => {
                        StartupProbe::Retry(format!("startup status error: {status}"))
                    }
                    Err(err) => StartupProbe::Retry(format!("startup transport error: {err}")),
                }
            },
        );
    }

    fn warmup(&mut self, count: usize) {
        for _ in 0..count {
            let _ = self.one_internal_latency_ns();
        }
    }

    fn drain_latencies(&self) {
        while self.latency_rx.try_recv().is_ok() {}
    }
}

fn bench_pingora_proxy_basic_latency_tls(c: &mut Criterion) {
    let mut harness = BenchHarness::start();

    let mut group = c.benchmark_group(BENCH_GROUP_NAME);
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(6));
    group.throughput(Throughput::Elements(1));

    group.bench_function(BENCH_FUNCTION_NAME, |b| {
        b.iter_custom(|iters| {
            common::measure_process_cpu_time(iters, || {
                let latency_ns = harness.one_internal_latency_ns();
                std::hint::black_box(latency_ns);
            })
        });
    });

    group.finish();
}

criterion_group!(benches, bench_pingora_proxy_basic_latency_tls);
criterion_main!(benches);