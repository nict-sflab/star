use std::io;

use openssl::ssl::{ExtensionContext, SslConnector, SslMethod, SslVerifyMode};
use pingora::protocols::tls::SslStream;
use tokio::net::TcpStream;

const STAR_TLS_EXTENSION_TYPE: u16 = 65280;

#[derive(Clone, Debug)]
pub struct StarUpstream {
    host: String,
    port: u16,
    server_name: String,
    shards: Vec<String>,
}

impl StarUpstream {
    pub fn new(host: String, port: u16, server_name: String) -> Self {
        Self {
            host,
            port,
            server_name,
            shards: Vec::new(),
        }
    }

    /// Endpoints in shard-index order. All shards use the same TLS server name.
    pub fn with_shards(mut self, endpoints: Vec<String>) -> io::Result<Self> {
        if endpoints.is_empty() || endpoints.len() > u32::MAX as usize {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "expected at least one shard endpoint",
            ));
        }
        for endpoint in &endpoints {
            let valid = endpoint.rsplit_once(':').is_some_and(|(host, port)| {
                !host.is_empty() && port.parse::<u16>().is_ok_and(|port| port != 0)
            });
            if !valid {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "shard endpoint must be host:port",
                ));
            }
        }
        self.shards = endpoints;
        Ok(self)
    }

    pub(crate) fn shard_count(&self) -> u32 {
        self.shards.len().max(1) as u32
    }

    fn endpoint(&self, shard: u32) -> io::Result<String> {
        if self.shards.is_empty() && shard == 0 {
            return Ok(format!("{}:{}", self.host, self.port));
        }
        self.shards
            .get(shard as usize)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "invalid STAR shard"))
    }

    pub(crate) async fn connect(
        &self,
        star_token: Vec<u8>,
        shard: u32,
    ) -> io::Result<SslStream<TcpStream>> {
        let tcp = if self.shards.is_empty() && shard == 0 {
            TcpStream::connect((self.host.as_str(), self.port)).await?
        } else {
            TcpStream::connect(self.endpoint(shard)?).await?
        };
        let mut builder = SslConnector::builder(SslMethod::tls()).map_err(io::Error::other)?;

        builder.set_verify(SslVerifyMode::NONE);

        builder
            .add_custom_ext(
                STAR_TLS_EXTENSION_TYPE,
                ExtensionContext::TLS_ONLY
                    | ExtensionContext::CLIENT_HELLO
                    | ExtensionContext::TLS1_2_AND_BELOW_ONLY
                    | ExtensionContext::TLS1_3_ONLY,
                move |_, _, _| Ok(Some(star_token.clone())),
                |_, _, _, _| Ok(()),
            )
            .map_err(io::Error::other)?;

        let ssl = builder
            .build()
            .configure()
            .map_err(io::Error::other)?
            .into_ssl(&self.server_name)
            .map_err(io::Error::other)?;
        let mut tls = SslStream::new(ssl, tcp).map_err(io::Error::other)?;
        tls.connect().await.map_err(io::Error::other)?;

        Ok(tls)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shard_endpoints_preserve_index_order_and_reject_invalid_routes() {
        let upstream = StarUpstream::new("localhost".into(), 8081, "localhost".into());
        assert_eq!(upstream.shard_count(), 1);
        assert!(upstream.endpoint(1).is_err());
        assert!(upstream.clone().with_shards(vec![]).is_err());
        assert!(upstream.clone().with_shards(vec!["host:0".into()]).is_err());
        let upstream = upstream
            .with_shards(vec!["a:8081".into(), "b:8083".into()])
            .unwrap();
        assert_eq!(upstream.shard_count(), 2);
        assert_eq!(upstream.endpoint(0).unwrap(), "a:8081");
        assert_eq!(upstream.endpoint(1).unwrap(), "b:8083");
        assert!(upstream.endpoint(2).is_err());
    }
}
