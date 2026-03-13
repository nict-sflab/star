mod common;

use common::retry::{StartupProbe, wait_until_ready};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use reqwest::blocking::Client;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

const STARTUP_RETRIES: u32 = 100;
const STARTUP_RETRY_DELAY: Duration = Duration::from_millis(10);
const WARMUP_REQUESTS: usize = 64;

fn find_headers_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|window| window == b"\r\n\r\n")
}

fn handle_connection(mut stream: TcpStream) {
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

        if find_headers_end(&pending).is_some() {
            let response = b"HTTP/1.1 200 OK\r\ncontent-length: 2\r\nconnection: close\r\n\r\nOK";
            if stream.write_all(response).is_err() {
                return;
            }
            let _ = stream.flush();
            return;
        }
    }
}

fn spawn_http_server() -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind benchmark HTTP server");
    let addr = listener
        .local_addr()
        .expect("read benchmark HTTP server address")
        .to_string();

    let handle = thread::spawn(move || {
        for incoming in listener.incoming() {
            match incoming {
                Ok(stream) => handle_connection(stream),
                Err(err) => eprintln!("benchmark HTTP server accept error: {err}"),
            }
        }
    });

    (addr, handle)
}

struct BenchHarness {
    url: String,
    client: Client,
    _server_handle: thread::JoinHandle<()>,
}

impl BenchHarness {
    fn start() -> Self {
        let (addr, server_handle) = spawn_http_server();
        let client = Client::builder()
            .pool_idle_timeout(Some(Duration::from_secs(30)))
            .build()
            .expect("build reqwest client");

        let mut harness = Self {
            url: format!("http://{addr}/"),
            client,
            _server_handle: server_handle,
        };

        harness.wait_until_ready();
        harness.warmup(WARMUP_REQUESTS);
        harness
    }

    fn send_request(&self) {
        let response = self
            .client
            .get(&self.url)
            .header("Connection", "close")
            .send()
            .expect("send benchmark request");
        let body = response.text().expect("read benchmark response body");
        assert_eq!(body, "OK");
    }

    fn wait_until_ready(&mut self) {
        wait_until_ready(
            "benchmark HTTP server",
            STARTUP_RETRIES,
            STARTUP_RETRY_DELAY,
            || match self
                .client
                .get(&self.url)
                .header("Connection", "close")
                .send()
            {
                Ok(response) => match response.text() {
                    Ok(body) if body == "OK" => StartupProbe::Ready,
                    Ok(body) => StartupProbe::Retry(format!("unexpected response body: {body:?}")),
                    Err(err) => StartupProbe::Retry(format!("failed to read response body: {err}")),
                },
                Err(err) => StartupProbe::Retry(err.to_string()),
            },
        );
    }

    fn warmup(&self, count: usize) {
        for _ in 0..count {
            self.send_request();
        }
    }
}

fn bench_client_http_request(c: &mut Criterion) {
    let harness = BenchHarness::start();

    let mut group = c.benchmark_group("client_http_request");
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(6));
    group.throughput(Throughput::Elements(1));
    group.bench_function("reqwest_blocking_get_localhost_process_cpu", |b| {
        b.iter_custom(|iters| common::measure_process_cpu_time(iters, || harness.send_request()));
    });
    group.finish();
}

criterion_group!(benches, bench_client_http_request);
criterion_main!(benches);
