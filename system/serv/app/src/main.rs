use async_trait::async_trait;
use std::time::Duration;

use pingora::server::Server;
use pingora::upstreams::peer::HttpPeer;

use pingora_http::RequestHeader;
use pingora_proxy::{http_proxy_service, ProxyHttp, Session};

use pingora_web::{
    App as WebApp, PingoraHttpRequest, PingoraWebHttpResponse, StatusCode, WebError,
};
struct StarProxy;

#[async_trait]
impl ProxyHttp for StarProxy {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn request_filter(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<bool> { 
        let target = session.req_header().headers.get("CONNECTION");
        println!("{:?}", target);

        Ok(false) 
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        let mut peer = HttpPeer::new(("example.com", 80), false, "example.com".to_string());
        peer.options.connection_timeout = Some(Duration::from_secs(3));
        peer.options.total_connection_timeout = Some(Duration::from_secs(5));
        Ok(Box::new(peer))
    }

//     async fn upstream_request_filter(
//         &self,
//         _session: &mut Session,
//         upstream_request: &mut RequestHeader,
//         _ctx: &mut Self::CTX,
//     ) -> pingora::Result<()> {
//         upstream_request.insert_header("Host", "example.com").unwrap();
//     Ok(())
// }
}

fn main() {
    env_logger::init();

    let mut server = Server::new(None).unwrap();
    server.bootstrap();

    let mut api_app = WebApp::default();
    api_app.get_fn(
        "/star/register",
        |_req: PingoraHttpRequest| -> Result<PingoraWebHttpResponse, WebError> {
            Ok(PingoraWebHttpResponse::text(StatusCode::OK, "ok"))
        },
    );

    let mut api_svc = api_app.to_service("api");
    api_svc.add_tcp("0.0.0.0:8080");
    server.add_service(api_svc);

    let mut proxy_svc = http_proxy_service(&server.configuration, StarProxy);
    proxy_svc.add_tcp("0.0.0.0:8081");
    server.add_service(proxy_svc);

    server.run_forever();
}
