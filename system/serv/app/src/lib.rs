use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use pingora::{listeners::tls::TlsSettings, server::Server};
use pingora_proxy::http_proxy_service;
use pingora_web::App as WebApp;
use rand::{TryRngCore, rngs::OsRng};
use simple_filter_box::dashmap::DashMapFilterBox;
use std::env;
use std::error::Error;
use std::sync::Arc;

use crate::filter::StarFilter;
use crate::key_manager::KeyManagerRuntime;
use crate::tls::configure_tls_token_extension;
use crate::{
    api::{DEFAULT_PUBLIC_KEY_API, DEFAULT_REGISTER_API},
    enclave::EnclaveRuntimes,
    proxy::StarProxy,
};

pub mod api;
pub mod enclave;
pub mod filter;
pub mod key_manager;
pub mod proxy;
pub mod qve;
pub mod tls;
pub(crate) mod tls_utils;

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

pub fn build_server(
    api_addr: &str,
    proxy_addr: &str,
    upstream_addr: &str,
    prefixes: Vec<u32>,
    max_prefix: u32,
) -> Result<Server, Box<dyn Error>> {
    build_server_with_startup_attestation(
        api_addr,
        proxy_addr,
        upstream_addr,
        prefixes,
        max_prefix,
        true,
    )
}

pub fn install_key_manager_keys_for_pool(
    key_manager: &KeyManagerRuntime,
    recipient: &EnclaveRuntimes,
) -> Result<(), Box<dyn Error>> {
    let key_manager_quote = key_manager.public_key_attestation()?;
    let recipient_quote = recipient.transfer_key_attestation()?;
    let key_manager_target_info = key_manager.qve_target_info()?;
    let recipient_target_info = recipient.qve_target_info()?;

    let recipient_evidence_for_key_manager =
        qve::evidence_for_verifier(&recipient_quote, &key_manager_target_info)?;
    let key_manager_evidence_for_recipient =
        qve::evidence_for_verifier(&key_manager_quote, &recipient_target_info)?;

    let keys = key_manager.export_keys(&recipient_evidence_for_key_manager)?;
    recipient.install_keys(&key_manager_evidence_for_recipient, &keys)?;

    Ok(())
}

pub fn install_key_manager_keys_for_enclave(
    key_manager: &KeyManagerRuntime,
    recipient: &enclave::EnclaveRuntime,
) -> Result<(), Box<dyn Error>> {
    let key_manager_quote = key_manager.public_key_attestation()?;
    let recipient_quote = recipient.transfer_key_attestation()?;
    let key_manager_target_info = key_manager.qve_target_info()?;
    let recipient_target_info = recipient.qve_target_info()?;

    let recipient_evidence_for_key_manager =
        qve::evidence_for_verifier(&recipient_quote, &key_manager_target_info)?;
    let key_manager_evidence_for_recipient =
        qve::evidence_for_verifier(&key_manager_quote, &recipient_target_info)?;

    let keys = key_manager.export_keys(&recipient_evidence_for_key_manager)?;
    recipient.install_keys(&key_manager_evidence_for_recipient, &keys)?;

    Ok(())
}

pub fn benchmark_key_bundle() -> Result<Vec<u8>, Box<dyn Error>> {
    let mut bundle = vec![0u8; 64];
    OsRng.try_fill_bytes(&mut bundle)?;
    Ok(bundle)
}

pub fn install_benchmark_keys_for_pool(
    recipient: &EnclaveRuntimes,
    key_bundle: &[u8],
) -> Result<(), Box<dyn Error>> {
    recipient.install_benchmark_keys(key_bundle)?;
    Ok(())
}

pub fn install_benchmark_keys_for_enclave(
    recipient: &enclave::EnclaveRuntime,
    key_bundle: &[u8],
) -> Result<(), Box<dyn Error>> {
    recipient.install_benchmark_keys(key_bundle)?;
    Ok(())
}

#[cfg(test)]
pub(crate) fn build_server_without_startup_attestation(
    api_addr: &str,
    proxy_addr: &str,
    upstream_addr: &str,
    prefixes: Vec<u32>,
    max_prefix: u32,
) -> Result<Server, Box<dyn Error>> {
    build_server_with_startup_attestation(
        api_addr,
        proxy_addr,
        upstream_addr,
        prefixes,
        max_prefix,
        false,
    )
}

fn build_server_with_startup_attestation(
    api_addr: &str,
    proxy_addr: &str,
    upstream_addr: &str,
    prefixes: Vec<u32>,
    max_prefix: u32,
    emit_startup_attestation: bool,
) -> Result<Server, Box<dyn Error>> {
    let key_manager = KeyManagerRuntime::from_env()?;
    let filter_enclave = EnclaveRuntimes::from_env(10)?;
    let issue_enclave = enclave::EnclaveRuntime::from_env()?;

    let mut server = Server::new(None)?;
    server.bootstrap();

    let mut api_app = WebApp::default();

    let public_key = key_manager.public_key()?;
    let key_manager_quote = key_manager.public_key_attestation()?;
    let filter_quote = filter_enclave.transfer_key_attestation()?;
    let issue_quote = issue_enclave.transfer_key_attestation()?;
    let key_manager_target_info = key_manager.qve_target_info()?;
    let filter_target_info = filter_enclave.qve_target_info()?;
    let issue_target_info = issue_enclave.qve_target_info()?;

    let filter_evidence_for_key_manager =
        qve::evidence_for_verifier(&filter_quote, &key_manager_target_info)?;
    let issue_evidence_for_key_manager =
        qve::evidence_for_verifier(&issue_quote, &key_manager_target_info)?;
    let key_manager_evidence_for_filter =
        qve::evidence_for_verifier(&key_manager_quote, &filter_target_info)?;
    let key_manager_evidence_for_issue =
        qve::evidence_for_verifier(&key_manager_quote, &issue_target_info)?;

    if emit_startup_attestation {
        print_startup_attestation(StartupAttestation {
            public_key,
            key_manager_quote: &key_manager_quote,
            filter_quote: &filter_quote,
            issue_quote: &issue_quote,
            filter_evidence_for_key_manager: &filter_evidence_for_key_manager,
            issue_evidence_for_key_manager: &issue_evidence_for_key_manager,
            key_manager_evidence_for_filter: &key_manager_evidence_for_filter,
            key_manager_evidence_for_issue: &key_manager_evidence_for_issue,
        })?;
    }

    let filter_keys = key_manager.export_keys(&filter_evidence_for_key_manager)?;
    filter_enclave.install_keys(&key_manager_evidence_for_filter, &filter_keys)?;

    let issue_keys = key_manager.export_keys(&issue_evidence_for_key_manager)?;
    issue_enclave.install_keys(&key_manager_evidence_for_issue, &issue_keys)?;

    let filter_box = DashMapFilterBox::new();
    let filter = Arc::new(StarFilter::new(
        filter_box,
        prefixes,
        max_prefix,
        filter_enclave,
    ));

    let public_key_api = api::PublicKeyApi::from_public_key(public_key);
    let issue_enclave = Arc::new(issue_enclave);
    let issue_api = api::IssueApi::init_shared(Arc::clone(&issue_enclave));

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
            filter: Arc::clone(&filter),
        },
    );

    proxy_svc.threads = env_usize("STAR_PROXY_THREADS");

    let tls_cert_path = env_or_default("TLS_CERT_PATH", "./examples/certs/server.crt");
    let tls_key_path = env_or_default("TLS_KEY_PATH", "./examples/certs/server.key");

    if !tls_cert_path.is_empty() && !tls_key_path.is_empty() {
        println!("use tls");

        let mut tls_settings = TlsSettings::intermediate(&tls_cert_path, &tls_key_path)?;

        configure_tls_token_extension(&mut tls_settings, filter)?;

        proxy_svc.add_tls_with_settings(proxy_addr, None, tls_settings);
    } else {
        println!("use plain");

        proxy_svc.add_tcp(proxy_addr);
    }

    server.add_service(proxy_svc);

    Ok(server)
}

struct StartupAttestation<'a> {
    public_key: [u8; 32],
    key_manager_quote: &'a [u8],
    filter_quote: &'a [u8],
    issue_quote: &'a [u8],
    filter_evidence_for_key_manager: &'a [u8],
    issue_evidence_for_key_manager: &'a [u8],
    key_manager_evidence_for_filter: &'a [u8],
    key_manager_evidence_for_issue: &'a [u8],
}

fn print_startup_attestation(attestation: StartupAttestation<'_>) -> Result<(), Box<dyn Error>> {
    println!(
        "STAR_ATTESTATION_PUBLIC_KEY_BASE64={}",
        URL_SAFE.encode(attestation.public_key)
    );
    println!(
        "STAR_ATTESTATION_QUOTE_BASE64={}",
        URL_SAFE.encode(attestation.key_manager_quote)
    );
    println!(
        "STAR_KEY_MANAGER_QUOTE_BASE64={}",
        URL_SAFE.encode(attestation.key_manager_quote)
    );
    println!(
        "STAR_FILTER_ENCLAVE_TRANSFER_QUOTE_BASE64={}",
        URL_SAFE.encode(attestation.filter_quote)
    );
    println!(
        "STAR_ISSUE_ENCLAVE_TRANSFER_QUOTE_BASE64={}",
        URL_SAFE.encode(attestation.issue_quote)
    );
    println!(
        "STAR_FILTER_ENCLAVE_QVE_EVIDENCE_FOR_KEY_MANAGER_BASE64={}",
        URL_SAFE.encode(attestation.filter_evidence_for_key_manager)
    );
    println!(
        "STAR_ISSUE_ENCLAVE_QVE_EVIDENCE_FOR_KEY_MANAGER_BASE64={}",
        URL_SAFE.encode(attestation.issue_evidence_for_key_manager)
    );
    println!(
        "STAR_KEY_MANAGER_QVE_EVIDENCE_FOR_FILTER_ENCLAVE_BASE64={}",
        URL_SAFE.encode(attestation.key_manager_evidence_for_filter)
    );
    println!(
        "STAR_KEY_MANAGER_QVE_EVIDENCE_FOR_ISSUE_ENCLAVE_BASE64={}",
        URL_SAFE.encode(attestation.key_manager_evidence_for_issue)
    );
    Ok(())
}

#[cfg(test)]
mod tests;
