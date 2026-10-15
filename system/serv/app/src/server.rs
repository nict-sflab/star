use pingora::{listeners::tls::TlsSettings, server::Server};
use pingora_proxy::http_proxy_service;
use pingora_web::App as WebApp;
use simple_filter_box::hashmap::HashMapFilterBox;
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::api::{DEFAULT_PUBLIC_KEY_API, DEFAULT_REGISTER_API, IssueApi, PublicKeyApi};
use crate::env_or_default;
use crate::filter::StarFilter;
use crate::provisioning::ProvisionedEnclave;
use crate::proxy::UpstreamProxy;
use crate::tls::{register_star_tls_extension, star_tls_client_hello_callback};

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
        panic!("no tls key");
    } else {
        println!("use tls");
        let mut tls_settings = TlsSettings::intermediate(&tls_cert_path, &tls_key_path)?;
        tls_settings.set_groups_list("X25519")?;
        register_star_tls_extension(&mut tls_settings)?;
        tls_settings.set_client_hello_callback(star_tls_client_hello_callback(Arc::clone(&filter))?);
        proxy_service.add_tls_with_settings(proxy_addr, None, tls_settings);
    }

    server.add_service(ShardService {
        api: Box::new(api_service),
        proxy: Box::new(proxy_service),
    });
    server.add_service(HlBatchService { filter });
    Ok(server)
}

const PERIOD_SECONDS: u64 = 60;

fn until_next_period(now: Duration) -> Duration {
    let seconds_left = PERIOD_SECONDS - now.as_secs() % PERIOD_SECONDS;
    Duration::from_secs(seconds_left) - Duration::from_nanos(u64::from(now.subsec_nanos()))
}

struct HlBatchService {
    filter: Arc<StarFilter<HashMapFilterBox>>,
}

#[async_trait::async_trait]
impl pingora::services::Service for HlBatchService {
    async fn start_service(
        &mut self,
        _fds: Option<pingora::server::ListenFds>,
        mut shutdown: pingora::server::ShutdownWatch,
        _listeners_per_fd: usize,
    ) {
        let mut maintained_period = None;
        loop {
            let period = u64::from_be_bytes(constant_time_utils::time::current_period());
            if maintained_period != Some(period) {
                if let Err(error) = self.filter.maintain_hls() {
                    log::error!("HL batch maintenance failed: {error:?}");
                }
                maintained_period = Some(period);
            }

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time before UNIX_EPOCH");
            tokio::select! {
                _ = tokio::time::sleep(until_next_period(now)) => {}
                changed = shutdown.changed() => {
                    if changed.is_err() || *shutdown.borrow() {
                        break;
                    }
                }
            }
        }
    }

    fn name(&self) -> &str {
        "star-hl-batch"
    }

    fn threads(&self) -> Option<usize> {
        Some(1)
    }
}

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
    fn waits_until_shared_period_boundary() {
        assert_eq!(
            until_next_period(Duration::ZERO),
            Duration::from_secs(PERIOD_SECONDS)
        );
        assert_eq!(
            until_next_period(Duration::from_secs(PERIOD_SECONDS - 1)),
            Duration::from_secs(1)
        );
        assert_eq!(
            until_next_period(Duration::new(PERIOD_SECONDS * 2 - 1, 250_000_000)),
            Duration::from_millis(750)
        );

        let before = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() / PERIOD_SECONDS;
        let actual = u64::from_be_bytes(constant_time_utils::time::current_period());
        let after = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() / PERIOD_SECONDS;
        assert!(actual == before || actual == after);
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
