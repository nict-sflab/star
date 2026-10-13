use pingora::{listeners::tls::TlsSettings, server::Server};
use pingora_proxy::http_proxy_service;
use pingora_web::App as WebApp;
use simple_filter_box::hashmap::HashMapFilterBox;
use std::error::Error;
use std::sync::Arc;

use crate::api::{DEFAULT_PUBLIC_KEY_API, DEFAULT_REGISTER_API, IssueApi, PublicKeyApi};
use crate::env_or_default;
use crate::filter::StarFilter;
use crate::provisioning::ProvisionedEnclave;
use crate::proxy::UpstreamProxy;
use crate::tls::star_tls_client_hello_callback;

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
    build_server_with_startup_attestation(api_addr, proxy_addr, upstream_addr, shard, true)
}

#[cfg(test)]
pub(crate) fn build_server_without_startup_attestation(
    api_addr: &str,
    proxy_addr: &str,
    upstream_addr: &str,
    shard: Shard,
) -> Result<Server, Box<dyn Error>> {
    build_server_with_startup_attestation(api_addr, proxy_addr, upstream_addr, shard, false)
}

fn build_server_with_startup_attestation(
    api_addr: &str,
    proxy_addr: &str,
    upstream_addr: &str,
    shard: Shard,
    emit_startup_attestation: bool,
) -> Result<Server, Box<dyn Error>> {
    let installed = ProvisionedEnclave::provision()?;
    if emit_startup_attestation {
        installed.emit_startup_attestation();
    }

    let filter = Arc::new(StarFilter::new(
        HashMapFilterBox::default(),
        vec![shard.index],
        shard.count,
        Arc::clone(&installed.enclave),
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
        Arc::new(IssueApi::new(installed.enclave)),
    );

    let mut api_service = api_app.to_service("api");
    api_service.threads = Some(1);
    api_service.add_tcp(api_addr);

    let mut proxy_service =
        http_proxy_service(&server.configuration, UpstreamProxy::new(upstream_addr));
    proxy_service.threads = Some(1);

    let tls_cert_path = env_or_default("TLS_CERT_PATH", "./examples/certs/server.crt");
    let tls_key_path = env_or_default("TLS_KEY_PATH", "./examples/certs/server.key");

    if tls_cert_path.is_empty() || tls_key_path.is_empty() {
        println!("use plain");
        proxy_service.add_tcp(proxy_addr);
    } else {
        println!("use tls");
        let mut tls_settings = TlsSettings::intermediate(&tls_cert_path, &tls_key_path)?;
        tls_settings.set_groups_list("X25519")?;
        tls_settings.set_client_hello_callback(star_tls_client_hello_callback(filter)?);
        proxy_service.add_tls_with_settings(proxy_addr, None, tls_settings);
    }

    server.add_service(ShardService {
        api: Box::new(api_service),
        proxy: Box::new(proxy_service),
    });
    Ok(server)
}

// Pingora normally creates one runtime per service. Group both listeners so
// registration and filtering share the shard's single request worker.
struct ShardService {
    api: Box<dyn pingora::services::Service>,
    proxy: Box<dyn pingora::services::Service>,
}

#[async_trait::async_trait]
impl pingora::services::Service for ShardService {
    async fn start_service(
        &mut self,
        fds: Option<pingora::server::ListenFds>,
        shutdown: pingora::server::ShutdownWatch,
        listeners_per_fd: usize,
    ) {
        tokio::join!(
            self.api
                .start_service(fds.clone(), shutdown.clone(), listeners_per_fd),
            self.proxy.start_service(fds, shutdown, listeners_per_fd),
        );
    }

    fn name(&self) -> &str {
        "shard"
    }
    fn threads(&self) -> Option<usize> {
        Some(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pingora::services::Service;
    use std::sync::Mutex;

    struct ListenerProbe {
        threads: Arc<Mutex<Vec<std::thread::ThreadId>>>,
        barrier: Arc<tokio::sync::Barrier>,
    }

    #[async_trait::async_trait]
    impl Service for ListenerProbe {
        async fn start_service(
            &mut self,
            _: Option<pingora::server::ListenFds>,
            _: pingora::server::ShutdownWatch,
            _: usize,
        ) {
            self.threads
                .lock()
                .unwrap()
                .push(std::thread::current().id());
            self.barrier.wait().await;
        }
        fn name(&self) -> &str {
            "probe"
        }
    }

    #[test]
    fn both_listeners_start_on_one_worker() {
        let threads = Arc::new(Mutex::new(Vec::new()));
        let barrier = Arc::new(tokio::sync::Barrier::new(2));
        let probe = || {
            Box::new(ListenerProbe {
                threads: Arc::clone(&threads),
                barrier: Arc::clone(&barrier),
            })
        };
        let mut service = ShardService {
            api: probe(),
            proxy: probe(),
        };
        assert_eq!(service.threads(), Some(1));
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let (_sender, shutdown) = tokio::sync::watch::channel(false);
        runtime.block_on(service.start_service(None, shutdown, 1));
        let threads = threads.lock().unwrap();
        assert_eq!(threads.len(), 2);
        assert_eq!(threads[0], threads[1]);
    }
}
