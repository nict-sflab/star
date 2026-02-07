use async_trait::async_trait;
use simple_filter_box::bloom_filter::bloom_filter::BloomFilterBox;
use simple_filter_box::core::{Entry, FilterBox};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use pingora::server::Server;
use pingora::upstreams::peer::HttpPeer;
use pingora_proxy::{http_proxy_service, ProxyHttp, Session};
use pingora_web::{App as WebApp, PingoraHttpRequest, PingoraWebHttpResponse, StatusCode, WebError};

use rand_core::OsRng; // <-- matches rand_core = "0.6"
use star_core::serv::enclave::Enclave;

static ENCLAVE: OnceLock<Enclave> = OnceLock::new();

struct StarProxy<T: FilterBox + Send> {
    filter_box: Mutex<T>,
}

#[async_trait]
impl<T: FilterBox + Send> ProxyHttp for StarProxy<T> {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX { () }

    async fn request_filter(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<bool> {
        // NOTE: header names are case-insensitive in HTTP, but your map behavior depends on Pingora.
        // If this never matches, try "connection" instead of "CONNECTION".
        let target = session.req_header().headers.get("CONNECTION");

        let x = match target {
            Some(x) => x,
            None => { panic!("ok"); },
        };

        let decoded = URL_SAFE.decode(x).unwrap();

        let enclave = ENCLAVE.get().unwrap();
        let token = enclave.issue_token(&decoded).unwrap();
        let token: Entry = token.as_slice().try_into().unwrap();

        let mut filter_box = self.filter_box.lock().unwrap();
        let has_used = filter_box.test(&token);

        if has_used {
            return Ok(true);
        } else {
            let _ = filter_box.insert(&token);
            return Ok(false);
        }
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        let mut peer = HttpPeer::new(("0.0.0.0", 3000), false, "0.0.0.0".to_string());
        peer.options.connection_timeout = Some(Duration::from_secs(3));
        peer.options.total_connection_timeout = Some(Duration::from_secs(5));
        Ok(Box::new(peer))
    }
}

fn main() {
    env_logger::init();

    // init enclave once
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
    api_app.get_fn("/star/register", |req: PingoraHttpRequest| -> Result<PingoraWebHttpResponse, WebError> {
        let body = req.body().as_ref();
        let decoded = URL_SAFE.decode(body).unwrap();

        // Don’t share an RNG across requests; create one per request.
        let mut rng = OsRng;
        let enclave = ENCLAVE.get().unwrap();
        let cred = enclave.issue_cred(&decoded, &mut rng);

        let out = URL_SAFE.encode(cred);
        Ok(PingoraWebHttpResponse::text(StatusCode::OK, out))
    });

    let mut api_svc = api_app.to_service("api");
    api_svc.add_tcp("0.0.0.0:8080");
    server.add_service(api_svc);

    let filter_box = BloomFilterBox::with_rate(1, 0.01);
    let mut proxy_svc = http_proxy_service(
        &server.configuration,
        StarProxy { filter_box: Mutex::new(filter_box) },
    );
    proxy_svc.add_tcp("0.0.0.0:8081");
    server.add_service(proxy_svc);

    server.run_forever();
}
