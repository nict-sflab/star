pub mod http;
pub mod latency_proxy;
pub mod retry;
pub mod tls;

pub fn manifest_path(relative_path: &str) -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), relative_path)
}
