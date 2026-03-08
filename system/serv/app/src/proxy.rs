use async_trait::async_trait;
use pingora::upstreams::peer::HttpPeer;
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::{ProxyHttp, Session};
use pingora_web::StatusCode;
use star_core::serv::core::bytes::Long;
use star_core::serv::core::token::Token;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::time::Duration;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

use simple_filter_box::core::{Entry, FilterBox};

use crate::core::ENCLAVE;
use crate::cors::with_cors;

pub struct StarProxy<T: FilterBox + Send> {
    pub filters: HashMap<u32, Mutex<T>>,
    pub prefix_max: u32,
    pub upstream_addr: String,
}

async fn exit_filter(status: impl TryInto<StatusCode>, session: &mut Session) -> pingora::Result<bool> {
    let mut header = ResponseHeader::build(status, None).unwrap();
    with_cors(&mut header, session.req_header())?;
    session.write_response_header(Box::new(header), true).await?;
    Ok(true)
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
            return exit_filter(204, session).await;
        }

        let req_headers = &session.req_header().headers;
        let target = req_headers
            .get("X-STAR")
            .or_else(|| req_headers.get("X-STAR"));

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

        const MIN_HPKE_LEN: usize = 32 + 32 + 32;
        if decoded.len() < MIN_HPKE_LEN {
            return exit_filter(400, session).await;
        }

        let enclave = ENCLAVE.get().unwrap();
        let current_period = constant_time_utils::time::current_period();

        let token_bytes = match enclave.issue_token(&decoded, current_period) {
            Ok(v) => v,
            Err(_) => {
                return exit_filter(403, session).await;
            }
        };

        let token = Token::new(token_bytes.try_into().unwrap());
        let route = token.route(self.prefix_max);

        let filter = match self.filters.get(&route) {
            Some(f) => f,
            None => {
                println!("{}", route);
                return exit_filter(403, session).await;
            }
        };

        let is_new = {
            let mut filter_box = filter.lock().await;
            let is_new = filter_box.test(&token.token);
            if is_new {
                let _ = filter_box.insert(&token.token);
            }
            is_new
        };

        if is_new {
            Ok(false)
        } else {
            return exit_filter(403, session).await;
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
