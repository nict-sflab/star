pub mod aesm_proxy;
pub mod http;
pub mod latency_proxy;
#[allow(dead_code)]
pub mod local_http;
pub mod retry;
pub mod tls;

use cpu_time::ProcessTime;
use std::time::Duration;

pub fn manifest_path(relative_path: &str) -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), relative_path)
}

pub fn measure_process_cpu_time<F>(iters: u64, mut run_iteration: F) -> Duration
where
    F: FnMut(),
{
    let start = ProcessTime::now();
    for _ in 0..iters {
        run_iteration();
    }
    start.elapsed()
}
