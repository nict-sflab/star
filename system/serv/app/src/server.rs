use pingora::{listeners::tls::TlsSettings, server::Server};
use pingora_proxy::http_proxy_service;
use pingora_web::App as WebApp;
use simple_filter_box::hashmap::HashMapFilterBox;
use std::error::Error;
use std::sync::Arc;

use crate::api::{IssueApi, PublicKeyApi, DEFAULT_PUBLIC_KEY_API, DEFAULT_REGISTER_API};
use crate::env_or_default;
use crate::filter::StarFilter;
use crate::provisioning::ProvisionedEnclaves;
use crate::proxy::UpstreamProxy;
use crate::tls::configure_tls_token_extension;

pub struct Shard {
    index: u32,
    count: u32,
}

impl Shard {
    pub fn new(index: u32, count: u32) -> Self {
        assert!(count > 0 && index < count, "invalid shard: {index}/{count}");
        Self { index, count }
    }
}

pub fn build_server(
    api_addr: &str,
    proxy_addr: &str,
    upstream_addr: &str,
    shard: Shard,
) -> Result<Server, Box<dyn Error>> {
    build_server_with_startup_attestation(
        api_addr,
        proxy_addr,
        upstream_addr,
        shard,
        true,
    )
}

#[cfg(test)]
pub(crate) fn build_server_without_startup_attestation(
    api_addr: &str,
    proxy_addr: &str,
    upstream_addr: &str,
    shard: Shard,
) -> Result<Server, Box<dyn Error>> {
    build_server_with_startup_attestation(
        api_addr,
        proxy_addr,
        upstream_addr,
        shard,
        false,
    )
}

fn build_server_with_startup_attestation(
    api_addr: &str,
    proxy_addr: &str,
    upstream_addr: &str,
    shard: Shard,
    emit_startup_attestation: bool,
) -> Result<Server, Box<dyn Error>> {
    let installed = ProvisionedEnclaves::provision()?;
    if emit_startup_attestation {
        installed.emit_startup_attestation();
    }

    let filter = Arc::new(StarFilter::new(
        HashMapFilterBox::default(),
        vec![shard.index],
        shard.count,
        installed.filter_enclave,
    ));

    let mut server = Server::new(None)?;
    server.bootstrap();

    let mut api_app = WebApp::default();
    api_app.get(
        DEFAULT_PUBLIC_KEY_API,
        Arc::new(PublicKeyApi::new(installed.public_key)),
    );
    api_app.post(
        DEFAULT_REGISTER_API,
        Arc::new(IssueApi::new(Arc::new(installed.issue_enclave))),
    );

    let mut api_service = api_app.to_service("api");
    api_service.threads = Some(1);
    api_service.add_tcp(api_addr);
    server.add_service(api_service);

    let mut proxy_service = http_proxy_service(
        &server.configuration,
        UpstreamProxy::new(upstream_addr),
    );
    proxy_service.threads = Some(1);

    let tls_cert_path = env_or_default("TLS_CERT_PATH", "./examples/certs/server.crt");
    let tls_key_path = env_or_default("TLS_KEY_PATH", "./examples/certs/server.key");

    if tls_cert_path.is_empty() || tls_key_path.is_empty() {
        println!("use plain");
        proxy_service.add_tcp(proxy_addr);
    } else {
        println!("use tls");
        let mut tls_settings = TlsSettings::intermediate(&tls_cert_path, &tls_key_path)?;
        configure_tls_token_extension(&mut tls_settings, filter)?;
        proxy_service.add_tls_with_settings(proxy_addr, None, tls_settings);
    }

    server.add_service(proxy_service);
    Ok(server)
}
