use async_trait::async_trait;
use pingora::upstreams::peer::HttpPeer;
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::{ProxyHttp, Session};
use std::sync::Mutex;
use std::time::Duration;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

use simple_filter_box::core::{Entry, FilterBox};

use crate::core::ENCLAVE;

pub struct StarProxy<T: FilterBox + Send> {
    pub filter_box: Mutex<T>,
    pub upstream_addr: String,
}

fn with_cors(res: &mut ResponseHeader, req: &RequestHeader) -> pingora::Result<()> {
    res.insert_header("access-control-allow-origin", "*")?;
    if let Some(req_method) = req
        .headers
        .get("access-control-request-method")
        .and_then(|v| v.to_str().ok())
        .filter(|v| !v.is_empty())
    {
        res.insert_header("access-control-allow-methods", req_method)?;
    } else {
        res.insert_header("access-control-allow-methods", "GET, POST, OPTIONS")?;
    }
    if let Some(req_headers) = req
        .headers
        .get("access-control-request-headers")
        .and_then(|v| v.to_str().ok())
        .filter(|v| !v.is_empty())
    {
        res.insert_header("access-control-allow-headers", req_headers)?;
    } else {
        res.insert_header("access-control-allow-headers", "*")?;
    }
    res.insert_header("access-control-max-age", "86400")?;
    Ok(())
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
        println!("method: {}\n", session.req_header().method);

        if session.req_header().method == "OPTIONS" {
            let mut header = ResponseHeader::build(204, None).unwrap();
            with_cors(&mut header, session.req_header())?;
            session.write_response_header(Box::new(header), true).await?;
            return Ok(true);
        }

        let req_headers = &session.req_header().headers;
        let target = req_headers
            .get("X-STAR")
            .or_else(|| req_headers.get("X-STAR"));

        let x = match target {
            Some(x) => x,
            None => {
                let mut header = ResponseHeader::build(400, None).unwrap();
                with_cors(&mut header, session.req_header())?;
                session.write_response_header(Box::new(header), true).await?;
                return Ok(true);
            }
        };

        let decoded = URL_SAFE.decode(x).unwrap();

        let enclave = ENCLAVE.get().unwrap();
        let token = enclave.issue_token(&decoded).unwrap();
        let token = token.as_slice().try_into().unwrap();

        let mut filter_box = self.filter_box.lock().unwrap();
        let is_new = filter_box.test(&token);

        if is_new {
            let _ = filter_box.insert(&token);
            Ok(false)
        } else {
            Ok(true)
        }
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
