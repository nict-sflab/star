use star_client_proxy::{build_server, ProxyConfig};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    env_logger::init();
    let server = build_server(ProxyConfig::from_env())?;
    server.run_forever()
}
