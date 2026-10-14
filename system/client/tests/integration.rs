use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use pingora::server::{RunArgs, ShutdownSignal, ShutdownSignalWatch};
use star_app::{Shard, build_server as build_star_server};
use star_client_proxy::{ProxyConfig, StarUpstream, build_server as build_client_server};
use tiny_http::{Response, Server as UpstreamServer};
use tokio::sync::{Mutex, oneshot};

struct TestShutdown(Mutex<oneshot::Receiver<()>>);

#[async_trait]
impl ShutdownSignalWatch for TestShutdown {
    async fn recv(&self) -> ShutdownSignal {
        let mut receiver = self.0.lock().await;
        let _ = (&mut *receiver).await;
        ShutdownSignal::FastShutdown
    }
}

fn free_addr() -> String {
    TcpListener::bind("127.0.0.1:0")
        .expect("bind test port")
        .local_addr()
        .expect("read test port")
        .to_string()
}

fn start_aesm_proxy() {
    let listener = TcpListener::bind("127.0.0.1:5555").expect("bind AESM proxy");
    thread::spawn(move || {
        for connection in listener.incoming() {
            let connection = connection.expect("accept AESM connection");
            thread::spawn(move || {
                forward_aesm(connection).expect("forward AESM connection");
            });
        }
    });
    unsafe {
        std::env::set_var("AESM_PROXY", "127.0.0.1:5555");
    }
}

fn forward_aesm(mut tcp: TcpStream) -> io::Result<()> {
    while let Some((len, request)) = read_aesm_frame(&mut tcp)? {
        let mut unix = UnixStream::connect("/run/aesmd/aesm.socket")?;
        unix.write_all(&len.to_ne_bytes())?;
        unix.write_all(&request)?;
        let (len, response) = read_aesm_frame(&mut unix)?.ok_or(io::ErrorKind::UnexpectedEof)?;
        tcp.write_all(&len.to_ne_bytes())?;
        tcp.write_all(&response)?;
    }
    Ok(())
}

fn read_aesm_frame(stream: &mut impl Read) -> io::Result<Option<(u32, Vec<u8>)>> {
    let mut len = [0u8; 4];
    match stream.read_exact(&mut len) {
        Ok(()) => {}
        Err(err)
            if matches!(
                err.kind(),
                io::ErrorKind::UnexpectedEof
                    | io::ErrorKind::ConnectionReset
                    | io::ErrorKind::BrokenPipe
            ) =>
        {
            return Ok(None);
        }
        Err(err) => return Err(err),
    }
    let len = u32::from_ne_bytes(len);
    if len > 16 * 1024 * 1024 {
        return Err(io::ErrorKind::InvalidData.into());
    }
    let mut body = vec![0; len as usize];
    stream.read_exact(&mut body)?;
    Ok(Some((len, body)))
}

fn wait_for_listener(addr: &str) {
    let deadline = Instant::now() + Duration::from_secs(30);
    while TcpStream::connect(addr).is_err() {
        assert!(Instant::now() < deadline, "listener {addr} did not start");
        thread::sleep(Duration::from_millis(50));
    }
}

fn request(addr: &str) -> String {
    let mut stream = TcpStream::connect(addr).expect("connect to client proxy");
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .expect("set read timeout");
    stream
        .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
        .expect("send request");
    let mut response = String::new();
    stream.read_to_string(&mut response).expect("read response");
    response
}

#[test]
fn client_proxy_routes_across_two_shards_and_rejects_exhausted_limit() {
    start_aesm_proxy();

    let certs = Path::new(env!("CARGO_MANIFEST_DIR")).join("../serv/app/examples/certs");
    unsafe {
        std::env::set_var("TLS_CERT_PATH", certs.join("server.crt"));
        std::env::set_var("TLS_KEY_PATH", certs.join("server.key"));
    }

    let upstream_done = Arc::new(AtomicBool::new(false));
    let mut upstream_workers = Vec::new();
    let mut star_stops = Vec::new();
    let mut star_threads = Vec::new();
    let mut star_proxy_addrs = Vec::new();
    let mut api_addrs = Vec::new();
    for shard_index in 0..2 {
        let api_addr = free_addr();
        let star_proxy_addr = free_addr();
        let upstream = UpstreamServer::http("127.0.0.1:0").expect("bind upstream");
        let upstream_addr = upstream.server_addr().to_ip().expect("upstream IP");
        let done = Arc::clone(&upstream_done);
        upstream_workers.push(thread::spawn(move || {
            while !done.load(Ordering::Relaxed) {
                if let Some(request) = upstream
                    .recv_timeout(Duration::from_millis(100))
                    .expect("receive upstream request")
                {
                    request
                        .respond(Response::from_string(format!("shard-{shard_index}")))
                        .expect("respond");
                }
            }
        }));

        let (star_stop, star_stopped) = oneshot::channel();
        let api_bind = api_addr.clone();
        let star_bind = star_proxy_addr.clone();
        star_threads.push(thread::spawn(move || {
            let server = build_star_server(
                &api_bind,
                &star_bind,
                &upstream_addr.to_string(),
                Shard::new(shard_index, 2),
            )
            .expect("build STAR server");
            server.run(RunArgs {
                shutdown_signal: Box::new(TestShutdown(Mutex::new(star_stopped))),
            });
        }));
        wait_for_listener(&api_addr);
        wait_for_listener(&star_proxy_addr);
        star_stops.push(star_stop);
        star_proxy_addrs.push(star_proxy_addr);
        api_addrs.push(api_addr);
    }

    let api_base = format!("http://{}", api_addrs[0]);
    let star_port = star_proxy_addrs[0]
        .rsplit_once(':')
        .unwrap()
        .1
        .parse()
        .unwrap();
    let mut covered_both_shards = false;
    for _ in 0..16 {
        let client_addr = free_addr();
        let client_bind = client_addr.clone();
        let api_base = api_base.clone();
        let shard_endpoints = star_proxy_addrs.clone();
        let (client_stop, client_stopped) = oneshot::channel();
        let client_thread = thread::spawn(move || {
            let config = ProxyConfig {
                listen_addr: client_bind,
                star_api_base: api_base,
                max_requests_per_period: 4,
                upstream: StarUpstream::new("127.0.0.1".into(), star_port, "localhost".into())
                    .with_shards(shard_endpoints)
                    .expect("configure STAR shards"),
            };
            let server = build_client_server(config).expect("register client and build proxy");
            server.run(RunArgs {
                shutdown_signal: Box::new(TestShutdown(Mutex::new(client_stopped))),
            });
        });
        wait_for_listener(&client_addr);

        let period = constant_time_utils::time::current_period();
        let mut seen = [false; 2];
        for _ in 0..4 {
            let success = request(&client_addr);
            assert!(success.starts_with("HTTP/1.1 200"), "{success}");
            let shard_index = if success.ends_with("shard-0") {
                0
            } else if success.ends_with("shard-1") {
                1
            } else {
                panic!("unexpected upstream response: {success}");
            };
            seen[shard_index] = true;
        }
        let rejected = request(&client_addr);
        let same_period = constant_time_utils::time::current_period() == period;
        let _ = client_stop.send(());
        client_thread.join().unwrap();
        if same_period {
            assert!(
                rejected.is_empty(),
                "exhausted client request was accepted: {rejected}"
            );
            if seen.into_iter().all(|seen| seen) {
                covered_both_shards = true;
                break;
            }
        }
    }
    assert!(covered_both_shards, "both shards must serve one ticket");
    for stop in star_stops {
        let _ = stop.send(());
    }
    for thread in star_threads {
        thread.join().unwrap();
    }
    upstream_done.store(true, Ordering::Relaxed);
    for worker in upstream_workers {
        worker.join().unwrap();
    }
}
