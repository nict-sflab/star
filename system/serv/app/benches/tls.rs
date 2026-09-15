mod common;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use openssl::ssl::{SslAcceptor, SslConnector, SslFiletype, SslMethod, SslVerifyMode};
use std::error::Error;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const WARMUP_REQUESTS: usize = 128;
const TLS_CERT_RELATIVE_PATH: &str = "examples/certs/server.crt";
const TLS_KEY_RELATIVE_PATH: &str = "examples/certs/server.key";
const IO_TIMEOUT: Duration = Duration::from_secs(2);
const LATENCY_RECV_TIMEOUT: Duration = Duration::from_secs(2);
const BENCH_GROUP_NAME: &str = "pure_tls_server_side_latency";
const TLS_HANDSHAKE_ONLY: &str = "server_side_tls_handshake";

struct PlainTlsClient {
    connector: SslConnector,
}

impl PlainTlsClient {
    fn new() -> Self {
        let mut builder = SslConnector::builder(SslMethod::tls()).expect("build TLS connector");
        builder.set_verify(SslVerifyMode::NONE);
        builder
            .set_groups_list("X25519")
            .expect("set client TLS group");
        Self {
            connector: builder.build(),
        }
    }

    fn handshake(&self, addr: SocketAddr) -> Result<(), Box<dyn Error + Send + Sync>> {
        let tcp = TcpStream::connect(addr)?;
        tcp.set_read_timeout(Some(IO_TIMEOUT))?;
        tcp.set_write_timeout(Some(IO_TIMEOUT))?;
        let _tls = self.connector.connect("localhost", tcp)?;
        Ok(())
    }
}

struct BenchHarness {
    server_addr: SocketAddr,
    client: PlainTlsClient,
    latencies: mpsc::Receiver<Duration>,
    _server: thread::JoinHandle<()>,
}

impl BenchHarness {
    fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind TLS benchmark server");
        let server_addr = listener.local_addr().expect("read TLS benchmark address");
        let cert_path = common::manifest_path(TLS_CERT_RELATIVE_PATH);
        let key_path = common::manifest_path(TLS_KEY_RELATIVE_PATH);
        let (latency_tx, latencies) = mpsc::channel();

        let server = thread::spawn(move || {
            let mut acceptor =
                SslAcceptor::mozilla_intermediate(SslMethod::tls()).expect("build TLS acceptor");
            acceptor
                .set_private_key_file(key_path, SslFiletype::PEM)
                .expect("set server TLS private key");
            acceptor
                .set_certificate_chain_file(cert_path)
                .expect("set server TLS certificate chain");
            acceptor
                .set_groups_list("X25519")
                .expect("set server TLS group");
            let acceptor = acceptor.build();

            for tcp in listener.incoming() {
                let tcp = tcp.expect("accept TCP connection");
                tcp.set_read_timeout(Some(IO_TIMEOUT))
                    .expect("set TCP read timeout");
                tcp.set_write_timeout(Some(IO_TIMEOUT))
                    .expect("set TCP write timeout");

                let start = Instant::now();
                let _tls = acceptor.accept(tcp).expect("accept TLS handshake");
                latency_tx
                    .send(start.elapsed())
                    .expect("record TLS handshake latency");
            }
        });

        let mut harness = Self {
            server_addr,
            client: PlainTlsClient::new(),
            latencies,
            _server: server,
        };
        for _ in 0..WARMUP_REQUESTS {
            harness.one_server_side_latency();
        }
        harness
    }

    fn one_server_side_latency(&mut self) -> Duration {
        self.client
            .handshake(self.server_addr)
            .expect("run TLS client");
        self.latencies
            .recv_timeout(LATENCY_RECV_TIMEOUT)
            .expect("wait server-side latency")
    }

    fn measure(&mut self, iters: u64) -> Duration {
        (0..iters)
            .map(|_| self.one_server_side_latency())
            .sum()
    }
}

fn bench_pure_tls_server_side_latency(c: &mut Criterion) {
    let mut harness = BenchHarness::start();
    let mut group = c.benchmark_group(BENCH_GROUP_NAME);
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(6));
    group.throughput(Throughput::Elements(1));
    group.bench_function(TLS_HANDSHAKE_ONLY, |b| {
        b.iter_custom(|iters| harness.measure(iters));
    });
    group.finish();
}

criterion_group!(benches, bench_pure_tls_server_side_latency);
criterion_main!(benches);
