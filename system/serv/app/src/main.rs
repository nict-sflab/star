use star_app::{Shard, build_server};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    if std::env::args().nth(1).as_deref() == Some("--prepare-enclave") {
        println!(
            "{}",
            star_app::enclave::EnclaveRuntime::prepare_image()?.display()
        );
        return Ok(());
    }
    let api_addr = env_or_default("STAR_API_ADDR", "0.0.0.0:8080");
    let proxy_addr = env_or_default("STAR_PROXY_ADDR", "0.0.0.0:8081");
    let upstream_addr = env_or_default("STAR_UPSTREAM_ADDR", "0.0.0.0:3000");
    let shard = Shard::new(
        env_or_default("STAR_SHARD_INDEX", "0").parse()?,
        env_or_default("STAR_SHARD_COUNT", "1").parse()?,
    );
    let server = build_server(&api_addr, &proxy_addr, &upstream_addr, shard)?;
    server.run_forever()
}

fn env_or_default(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}
