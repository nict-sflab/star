use async_trait::async_trait;
use pingora::upstreams::peer::HttpPeer;
use pingora_http::ResponseHeader;
use pingora_proxy::{ProxyHttp, Session};
use simple_filter_box::hashmap::HashMapFilterBox;
use star_app::proxy::StarProxy;
use std::sync::mpsc;
use std::time::{Instant, SystemTime};

pub struct StarLatencyProxy {
    pub inner: StarProxy<HashMapFilterBox>,
    pub latency_tx: mpsc::Sender<u64>,
    pub bypass_request_filter: bool,
}

pub struct StarLatencyCtx {
    start: Instant,
    inner_ctx: (),
}

#[async_trait]
impl ProxyHttp for StarLatencyProxy {
    type CTX = StarLatencyCtx;

    fn new_ctx(&self) -> Self::CTX {
        Self::CTX {
            start: Instant::now(),
            inner_ctx: self.inner.new_ctx(),
        }
    }

    async fn early_request_filter(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<()>
    where
        Self::CTX: Send + Sync,
    {
        self.inner.early_request_filter(session, &mut ctx.inner_ctx).await
    }

    async fn request_filter(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<bool>
    where
        Self::CTX: Send + Sync,
    {
        if self.bypass_request_filter {
            return Ok(false);
        }
        self.inner.request_filter(session, &mut ctx.inner_ctx).await
    }

    async fn upstream_peer(
        &self,
        session: &mut Session,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        self.inner.upstream_peer(session, &mut ctx.inner_ctx).await
    }

    async fn response_filter(
        &self,
        session: &mut Session,
        upstream_response: &mut ResponseHeader,
        ctx: &mut Self::CTX,
    ) -> pingora::Result<()>
    where
        Self::CTX: Send + Sync,
    {
        self.inner
            .response_filter(session, upstream_response, &mut ctx.inner_ctx)
            .await
    }

    async fn logging(
        &self,
        session: &mut Session,
        e: Option<&pingora::Error>,
        ctx: &mut Self::CTX,
    ) where
        Self::CTX: Send + Sync,
    {
        self.inner.logging(session, e, &mut ctx.inner_ctx).await;
        let elapsed_ns = downstream_connection_elapsed_ns(session)
            .unwrap_or_else(|| ctx.start.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64);
        let _ = self.latency_tx.send(elapsed_ns);
    }
}

fn downstream_connection_elapsed_ns(session: &Session) -> Option<u64> {
    let established_at = session
        .as_downstream()
        .digest()?
        .timing_digest
        .iter()
        .flatten()
        .map(|timing| timing.established_ts)
        .filter(|ts| *ts != SystemTime::UNIX_EPOCH)
        .min()?;

    let elapsed = SystemTime::now().duration_since(established_at).ok()?;
    Some(elapsed.as_nanos().min(u128::from(u64::MAX)) as u64)
}
