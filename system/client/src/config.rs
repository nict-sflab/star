use std::env;

use star_core::serv::enclave::DEFAULT_MAX_TICKET;

use crate::upstream::StarUpstream;

#[derive(Clone, Debug)]
pub struct ProxyConfig {
    pub listen_addr: String,
    pub star_api_base: String,
    pub max_requests_per_period: u64,
    pub upstream: StarUpstream,
}

impl ProxyConfig {
    pub fn from_env() -> Self {
        let upstream_host =
            env::var("STAR_UPSTREAM_HOST").unwrap_or_else(|_| "localhost".to_string());
        let upstream_port = env::var("STAR_UPSTREAM_PORT")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(8081);
        let upstream_sni = env::var("STAR_UPSTREAM_SNI").unwrap_or_else(|_| upstream_host.clone());

        let mut upstream = StarUpstream::new(upstream_host, upstream_port, upstream_sni);
        if let Ok(endpoints) = env::var("STAR_UPSTREAM_SHARDS") {
            upstream = upstream
                .with_shards(endpoints.split(',').map(|s| s.trim().to_owned()).collect())
                .expect("invalid STAR_UPSTREAM_SHARDS");
        }

        Self {
            listen_addr: env::var("STAR_CLIENT_PROXY_LISTEN")
                .unwrap_or_else(|_| "127.0.0.1:18080".to_string()),
            star_api_base: env::var("STAR_API_BASE")
                .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string()),
            max_requests_per_period: env::var("STAR_MAX_COUNT")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(u64::from(DEFAULT_MAX_TICKET)),
            upstream,
        }
    }
}
