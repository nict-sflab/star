use star_app::key_manager_service::{
    DEFAULT_KEY_MANAGER_ADDR, STAR_KEY_MANAGER_ADDR, build_key_manager_server,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = std::env::var(STAR_KEY_MANAGER_ADDR)
        .unwrap_or_else(|_| DEFAULT_KEY_MANAGER_ADDR.to_string());
    let server = build_key_manager_server(&addr)?;
    server.run_forever()
}
