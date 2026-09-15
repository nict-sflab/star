use std::io;
use std::sync::Arc;

use async_trait::async_trait;
use log::debug;
use pingora::apps::ServerApp;
use pingora::protocols::Stream;
use pingora::server::{Server, ShutdownWatch};
use pingora::services::listening::Service;
use tokio::io::AsyncWriteExt;

use crate::auth::StarClient;
use crate::http::{CORS_PREFLIGHT_RESPONSE, HttpHead};
use crate::{BoxError, ProxyConfig};

struct StarReverseProxy {
    star: StarClient,
    upstream: crate::upstream::StarUpstream,
}

impl StarReverseProxy {
    async fn handle(&self, downstream: &mut Stream) -> Result<(), BoxError> {
        let mut request = HttpHead::read_from(downstream).await?;

        if request.is_cors_preflight() {
            (&mut **downstream)
                .write_all(CORS_PREFLIGHT_RESPONSE)
                .await?;
            (&mut **downstream).flush().await?;
            (&mut **downstream).shutdown().await?;
            return Ok(());
        }

        let token = self.star.token_request()?;
        let mut upstream = self.upstream.connect(token).await?;

        request.close_connection();
        upstream.write_all(request.as_bytes()).await?;
        upstream.flush().await?;

        let (mut downstream_read, mut downstream_write) = tokio::io::split(&mut **downstream);
        let (mut upstream_read, mut upstream_write) = tokio::io::split(upstream);

        let upload = async {
            let _ = tokio::io::copy(&mut downstream_read, &mut upstream_write).await;
            let _ = upstream_write.shutdown().await;
        };

        let download = async {
            let mut response = HttpHead::read_from(&mut upstream_read).await?;
            response.allow_cors();

            downstream_write.write_all(response.as_bytes()).await?;
            tokio::io::copy(&mut upstream_read, &mut downstream_write).await?;
            downstream_write.shutdown().await?;
            Ok::<(), io::Error>(())
        };

        tokio::pin!(upload);
        tokio::pin!(download);

        tokio::select! {
            result = &mut download => result?,
            _ = &mut upload => download.await?,
        }

        Ok(())
    }
}

#[async_trait]
impl ServerApp for StarReverseProxy {
    async fn process_new(
        self: &Arc<Self>,
        mut io: Stream,
        _shutdown: &ShutdownWatch,
    ) -> Option<Stream> {
        if let Err(error) = self.handle(&mut io).await {
            debug!("STAR reverse proxy connection closed: {error}");
        }
        None
    }
}

pub fn build_server(config: ProxyConfig) -> Result<Server, BoxError> {
    let listen_addr = config.listen_addr.clone();
    let proxy = StarReverseProxy {
        star: StarClient::register(
            &config.star_api_base,
            config.max_requests_per_period,
        )?,
        upstream: config.upstream,
    };

    let mut server = Server::new(None)?;
    server.bootstrap();

    let mut service = Service::new("STAR reverse proxy".to_string(), proxy);
    service.add_tcp(&listen_addr);
    server.add_service(service);

    Ok(server)
}
