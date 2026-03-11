use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use constant_time_utils::time;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use pingora::server::{RunArgs, ShutdownSignal, ShutdownSignalWatch};
use rand_core::OsRng;
use star_app::{all_prefix, build_server};
use star_core::client::User;
use star_core::secure_channel::client::SecureChannelClient;
use std::collections::VecDeque;
use std::fs;
use std::net::TcpListener;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;
use tiny_http::{Response, Server as TinyServer};
use tokio::sync::{Mutex as AsyncMutex, oneshot};
use ureq::rustls::client::danger::{
    HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier,
};
use ureq::rustls::pki_types::pem::PemObject;
use ureq::rustls::pki_types::{CertificateDer, ServerName, UnixTime};

const MAX_COUNT_PER_CREDENTIAL: u64 = 10;
const STARTUP_RETRIES: u32 = 100;
const STARTUP_RETRY_DELAY: Duration = Duration::from_millis(50);
const TLS_CERT_RELATIVE_PATH: &str = "examples/certs/server.crt";
const TLS_KEY_RELATIVE_PATH: &str = "examples/certs/server.key";
const PREPARED_REQUEST_BUFFER: usize = 1_024 * 16;
const API_SERVER_THREADS: usize = 1;
const PROXY_SERVER_THREADS: usize = 2;
const CLIENT_THREADS: usize = 24;

#[derive(Debug)]
struct PinnedServerCertVerifier {
    expected_leaf_der: Vec<u8>,
}

impl ServerCertVerifier for PinnedServerCertVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, ureq::rustls::Error> {
        if end_entity.as_ref() == self.expected_leaf_der.as_slice() {
            Ok(ServerCertVerified::assertion())
        } else {
            Err(ureq::rustls::Error::General(
                "unexpected TLS server certificate".to_string(),
            ))
        }
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &ureq::rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, ureq::rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &ureq::rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, ureq::rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<ureq::rustls::SignatureScheme> {
        vec![
            ureq::rustls::SignatureScheme::RSA_PKCS1_SHA1,
            ureq::rustls::SignatureScheme::RSA_PKCS1_SHA256,
            ureq::rustls::SignatureScheme::RSA_PKCS1_SHA384,
            ureq::rustls::SignatureScheme::RSA_PKCS1_SHA512,
            ureq::rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            ureq::rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            ureq::rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            ureq::rustls::SignatureScheme::RSA_PSS_SHA256,
            ureq::rustls::SignatureScheme::RSA_PSS_SHA384,
            ureq::rustls::SignatureScheme::RSA_PSS_SHA512,
            ureq::rustls::SignatureScheme::ED25519,
            ureq::rustls::SignatureScheme::ED448,
        ]
    }
}

fn load_first_cert_der(cert_path: &str) -> Vec<u8> {
    let cert_pem = fs::read(cert_path).expect("read TLS cert PEM");
    let mut certs = ureq::rustls::pki_types::CertificateDer::pem_slice_iter(&cert_pem)
        .collect::<Result<Vec<_>, _>>()
        .expect("parse TLS cert PEM");
    certs
        .drain(..)
        .next()
        .expect("TLS cert PEM did not contain any certificates")
        .as_ref()
        .to_vec()
}

struct TestShutdown {
    rx: AsyncMutex<oneshot::Receiver<()>>,
}

#[async_trait]
impl ShutdownSignalWatch for TestShutdown {
    async fn recv(&self) -> ShutdownSignal {
        let mut rx = self.rx.lock().await;
        let _ = (&mut *rx).await;
        ShutdownSignal::FastShutdown
    }
}

fn pick_addr() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    listener
        .local_addr()
        .expect("read local addr")
        .to_string()
}

fn retry_request<T, F>(mut op: F) -> T
where
    F: FnMut() -> Result<T, ureq::Error>,
{
    match op() {
        Ok(value) => value,
        Err(ureq::Error::Transport(err)) => panic!("http request failed: {err}"),
        Err(ureq::Error::Status(code, _)) => panic!("http request failed: status {code}"),
    }
}

fn get_text_retry(url: &str) -> String {
    retry_request(|| Ok(ureq::get(url).call()?.into_string()?))
}

fn post_text_retry(url: &str, body: &[u8]) -> String {
    retry_request(|| Ok(ureq::post(url).send_bytes(body)?.into_string()?))
}

fn spawn_upstream(bind_addr: &str) -> thread::JoinHandle<()> {
    let server = TinyServer::http(bind_addr).expect("bind upstream");
    thread::spawn(move || {
        while let Ok(request) = server.recv() {
            let should_shutdown = request.url() == "/__shutdown";
            let response = Response::from_string("OK");
            let _ = request.respond(response);
            if should_shutdown {
                break;
            }
        }
    })
}

fn bench_tls_cert_path() -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), TLS_CERT_RELATIVE_PATH)
}

fn bench_tls_key_path() -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), TLS_KEY_RELATIVE_PATH)
}

fn proxy_url(proxy_addr: &str) -> String {
    let port = proxy_addr
        .rsplit(':')
        .next()
        .expect("proxy addr should include a port");
    format!("https://localhost:{port}/")
}

fn build_tls_agent(cert_path: &str) -> ureq::Agent {
    let verifier = PinnedServerCertVerifier {
        expected_leaf_der: load_first_cert_der(cert_path),
    };
    let tls_config = ureq::rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(verifier))
        .with_no_client_auth();

    ureq::AgentBuilder::new()
        .tls_config(Arc::new(tls_config))
        .max_idle_connections(0)
        .max_idle_connections_per_host(0)
        .timeout_connect(Duration::from_secs(2))
        .timeout_read(Duration::from_secs(5))
        .build()
}

struct BenchHarness {
    api_addr: String,
    proxy_addr: String,
    upstream_addr: String,
    tls_cert_path: String,
    prev_tls_cert: Option<String>,
    prev_tls_key: Option<String>,
    prev_api_threads: Option<String>,
    prev_proxy_threads: Option<String>,
    shutdown_tx: Option<oneshot::Sender<()>>,
    server_handle: Option<thread::JoinHandle<()>>,
    upstream_handle: Option<thread::JoinHandle<()>>,
}

impl BenchHarness {
    fn start() -> Self {
        let api_addr = pick_addr();
        let proxy_addr = pick_addr();
        let upstream_addr = pick_addr();
        let tls_cert_path = bench_tls_cert_path();
        let tls_key_path = bench_tls_key_path();
        let prev_tls_cert = std::env::var("TLS_CERT_PATH").ok();
        let prev_tls_key = std::env::var("TLS_KEY_PATH").ok();
        let prev_api_threads = std::env::var("STAR_API_THREADS").ok();
        let prev_proxy_threads = std::env::var("STAR_PROXY_THREADS").ok();

        // Rust 2024 marks environment mutation as unsafe because process-wide env is shared.
        unsafe {
            std::env::set_var("TLS_CERT_PATH", &tls_cert_path);
            std::env::set_var("TLS_KEY_PATH", &tls_key_path);
            std::env::set_var("STAR_API_THREADS", API_SERVER_THREADS.to_string());
            std::env::set_var("STAR_PROXY_THREADS", PROXY_SERVER_THREADS.to_string());
        }

        let upstream_handle = spawn_upstream(&upstream_addr);

        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let server_api_addr = api_addr.clone();
        let server_proxy_addr = proxy_addr.clone();
        let server_upstream_addr = upstream_addr.clone();
        let server_handle = thread::spawn(move || {
            let server = build_server(
                &server_api_addr,
                &server_proxy_addr,
                &server_upstream_addr,
                all_prefix(4),
                4,
            );
            let shutdown = TestShutdown {
                rx: AsyncMutex::new(shutdown_rx),
            };
            let run_args = RunArgs {
                shutdown_signal: Box::new(shutdown),
            };
            server.run(run_args);
        });

        let public_key_url = format!("http://{api_addr}/star/public_key");
        let mut api_ready = false;
        let mut last_api_transport_err = None;
        for _ in 0..STARTUP_RETRIES {
            match ureq::get(&public_key_url).call() {
                Ok(_) => {
                    api_ready = true;
                    break;
                }
                Err(ureq::Error::Transport(err)) => {
                    last_api_transport_err = Some(err.to_string());
                    thread::sleep(STARTUP_RETRY_DELAY);
                }
                Err(ureq::Error::Status(code, _)) => {
                    panic!("server failed to start cleanly: status {code}");
                }
            }
        }
        if !api_ready {
            panic!(
                "API did not become ready within startup timeout, last transport error: {:?}",
                last_api_transport_err
            );
        }

        let ready_agent = build_tls_agent(&tls_cert_path);
        let ready_proxy_url = proxy_url(&proxy_addr);
        let mut last_proxy_transport_err = None;
        for _ in 0..STARTUP_RETRIES {
            match ready_agent
                .get(&ready_proxy_url)
                .set("Connection", "close")
                .call()
            {
                Err(ureq::Error::Status(403, _)) => {
                    return Self {
                        api_addr,
                        proxy_addr,
                        upstream_addr,
                        tls_cert_path,
                        prev_tls_cert,
                        prev_tls_key,
                        prev_api_threads,
                        prev_proxy_threads,
                        shutdown_tx: Some(shutdown_tx),
                        server_handle: Some(server_handle),
                        upstream_handle: Some(upstream_handle),
                    };
                }
                Err(ureq::Error::Transport(err)) => {
                    last_proxy_transport_err = Some(err.to_string());
                    thread::sleep(STARTUP_RETRY_DELAY);
                }
                Err(ureq::Error::Status(code, _)) => {
                    panic!("proxy failed to start cleanly: status {code}");
                }
                Ok(_) => panic!("expected unauthorized probe request to be rejected"),
            }
        }

        panic!(
            "server did not become ready within startup timeout, last transport error: {:?}",
            last_proxy_transport_err
        );
    }
}

impl Drop for BenchHarness {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }

        if let Some(handle) = self.server_handle.take() {
            let _ = handle.join();
        }

        let shutdown_url = format!("http://{}/__shutdown", self.upstream_addr);
        let _ = ureq::get(&shutdown_url).call();

        if let Some(handle) = self.upstream_handle.take() {
            let _ = handle.join();
        }

        unsafe {
            if let Some(value) = self.prev_tls_cert.as_ref() {
                std::env::set_var("TLS_CERT_PATH", value);
            } else {
                std::env::remove_var("TLS_CERT_PATH");
            }
            if let Some(value) = self.prev_tls_key.as_ref() {
                std::env::set_var("TLS_KEY_PATH", value);
            } else {
                std::env::remove_var("TLS_KEY_PATH");
            }
            if let Some(value) = self.prev_api_threads.as_ref() {
                std::env::set_var("STAR_API_THREADS", value);
            } else {
                std::env::remove_var("STAR_API_THREADS");
            }
            if let Some(value) = self.prev_proxy_threads.as_ref() {
                std::env::set_var("STAR_PROXY_THREADS", value);
            } else {
                std::env::remove_var("STAR_PROXY_THREADS");
            }
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
        let body = get_text_retry(&public_key_url);
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
        let body = post_text_retry(&register_url, cred_request_b64.as_bytes());
        let cred_cipher = URL_SAFE
            .decode(body.trim())
            .expect("decode credential response");
        user.receive_credential(&cred_cipher, &shared_key);
        user
    }

    fn new(api_addr: &str, proxy_addr: &str, tls_cert_path: &str) -> Self {
        let mut rng = OsRng;
        let user = Self::register_user(api_addr, &mut rng);
        let agent = build_tls_agent(tls_cert_path);

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
        self.prepare_tokens(PREPARED_REQUEST_BUFFER);
        self.prepared_tokens
            .pop_front()
            .expect("prepared token queue should not be empty")
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
    let harness = BenchHarness::start();
    let clients = ClientWorkers::start(
        &harness.api_addr,
        &harness.proxy_addr,
        &harness.tls_cert_path,
        CLIENT_THREADS,
    );
    clients.run_batch();

    let mut group = c.benchmark_group("star_e2e_proxy_throughput");
    group.sample_size(20);
    group.measurement_time(Duration::from_secs(8));
    group.throughput(Throughput::Elements(CLIENT_THREADS as u64));
    group.bench_function("authenticated_request", |b| {
        b.iter(|| clients.run_batch());
    });
    group.finish();
}

criterion_group!(benches, bench_star_e2e_throughput);
criterion_main!(benches);
