mod common;

use common::local_http::spawn_ok_server;
use common::retry::{StartupProbe, wait_until_ready};
use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use reqwest::blocking::Client;
use std::thread;
use std::time::Duration;

const STARTUP_RETRIES: u32 = 100;
const STARTUP_RETRY_DELAY: Duration = Duration::from_millis(10);
const WARMUP_REQUESTS: usize = 64;
const HTTP_SERVER_BIND_ADDR: &str = "127.0.0.1:0";
const CLIENT_POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(30);
const BENCH_GROUP_NAME: &str = "client_http_request";
const BENCH_FUNCTION_NAME: &str = "reqwest_blocking_get_localhost_process_cpu";

struct BenchHarness {
    url: String,
    client: Client,
    _server_handle: thread::JoinHandle<()>,
}

impl BenchHarness {
    fn start() -> Self {
        let (addr, server_handle) = spawn_ok_server(HTTP_SERVER_BIND_ADDR, "benchmark HTTP server");
        let client = build_client();

        let harness = Self {
            url: format!("http://{addr}/"),
            client,
            _server_handle: server_handle,
        };

        harness.wait_until_ready();
        harness.warmup(WARMUP_REQUESTS);
        harness
    }

    fn request(&self) -> reqwest::Result<String> {
        self.client
            .get(&self.url)
            .header("Connection", "close")
            .send()?
            .text()
    }

    fn send_request(&self) {
        let body = self.request().expect("send benchmark request");
        assert_eq!(body, "OK");
    }

    fn wait_until_ready(&self) {
        wait_until_ready(
            "benchmark HTTP server",
            STARTUP_RETRIES,
            STARTUP_RETRY_DELAY,
            || match self.request() {
                Ok(body) if body == "OK" => StartupProbe::Ready,
                Ok(body) => StartupProbe::Retry(format!("unexpected response body: {body:?}")),
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

fn build_client() -> Client {
    Client::builder()
        .pool_idle_timeout(Some(CLIENT_POOL_IDLE_TIMEOUT))
        .build()
        .expect("build reqwest client")
}

fn bench_client_http_request(c: &mut Criterion) {
    let harness = BenchHarness::start();

    let mut group = c.benchmark_group(BENCH_GROUP_NAME);
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(6));
    group.throughput(Throughput::Elements(1));
    group.bench_function(BENCH_FUNCTION_NAME, |b| {
        b.iter_custom(|iters| common::measure_process_cpu_time(iters, || harness.send_request()));
    });
    group.finish();
}

criterion_group!(benches, bench_client_http_request);
criterion_main!(benches);
