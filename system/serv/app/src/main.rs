use star_app::{build_server, Shard};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
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
