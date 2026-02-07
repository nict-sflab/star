use async_trait::async_trait;
use pingora::upstreams::peer::HttpPeer;
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
}
