use pingora::{listeners::tls::TlsSettings, server::Server};
use pingora_proxy::http_proxy_service;
use pingora_web::App as WebApp;
use simple_filter_box::dashmap::DashMapFilterBox;
use std::env;
use std::error::Error;
use std::sync::{Arc, Once};
use tokio::sync::Mutex;

use crate::enclave::EnclaveRuntime;
use crate::filter::StarFilter;
use crate::{
    api::{DEFAULT_PUBLIC_KEY_API, DEFAULT_REGISTER_API},
    cors::Cors,
    enclave::EnclaveRuntimes,
    proxy::StarProxy,
};

pub mod api;
pub mod cors;
pub mod enclave;
pub mod filter;
pub mod proxy;

static RUSTLS_PROVIDER_INIT: Once = Once::new();

pub fn all_prefix(max: u32) -> Vec<u32> {
    (0u32..max).collect()
}

fn env_or_default(name: &str, default: &str) -> String {
    match env::var(name) {
        Ok(value) => value,
        Err(_) => default.to_string(),
    }
}

fn env_usize(name: &str) -> Option<usize> {
    env::var(name).ok()?.parse::<usize>().ok()
}

fn ensure_rustls_provider() {
    RUSTLS_PROVIDER_INIT.call_once(|| {
        // Rustls 0.23 may have multiple providers compiled in transitively; choose one explicitly.
        // Also force TLS key exchange groups to X25519 only.
        let mut provider = rustls::crypto::ring::default_provider();
        provider.kx_groups = vec![rustls::crypto::ring::kx_group::X25519];
        let _ = provider.install_default();
    });
}

pub fn build_server(
    api_addr: &str,
    proxy_addr: &str,
    upstream_addr: &str,
    prefixes: Vec<u32>,
    max_prefix: u32,
) -> Result<Server, Box<dyn Error>> {
    ensure_rustls_provider();
    let filter_enclave = EnclaveRuntimes::from_env(10)?;
    let mut issue_enclave = EnclaveRuntime::from_env()?;

    let filter_box = DashMapFilterBox::new();
    let filter = StarFilter::new(filter_box, prefixes, max_prefix, filter_enclave);

    let mut server = Server::new(None)?;
    server.bootstrap();

    let mut api_app = WebApp::default();
    api_app.use_middleware(Cors::default());

    let public_key_api = api::PublicKeyApi::init(&mut issue_enclave)?;
    let issue_api = api::IssueApi::init(issue_enclave);

    api_app.get(DEFAULT_PUBLIC_KEY_API, Arc::new(public_key_api));
    api_app.post(DEFAULT_REGISTER_API, Arc::new(issue_api));

    let mut api_svc = api_app.to_service("api");
    api_svc.threads = env_usize("STAR_API_THREADS");
    api_svc.add_tcp(api_addr);
    server.add_service(api_svc);

    let mut proxy_svc = http_proxy_service(
        &server.configuration,
        StarProxy {
            upstream_addr: upstream_addr.to_string(),
            filter: Mutex::new(filter),
        },
    );
    proxy_svc.threads = env_usize("STAR_PROXY_THREADS");

    let tls_cert_path = env_or_default("TLS_CERT_PATH", "");
    let tls_key_path = env_or_default("TLS_KEY_PATH", "");

    if tls_cert_path != "" && tls_key_path != "" {
        println!("use tls");
        let tls_settings = TlsSettings::intermediate(&tls_cert_path, &tls_key_path)?;
        proxy_svc.add_tls_with_settings(proxy_addr, None, tls_settings);
    } else {
        println!("use plain");
        proxy_svc.add_tcp(proxy_addr);
    }

    server.add_service(proxy_svc);

    Ok(server)
}

#[cfg(test)]
mod tests;
