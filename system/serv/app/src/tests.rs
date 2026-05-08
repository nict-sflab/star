use crate::api::{DEFAULT_PUBLIC_KEY_API, DEFAULT_REGISTER_API};
use crate::{all_prefix, build_server};

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use constant_time_utils::time;
use pingora::server::{RunArgs, ShutdownSignal, ShutdownSignalWatch};
use rand_core::OsRng;
use star_core::client::User;
use star_core::secure_channel::client::SecureChannelClient;
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

fn spawn_upstream() -> (String, thread::JoinHandle<()>) {
    let server = TinyServer::http("127.0.0.1:3000").expect("bind upstream");
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
    let server_handle = thread::spawn(move || {
        let server = build_server(api_addr, proxy_addr, &upstream_addr, all_prefix(1), 1);
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
