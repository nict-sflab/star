fn main() {
    if let Err(err) = enclave::run_stdio() {
        eprintln!("enclave stdio service failed: {err}");
        std::process::exit(1);
    }
}
