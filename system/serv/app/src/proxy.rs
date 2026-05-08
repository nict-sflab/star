use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use pingora::upstreams::peer::HttpPeer;
use pingora_http::ResponseHeader;
use pingora_proxy::{ProxyHttp, Session};
use pingora_web::StatusCode;
use std::time::Duration;
use tokio::sync::Mutex;

use simple_filter_box::core::FilterBox;

use crate::cors::with_cors;
use crate::filter::StarFilter;

pub struct StarProxy<T: FilterBox + Send> {
    pub filter: Mutex<StarFilter<T>>,
    pub upstream_addr: String,
}

async fn exit_filter(
    status: impl TryInto<StatusCode>,
    session: &mut Session,
) -> pingora::Result<bool> {
    let mut header = ResponseHeader::build(status, None)?;
    with_cors(&mut header, session.req_header())?;
    session
        .write_response_header(Box::new(header), true)
        .await?;
    Ok(true)
}

#[async_trait]
impl<T: FilterBox + Send> ProxyHttp for StarProxy<T> {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn request_filter(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<bool> {
        if session.req_header().method == "OPTIONS" {
            return exit_filter(204, session).await;
        }

        let req_headers = &session.req_header().headers;
        let target = req_headers.get("X-STAR");

        let x = match target {
            Some(x) => x,
            None => {
                return exit_filter(403, session).await;
            }
        };

        let decoded = match URL_SAFE.decode(x) {
            Ok(v) => v,
            Err(_) => {
                return exit_filter(400, session).await;
            }
        };

        match self.filter.lock().await.filter(decoded.as_slice()).await {
            Ok(_) => Ok(false),
            Err(err) => exit_filter(err.status_code(), session).await,
        }
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        let sni = match self.upstream_addr.split(':').next() {
            Some(host) => host,
            None => self.upstream_addr.as_str(),
        }
        .to_string();
        let mut peer = HttpPeer::new(self.upstream_addr.as_str(), false, sni);
        peer.options.connection_timeout = Some(Duration::from_secs(3));
        peer.options.total_connection_timeout = Some(Duration::from_secs(5));
        Ok(Box::new(peer))
    }

    async fn response_filter(
        &self,
        session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<()>
    where
        Self::CTX: Send + Sync,
    {
        with_cors(upstream_response, session.req_header())
    }
}
