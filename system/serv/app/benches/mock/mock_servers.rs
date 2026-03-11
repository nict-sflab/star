use star_app::{all_prefix, build_server};
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const E2E_API_ADDR_ENV: &str = "STAR_E2E_API_ADDR";
const E2E_PROXY_ADDR_ENV: &str = "STAR_E2E_PROXY_ADDR";
const E2E_UPSTREAM_ADDR_ENV: &str = "STAR_E2E_UPSTREAM_ADDR";
const E2E_MAX_PREFIX_ENV: &str = "STAR_E2E_MAX_PREFIX";
const E2E_API_THREADS_ENV: &str = "STAR_E2E_API_THREADS";
const E2E_PROXY_THREADS_ENV: &str = "STAR_E2E_PROXY_THREADS";
const E2E_TLS_CERT_PATH_ENV: &str = "STAR_E2E_TLS_CERT_PATH";
const E2E_TLS_KEY_PATH_ENV: &str = "STAR_E2E_TLS_KEY_PATH";

const DEFAULT_API_ADDR: &str = "0.0.0.0:8080";
const DEFAULT_PROXY_ADDR: &str = "0.0.0.0:8443";
const DEFAULT_UPSTREAM_ADDR: &str = "127.0.0.1:3000";
const DEFAULT_MAX_PREFIX: u32 = 4;
const DEFAULT_API_THREADS: usize = 1;
const DEFAULT_PROXY_THREADS: usize = 2;

fn env_or_default(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

fn parse_env_u32(name: &str, default: u32) -> u32 {
    match env::var(name) {
        Ok(raw) => raw
            .parse::<u32>()
            .unwrap_or_else(|err| panic!("invalid {name} value {raw:?}: {err}")),
        Err(_) => default,
    }
}

fn parse_env_usize(name: &str, default: usize) -> usize {
    match env::var(name) {
        Ok(raw) => raw
            .parse::<usize>()
            .unwrap_or_else(|err| panic!("invalid {name} value {raw:?}: {err}")),
        Err(_) => default,
    }
}

fn default_tls_cert_path() -> String {
    format!("{}/examples/certs/server.crt", env!("CARGO_MANIFEST_DIR"))
}

fn default_tls_key_path() -> String {
    format!("{}/examples/certs/server.key", env!("CARGO_MANIFEST_DIR"))
}

fn respond_ok(mut stream: TcpStream) {
    let mut request_buffer = [0u8; 1024];
    let _ = stream.read(&mut request_buffer);

    let response = b"HTTP/1.1 200 OK\r\ncontent-length: 2\r\nconnection: close\r\n\r\nOK";
    let _ = stream.write_all(response);
    let _ = stream.flush();
}

fn spawn_upstream(bind_addr: &str) -> thread::JoinHandle<()> {
    let listener = TcpListener::bind(bind_addr).expect("bind e2e upstream");
    thread::spawn(move || {
        for incoming in listener.incoming() {
            match incoming {
                Ok(stream) => respond_ok(stream),
                Err(err) => eprintln!("upstream accept error: {err}"),
            }
        }
    })
}

fn main() {
    env_logger::init();

    let api_addr = env_or_default(E2E_API_ADDR_ENV, DEFAULT_API_ADDR);
    let proxy_addr = env_or_default(E2E_PROXY_ADDR_ENV, DEFAULT_PROXY_ADDR);
    let upstream_addr = env_or_default(E2E_UPSTREAM_ADDR_ENV, DEFAULT_UPSTREAM_ADDR);
    let max_prefix = parse_env_u32(E2E_MAX_PREFIX_ENV, DEFAULT_MAX_PREFIX);
    let api_threads = parse_env_usize(E2E_API_THREADS_ENV, DEFAULT_API_THREADS);
    let proxy_threads = parse_env_usize(E2E_PROXY_THREADS_ENV, DEFAULT_PROXY_THREADS);
    let tls_cert_path = env::var(E2E_TLS_CERT_PATH_ENV).unwrap_or_else(|_| default_tls_cert_path());
    let tls_key_path = env::var(E2E_TLS_KEY_PATH_ENV).unwrap_or_else(|_| default_tls_key_path());

    unsafe {
        env::set_var("TLS_CERT_PATH", &tls_cert_path);
        env::set_var("TLS_KEY_PATH", &tls_key_path);
        env::set_var("STAR_API_THREADS", api_threads.to_string());
        env::set_var("STAR_PROXY_THREADS", proxy_threads.to_string());
    }

    let _upstream = spawn_upstream(&upstream_addr);
    println!(
        "starting e2e benchmark server: api={api_addr}, proxy={proxy_addr}, upstream={upstream_addr}, max_prefix={max_prefix}, api_threads={api_threads}, proxy_threads={proxy_threads}, tls_cert_path={tls_cert_path}"
    );

    let server = build_server(
        &api_addr,
        &proxy_addr,
        &upstream_addr,
        all_prefix(max_prefix),
        max_prefix,
    );
    server.run_forever();
}
