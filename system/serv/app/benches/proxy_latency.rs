use async_trait::async_trait;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use constant_time_utils::time as ct_time;
use pingora::listeners::tls::TlsSettings;
use pingora::server::Server;
use pingora::upstreams::peer::HttpPeer;
use pingora_http::ResponseHeader;
use pingora_proxy::{ProxyHttp, Session, http_proxy_service};
use rand_core::{OsRng, RngCore};
use simple_filter_box::hashmap::HashMapFilterBox;
use star_app::core::ENCLAVE;
use star_app::filter::StarFilter;
use star_app::proxy::StarProxy;
use star_core::{
    client::User, secure_channel::client::SecureChannelClient, serv::enclave::Enclave,
};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::Once;
use std::thread;
use std::time::Duration;
use tokio::sync::Mutex;
use ureq::rustls::client::danger::{
    HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier,
};
use ureq::rustls::pki_types::pem::PemObject;
use ureq::rustls::pki_types::{CertificateDer, ServerName, UnixTime};

const STARTUP_RETRIES: u32 = 100;
const STARTUP_RETRY_DELAY: Duration = Duration::from_millis(50);
const WARMUP_REQUESTS: usize = 128;
const TLS_CERT_RELATIVE_PATH: &str = "examples/certs/server.crt";
const TLS_KEY_RELATIVE_PATH: &str = "examples/certs/server.key";
const ENCLAVE_MAX_COUNT: u32 = 1_000_000;
const MAX_COUNT_PER_CREDENTIAL: u64 = ENCLAVE_MAX_COUNT as u64;

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

struct StarLatencyProxy {
    inner: StarProxy<HashMapFilterBox>,
    latency_tx: mpsc::Sender<u64>,
}

struct StarLatencyCtx {
    start: std::time::Instant,
    inner_ctx: (),
}

#[async_trait]
impl ProxyHttp for StarLatencyProxy {
    type CTX = StarLatencyCtx;

    fn new_ctx(&self) -> Self::CTX {
        Self::CTX {
            start: std::time::Instant::now(),
            inner_ctx: self.inner.new_ctx(),
        }
    }

    async fn early_request_filter(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<()>
    where
        Self::CTX: Send + Sync,
    {
        self.inner.early_request_filter(session, &mut ctx.inner_ctx).await
    }

    async fn request_filter(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<bool>
    where
        Self::CTX: Send + Sync,
    {
        self.inner.request_filter(session, &mut ctx.inner_ctx).await
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        self.inner.upstream_peer(session, &mut ctx.inner_ctx).await
    }

    async fn response_filter(
        &self,
        session: &mut Session,
        upstream_response: &mut ResponseHeader,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<()>
    where
        Self::CTX: Send + Sync,
    {
        self.inner
            .response_filter(session, upstream_response, &mut ctx.inner_ctx)
            .await
    }

    async fn logging(
        &self,
        session: &mut Session,
        e: Option<&pingora::Error>,
        ctx: &mut Self::CTX,
    ) where
        Self::CTX: Send + Sync,
    {
        self.inner.logging(session, e, &mut ctx.inner_ctx).await;
        let elapsed_ns = ctx.start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
        let _ = self.latency_tx.send(elapsed_ns);
    }
}

fn init_enclave() {
    ENCLAVE.get_or_init(|| {
        let mut rng = OsRng;
        let mut cipher_receiver = [0u8; 32];
        let mut sk = [0u8; 32];
        rng.fill_bytes(&mut cipher_receiver);
        rng.fill_bytes(&mut sk);
        Enclave::new(ENCLAVE_MAX_COUNT, sk, cipher_receiver).expect("init enclave")
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

fn ensure_rustls_provider() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
}

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

fn cert_path() -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), TLS_CERT_RELATIVE_PATH)
}

fn key_path() -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), TLS_KEY_RELATIVE_PATH)
}

fn load_first_cert_der(path: &str) -> Vec<u8> {
    let cert_pem = fs::read(path).expect("read TLS cert");
    let mut certs = CertificateDer::pem_slice_iter(&cert_pem)
        .collect::<Result<Vec<_>, _>>()
        .expect("parse TLS cert PEM");
    certs
        .drain(..)
        .next()
        .expect("TLS cert PEM did not contain any certificates")
        .as_ref()
        .to_vec()
}

fn build_tls_agent(cert_pem_path: &str) -> ureq::Agent {
    ensure_rustls_provider();

    let verifier = PinnedServerCertVerifier {
        expected_leaf_der: load_first_cert_der(cert_pem_path),
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
        .timeout_read(Duration::from_secs(2))
        .build()
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
        ensure_rustls_provider();
        let mut server = Server::new(None).expect("create pingora server");
        server.bootstrap();

        let mut proxy_svc = http_proxy_service(
            &server.configuration,
            StarLatencyProxy {
                inner: build_star_proxy(upstream_addr),
                latency_tx,
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
        let agent = build_tls_agent(&cert_pem_path);

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
        let mut last_reason = None;
        for _ in 0..STARTUP_RETRIES {
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
                        return;
                    }
                    last_reason = Some(format!("unexpected startup body: {body:?}"));
                }
                Err(ureq::Error::Status(code, _)) => {
                    last_reason = Some(format!("startup status error: {code}"));
                }
                Err(ureq::Error::Transport(err)) => {
                    last_reason = Some(format!("startup transport error: {err}"));
                }
            }
            thread::sleep(STARTUP_RETRY_DELAY);
        }

        panic!(
            "proxy benchmark service did not become ready in time, last reason: {:?}",
            last_reason
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
    group.bench_function("single_thread_tls_original_process", |b| {
        b.iter_custom(|iters| {
            let mut total_ns = 0u128;
            for _ in 0..iters {
                total_ns += u128::from(harness.one_internal_latency_ns());
            }
            let total_ns = total_ns.min(u128::from(u64::MAX)) as u64;
            Duration::from_nanos(total_ns)
        });
    });
    group.finish();
}

criterion_group!(benches, bench_pingora_proxy_basic_latency_tls);
criterion_main!(benches);
