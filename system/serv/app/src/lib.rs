use pingora::{server::Server, tls::listeners::TlsSettings};
use pingora_proxy::http_proxy_service;
use pingora_web::App as WebApp;
use rand::distr::Map;
use rand_core::{OsRng, RngCore};
use simple_filter_box::{bloom_filter::bloom_filter::BloomFilterBox, core::FilterBox};
use star_core::serv::enclave::Enclave;
use tokio::sync::Mutex;
use std::{collections::HashMap};
use std::env;

use crate::{
    api::{DEFAULT_PUBLIC_KEY_API, DEFAULT_REGISTER_API, issue_api, public_key_api},
    cors::Cors,
    core::ENCLAVE,
    proxy::StarProxy,
};


pub mod core;
pub mod api;
pub mod proxy;
pub mod cors;

const MAX_COUNT : u32 = 10;

pub fn all_prefix(max: u32) -> Vec<u32> {
    (0u32..max).collect()
}

fn env_or_default(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

pub fn build_server(api_addr: &str, proxy_addr: &str, upstream_addr: &str, prefixes: Vec<u32>, max_prefix: u32) -> Server {
    ENCLAVE.get_or_init(|| {
        let mut rng = OsRng;
        let mut cipher_receiver = [0u8; 32];
        let mut sk = [0u8; 32];
        rng.fill_bytes(&mut cipher_receiver);
        rng.fill_bytes(&mut sk);
        Enclave::new(MAX_COUNT, sk, cipher_receiver).expect("init enclave")
    });
    
    let mut server = Server::new(None).expect("create server");
    server.bootstrap();

    let mut api_app = WebApp::default();
    api_app.use_middleware(Cors::default());
    api_app.get_fn(DEFAULT_PUBLIC_KEY_API, public_key_api);
    api_app.post_fn(DEFAULT_REGISTER_API, issue_api);

    let mut api_svc = api_app.to_service("api");
    api_svc.add_tcp(api_addr);
    server.add_service(api_svc);

    let mut filter_boxes = HashMap::new();

    for prefix in prefixes {
        let filter_box: BloomFilterBox = BloomFilterBox::with_rate(1, 1e-9);
        filter_boxes.insert(prefix, Mutex::new(filter_box));
    }

    let mut proxy_svc = http_proxy_service(
        &server.configuration,
        StarProxy {
            filters: filter_boxes,
            prefix_max: max_prefix,
            upstream_addr: upstream_addr.to_string(),
        },
    );

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
