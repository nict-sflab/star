use axum::{
    Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use key_manager_client::{
    DEFAULT_KEY_MANAGER_ATTESTATION_API, DEFAULT_KEY_MANAGER_EXPORT_API,
    DEFAULT_KEY_MANAGER_PUBLIC_KEY_API, KeyManagerExport,
};
use star_attestation::Evidence;
use std::sync::{Arc, Mutex};
use tracing::{error, info};

use key_manager_service::KeyManagerRuntime;

type SharedKeyManager = Arc<Mutex<KeyManagerRuntime>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let addr =
        std::env::var("STAR_KEY_MANAGER_ADDR").unwrap_or_else(|_| "0.0.0.0:8090".to_string());

    info!("Starting key-manager enclave...");
    let runtime = KeyManagerRuntime::from_env()?;
    let key_manager = Arc::new(Mutex::new(runtime));

    let app = Router::new()
        .route(DEFAULT_KEY_MANAGER_PUBLIC_KEY_API, get(handle_public_key))
        .route(DEFAULT_KEY_MANAGER_ATTESTATION_API, get(handle_attestation))
        .route(DEFAULT_KEY_MANAGER_EXPORT_API, post(handle_export))
        .with_state(key_manager);

    info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_public_key(
    State(key_manager): State<SharedKeyManager>,
) -> Result<String, (StatusCode, String)> {
    let key_manager = key_manager.lock().unwrap();

    key_manager
        .public_key()
        .map(|key| URL_SAFE.encode(key))
        .map_err(internal_error)
}

async fn handle_attestation(
    State(key_manager): State<SharedKeyManager>,
) -> Result<String, (StatusCode, String)> {
    let key_manager = key_manager.lock().unwrap();

    key_manager
        .public_key_attestation()
        .map(|quote| URL_SAFE.encode(quote))
        .map_err(internal_error)
}

async fn handle_export(
    State(key_manager): State<SharedKeyManager>,
    body: String,
) -> Result<String, (StatusCode, String)> {
    let recipient_quote = URL_SAFE
        .decode(body.trim())
        .map_err(|err| (StatusCode::BAD_REQUEST, format!("invalid base64: {}", err)))?;

    let key_manager_quote = {
        let key_manager = key_manager.lock().unwrap();

        key_manager.public_key_attestation().map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("attestation error: {}", err),
            )
        })?
    };

    let recipient_evidence = Evidence::encode_for_verifier(&recipient_quote)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("evidence error: {}", err),
            )
        })?;

    let envelope = {
        let key_manager = key_manager.lock().unwrap();

        key_manager
            .export_keys(&recipient_evidence)
            .map_err(|err| (StatusCode::BAD_REQUEST, format!("export error: {}", err)))?
    };

    let export = KeyManagerExport {
        key_manager_quote,
        recipient_evidence,
        envelope,
    };

    Ok(URL_SAFE.encode(export.encode()))
}

fn internal_error(err: impl std::fmt::Display) -> (StatusCode, String) {
    error!("{err}");
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
