use star_app::{all_prefix, build_server};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let server = build_server(
        "0.0.0.0:8080",
        "0.0.0.0:8081",
        "0.0.0.0:3000",
        all_prefix(1),
        1,
    )?;
    server.run_forever()
}
