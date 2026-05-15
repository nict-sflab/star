use crate::api::{DEFAULT_PUBLIC_KEY_API, DEFAULT_REGISTER_API};
use crate::{all_prefix, build_server};

use crate::tls_token::STAR_TLS_EXTENSION_TYPE;
use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use constant_time_utils::time;
use openssl::ssl::{ExtensionContext, SslConnector, SslMethod, SslVerifyMode};
use pingora::server::{RunArgs, ShutdownSignal, ShutdownSignalWatch};
use rand_core::OsRng;
use star_core::client::User;
use star_core::secure_channel::client::SecureChannelClient;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::thread;
use std::time::Duration;
use tiny_http::{Response, Server as TinyServer};
use tokio::sync::{Mutex as AsyncMutex, oneshot};

struct TestShutdown {
    rx: AsyncMutex<oneshot::Receiver<()>>,
}

#[async_trait]
impl ShutdownSignalWatch for TestShutdown {
    async fn recv(&self) -> ShutdownSignal {
        let mut rx = self.rx.lock().await;
        let _ = (&mut *rx).await;
        ShutdownSignal::FastShutdown
    }
}

fn retry_request<T, F>(mut op: F) -> T
where
    F: FnMut() -> Result<T, ureq::Error>,
{
    match op() {
        Ok(value) => value,
        Err(ureq::Error::Transport(err)) => panic!("http request failed: {err}"),
        Err(ureq::Error::Status(code, _)) => panic!("http request failed: status {code}"),
    }
}

fn get_text_retry(url: &str) -> String {
    retry_request(|| Ok(ureq::get(url).call()?.into_string()?))
}

fn post_text_retry(url: &str, body: &[u8]) -> String {
    retry_request(|| Ok(ureq::post(url).send_bytes(body)?.into_string()?))
}

fn https_get_with_star_extension(
    proxy_addr: SocketAddr,
    token_request: &[u8],
) -> Result<(u16, String), String> {
    let mut tls = SslConnector::builder(SslMethod::tls()).map_err(|e| e.to_string())?;
    tls.set_verify(SslVerifyMode::NONE);

    let ext = token_request.to_vec();
    tls.add_custom_ext(
        STAR_TLS_EXTENSION_TYPE,
        ExtensionContext::TLS_ONLY
            | ExtensionContext::CLIENT_HELLO
            | ExtensionContext::TLS1_2_AND_BELOW_ONLY
            | ExtensionContext::TLS1_3_ONLY,
        move |_, _, _| Ok(Some(ext.clone())),
        |_, _, _, _| Ok(()),
    )
    .map_err(|e| e.to_string())?;

    let tls = tls.build();
    https_get_with_connector(proxy_addr, tls)
}

fn https_get_without_star_extension(proxy_addr: SocketAddr) -> Result<(u16, String), String> {
    let mut tls = SslConnector::builder(SslMethod::tls()).map_err(|e| e.to_string())?;
    tls.set_verify(SslVerifyMode::NONE);

    let tls = tls.build();
    https_get_with_connector(proxy_addr, tls)
}

fn https_get_with_connector(
    proxy_addr: SocketAddr,
    tls: openssl::ssl::SslConnector,
) -> Result<(u16, String), String> {
    let stream = TcpStream::connect_timeout(&proxy_addr, Duration::from_secs(2))
        .map_err(|e| e.to_string())?;
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .map_err(|e| e.to_string())?;
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .map_err(|e| e.to_string())?;

    let mut stream = tls
        .connect("localhost", stream)
        .map_err(|e| format!("tls connect failed: {e}"))?;

    stream
        .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .map_err(|e| e.to_string())?;
    stream.flush().map_err(|e| e.to_string())?;

    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).map_err(|e| e.to_string())?;
    let raw = String::from_utf8_lossy(&raw);
    let (head, body) = raw
        .split_once("\r\n\r\n")
        .ok_or_else(|| "invalid HTTP response".to_string())?;
    let status = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .ok_or_else(|| "missing HTTP status".to_string())?
        .parse::<u16>()
        .map_err(|e| e.to_string())?;

    Ok((status, body.to_string()))
}

fn spawn_upstream() -> (String, thread::JoinHandle<()>) {
    let server = TinyServer::http("127.0.0.1:0").expect("bind upstream");
    let addr = server
        .server_addr()
        .to_ip()
        .expect("upstream should bind to IP");
    let addr = addr.to_string();
    let handle = thread::spawn(move || {
        if let Ok(request) = server.recv() {
            let response = Response::from_string("OK");
            let _ = request.respond(response);
        }
    });
    (addr, handle)
}

#[test]
fn test_all() {
    let api_addr = "127.0.0.1:8080";
    let proxy_addr = "127.0.0.1:8081";
    let (upstream_addr, upstream_handle) = spawn_upstream();

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let tls_cert_path = format!("{}/examples/certs/server.crt", env!("CARGO_MANIFEST_DIR"));
    let tls_key_path = format!("{}/examples/certs/server.key", env!("CARGO_MANIFEST_DIR"));

    unsafe {
        std::env::set_var("TLS_CERT_PATH", tls_cert_path.as_str());
        std::env::set_var("TLS_KEY_PATH", tls_key_path.as_str());
    }

    let server_handle = thread::spawn(move || {
        let server = build_server(api_addr, proxy_addr, &upstream_addr, all_prefix(1), 1)
            .expect("build test server");
        let shutdown = TestShutdown {
            rx: AsyncMutex::new(shutdown_rx),
        };
        let run_args = RunArgs {
            shutdown_signal: Box::new(shutdown),
        };
        server.run(run_args);
    });

    thread::sleep(Duration::from_millis(100));

    let public_key_url = format!("http://{api_addr}{DEFAULT_PUBLIC_KEY_API}");
    let body = get_text_retry(&public_key_url);
    let public_key = URL_SAFE.decode(body.trim()).unwrap();
    let public_key: [u8; 32] = public_key.as_slice().try_into().unwrap();
    println!("{:?}", public_key);

    let client = SecureChannelClient::new(public_key).expect("client init");
    let mut user = User::new(1, client);
    let mut rng = OsRng;

    let (cred_request, shared_key) = user.request_credential(&mut rng);
    let cred_request_b64 = URL_SAFE.encode(cred_request);

    println!("{cred_request_b64}");

    let register_url = format!("http://{api_addr}{DEFAULT_REGISTER_API}");
    let body = post_text_retry(&register_url, cred_request_b64.as_bytes());
    let cred_cipher = URL_SAFE.decode(body.trim()).unwrap();
    user.receive_credential(&cred_cipher, &shared_key);

    let current_period = u64::from_be_bytes(time::current_period());
    let token_request: Vec<u8> = user
        .request_auth(0, current_period, &mut rng)
        .expect("token request");

    let proxy_socket = proxy_addr
        .to_socket_addrs()
        .expect("resolve proxy address")
        .next()
        .expect("proxy address should resolve");

    let (status, body) =
        https_get_with_star_extension(proxy_socket, &token_request).expect("proxy request");
    assert_eq!(status, 200);
    assert_eq!(body.trim(), "OK");

    match https_get_with_star_extension(proxy_socket, &token_request) {
        Ok((status, _)) => panic!("expected handshake rejection for duplicate token, got {status}"),
        Err(_) => {}
    }

    match https_get_with_star_extension(proxy_socket, &[]) {
        Ok((status, _)) => panic!("expected handshake rejection for empty token, got {status}"),
        Err(_) => {}
    }

    match https_get_without_star_extension(proxy_socket) {
        Ok((status, _)) => {
            panic!("expected handshake rejection when STAR extension is missing, got {status}")
        }
        Err(_) => {}
    }

    let _ = shutdown_tx.send(());
    let _ = server_handle.join();
    let _ = upstream_handle.join();
}