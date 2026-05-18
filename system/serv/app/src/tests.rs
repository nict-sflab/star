use crate::api::{DEFAULT_PUBLIC_KEY_API, DEFAULT_REGISTER_API};
use crate::enclave::{EnclaveRuntime, report_data_for_public_key};
use crate::{all_prefix, build_server_without_startup_attestation};

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use constant_time_utils::time;
use pingora::server::{RunArgs, ShutdownSignal, ShutdownSignalWatch};
use rand_core::OsRng;
use star_core::client::User;
use star_core::secure_channel::client::SecureChannelClient;
use star_tools::{QuoteVerificationPolicy, decode_quote_input};
use std::ffi::{OsStr, OsString};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
#[cfg(unix)]
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::Mutex as StdMutex;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tiny_http::{Response, Server as TinyServer};
use tokio::sync::{Mutex as AsyncMutex, oneshot};

static SGX_TEST_LOCK: StdMutex<()> = StdMutex::new(());
const DEFAULT_AESM_SOCKET: &str = "/run/aesmd/aesm.socket";

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
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        match op() {
            Ok(value) => return value,
            Err(ureq::Error::Status(code, _)) => panic!("http request failed: status {code}"),
            Err(ureq::Error::Transport(err)) => {
                if Instant::now() >= deadline {
                    panic!("http request failed after retrying: {err}");
                }
                thread::sleep(Duration::from_millis(50));
            }
        }
    }
}

fn get_text_retry(url: &str) -> String {
    retry_request(|| Ok(ureq::get(url).call()?.into_string()?))
}

fn post_text_retry(url: &str, body: &[u8]) -> String {
    retry_request(|| Ok(ureq::post(url).send_bytes(body)?.into_string()?))
}

fn unused_loopback_addr() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    listener
        .local_addr()
        .expect("read ephemeral address")
        .to_string()
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

struct EnvVarGuard {
    name: &'static str,
    previous: Option<OsString>,
}

impl EnvVarGuard {
    fn set(name: &'static str, value: impl AsRef<OsStr>) -> Self {
        let previous = std::env::var_os(name);
        unsafe {
            std::env::set_var(name, value);
        }
        Self { name, previous }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        unsafe {
            if let Some(previous) = &self.previous {
                std::env::set_var(self.name, previous);
            } else {
                std::env::remove_var(self.name);
            }
        }
    }
}

struct AesmProxy {
    addr: String,
    shutdown: Option<mpsc::Sender<()>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl AesmProxy {
    fn start_on(addr: &str) -> io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        listener.set_nonblocking(true)?;
        let addr = listener.local_addr()?.to_string();
        let socket_path = aesm_socket_path();
        let (shutdown, shutdown_rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            loop {
                if shutdown_rx.try_recv().is_ok() {
                    break;
                }

                match listener.accept() {
                    Ok((stream, _)) => {
                        let socket_path = socket_path.clone();
                        thread::spawn(move || {
                            if let Err(err) = forward_aesm_connection(stream, socket_path) {
                                eprintln!("AESM proxy connection failed: {err}");
                            }
                        });
                    }
                    Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(err) => panic!("accept AESM proxy connection: {err}"),
                }
            }
        });

        Ok(Self {
            addr,
            shutdown: Some(shutdown),
            handle: Some(handle),
        })
    }
}

impl Drop for AesmProxy {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn aesm_socket_path() -> PathBuf {
    std::env::var_os("AESM_SOCKET")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_AESM_SOCKET))
}

#[cfg(unix)]
fn forward_aesm_connection(mut tcp: TcpStream, socket_path: PathBuf) -> io::Result<()> {
    loop {
        let Some((request_len, request)) = read_aesm_frame(&mut tcp)? else {
            return Ok(());
        };

        let mut unix = UnixStream::connect(&socket_path)?;
        unix.write_all(&request_len.to_ne_bytes())?;
        unix.write_all(&request)?;

        let Some((response_len, response)) = read_aesm_frame(&mut unix)? else {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "AESM closed before sending a response",
            ));
        };
        tcp.write_all(&response_len.to_ne_bytes())?;
        tcp.write_all(&response)?;
    }
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
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("AESM frame too large: {len} bytes"),
        ));
    }

    let mut body = vec![0u8; len as usize];
    stream.read_exact(&mut body)?;
    Ok(Some((len, body)))
}

#[test]
fn test_all() {
    let _sgx_test_guard = SGX_TEST_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let api_addr = unused_loopback_addr();
    let proxy_addr = unused_loopback_addr();
    let (upstream_addr, upstream_handle) = spawn_upstream();

    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let api_bind_addr = api_addr.clone();
    let proxy_bind_addr = proxy_addr.clone();
    let server_handle = thread::spawn(move || {
        let server = build_server_without_startup_attestation(
            &api_bind_addr,
            &proxy_bind_addr,
            &upstream_addr,
            all_prefix(1),
            1,
        )
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
    let token_b64 = URL_SAFE.encode(token_request);
    println!("{token_b64}");

    let proxy_url = format!("http://{proxy_addr}/");
    let body = retry_request(|| {
        Ok(ureq::get(&proxy_url)
            .set("X-STAR", token_b64.as_str())
            .call()?
            .into_string()?)
    });
    assert_eq!(body.trim(), "OK");

    match ureq::get(&proxy_url)
        .set("X-STAR", token_b64.as_str())
        .call()
    {
        Err(ureq::Error::Status(code, _)) => assert_eq!(code, 429),
        Ok(_) => panic!("expected duplicate STAR token to be rejected"),
        Err(ureq::Error::Transport(err)) => panic!("http request failed: {err}"),
    }

    let _ = shutdown_tx.send(());
    let _ = server_handle.join();
    let _ = upstream_handle.join();
}

#[test]
fn public_key_attestation_quote_verifies_with_tools_policy() {
    let _sgx_test_guard = SGX_TEST_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let aesm_proxy = match AesmProxy::start_on("127.0.0.1:5555") {
        Ok(proxy) => Some(proxy),
        Err(err) if err.kind() == io::ErrorKind::AddrInUse => None,
        Err(err) => panic!("start AESM proxy: {err}"),
    };
    let _aesm_proxy_env = EnvVarGuard::set(
        "AESM_PROXY",
        aesm_proxy
            .as_ref()
            .map(|proxy| proxy.addr.as_str())
            .unwrap_or("127.0.0.1:5555"),
    );

    let mut enclave = EnclaveRuntime::from_env().expect("initialize enclave");
    let public_key = enclave.public_key().expect("read enclave public key");
    let quote = enclave
        .public_key_attestation(public_key)
        .expect("get public key attestation quote");

    let startup_log = format!(
        "STAR_ATTESTATION_PUBLIC_KEY_BASE64={}\nSTAR_ATTESTATION_QUOTE_BASE64={}\n",
        URL_SAFE.encode(public_key),
        URL_SAFE.encode(&quote)
    );
    let quote = decode_quote_input(startup_log.as_bytes()).expect("decode quote from startup log");
    let parsed = star_tools::parse_quote(&quote).expect("parse quote");
    let expected_mrsigner = parsed.mrsigner;
    let expected_mrenclave = parsed.mrenclave;

    assert_eq!(
        star_tools::public_key_from_report_data(&parsed).expect("extract attested public key"),
        public_key
    );
    assert_eq!(parsed.report_data, report_data_for_public_key(public_key));

    let policy = QuoteVerificationPolicy {
        expected_mrsigner: &expected_mrsigner,
        expected_mrenclave: Some(&expected_mrenclave),
        expected_public_key: Some(&public_key),
        allow_debug: true,
        allow_advisory: true,
        qvl_library: None,
        qpl_library: None,
    };
    let verified = star_tools::verify_quote(&quote, &policy).expect("verify quote with tools");

    assert_eq!(verified.public_key, public_key);
    assert_eq!(
        verified.parsed.report_data,
        report_data_for_public_key(public_key)
    );
}
