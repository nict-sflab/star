pub const DEFAULT_REGISTER_API: &str = "/star/register";
pub const DEFAULT_PUBLIC_KEY_API: &str = "/star/public_key";

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use constant_time_utils::bytes::Long;
use pingora_web::{Handler, PingoraHttpRequest, PingoraWebHttpResponse, StatusCode, WebError};
use std::sync::Arc;

use crate::enclave::EnclaveRuntime;

pub(crate) struct IssueApi {
    enclave: Arc<EnclaveRuntime>,
}

impl IssueApi {
    pub(crate) fn new(enclave: Arc<EnclaveRuntime>) -> Self {
        Self { enclave }
    }
}

#[async_trait]
impl Handler for IssueApi {
    async fn handle(&self, req: PingoraHttpRequest) -> Result<PingoraWebHttpResponse, WebError> {
        let decoded = match URL_SAFE.decode(req.body().as_ref()) {
            Ok(decoded) => decoded,
            Err(_) => return Ok(PingoraWebHttpResponse::text(StatusCode::BAD_REQUEST, "")),
        };

        let credential = match self.enclave.issue_cred(&decoded) {
            Ok(credential) => credential,
            Err(_) => {
                return Ok(PingoraWebHttpResponse::text(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "",
                ));
            }
        };

        Ok(PingoraWebHttpResponse::text(
            StatusCode::OK,
            URL_SAFE.encode(credential),
        ))
    }
}

pub(crate) struct PublicKeyApi {
    public_key: Long,
}

impl PublicKeyApi {
    pub(crate) fn new(public_key: Long) -> Self {
        Self { public_key }
    }
}

#[async_trait]
impl Handler for PublicKeyApi {
    async fn handle(&self, _req: PingoraHttpRequest) -> Result<PingoraWebHttpResponse, WebError> {
        Ok(PingoraWebHttpResponse::text(
            StatusCode::OK,
            URL_SAFE.encode(self.public_key),
        ))
    }
}
