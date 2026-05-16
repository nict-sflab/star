use async_trait::async_trait;
use pingora::upstreams::peer::HttpPeer;
use pingora_http::ResponseHeader;
use pingora_proxy::{ProxyHttp, Session};
use pingora_web::StatusCode;
use std::sync::Arc;
use std::time::Duration;

use simple_filter_box::core::FilterBox;

use crate::cors::with_cors;
use crate::filter::StarFilter;
use crate::tls_token::request_from_tls_extension;

pub struct StarProxy<T: FilterBox + Send> {
    pub filter: Arc<StarFilter<T>>,
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

fn request_from_tls_extension_required(session: &Session) -> Result<Vec<u8>, u16> {
    let stream = session.as_downstream().stream().ok_or(403u16)?;
    let ssl = stream.get_ssl().ok_or(403u16)?;
    match request_from_tls_extension(ssl) {
        Ok(request) => Ok(request),
        Err("empty STAR TLS extension") => Err(400u16),
        Err(_) => Err(403u16),
    }
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

        Ok(false)
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
        Ok(())
    }
}
