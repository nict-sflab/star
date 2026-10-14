pub mod aesm_proxy;
pub mod scale_out;
pub mod star;
pub mod tls;

pub fn manifest_path(relative_path: &str) -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), relative_path)
}
