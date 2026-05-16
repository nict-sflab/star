use openssl::ssl::{ExtensionContext, SslConnector, SslMethod, SslVerifyMode};
use pingora::protocols::Stream;
use pingora::protocols::tls::SslStream;
use pingora::server::{Server};
use pingora::services::listening::Service;
use std::io;
use tokio::io::{AsyncWriteExt};
use tokio::net::TcpStream;
use crate::auth::StarAuthenticator;
use crate::{BoxError, STAR_TLS_EXTENSION_TYPE};
use crate::config::ProxyConfig;
use crate::utils::{io_error, io_other, read_header};

pub struct StarReverseProxy {
    upstream_host: String,
    upstream_port: u16,
    upstream_sni: String,
    authenticator: StarAuthenticator,
    insecure_tls: bool,
}

impl StarReverseProxy {
    pub fn new(config: ProxyConfig) -> Result<Self, BoxError> {
        let authenticator = StarAuthenticator::register(config.api_base.clone(), config.max_count)?;

        Ok(Self {
            upstream_host: config.upstream_host,
            upstream_port: config.upstream_port,
            upstream_sni: config.upstream_sni,
            authenticator,
            insecure_tls: config.insecure_tls,
        })
    }

    pub async fn handle(&self, downstream: &mut Stream) -> io::Result<()> {
        let request_head = read_header(downstream).await?;

        let token = self
            .authenticator
            .token_request()
            .map_err(|e| io_error(e.to_string()))?;

        let mut upstream = self.connect_upstream(token).await?;

        upstream.write_all(&request_head).await?;
        upstream.flush().await?;

        let _ = tokio::io::copy_bidirectional(&mut **downstream, &mut upstream).await;

        Ok(())
    }

    async fn connect_upstream(&self, star_token: Vec<u8>) -> io::Result<SslStream<TcpStream>> {
        let tcp = TcpStream::connect((self.upstream_host.as_str(), self.upstream_port)).await?;

        let mut builder = SslConnector::builder(SslMethod::tls()).map_err(io_other)?;

        if self.insecure_tls {
            builder.set_verify(SslVerifyMode::NONE);
        }

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
            .map_err(io_other)?;

        let ssl = builder
            .build()
            .configure()
            .map_err(io_other)?
            .into_ssl(&self.upstream_sni)
            .map_err(io_other)?;

        let mut tls = SslStream::new(ssl, tcp).map_err(io_other)?;
        tls.connect().await.map_err(io_other)?;

        Ok(tls)
    }
}

pub fn build_server(config: ProxyConfig) -> Result<Server, BoxError> {
    let listen_addr = config.listen_addr.clone();
    let proxy = StarReverseProxy::new(config)?;

    let mut server = Server::new(None)?;
    server.bootstrap();

    let mut service = Service::new("STAR reverse proxy".to_string(), proxy);
    service.add_tcp(&listen_addr);
    server.add_service(service);

    Ok(server)
}


