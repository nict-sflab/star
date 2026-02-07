mod core;
mod api;
mod proxy;

use simple_filter_box::bloom_filter::bloom_filter::BloomFilterBox;
use std::sync::{Mutex};

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use pingora::server::Server;
use pingora_proxy::{http_proxy_service};
use pingora_web::{App as WebApp};

use rand_core::OsRng;

use crate::api::{issue_api, DEFAULT_REGISTER_API};
use crate::core::ENCLAVE;

use crate::proxy::StarProxy;

use star_core::serv::enclave::Enclave;


fn main() {
    env_logger::init();

    let mut rng = OsRng;
    let enclave = Enclave::default(&mut rng);
    if ENCLAVE.set(enclave).is_err() {
        panic!("failed to init ENCLAVE (already initialized)");
    }

    let data = ENCLAVE.get().unwrap().public_key();
    let encode = URL_SAFE.encode(data);

    println!("public key: {}", encode);

    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    let mut api_app = WebApp::default();
    api_app.get_fn(DEFAULT_REGISTER_API, issue_api);

    let mut api_svc = api_app.to_service("api");
    api_svc.add_tcp("0.0.0.0:8080");
    server.add_service(api_svc);

    let filter_box: BloomFilterBox = BloomFilterBox::with_rate(1, 0.01);
    let mut proxy_svc = http_proxy_service(
        &server.configuration,
        StarProxy { filter_box: Mutex::new(filter_box) },
    );

    proxy_svc.add_tcp("0.0.0.0:8081");
    server.add_service(proxy_svc);
    server.run_forever();
}
