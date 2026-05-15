use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use openssl::ssl::{ExtensionContext, SslAcceptor, SslFiletype, SslMethod, SslVerifyMode};
use pingora::server::{RunArgs, ShutdownSignal, ShutdownSignalWatch};
use rand::rngs::OsRng;
use star_client_proxy::{ProxyConfig, STAR_TLS_EXTENSION_TYPE, TargetRule, build_server};
use star_core::serv::enclave::Enclave;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
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

struct ProxyHandle {
    shutdown_tx: Option<oneshot::Sender<()>>,
    thread: Option<thread::JoinHandle<()>>,
}

impl ProxyHandle {
    fn stop(mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

fn free_addr() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    listener.local_addr().expect("read local addr")
}

fn wait_for_tcp(addr: SocketAddr) {
    for _ in 0..100 {
        if TcpStream::connect_timeout(&addr, Duration::from_millis(50)).is_ok() {
            return;
        }
        thread::sleep(Duration::from_millis(20));
    }
    panic!("server did not listen on {addr}");
}

fn spawn_proxy(config: ProxyConfig) -> ProxyHandle {
    let addr: SocketAddr = config.listen_addr.parse().expect("proxy listen addr");
    let server = build_server(config).expect("build proxy");
    let (shutdown_tx, shutdown_rx) = oneshot::channel();

    let thread = thread::spawn(move || {
        server.run(RunArgs {
            shutdown_signal: Box::new(TestShutdown {
                rx: AsyncMutex::new(shutdown_rx),
            }),
        });
    });

    wait_for_tcp(addr);
    ProxyHandle {
        shutdown_tx: Some(shutdown_tx),
        thread: Some(thread),
    }
}

fn spawn_star_api() -> (String, thread::JoinHandle<()>) {
    let server = TinyServer::http("127.0.0.1:0").expect("bind STAR API");
    let addr = server.server_addr().to_ip().expect("STAR API IP addr");
    let api_base = format!("http://{addr}");
    let handle = thread::spawn(move || {
        let mut rng = OsRng;
        let enclave = Enclave::default(&mut rng);

        for _ in 0..2 {
            let mut request = server.recv().expect("receive STAR API request");
            let url = request.url().to_string();

            if url == "/star/public_key" {
                let public_key = URL_SAFE.encode(enclave.public_key());
                let _ = request.respond(Response::from_string(public_key));
                continue;
            }

            if url == "/star/register" {
                let mut body = String::new();
                request
                    .as_reader()
                    .read_to_string(&mut body)
                    .expect("read register body");
                let credential_request = URL_SAFE
                    .decode(body.trim())
                    .expect("decode credential request");
                let credential = enclave.issue_cred(&credential_request, &mut rng);
                let _ = request.respond(Response::from_string(URL_SAFE.encode(credential)));
                continue;
            }

            let _ = request.respond(Response::empty(404));
        }
    });

    (api_base, handle)
}

fn cert_path(name: &str) -> String {
    format!(
        "{}/../../serv/app/examples/certs/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn spawn_tls_upstream() -> (
    SocketAddr,
    Arc<Mutex<Option<Vec<u8>>>>,
    Arc<Mutex<Vec<u8>>>,
    thread::JoinHandle<()>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind TLS upstream");
    let addr = listener.local_addr().expect("TLS upstream addr");
    let seen_extension = Arc::new(Mutex::new(None));
    let seen_request = Arc::new(Mutex::new(Vec::new()));
    let ext_ref = Arc::clone(&seen_extension);
    let req_ref = Arc::clone(&seen_request);

    let handle = thread::spawn(move || {
        let mut acceptor =
            SslAcceptor::mozilla_intermediate(SslMethod::tls()).expect("build TLS acceptor");
        acceptor
            .set_certificate_chain_file(cert_path("server.crt"))
            .expect("load cert");
        acceptor
            .set_private_key_file(cert_path("server.key"), SslFiletype::PEM)
            .expect("load key");
        acceptor.set_verify(SslVerifyMode::NONE);

        acceptor
            .add_custom_ext(
                STAR_TLS_EXTENSION_TYPE,
                ExtensionContext::CLIENT_HELLO,
                |_ssl, _context, _cert| Ok(None::<Vec<u8>>),
                move |_ssl, _context, ext_data, _cert| {
                    *ext_ref.lock().expect("extension mutex") = Some(ext_data.to_vec());
                    Ok(())
                },
            )
            .expect("register custom extension");

        let acceptor = acceptor.build();
        let (stream, _) = listener.accept().expect("accept TLS upstream");
        let mut stream = acceptor.accept(stream).expect("accept TLS");

        let mut request = Vec::new();
        let mut buf = [0u8; 1024];
        loop {
            let n = stream.read(&mut buf).expect("read proxied request");
            if n == 0 {
                break;
            }
            request.extend_from_slice(&buf[..n]);
            if request.windows(4).any(|w| w == b"\r\n\r\n") {
                break;
            }
        }

        *req_ref.lock().expect("request mutex") = request;
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nOK")
            .expect("write TLS upstream response");
        let _ = stream.flush();
    });

    (addr, seen_extension, seen_request, handle)
}

fn spawn_tcp_echo() -> (SocketAddr, thread::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind echo upstream");
    let addr = listener.local_addr().expect("echo upstream addr");

    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("accept echo upstream");
        let mut buf = [0u8; 4];
        stream.read_exact(&mut buf).expect("read echo request");
        stream.write_all(&buf).expect("write echo response");
        let _ = stream.flush();
    });

    (addr, handle)
}

fn read_http_header(stream: &mut TcpStream) -> String {
    let mut raw = Vec::new();
    let mut buf = [0u8; 256];
    loop {
        let n = stream.read(&mut buf).expect("read HTTP header");
        assert_ne!(n, 0, "connection closed before HTTP header");
        raw.extend_from_slice(&buf[..n]);
        if raw.windows(4).any(|w| w == b"\r\n\r\n") {
            return String::from_utf8(raw).expect("HTTP header is UTF-8");
        }
    }
}

fn proxy_get(proxy_addr: SocketAddr, uri: &str) -> String {
    let mut stream =
        TcpStream::connect_timeout(&proxy_addr, Duration::from_secs(2)).expect("connect proxy");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("set read timeout");
    stream
        .set_write_timeout(Some(Duration::from_secs(5)))
        .expect("set write timeout");

    let request = format!(
        "GET {uri} HTTP/1.1\r\nHost: ignored-by-absolute-form\r\nConnection: close\r\n\r\n"
    );
    stream
        .write_all(request.as_bytes())
        .expect("write proxy request");
    stream.flush().expect("flush proxy request");

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("read proxy response");
    response
}

#[test]
fn attaches_star_extension_only_for_configured_https_targets() {
    let (api_base, api_thread) = spawn_star_api();
    let proxy_addr = free_addr();
    let (star_addr, star_ext, star_request, star_thread) = spawn_tls_upstream();

    let proxy = spawn_proxy(ProxyConfig {
        listen_addr: proxy_addr.to_string(),
        api_base,
        star_targets: vec![TargetRule::parse(&format!("https://{star_addr}")).unwrap()],
        insecure_tls: true,
        max_count: 10,
    });

    let response = proxy_get(proxy_addr, &format!("https://{star_addr}/star?q=1"));
    assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
    assert_eq!(response.split("\r\n\r\n").nth(1), Some("OK"));

    let star_thread_result = star_thread.join();
    assert!(star_thread_result.is_ok());

    let extension = star_ext.lock().expect("extension mutex").clone();
    assert!(extension.as_ref().is_some_and(|value| !value.is_empty()));

    let request = String::from_utf8(star_request.lock().expect("request mutex").clone()).unwrap();
    assert!(request.starts_with("GET /star?q=1 HTTP/1.1\r\n"));
    assert!(request.contains(&format!("Host: {star_addr}\r\n")));
    assert!(!request.to_ascii_lowercase().contains("x-star"));

    proxy.stop();
    assert!(api_thread.join().is_ok());

    let proxy_addr = free_addr();
    let (plain_addr, plain_ext, _, plain_thread) = spawn_tls_upstream();
    let proxy = spawn_proxy(ProxyConfig {
        listen_addr: proxy_addr.to_string(),
        api_base: "http://127.0.0.1:1".to_string(),
        star_targets: Vec::new(),
        insecure_tls: true,
        max_count: 10,
    });

    let response = proxy_get(proxy_addr, &format!("https://{plain_addr}/plain"));
    assert!(response.starts_with("HTTP/1.1 200 OK"), "{response}");
    assert!(plain_thread.join().is_ok());
    assert!(plain_ext.lock().expect("extension mutex").is_none());

    proxy.stop();
}

#[test]
fn connect_tunnels_non_star_and_rejects_star_targets() {
    let proxy_addr = free_addr();
    let (echo_addr, echo_thread) = spawn_tcp_echo();
    let proxy = spawn_proxy(ProxyConfig {
        listen_addr: proxy_addr.to_string(),
        api_base: "http://127.0.0.1:1".to_string(),
        star_targets: Vec::new(),
        insecure_tls: true,
        max_count: 10,
    });

    let mut stream =
        TcpStream::connect_timeout(&proxy_addr, Duration::from_secs(2)).expect("connect proxy");
    stream
        .write_all(format!("CONNECT {echo_addr} HTTP/1.1\r\nHost: {echo_addr}\r\n\r\n").as_bytes())
        .expect("write CONNECT");

    let header = read_http_header(&mut stream);
    assert!(
        header.starts_with("HTTP/1.1 200 Connection Established"),
        "{header}"
    );

    stream.write_all(b"PING").expect("write tunneled bytes");
    let mut echoed = [0u8; 4];
    stream.read_exact(&mut echoed).expect("read tunneled bytes");
    assert_eq!(&echoed, b"PING");

    drop(stream);
    assert!(echo_thread.join().is_ok());
    proxy.stop();

    let (api_base, api_thread) = spawn_star_api();
    let proxy_addr = free_addr();
    let proxy = spawn_proxy(ProxyConfig {
        listen_addr: proxy_addr.to_string(),
        api_base,
        star_targets: vec![TargetRule::parse("https://example.test:443").unwrap()],
        insecure_tls: true,
        max_count: 10,
    });

    let mut stream =
        TcpStream::connect_timeout(&proxy_addr, Duration::from_secs(2)).expect("connect proxy");
    stream
        .write_all(b"CONNECT example.test:443 HTTP/1.1\r\nHost: example.test:443\r\n\r\n")
        .expect("write STAR CONNECT");

    let header = read_http_header(&mut stream);
    assert!(
        header.starts_with("HTTP/1.1 421 Misdirected Request"),
        "{header}"
    );

    proxy.stop();
    assert!(api_thread.join().is_ok());
}
