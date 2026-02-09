use pingora::server::Server;
use pingora_proxy::http_proxy_service;
use pingora_web::App as WebApp;
use rand_core::{OsRng, RngCore};
use simple_filter_box::bloom_filter::bloom_filter::BloomFilterBox;
use star_core::serv::enclave::Enclave;
use std::sync::Mutex;

use crate::{
    api::{DEFAULT_PUBLIC_KEY_API, DEFAULT_REGISTER_API, issue_api, public_key_api},
    core::ENCLAVE,
    proxy::StarProxy,
};


pub mod core;
pub mod api;
pub mod proxy;


pub fn build_server(api_addr: &str, proxy_addr: &str, upstream_addr: &str) -> Server {
    ENCLAVE.get_or_init(|| {
        let mut rng = OsRng;
        let mut cipher_receiver = [0u8; 32];
        let mut sk = [0u8; 32];
        rng.fill_bytes(&mut cipher_receiver);
        rng.fill_bytes(&mut sk);
        Enclave::new(1, sk, cipher_receiver).expect("init enclave")
    });
    
    let mut server = Server::new(None).expect("create server");
    server.bootstrap();

    let mut api_app = WebApp::default();
    api_app.get_fn(DEFAULT_PUBLIC_KEY_API, public_key_api);
    api_app.post_fn(DEFAULT_REGISTER_API, issue_api);

    let mut api_svc = api_app.to_service("api");
    api_svc.add_tcp(api_addr);
    server.add_service(api_svc);

    let filter_box: BloomFilterBox = BloomFilterBox::with_rate(1, 1e-9);
    let mut proxy_svc = http_proxy_service(
        &server.configuration,
        StarProxy {
            filter_box: Mutex::new(filter_box),
            upstream_addr: upstream_addr.to_string(),
        },
    );
    proxy_svc.add_tcp(proxy_addr);
    server.add_service(proxy_svc);

    server
}

#[cfg(test)]
mod tests;
