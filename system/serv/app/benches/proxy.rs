mod common;

use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use common::latency_proxy::StarLatencyProxy;
use common::retry::{StartupProbe, wait_until_ready};
use common::tls::{build_pinned_tls_agent, ensure_rustls_ring_provider};
use constant_time_utils::time as ct_time;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use pingora::listeners::tls::TlsSettings;
use pingora::server::Server;
use pingora_proxy::http_proxy_service;
use rand_core::{OsRng, RngCore};
use simple_filter_box::hashmap::HashMapFilterBox;
use star_app::core::{ENCLAVE, EnclaveRuntime};
use star_app::filter::StarFilter;
use star_app::proxy::StarProxy;
use star_core::{client::User, secure_channel::client::SecureChannelClient};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;

const STARTUP_RETRIES: u32 = 100;
const STARTUP_RETRY_DELAY: Duration = Duration::from_millis(50);
const WARMUP_REQUESTS: usize = 128;
const TLS_CERT_RELATIVE_PATH: &str = "examples/certs/server.crt";
const TLS_KEY_RELATIVE_PATH: &str = "examples/certs/server.key";
const ENCLAVE_MAX_COUNT: u32 = 1_000_000;
const MAX_COUNT_PER_CREDENTIAL: u64 = ENCLAVE_MAX_COUNT as u64;

const PROXY_ONLY: bool = false;

type BenchFilter = StarFilter<HashMapFilterBox>;

fn build_star_proxy(upstream_addr: String) -> StarProxy<HashMapFilterBox> {
    let mut filters: HashMap<u32, Mutex<HashMapFilterBox>> = HashMap::new();
    filters.insert(0, Mutex::new(HashMapFilterBox::new()));
    let filter = BenchFilter::new(filters, 1);
    StarProxy {
        filter,
        upstream_addr,
    }
}

fn init_enclave() {
    ENCLAVE.get_or_init(|| {
        let mut rng = OsRng;
        let mut cipher_receiver = [0u8; 32];
        let mut sk = [0u8; 32];
        rng.fill_bytes(&mut cipher_receiver);
        rng.fill_bytes(&mut sk);
        EnclaveRuntime::stdio(ENCLAVE_MAX_COUNT).expect("init enclave")
    });
}

struct TokenGenerator {
    user: User,
    counter: u64,
    period: u64,
    rng: OsRng,
}

impl TokenGenerator {
    fn register_user(rng: &mut OsRng) -> User {
        let enclave = ENCLAVE.get().expect("enclave initialized");
        let sender = SecureChannelClient::new(enclave.public_key()).expect("secure channel");
        let mut user = User::new(MAX_COUNT_PER_CREDENTIAL, sender);
        let (cred_request, shared_key) = user.request_credential(rng);
        let cred_cipher = enclave.issue_cred(&cred_request, rng);
        user.receive_credential(&cred_cipher, &shared_key);
        user
    }

    fn new() -> Self {
        init_enclave();
        let mut rng = OsRng;
        let user = Self::register_user(&mut rng);
        Self {
            user,
            counter: 0,
            period: u64::from_be_bytes(ct_time::current_period()),
            rng,
        }
    }

    fn rotate_credential(&mut self) {
        self.user = Self::register_user(&mut self.rng);
        self.counter = 0;
    }

    fn next_token(&mut self) -> String {
        let current_period = u64::from_be_bytes(ct_time::current_period());
        if current_period != self.period {
            self.period = current_period;
            self.rotate_credential();
        }
        if self.counter >= MAX_COUNT_PER_CREDENTIAL {
            self.rotate_credential();
        }

        let token_request = self
            .user
            .request_auth(self.counter, self.period, &mut self.rng)
            .expect("build auth request");
        self.counter += 1;
        URL_SAFE.encode(token_request)
    }
}

fn cert_path() -> String {
    common::manifest_path(TLS_CERT_RELATIVE_PATH)
}

fn key_path() -> String {
    common::manifest_path(TLS_KEY_RELATIVE_PATH)
}

fn find_headers_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|window| window == b"\r\n\r\n")
}

fn handle_upstream_connection(mut stream: TcpStream) {
    let _ = stream.set_nodelay(true);
    let mut read_buf = [0u8; 4096];
    let mut pending = Vec::with_capacity(8192);

    loop {
        let read = match stream.read(&mut read_buf) {
            Ok(0) => return,
            Ok(n) => n,
            Err(_) => return,
        };
        pending.extend_from_slice(&read_buf[..read]);

        while let Some(end) = find_headers_end(&pending) {
            let _ = pending.split_off(end + 4);
            let response = b"HTTP/1.1 200 OK\r\ncontent-length: 2\r\nconnection: close\r\n\r\nOK";
            if stream.write_all(response).is_err() {
                return;
            }
            if stream.flush().is_err() {
                return;
            }
            return;
        }
    }
}

fn spawn_upstream(bind_addr: &str) -> thread::JoinHandle<()> {
    let listener = TcpListener::bind(bind_addr).expect("bind benchmark upstream");
    thread::spawn(move || {
        for incoming in listener.incoming() {
            match incoming {
                Ok(stream) => handle_upstream_connection(stream),
                Err(err) => eprintln!("upstream accept error: {err}"),
            }
        }
    })
}

fn spawn_proxy(
    proxy_addr: String,
    upstream_addr: String,
    cert_pem_path: String,
    key_pem_path: String,
    latency_tx: mpsc::Sender<u64>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        ensure_rustls_ring_provider();
        let mut server = Server::new(None).expect("create pingora server");
        server.bootstrap();

        let mut proxy_svc = http_proxy_service(
            &server.configuration,
            StarLatencyProxy {
                inner: build_star_proxy(upstream_addr),
                latency_tx,
                bypass_request_filter: PROXY_ONLY,
            },
        );
        proxy_svc.threads = Some(1);

        let tls_settings = TlsSettings::intermediate(&cert_pem_path, &key_pem_path)
            .expect("build pingora TLS settings");
        proxy_svc.add_tls_with_settings(&proxy_addr, None, tls_settings);

        server.add_service(proxy_svc);
        server.run_forever();
    })
}

fn reserve_local_addr() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("reserve local port");
    let addr = listener.local_addr().expect("read reserved local port");
    drop(listener);
    addr.to_string()
}

struct BenchHarness {
    proxy_url: String,
    agent: ureq::Agent,
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
            cert_pem_path.clone(),
            key_pem_path,
            latency_tx,
        );

        let proxy_url = format!("https://{proxy_addr}/");
        let agent = build_pinned_tls_agent(&cert_pem_path, Duration::from_secs(2));

        let mut harness = Self {
            proxy_url,
            agent,
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

    fn send_request(&self, token: &str) {
        let response = self
            .agent
            .get(&self.proxy_url)
            .set("Connection", "close")
            .set("X-STAR", token)
            .call()
            .expect("request through proxy");
        let body = response.into_string().expect("read proxy body");
        assert_eq!(body.trim(), "OK");
    }

    fn one_internal_latency_ns(&mut self) -> u64 {
        let token = self.token_generator.next_token();
        self.send_request(&token);
        self.latency_rx
            .recv_timeout(Duration::from_secs(2))
            .expect("wait internal proxy latency")
    }

    fn wait_until_ready(&mut self) {
        wait_until_ready(
            "proxy benchmark service",
            STARTUP_RETRIES,
            STARTUP_RETRY_DELAY,
            || {
                let token = self.token_generator.next_token();
                let result = self
                    .agent
                    .get(&self.proxy_url)
                    .set("Connection", "close")
                    .set("X-STAR", &token)
                    .call();
                match result {
                    Ok(response) => {
                        let body = response.into_string().expect("read startup body");
                        if body.trim() == "OK" {
                            let _ = self.latency_rx.recv_timeout(Duration::from_secs(2));
                            StartupProbe::Ready
                        } else {
                            StartupProbe::Retry(format!("unexpected startup body: {body:?}"))
                        }
                    }
                    Err(ureq::Error::Status(code, _)) => {
                        StartupProbe::Retry(format!("startup status error: {code}"))
                    }
                    Err(ureq::Error::Transport(err)) => {
                        StartupProbe::Retry(format!("startup transport error: {err}"))
                    }
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

    let mut group = c.benchmark_group("pingora_proxy_basic_latency_tls");
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(6));
    group.throughput(Throughput::Elements(1));
    group.bench_function("single_thread_tls_process_cpu", |b| {
        b.iter_custom(|iters| {
            common::measure_process_cpu_time(iters, || {
                let _ = harness.one_internal_latency_ns();
            })
        });
    });
    group.finish();
}

criterion_group!(benches, bench_pingora_proxy_basic_latency_tls);
criterion_main!(benches);
