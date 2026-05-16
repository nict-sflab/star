mod common;

use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use common::http::{get_text, post_text};
use common::retry::{StartupProbe, wait_until_ready};
use common::tls::{StarTlsClient, build_pinned_tls_agent};
use constant_time_utils::time;
use criterion::{
    AxisScale, BenchmarkId, Criterion, PlotConfiguration, Throughput, criterion_group,
    criterion_main,
};
use rand_core::OsRng;
use star_core::client::User;
use star_core::secure_channel::client::SecureChannelClient;
use std::collections::VecDeque;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const MAX_COUNT_PER_CREDENTIAL: u64 = 10;
const STARTUP_RETRIES: u32 = 100;
const STARTUP_RETRY_DELAY: Duration = Duration::from_millis(50);
const TLS_CERT_RELATIVE_PATH: &str = "examples/certs/server.crt";
const PREPARED_REQUEST_BUFFER: usize = 2_048;
const PROXY_READ_TIMEOUT: Duration = Duration::from_secs(5);
const REPORT_PATH: &str = "target/criterion/star_e2e_proxy_throughput/report/index.html";
const DEFAULT_CLIENT_THREADS: usize = 24;
const DEFAULT_API_ADDR: &str = "127.0.0.1:8080";
const DEFAULT_PROXY_ADDR: &str = "127.0.0.1:8443";
const E2E_API_ADDR_ENV: &str = "STAR_E2E_API_ADDR";
const E2E_PROXY_ADDR_ENV: &str = "STAR_E2E_PROXY_ADDR";
const E2E_TLS_CERT_PATH_ENV: &str = "STAR_E2E_TLS_CERT_PATH";
const E2E_CLIENT_THREADS_ENV: &str = "STAR_E2E_CLIENT_THREADS";
const E2E_CLIENT_SWEEP_ENV: &str = "STAR_E2E_CLIENT_SWEEP";

fn current_star_period() -> u64 {
    u64::from_be_bytes(time::current_period())
}

fn bench_tls_cert_path() -> String {
    common::manifest_path(TLS_CERT_RELATIVE_PATH)
}

fn proxy_url_from_addr(proxy_addr: &str) -> String {
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

fn authority_from_urlish(addr: &str) -> String {
    let addr = addr
        .strip_prefix("https://")
        .or_else(|| addr.strip_prefix("http://"))
        .unwrap_or(addr);
    addr.trim_end_matches('/').to_string()
}

fn resolve_proxy_addr(authority: &str) -> SocketAddr {
    authority
        .to_socket_addrs()
        .expect("resolve proxy authority")
        .next()
        .expect("proxy authority should resolve")
}

fn api_url(api_addr: &str, path: &str) -> String {
    format!("http://{api_addr}{path}")
}

fn wait_for_api_ready(public_key_url: &str) {
    wait_until_ready(
        "API",
        STARTUP_RETRIES,
        STARTUP_RETRY_DELAY,
        || match ureq::get(public_key_url).call() {
            Ok(_) => StartupProbe::Ready,
            Err(ureq::Error::Transport(err)) => StartupProbe::Retry(err.to_string()),
            Err(ureq::Error::Status(code, _)) => StartupProbe::Fatal(format!("status {code}")),
        },
    );
}

fn response_snippet(response: ureq::Response) -> String {
    const LIMIT: usize = 300;

    match response.into_string() {
        Ok(body) => {
            let mut snippet: String = body.chars().take(LIMIT).collect();
            if body.chars().count() > LIMIT {
                snippet.push_str("...");
            }
            snippet
        }
        Err(err) => format!("<failed to read response body: {err}>"),
    }
}

fn is_expected_unauthorized_tls_error(message: &str) -> bool {
    message.contains("illegal parameter")
        || message.contains("DecodeError")
        || message.contains("decode error")
        || message.contains("tls connection init failed")
}

fn is_invalid_token_tls_error(message: &str) -> bool {
    message.contains("alert decode error")
        || message.contains("SSL alert number 50")
        || message.contains("bad extension")
        || message.contains("custom_ext_parse")
        || message.contains("decode error")
        || message.contains("DecodeError")
        || message.contains("tls connect failed")
        || message.contains("TLSHandshakeFailure")
}

fn wait_for_proxy_ready(proxy_url: &str, tls_cert_path: &str) {
    let ready_agent = build_pinned_tls_agent(tls_cert_path, PROXY_READ_TIMEOUT);

    wait_until_ready(
        "proxy",
        STARTUP_RETRIES,
        STARTUP_RETRY_DELAY,
        || match ready_agent.get(proxy_url).set("Connection", "close").call() {
            Err(ureq::Error::Status(401 | 403, _)) => StartupProbe::Ready,

            Err(ureq::Error::Transport(err)) => {
                let message = err.to_string();

                if is_expected_unauthorized_tls_error(&message) {
                    StartupProbe::Ready
                } else {
                    StartupProbe::Retry(message)
                }
            }

            Err(ureq::Error::Status(code, response)) => StartupProbe::Fatal(format!(
                "expected unauthorized probe request to {proxy_url}, got status {code}, body {:?}",
                response_snippet(response),
            )),

            Ok(response) => {
                let status = response.status();
                StartupProbe::Fatal(format!(
                    "expected unauthorized probe request to {proxy_url}, but request succeeded with status {status}, body {:?}",
                    response_snippet(response),
                ))
            }
        },
    );
}

fn env_or_default(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}

fn parse_usize_env(name: &str, default: usize) -> usize {
    match std::env::var(name) {
        Ok(raw) => raw
            .parse::<usize>()
            .unwrap_or_else(|err| panic!("invalid {name} value {raw:?}: {err}")),
        Err(_) => default,
    }
}

fn default_client_sweep() -> Vec<usize> {
    let baseline = parse_usize_env(E2E_CLIENT_THREADS_ENV, DEFAULT_CLIENT_THREADS).max(1);
    let mut sweep = vec![1usize];
    let mut next = 2usize;

    while next < baseline {
        sweep.push(next);
        next = next.saturating_mul(2);
    }

    if !sweep.contains(&baseline) {
        sweep.push(baseline);
    }

    let extended = baseline.saturating_mul(2);
    if !sweep.contains(&extended) {
        sweep.push(extended);
    }

    sweep
}

fn normalize_client_sweep(mut sweep: Vec<usize>) -> Vec<usize> {
    if sweep.is_empty() {
        panic!("{E2E_CLIENT_SWEEP_ENV} resolved to an empty client list");
    }

    if let Some(invalid) = sweep.iter().find(|&&value| value == 0) {
        panic!("{E2E_CLIENT_SWEEP_ENV} entries must be > 0, got {invalid}");
    }

    sweep.sort_unstable();
    sweep.dedup();
    sweep
}

fn parse_client_sweep(raw: &str) -> Vec<usize> {
    raw.split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            entry.parse::<usize>().unwrap_or_else(|err| {
                panic!("invalid {E2E_CLIENT_SWEEP_ENV} item {entry:?}: {err}")
            })
        })
        .collect()
}

fn e2e_client_sweep() -> Vec<usize> {
    let sweep = match std::env::var(E2E_CLIENT_SWEEP_ENV) {
        Ok(raw) => parse_client_sweep(&raw),
        Err(_) => default_client_sweep(),
    };

    normalize_client_sweep(sweep)
}

#[derive(Clone)]
struct RemoteTarget {
    tls_cert_path: String,
    public_key_url: String,
    register_url: String,
    proxy_addr: SocketAddr,
}

impl RemoteTarget {
    fn from_env() -> Self {
        let api_addr = env_or_default(E2E_API_ADDR_ENV, DEFAULT_API_ADDR);
        let proxy_addr = env_or_default(E2E_PROXY_ADDR_ENV, DEFAULT_PROXY_ADDR);
        let tls_cert_path =
            std::env::var(E2E_TLS_CERT_PATH_ENV).unwrap_or_else(|_| bench_tls_cert_path());
        let public_key_url = api_url(&api_addr, "/star/public_key");
        let register_url = api_url(&api_addr, "/star/register");
        let proxy_url = proxy_url_from_addr(&proxy_addr);
        let proxy_authority = authority_from_urlish(&proxy_url);
        let proxy_addr = resolve_proxy_addr(&proxy_authority);

        wait_for_api_ready(&public_key_url);
        wait_for_proxy_ready(&proxy_url, &tls_cert_path);

        println!(
            "use remote benchmark server: api={api_addr}, proxy={proxy_addr}, tls_cert_path={tls_cert_path}"
        );

        Self {
            tls_cert_path,
            public_key_url,
            register_url,
            proxy_addr,
        }
    }
}

struct PreparedRequest {
    period: u64,
    token: Vec<u8>,
}

#[derive(Debug)]
enum ProxyRequestError {
    Status(u16),
    Transport(String),
}

struct BenchClient {
    target: RemoteTarget,
    tls_client: StarTlsClient,
    user: User,
    counter: u64,
    prepared_period: u64,
    rng: OsRng,
    prepared_requests: VecDeque<PreparedRequest>,
}

impl BenchClient {
    fn register_user(target: &RemoteTarget, rng: &mut OsRng) -> User {
        let body = get_text(&target.public_key_url);
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

        let body = post_text(&target.register_url, cred_request_b64.as_bytes());
        let cred_cipher = URL_SAFE
            .decode(body.trim())
            .expect("decode credential response");
        user.receive_credential(&cred_cipher, &shared_key);
        user
    }

    fn new(target: RemoteTarget) -> Self {
        let mut rng = OsRng;
        let user = Self::register_user(&target, &mut rng);
        let tls_client = StarTlsClient::new().expect("init STAR TLS client");

        Self {
            target,
            tls_client,
            user,
            counter: 0,
            prepared_period: current_star_period(),
            rng,
            prepared_requests: VecDeque::with_capacity(PREPARED_REQUEST_BUFFER),
        }
    }

    fn rotate_credential(&mut self) {
        self.user = Self::register_user(&self.target, &mut self.rng);
        self.counter = 0;
    }

    fn refresh_prepared_period(&mut self) {
        let current_period = current_star_period();

        if current_period != self.prepared_period {
            self.prepared_period = current_period;
            self.prepared_requests.clear();
        }
    }

    fn build_request(&mut self) -> PreparedRequest {
        self.refresh_prepared_period();

        if self.counter >= MAX_COUNT_PER_CREDENTIAL {
            self.rotate_credential();

            // Credential rotation performs API I/O and may cross a STAR period
            // boundary. Re-check the period before creating the auth token.
            self.refresh_prepared_period();
        }

        let period = self.prepared_period;
        let token = self
            .user
            .request_auth(self.counter, period, &mut self.rng)
            .expect("build auth request");

        self.counter += 1;

        PreparedRequest { period, token }
    }

    fn prepare_tokens(&mut self, min_len: usize) {
        self.refresh_prepared_period();

        while self.prepared_requests.len() < min_len {
            self.refresh_prepared_period();

            let request = self.build_request();

            if request.period == current_star_period() {
                self.prepared_requests.push_back(request);
            } else {
                // The period rolled exactly while building the token.
                // Drop this token and retry in the next loop iteration.
                self.refresh_prepared_period();
            }
        }
    }

    fn take_prepared_request(&mut self) -> PreparedRequest {
        loop {
            self.refresh_prepared_period();

            let current_period = current_star_period();

            while let Some(request) = self.prepared_requests.pop_front() {
                if request.period == current_period {
                    return request;
                }

                // Drop stale prepared tokens. They would be rejected during
                // ClientHello parsing and surface as TLS decode_error.
            }

            let request = self.build_request();

            if request.period == current_star_period() {
                return request;
            }

            self.refresh_prepared_period();
        }
    }

    fn build_fresh_request_for_retry(&mut self) -> PreparedRequest {
        self.prepared_requests.clear();
        self.refresh_prepared_period();

        loop {
            let request = self.build_request();

            if request.period == current_star_period() {
                return request;
            }

            self.refresh_prepared_period();
        }
    }

    fn request_with_token(&mut self, token_request: &[u8]) -> Result<(), ProxyRequestError> {
        match self
            .tls_client
            .get(self.target.proxy_addr, token_request, PROXY_READ_TIMEOUT)
        {
            Ok((status, body)) => {
                if status != 200 {
                    return Err(ProxyRequestError::Status(status));
                }

                assert_eq!(body.trim(), "OK");
                Ok(())
            }
            Err(err) => Err(ProxyRequestError::Transport(err)),
        }
    }

    fn build_and_consume_replay_token(&mut self) -> Vec<u8> {
        loop {
            let request = self.build_fresh_request_for_retry();
            let token = request.token.clone();

            match self.request_with_token(&token) {
                Ok(()) => return token,

                Err(ProxyRequestError::Status(401)) => {
                    // The token may have expired just before server-side
                    // validation. Build a fresh token and try priming again.
                }

                Err(ProxyRequestError::Transport(message))
                    if is_invalid_token_tls_error(&message) =>
                {
                    // Expired tokens can fail during ClientHello parsing.
                    // Retry so the measured phase starts with a token that was
                    // definitely accepted once.
                }

                Err(ProxyRequestError::Status(code)) => {
                    panic!("unexpected proxy status while priming replay token: {code}");
                }

                Err(ProxyRequestError::Transport(message)) => {
                    panic!("proxy request transport error while priming replay token: {message}");
                }
            }
        }
    }

    fn expect_reused_token_rejected(&mut self, token_request: &[u8]) {
        match self
            .tls_client
            .get(self.target.proxy_addr, token_request, PROXY_READ_TIMEOUT)
        {
            Ok((200, body)) => {
                panic!(
                    "reused token was accepted unexpectedly with body {:?}",
                    body.trim()
                );
            }

            Ok((401 | 403, _)) => {}

            Ok((status, body)) => {
                panic!(
                    "unexpected proxy status for reused-token rejection: {status}, body {:?}",
                    body.trim()
                );
            }

            Err(message) if is_invalid_token_tls_error(&message) => {}

            Err(message) => {
                panic!("unexpected transport error for reused-token rejection: {message}");
            }
        }
    }

    fn send_prepared_request(&mut self) {
        let request = self.take_prepared_request();

        match self.request_with_token(&request.token) {
            Ok(()) => {}

            Err(ProxyRequestError::Status(401)) => {
                // If the token expires between the final local period check
                // and server-side ClientHello validation, retry once.
                let retry_request = self.build_fresh_request_for_retry();

                if let Err(err) = self.request_with_token(&retry_request.token) {
                    panic!("unexpected proxy error during benchmark after 401 retry: {err:?}");
                }
            }

            Err(ProxyRequestError::Transport(message))
                if is_invalid_token_tls_error(&message) =>
            {
                // STAR token validation happens during TLS ClientHello parsing.
                // Expired tokens may therefore appear as TLS handshake failures,
                // not HTTP 401/403 responses.
                let retry_request = self.build_fresh_request_for_retry();

                if let Err(err) = self.request_with_token(&retry_request.token) {
                    panic!(
                        "unexpected proxy error during benchmark after TLS invalid-token retry: {err:?}"
                    );
                }
            }

            Err(ProxyRequestError::Status(code)) => {
                panic!("unexpected proxy status during benchmark: {code}");
            }

            Err(ProxyRequestError::Transport(message)) => {
                panic!("proxy request transport error: {message}");
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
    PrepareReplayRejection(mpsc::Sender<()>),
    RunReplayRejection(mpsc::Sender<()>),
    Shutdown,
}

fn run_client_worker(target: RemoteTarget, rx: mpsc::Receiver<ClientWorkerCommand>) {
    let mut client = BenchClient::new(target);
    let mut replay_token: Option<Vec<u8>> = None;

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
                client.send_prepared_request();
                let _ = done_tx.send(());
            }
            ClientWorkerCommand::PrepareReplayRejection(done_tx) => {
                replay_token = Some(client.build_and_consume_replay_token());
                let _ = done_tx.send(());
            }
            ClientWorkerCommand::RunReplayRejection(done_tx) => {
                let token = replay_token
                    .as_deref()
                    .expect("replay token should be primed before rejection run");
                client.expect_reused_token_rejected(token);
                let _ = done_tx.send(());
            }
            ClientWorkerCommand::Shutdown => break,
        }
    }
}

struct ClientWorkers {
    senders: Vec<mpsc::Sender<ClientWorkerCommand>>,
    handles: Vec<thread::JoinHandle<()>>,
}

impl ClientWorkers {
    fn start(target: &RemoteTarget, worker_count: usize) -> Self {
        let mut senders = Vec::with_capacity(worker_count);
        let mut handles = Vec::with_capacity(worker_count);

        for _ in 0..worker_count {
            let (tx, rx) = mpsc::channel::<ClientWorkerCommand>();
            let target = target.clone();
            let handle = thread::spawn(move || run_client_worker(target, rx));

            senders.push(tx);
            handles.push(handle);
        }

        Self { senders, handles }
    }

    fn dispatch_and_wait<F>(&self, mut build_command: F, dispatch_message: &str, wait_message: &str)
    where
        F: FnMut(mpsc::Sender<()>) -> ClientWorkerCommand,
    {
        let mut done_rxs = Vec::with_capacity(self.senders.len());

        for tx in &self.senders {
            let (done_tx, done_rx) = mpsc::channel();
            tx.send(build_command(done_tx)).expect(dispatch_message);
            done_rxs.push(done_rx);
        }

        for done_rx in done_rxs {
            done_rx.recv().expect(wait_message);
        }
    }

    fn prepare_for_batches(&self, batches: usize) {
        self.dispatch_and_wait(
            |done_tx| ClientWorkerCommand::Prepare {
                min_tokens: batches,
                done_tx,
            },
            "dispatch token pre-generation",
            "wait for token pre-generation",
        );
    }

    fn run_batch(&self) {
        self.dispatch_and_wait(
            ClientWorkerCommand::Run,
            "dispatch benchmark request",
            "wait for benchmark request completion",
        );
    }

    fn prepare_replay_rejection(&self) {
        self.dispatch_and_wait(
            ClientWorkerCommand::PrepareReplayRejection,
            "dispatch replay-token priming request",
            "wait for replay-token priming request",
        );
    }

    fn run_replay_rejection_batch(&self) {
        self.dispatch_and_wait(
            ClientWorkerCommand::RunReplayRejection,
            "dispatch replay-token rejection request",
            "wait for replay-token rejection request completion",
        );
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


enum HarnessWorkerCommand {
    Run(mpsc::Sender<()>),
    Shutdown,
}

fn run_harness_worker(rx: mpsc::Receiver<HarnessWorkerCommand>) {
    loop {
        match rx.recv().expect("harness worker channel should stay open") {
            HarnessWorkerCommand::Run(done_tx) => {
                let _ = done_tx.send(());
            }
            HarnessWorkerCommand::Shutdown => break,
        }
    }
}

struct HarnessWorkers {
    senders: Vec<mpsc::Sender<HarnessWorkerCommand>>,
    handles: Vec<thread::JoinHandle<()>>,
}

impl HarnessWorkers {
    fn start(worker_count: usize) -> Self {
        let mut senders = Vec::with_capacity(worker_count);
        let mut handles = Vec::with_capacity(worker_count);

        for _ in 0..worker_count {
            let (tx, rx) = mpsc::channel::<HarnessWorkerCommand>();
            let handle = thread::spawn(move || run_harness_worker(rx));

            senders.push(tx);
            handles.push(handle);
        }

        Self { senders, handles }
    }

    fn run_batch(&self) {
        let mut done_rxs = Vec::with_capacity(self.senders.len());

        for tx in &self.senders {
            let (done_tx, done_rx) = mpsc::channel();
            tx.send(HarnessWorkerCommand::Run(done_tx))
                .expect("dispatch no-op harness command");
            done_rxs.push(done_rx);
        }

        for done_rx in done_rxs {
            done_rx.recv().expect("wait for no-op harness completion");
        }
    }
}

impl Drop for HarnessWorkers {
    fn drop(&mut self) {
        for tx in &self.senders {
            let _ = tx.send(HarnessWorkerCommand::Shutdown);
        }

        for handle in self.handles.drain(..) {
            let _ = handle.join();
        }
    }
}

fn timed_harness_batches(workers: &HarnessWorkers, batches: usize) -> Duration {
    let start = Instant::now();

    for _ in 0..batches {
        workers.run_batch();
    }

    start.elapsed()
}


struct LocalHarnessChannels {
    pairs: Vec<(
        mpsc::Sender<HarnessWorkerCommand>,
        mpsc::Receiver<HarnessWorkerCommand>,
    )>,
}

impl LocalHarnessChannels {
    fn start(worker_count: usize) -> Self {
        let mut pairs = Vec::with_capacity(worker_count);

        for _ in 0..worker_count {
            pairs.push(mpsc::channel::<HarnessWorkerCommand>());
        }

        Self { pairs }
    }

    fn run_batch(&self) {
        let mut done_rxs = Vec::with_capacity(self.pairs.len());

        for (tx, _) in &self.pairs {
            let (done_tx, done_rx) = mpsc::channel();
            tx.send(HarnessWorkerCommand::Run(done_tx))
                .expect("dispatch same-thread harness command");
            done_rxs.push(done_rx);
        }

        for (_, rx) in &self.pairs {
            match rx.recv().expect("receive same-thread harness command") {
                HarnessWorkerCommand::Run(done_tx) => {
                    let _ = done_tx.send(());
                }
                HarnessWorkerCommand::Shutdown => {
                    panic!("unexpected same-thread harness shutdown command");
                }
            }
        }

        for done_rx in done_rxs {
            done_rx
                .recv()
                .expect("wait for same-thread harness completion");
        }
    }
}

fn timed_local_harness_batches(channels: &LocalHarnessChannels, batches: usize) -> Duration {
    let start = Instant::now();

    for _ in 0..batches {
        channels.run_batch();
    }

    start.elapsed()
}

fn timed_request_batches(clients: &ClientWorkers, batches: usize) -> Duration {
    let mut remaining = batches;
    let mut elapsed = Duration::ZERO;

    while remaining > 0 {
        let chunk = remaining.min(PREPARED_REQUEST_BUFFER);
        clients.prepare_for_batches(chunk);

        let start = Instant::now();
        for _ in 0..chunk {
            clients.run_batch();
        }
        elapsed += start.elapsed();
        remaining -= chunk;
    }

    elapsed
}

fn timed_replay_rejection_batches(clients: &ClientWorkers, batches: usize) -> Duration {
    if batches == 0 {
        return Duration::ZERO;
    }

    clients.prepare_replay_rejection();

    let start = Instant::now();
    for _ in 0..batches {
        clients.run_replay_rejection_batch();
    }
    start.elapsed()
}

fn bench_client_harness_overhead(c: &mut Criterion) {
    let client_sweep = e2e_client_sweep();

    let mut group = c.benchmark_group("star_e2e_client_harness_overhead");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(5));
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Linear));

    for client_threads in client_sweep {
        let local_channels = LocalHarnessChannels::start(client_threads);
        let workers = HarnessWorkers::start(client_threads);

        // Warm the worker threads once outside the measured section.
        workers.run_batch();

        group.throughput(Throughput::Elements(client_threads as u64));
        group.bench_with_input(
            BenchmarkId::new("mpsc_dispatch_wait_same_thread", client_threads),
            &client_threads,
            |b, &_client_threads| {
                b.iter_custom(|iters| {
                    let batches =
                        usize::try_from(iters).expect("iteration count should fit in usize");
                    timed_local_harness_batches(&local_channels, batches)
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("mpsc_dispatch_wait_noop_worker", client_threads),
            &client_threads,
            |b, &_client_threads| {
                b.iter_custom(|iters| {
                    let batches =
                        usize::try_from(iters).expect("iteration count should fit in usize");
                    timed_harness_batches(&workers, batches)
                });
            },
        );
    }

    group.finish();
}

fn bench_star_e2e_throughput(c: &mut Criterion) {
    let target = RemoteTarget::from_env();
    let client_sweep = e2e_client_sweep();

    println!("criterion report path: {REPORT_PATH}");

    let mut group = c.benchmark_group("star_e2e_proxy_throughput");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(8));
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Linear));

    for client_threads in client_sweep {
        let clients = ClientWorkers::start(&target, client_threads);

        clients.prepare_for_batches(1);
        clients.run_batch();

        group.throughput(Throughput::Elements(client_threads as u64));
        group.bench_with_input(
            BenchmarkId::new("authenticated_request", client_threads),
            &client_threads,
            |b, &_client_threads| {
                b.iter_custom(|iters| {
                    let batches =
                        usize::try_from(iters).expect("iteration count should fit in usize");
                    timed_request_batches(&clients, batches)
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("replayed_token_rejection", client_threads),
            &client_threads,
            |b, &_client_threads| {
                b.iter_custom(|iters| {
                    let batches =
                        usize::try_from(iters).expect("iteration count should fit in usize");
                    timed_replay_rejection_batches(&clients, batches)
                });
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_client_harness_overhead, bench_star_e2e_throughput);
criterion_main!(benches);