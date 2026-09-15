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
}

impl StarUpstream {
    pub fn new(host: String, port: u16, server_name: String) -> Self {
        Self {
            host,
            port,
            server_name,
        }
    }

    pub(crate) async fn connect(&self, star_token: Vec<u8>) -> io::Result<SslStream<TcpStream>> {
        let tcp = TcpStream::connect((self.host.as_str(), self.port)).await?;
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
