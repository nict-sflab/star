use star_app::{all_prefix, build_server};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let server = build_server(
        "127.0.0.1:8080",
        "127.0.0.1:8081",
        "127.0.0.1:3000",
        all_prefix(1),
        1,
    )?;
    server.run_forever()
}
