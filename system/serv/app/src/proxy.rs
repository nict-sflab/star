use async_trait::async_trait;
use pingora::upstreams::peer::HttpPeer;
use pingora_http::ResponseHeader;
use pingora_proxy::{ProxyHttp, Session};
use std::time::Duration;

pub(crate) struct UpstreamProxy {
    upstream_addr: String,
}

impl UpstreamProxy {
    pub(crate) fn new(upstream_addr: impl Into<String>) -> Self {
        Self {
            upstream_addr: upstream_addr.into(),
        }
    }
}

#[async_trait]
impl ProxyHttp for UpstreamProxy {
    type CTX = ();

    fn new_ctx(&self) -> Self::CTX {
        ()
    }

    async fn request_filter(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<bool> {
        if session.req_header().method != "OPTIONS" {
            return Ok(false);
        }

        let header = ResponseHeader::build(204, None)?;
        session
            .write_response_header(Box::new(header), true)
            .await?;
        Ok(true)
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        let sni = self
            .upstream_addr
            .split(':')
            .next()
            .unwrap_or(self.upstream_addr.as_str())
            .to_string();
        let mut peer = HttpPeer::new(self.upstream_addr.as_str(), false, sni);
        peer.options.connection_timeout = Some(Duration::from_secs(3));
        peer.options.total_connection_timeout = Some(Duration::from_secs(5));
        Ok(Box::new(peer))
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        _upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<()>
    where
        Self::CTX: Send + Sync,
    {
        Ok(())
    }
}
