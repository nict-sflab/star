use pingora::upstreams::peer::HttpPeer;
use pingora_proxy::{ProxyHttp, Session};
use async_trait::async_trait;
use std::time::Duration;
use std::sync::Mutex;

use base64::{engine::general_purpose::URL_SAFE, Engine as _};

use simple_filter_box::core::{Entry, FilterBox};

use crate::core::ENCLAVE;

pub struct StarProxy<T: FilterBox + Send> {
    pub filter_box: Mutex<T>,
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
