#[path = "../common/latency_proxy.rs"]
mod latency_proxy;

use latency_proxy::StarLatencyProxy;
use pingora::{listeners::tls::TlsSettings, server::Server};
use pingora_proxy::http_proxy_service;
use pingora_web::App as WebApp;
use simple_filter_box::dashmap::DashMapFilterBox;
use star_app::all_prefix;
use star_app::api::{DEFAULT_PUBLIC_KEY_API, DEFAULT_REGISTER_API, IssueApi, PublicKeyApi};
use star_app::enclave::{EnclaveRuntime, EnclaveRuntimes};
use star_app::filter::StarFilter;
use star_app::proxy::StarProxy;
use star_app::tls::configure_tls_token_extension;
use std::env;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, mpsc};
use std::thread;

const E2E_API_ADDR_ENV: &str = "STAR_E2E_API_ADDR";
const E2E_PROXY_ADDR_ENV: &str = "STAR_E2E_PROXY_ADDR";
const E2E_UPSTREAM_ADDR_ENV: &str = "STAR_E2E_UPSTREAM_ADDR";
const E2E_MAX_PREFIX_ENV: &str = "STAR_E2E_MAX_PREFIX";
const E2E_API_THREADS_ENV: &str = "STAR_E2E_API_THREADS";
const E2E_PROXY_THREADS_ENV: &str = "STAR_E2E_PROXY_THREADS";
const E2E_TLS_CERT_PATH_ENV: &str = "STAR_E2E_TLS_CERT_PATH";
const E2E_TLS_KEY_PATH_ENV: &str = "STAR_E2E_TLS_KEY_PATH";

const DEFAULT_API_ADDR: &str = "127.0.0.1:8080";
const DEFAULT_PROXY_ADDR: &str = "127.0.0.1:8443";
const DEFAULT_UPSTREAM_ADDR: &str = "127.0.0.1:3001";
const DEFAULT_MAX_PREFIX: u32 = 2;
const DEFAULT_API_THREADS: usize = 1;
const DEFAULT_PROXY_THREADS: usize = 2;
const TLS_CERT_RELATIVE_PATH: &str = "examples/certs/server.crt";
const TLS_KEY_RELATIVE_PATH: &str = "examples/certs/server.key";
const ONLY_PROXY: bool = false;

type BenchFilter = StarFilter<DashMapFilterBox>;

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

fn manifest_path(relative_path: &str) -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), relative_path)
}

fn default_tls_cert_path() -> String {
    manifest_path(TLS_CERT_RELATIVE_PATH)
}

fn default_tls_key_path() -> String {
    manifest_path(TLS_KEY_RELATIVE_PATH)
}

fn build_server_with_latency(
    api_addr: &str,
    proxy_addr: &str,
    upstream_addr: &str,
    prefixes: Vec<u32>,
    max_prefix: u32,
    api_threads: usize,
    proxy_threads: usize,
    tls_cert_path: &str,
    tls_key_path: &str,
    latency_tx: mpsc::Sender<u64>,
) -> Result<Server, Box<dyn Error>> {
    let mut issue_enclave = EnclaveRuntime::from_env()?;
    let filter_enclave = EnclaveRuntimes::from_env(10)?;

    let mut server = Server::new(None).expect("create server");
    server.bootstrap();

    let mut api_app = WebApp::default();
    let public_key_api = PublicKeyApi::init(&mut issue_enclave)?;
    let issue_api = IssueApi::init(issue_enclave);
    api_app.get(DEFAULT_PUBLIC_KEY_API, Arc::new(public_key_api));
    api_app.post(DEFAULT_REGISTER_API, Arc::new(issue_api));

    let mut api_svc = api_app.to_service("api");
    api_svc.threads = Some(api_threads);
    api_svc.add_tcp(api_addr);
    server.add_service(api_svc);

    let filter = Arc::new(BenchFilter::new(
        DashMapFilterBox::new(),
        prefixes,
        max_prefix,
        filter_enclave,
    ));

    let mut proxy_svc = http_proxy_service(
        &server.configuration,
        StarLatencyProxy {
            inner: StarProxy {
                filter: Arc::clone(&filter),
                upstream_addr: upstream_addr.to_string(),
            },
            latency_tx,
            bypass_request_filter: ONLY_PROXY,
        },
    );
    proxy_svc.threads = Some(proxy_threads);

    let mut tls_settings =
        TlsSettings::intermediate(tls_cert_path, tls_key_path).expect("build pingora TLS settings");
    if !ONLY_PROXY {
        configure_tls_token_extension(&mut tls_settings, filter)
            .expect("configure STAR TLS extension");
    }

    proxy_svc.add_tls_with_settings(proxy_addr, None, tls_settings);

    server.add_service(proxy_svc);
    Ok(server)
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

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let api_addr = env_or_default(E2E_API_ADDR_ENV, DEFAULT_API_ADDR);
    let proxy_addr = env_or_default(E2E_PROXY_ADDR_ENV, DEFAULT_PROXY_ADDR);
    let upstream_addr = env_or_default(E2E_UPSTREAM_ADDR_ENV, DEFAULT_UPSTREAM_ADDR);
    let max_prefix = parse_env_u32(E2E_MAX_PREFIX_ENV, DEFAULT_MAX_PREFIX);
    let api_threads = parse_env_usize(E2E_API_THREADS_ENV, DEFAULT_API_THREADS);
    let proxy_threads = parse_env_usize(E2E_PROXY_THREADS_ENV, DEFAULT_PROXY_THREADS);
    let tls_cert_path = env::var(E2E_TLS_CERT_PATH_ENV).unwrap_or_else(|_| default_tls_cert_path());
    let tls_key_path = env::var(E2E_TLS_KEY_PATH_ENV).unwrap_or_else(|_| default_tls_key_path());

    let _upstream = spawn_upstream(&upstream_addr);
    println!(
        "starting e2e benchmark server: api={api_addr}, proxy={proxy_addr}, upstream={upstream_addr}, max_prefix={max_prefix}, api_threads={api_threads}, proxy_threads={proxy_threads}, tls_cert_path={tls_cert_path}"
    );

    let (latency_tx, _latency_rx) = mpsc::channel::<u64>();
    let server = build_server_with_latency(
        &api_addr,
        &proxy_addr,
        &upstream_addr,
        all_prefix(max_prefix),
        max_prefix,
        api_threads,
        proxy_threads,
        &tls_cert_path,
        &tls_key_path,
        latency_tx,
    )?;
    server.run_forever()
}
