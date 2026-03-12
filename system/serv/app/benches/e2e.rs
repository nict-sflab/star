mod common;

use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use common::http::{get_text, post_text};
use common::retry::{StartupProbe, wait_until_ready};
use common::tls::build_pinned_tls_agent;
use constant_time_utils::time;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use rand_core::OsRng;
use star_core::client::User;
use star_core::secure_channel::client::SecureChannelClient;
use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const MAX_COUNT_PER_CREDENTIAL: u64 = 10;
const STARTUP_RETRIES: u32 = 100;
const STARTUP_RETRY_DELAY: Duration = Duration::from_millis(50);
const TLS_CERT_RELATIVE_PATH: &str = "examples/certs/server.crt";
const PREPARED_REQUEST_BUFFER: usize = 1_024 * 32;
const DEFAULT_CLIENT_THREADS: usize = 24;
const DEFAULT_API_ADDR: &str = "127.0.0.1:8080";
const DEFAULT_PROXY_ADDR: &str = "127.0.0.1:8443";
const E2E_API_ADDR_ENV: &str = "STAR_E2E_API_ADDR";
const E2E_PROXY_ADDR_ENV: &str = "STAR_E2E_PROXY_ADDR";
const E2E_TLS_CERT_PATH_ENV: &str = "STAR_E2E_TLS_CERT_PATH";
const E2E_CLIENT_THREADS_ENV: &str = "STAR_E2E_CLIENT_THREADS";

fn bench_tls_cert_path() -> String {
    common::manifest_path(TLS_CERT_RELATIVE_PATH)
}

fn proxy_url(proxy_addr: &str) -> String {
    if proxy_addr.starts_with("https://") || proxy_addr.starts_with("http://") {
        if proxy_addr.ends_with('/') {
            proxy_addr.to_string()
        } else {
            format!("{proxy_addr}/")
        }
    } else {
        format!("https://{proxy_addr}/")
    }
}

fn wait_for_api_ready(api_addr: &str) {
    let public_key_url = format!("http://{api_addr}/star/public_key");
    wait_until_ready("API", STARTUP_RETRIES, STARTUP_RETRY_DELAY, || {
        match ureq::get(&public_key_url).call() {
            Ok(_) => StartupProbe::Ready,
            Err(ureq::Error::Transport(err)) => StartupProbe::Retry(err.to_string()),
            Err(ureq::Error::Status(code, _)) => StartupProbe::Fatal(format!("status {code}")),
        }
    });
}

fn wait_for_proxy_ready(proxy_addr: &str, tls_cert_path: &str) {
    let ready_agent = build_pinned_tls_agent(tls_cert_path, Duration::from_secs(5));
    let ready_proxy_url = proxy_url(proxy_addr);
    wait_until_ready("proxy", STARTUP_RETRIES, STARTUP_RETRY_DELAY, || {
        match ready_agent
            .get(&ready_proxy_url)
            .set("Connection", "close")
            .call()
        {
            Err(ureq::Error::Status(403, _)) => StartupProbe::Ready,
            Err(ureq::Error::Transport(err)) => StartupProbe::Retry(err.to_string()),
            Err(ureq::Error::Status(code, _)) => StartupProbe::Fatal(format!("status {code}")),
            Ok(_) => StartupProbe::Fatal("expected unauthorized probe request".to_string()),
        }
    });
}

fn env_or_default(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}

fn e2e_client_threads() -> usize {
    match std::env::var(E2E_CLIENT_THREADS_ENV) {
        Ok(raw) => raw.parse::<usize>().unwrap_or_else(|err| {
            panic!("invalid {E2E_CLIENT_THREADS_ENV} value {raw:?}: {err}")
        }),
        Err(_) => DEFAULT_CLIENT_THREADS,
    }
}

struct RemoteTarget {
    api_addr: String,
    proxy_addr: String,
    tls_cert_path: String,
}

impl RemoteTarget {
    fn from_env() -> Self {
        let api_addr = env_or_default(E2E_API_ADDR_ENV, DEFAULT_API_ADDR);
        let proxy_addr = env_or_default(E2E_PROXY_ADDR_ENV, DEFAULT_PROXY_ADDR);
        let tls_cert_path =
            std::env::var(E2E_TLS_CERT_PATH_ENV).unwrap_or_else(|_| bench_tls_cert_path());

        wait_for_api_ready(&api_addr);
        wait_for_proxy_ready(&proxy_addr, &tls_cert_path);

        println!(
            "use remote benchmark server: api={api_addr}, proxy={proxy_addr}, tls_cert_path={tls_cert_path}"
        );

        Self {
            api_addr,
            proxy_addr,
            tls_cert_path,
        }
    }
}

struct BenchClient {
    api_addr: String,
    proxy_url: String,
    user: User,
    counter: u64,
    prepared_period: u64,
    rng: OsRng,
    agent: ureq::Agent,
    prepared_tokens: VecDeque<String>,
}

impl BenchClient {
    fn register_user(api_addr: &str, rng: &mut OsRng) -> User {
        let public_key_url = format!("http://{api_addr}/star/public_key");
        let body = get_text(&public_key_url);
        let public_key = URL_SAFE
            .decode(body.trim())
            .expect("decode public key response");
        let public_key: [u8; 32] = public_key
            .as_slice()
            .try_into()
            .expect("public key should be 32 bytes");

        let client = SecureChannelClient::new(public_key).expect("init secure channel client");
        let mut user = User::new(MAX_COUNT_PER_CREDENTIAL, client);
        let (cred_request, shared_key) = user.request_credential(rng);
        let cred_request_b64 = URL_SAFE.encode(cred_request);

        let register_url = format!("http://{api_addr}/star/register");
        let body = post_text(&register_url, cred_request_b64.as_bytes());
        let cred_cipher = URL_SAFE
            .decode(body.trim())
            .expect("decode credential response");
        user.receive_credential(&cred_cipher, &shared_key);
        user
    }

    fn new(api_addr: &str, proxy_addr: &str, tls_cert_path: &str) -> Self {
        let mut rng = OsRng;
        let user = Self::register_user(api_addr, &mut rng);
        let agent = build_pinned_tls_agent(tls_cert_path, Duration::from_secs(5));

        let mut client = Self {
            api_addr: api_addr.to_string(),
            proxy_url: proxy_url(proxy_addr),
            user,
            counter: 0,
            prepared_period: u64::from_be_bytes(time::current_period()),
            rng,
            agent,
            prepared_tokens: VecDeque::with_capacity(PREPARED_REQUEST_BUFFER),
        };
        client.prepare_tokens(PREPARED_REQUEST_BUFFER);
        client
    }

    fn rotate_credential(&mut self) {
        self.user = Self::register_user(&self.api_addr, &mut self.rng);
        self.counter = 0;
    }

    fn build_token(&mut self) -> String {
        if self.counter >= MAX_COUNT_PER_CREDENTIAL {
            self.rotate_credential();
        }

        let token_request = self
            .user
            .request_auth(self.counter, self.prepared_period, &mut self.rng)
            .expect("build auth request");
        self.counter += 1;
        URL_SAFE.encode(token_request)
    }

    fn refresh_prepared_period(&mut self) {
        let current_period = u64::from_be_bytes(time::current_period());
        if current_period != self.prepared_period {
            self.prepared_period = current_period;
            self.prepared_tokens.clear();
        }
    }

    fn prepare_tokens(&mut self, min_len: usize) {
        self.refresh_prepared_period();
        while self.prepared_tokens.len() < min_len {
            let token = self.build_token();
            self.prepared_tokens.push_back(token);
        }
    }

    fn take_prepared_token(&mut self) -> String {
        self.prepared_tokens
            .pop_front()
            .expect("prepared token queue is empty; prepare tokens before measurement")
    }

    fn request_with_token(&mut self, token: &str) {
        let response = self
            .agent
            .get(&self.proxy_url)
            .set("Connection", "close")
            .set("X-STAR", token)
            .call();

        match response {
            Ok(resp) => {
                let body = resp.into_string().expect("read proxy body");
                assert_eq!(body.trim(), "OK");
            }
            Err(ureq::Error::Status(code, _)) => {
                panic!("unexpected proxy status during benchmark: {code}");
            }
            Err(ureq::Error::Transport(err)) => {
                panic!("proxy request transport error: {err}");
            }
        }
    }
}

enum ClientWorkerCommand {
    Prepare {
        min_tokens: usize,
        done_tx: mpsc::Sender<()>,
    },
    Run(mpsc::Sender<()>),
    Shutdown,
}

struct ClientWorkers {
    senders: Vec<mpsc::Sender<ClientWorkerCommand>>,
    handles: Vec<thread::JoinHandle<()>>,
}

impl ClientWorkers {
    fn start(
        api_addr: &str,
        proxy_addr: &str,
        tls_cert_path: &str,
        worker_count: usize,
    ) -> Self {
        let mut senders = Vec::with_capacity(worker_count);
        let mut handles = Vec::with_capacity(worker_count);

        for _ in 0..worker_count {
            let (tx, rx) = mpsc::channel::<ClientWorkerCommand>();
            let api_addr = api_addr.to_string();
            let proxy_addr = proxy_addr.to_string();
            let tls_cert_path = tls_cert_path.to_string();
            let handle = thread::spawn(move || {
                let mut client = BenchClient::new(&api_addr, &proxy_addr, &tls_cert_path);
                loop {
                    match rx.recv().expect("client worker channel should stay open") {
                        ClientWorkerCommand::Prepare {
                            min_tokens,
                            done_tx,
                        } => {
                            client.prepare_tokens(min_tokens);
                            let _ = done_tx.send(());
                        }
                        ClientWorkerCommand::Run(done_tx) => {
                            let token = client.take_prepared_token();
                            client.request_with_token(&token);
                            let _ = done_tx.send(());
                        }
                        ClientWorkerCommand::Shutdown => break,
                    }
                }
            });

            senders.push(tx);
            handles.push(handle);
        }

        Self { senders, handles }
    }

    fn prepare_for_batches(&self, batches: usize) {
        let mut done_rxs = Vec::with_capacity(self.senders.len());
        for tx in &self.senders {
            let (done_tx, done_rx) = mpsc::channel();
            tx.send(ClientWorkerCommand::Prepare {
                min_tokens: batches,
                done_tx,
            })
            .expect("dispatch token pre-generation");
            done_rxs.push(done_rx);
        }

        for done_rx in done_rxs {
            done_rx.recv().expect("wait for token pre-generation");
        }
    }

    fn run_batch(&self) {
        let mut done_rxs = Vec::with_capacity(self.senders.len());
        for tx in &self.senders {
            let (done_tx, done_rx) = mpsc::channel();
            tx.send(ClientWorkerCommand::Run(done_tx))
                .expect("dispatch benchmark request");
            done_rxs.push(done_rx);
        }

        for done_rx in done_rxs {
            done_rx.recv().expect("wait for benchmark request completion");
        }
    }
}

impl Drop for ClientWorkers {
    fn drop(&mut self) {
        for tx in &self.senders {
            let _ = tx.send(ClientWorkerCommand::Shutdown);
        }
        for handle in self.handles.drain(..) {
            let _ = handle.join();
        }
    }
}

fn bench_star_e2e_throughput(c: &mut Criterion) {
    let target = RemoteTarget::from_env();
    let client_threads = e2e_client_threads();
    let clients = ClientWorkers::start(
        &target.api_addr,
        &target.proxy_addr,
        &target.tls_cert_path,
        client_threads,
    );
    clients.prepare_for_batches(1);
    clients.run_batch();

    let mut group = c.benchmark_group("star_e2e_proxy_throughput");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(8));
    group.throughput(Throughput::Elements(client_threads as u64));
    group.bench_function("authenticated_request", |b| {
        b.iter_custom(|iters| {
            let batches = usize::try_from(iters).expect("iteration count should fit in usize");
            clients.prepare_for_batches(batches);

            let start = Instant::now();
            for _ in 0..batches {
                clients.run_batch();
            }
            start.elapsed()
        });
    });
    group.finish();
}

criterion_group!(benches, bench_star_e2e_throughput);
criterion_main!(benches);
