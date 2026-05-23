use std::error::Error;
use std::sync::{Arc, Mutex, MutexGuard};

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use pingora::server::Server;
use pingora_web::{
    App as WebApp, Handler, PingoraHttpRequest, PingoraWebHttpResponse, StatusCode, WebError,
};

use crate::key_manager::{
    DEFAULT_KEY_MANAGER_ATTESTATION_API, DEFAULT_KEY_MANAGER_EXPORT_API,
    DEFAULT_KEY_MANAGER_PUBLIC_KEY_API, KeyManagerExport, KeyManagerRuntime,
    encode_key_manager_export_response,
};
use star_attestation::qve;

pub const DEFAULT_KEY_MANAGER_ADDR: &str = "0.0.0.0:8090";
pub const STAR_KEY_MANAGER_ADDR: &str = "STAR_KEY_MANAGER_ADDR";
const STAR_KEY_MANAGER_THREADS: &str = "STAR_KEY_MANAGER_THREADS";

pub fn build_key_manager_server(addr: &str) -> Result<Server, Box<dyn Error>> {
    build_key_manager_server_with_runtime(addr, KeyManagerRuntime::from_env()?)
}

pub fn build_key_manager_server_with_runtime(
    addr: &str,
    key_manager: KeyManagerRuntime,
) -> Result<Server, Box<dyn Error>> {
    let mut server = Server::new(None)?;
    server.bootstrap();

    let key_manager = Arc::new(Mutex::new(key_manager));
    let mut app = WebApp::default();
    app.get(
        DEFAULT_KEY_MANAGER_PUBLIC_KEY_API,
        Arc::new(KeyManagerPublicKeyApi::new(Arc::clone(&key_manager))),
    );
    app.get(
        DEFAULT_KEY_MANAGER_ATTESTATION_API,
        Arc::new(KeyManagerAttestationApi::new(Arc::clone(&key_manager))),
    );
    app.post(
        DEFAULT_KEY_MANAGER_EXPORT_API,
        Arc::new(KeyManagerExportApi::new(key_manager)),
    );

    let mut svc = app.to_service("key-manager");
    svc.threads = env_usize(STAR_KEY_MANAGER_THREADS);
    svc.add_tcp(addr);
    server.add_service(svc);

    Ok(server)
}

fn env_usize(name: &str) -> Option<usize> {
    std::env::var(name).ok()?.parse::<usize>().ok()
}

type SharedKeyManager = Arc<Mutex<KeyManagerRuntime>>;

struct KeyManagerPublicKeyApi {
    key_manager: SharedKeyManager,
}

impl KeyManagerPublicKeyApi {
    fn new(key_manager: SharedKeyManager) -> Self {
        Self { key_manager }
    }
}

#[async_trait]
impl Handler for KeyManagerPublicKeyApi {
    async fn handle(&self, _req: PingoraHttpRequest) -> Result<PingoraWebHttpResponse, WebError> {
        let key_manager = match lock_key_manager(&self.key_manager) {
            Ok(key_manager) => key_manager,
            Err(response) => return Ok(response),
        };

        match key_manager.public_key() {
            Ok(public_key) => Ok(base64_response(&public_key)),
            Err(err) => Ok(error_response(StatusCode::INTERNAL_SERVER_ERROR, err)),
        }
    }
}

struct KeyManagerAttestationApi {
    key_manager: SharedKeyManager,
}

impl KeyManagerAttestationApi {
    fn new(key_manager: SharedKeyManager) -> Self {
        Self { key_manager }
    }
}

#[async_trait]
impl Handler for KeyManagerAttestationApi {
    async fn handle(&self, _req: PingoraHttpRequest) -> Result<PingoraWebHttpResponse, WebError> {
        let key_manager = match lock_key_manager(&self.key_manager) {
            Ok(key_manager) => key_manager,
            Err(response) => return Ok(response),
        };

        match key_manager.public_key_attestation() {
            Ok(quote) => Ok(base64_response(&quote)),
            Err(err) => Ok(error_response(StatusCode::INTERNAL_SERVER_ERROR, err)),
        }
    }
}

struct KeyManagerExportApi {
    key_manager: SharedKeyManager,
}

impl KeyManagerExportApi {
    fn new(key_manager: SharedKeyManager) -> Self {
        Self { key_manager }
    }
}

#[async_trait]
impl Handler for KeyManagerExportApi {
    async fn handle(&self, req: PingoraHttpRequest) -> Result<PingoraWebHttpResponse, WebError> {
        let recipient_quote = match URL_SAFE.decode(req.body().as_ref()) {
            Ok(quote) => quote,
            Err(err) => return Ok(error_response(StatusCode::BAD_REQUEST, err)),
        };

        let key_manager = match lock_key_manager(&self.key_manager) {
            Ok(key_manager) => key_manager,
            Err(response) => return Ok(response),
        };

        let key_manager_quote = match key_manager.public_key_attestation() {
            Ok(quote) => quote,
            Err(err) => return Ok(error_response(StatusCode::INTERNAL_SERVER_ERROR, err)),
        };
        let key_manager_target_info = match key_manager.qve_target_info() {
            Ok(target_info) => target_info,
            Err(err) => return Ok(error_response(StatusCode::INTERNAL_SERVER_ERROR, err)),
        };
        let recipient_evidence =
            match qve::evidence_for_verifier(&recipient_quote, &key_manager_target_info) {
                Ok(evidence) => evidence,
                Err(err) => return Ok(error_response(StatusCode::INTERNAL_SERVER_ERROR, err)),
            };
        let envelope = match key_manager.export_keys(&recipient_evidence) {
            Ok(envelope) => envelope,
            Err(err) => return Ok(error_response(StatusCode::BAD_REQUEST, err)),
        };

        let export = KeyManagerExport {
            key_manager_quote,
            recipient_evidence,
            envelope,
        };
        Ok(base64_response(&encode_key_manager_export_response(
            &export,
        )))
    }
}

fn lock_key_manager(
    key_manager: &SharedKeyManager,
) -> Result<MutexGuard<'_, KeyManagerRuntime>, PingoraWebHttpResponse> {
    key_manager.lock().map_err(|_| {
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "key-manager lock poisoned",
        )
    })
}

fn base64_response(bytes: &[u8]) -> PingoraWebHttpResponse {
    PingoraWebHttpResponse::text(StatusCode::OK, URL_SAFE.encode(bytes))
}

fn error_response(status: StatusCode, err: impl std::fmt::Display) -> PingoraWebHttpResponse {
    PingoraWebHttpResponse::text(status, err.to_string())
}
