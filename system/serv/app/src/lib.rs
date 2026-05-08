use pingora::{listeners::tls::TlsSettings, server::Server};
use pingora_proxy::http_proxy_service;
use pingora_web::App as WebApp;
use simple_filter_box::hashmap::HashMapFilterBox;
use std::collections::HashMap;
use std::env;
use std::sync::Once;
use tokio::sync::Mutex;

use crate::filter::StarFilter;
use crate::{
    api::{DEFAULT_PUBLIC_KEY_API, DEFAULT_REGISTER_API, issue_api, public_key_api},
    core::{ENCLAVE, EnclaveRuntime},
    cors::Cors,
    proxy::StarProxy,
};

pub mod api;
pub mod core;
pub mod cors;
pub mod filter;
pub mod proxy;

const MAX_COUNT: u32 = 10;
static RUSTLS_PROVIDER_INIT: Once = Once::new();

pub fn all_prefix(max: u32) -> Vec<u32> {
    (0u32..max).collect()
}

fn env_or_default(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

fn env_usize(name: &str) -> Option<usize> {
    env::var(name).ok()?.parse::<usize>().ok()
}

fn ensure_rustls_provider() {
    RUSTLS_PROVIDER_INIT.call_once(|| {
        // Rustls 0.23 may have multiple providers compiled in transitively; choose one explicitly.
        let _ = rustls::crypto::ring::default_provider().install_default();
    });
}

pub fn build_server(
    api_addr: &str,
    proxy_addr: &str,
    upstream_addr: &str,
    prefixes: Vec<u32>,
    max_prefix: u32,
) -> Server {
    ensure_rustls_provider();

    ENCLAVE.get_or_init(|| EnclaveRuntime::from_env(MAX_COUNT).expect("init enclave"));

    let mut server = Server::new(None).expect("create server");
    server.bootstrap();

    let mut api_app = WebApp::default();
    api_app.use_middleware(Cors::default());
    api_app.get_fn(DEFAULT_PUBLIC_KEY_API, public_key_api);
    api_app.post_fn(DEFAULT_REGISTER_API, issue_api);

    let mut api_svc = api_app.to_service("api");
    api_svc.threads = env_usize("STAR_API_THREADS");
    api_svc.add_tcp(api_addr);
    server.add_service(api_svc);

    let mut filter_boxes = HashMap::new();

    for prefix in prefixes {
        let filter_box = HashMapFilterBox::new();
        filter_boxes.insert(prefix, Mutex::new(filter_box));
    }

    let filter = StarFilter::new(filter_boxes, max_prefix);

    let mut proxy_svc = http_proxy_service(
        &server.configuration,
        StarProxy {
            filter,
            upstream_addr: upstream_addr.to_string(),
        },
    );
    proxy_svc.threads = env_usize("STAR_PROXY_THREADS");

    let tls_cert_path = env_or_default("TLS_CERT_PATH", "");
    let tls_key_path = env_or_default("TLS_KEY_PATH", "");

    if tls_cert_path != "" && tls_key_path != "" {
        println!("use tls");
        let tls_settings = TlsSettings::intermediate(&tls_cert_path, &tls_key_path).unwrap();
        proxy_svc.add_tls_with_settings(proxy_addr, None, tls_settings);
    } else {
        println!("use plain");
        proxy_svc.add_tcp(proxy_addr);
    }

    server.add_service(proxy_svc);

    server
}

#[cfg(test)]
mod tests;
