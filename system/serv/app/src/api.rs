pub const DEFAULT_REGISTER_API: &str = "/star/register";
pub const DEFAULT_PUBLIC_KEY_API: &str = "/star/public_key";

use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};

use constant_time_utils::bytes::Long;
use pingora_web::{Handler, PingoraHttpRequest, PingoraWebHttpResponse, StatusCode, WebError};
use star_core::serv::enclave;
use tokio::sync::{Mutex};

use crate::core::EnclaveRuntime;

pub struct IssueApi {
    enclave: Mutex<EnclaveRuntime>
}

impl IssueApi {
    pub fn init(enclave: EnclaveRuntime) -> Self{
        let enclave = Mutex::new(enclave);
        IssueApi { enclave }
     }
}

#[async_trait]
impl Handler for IssueApi {
    async fn handle(
        &self,
        req: PingoraHttpRequest,
    ) -> Result<PingoraWebHttpResponse, WebError> {
        let body = req.body().as_ref();
        let decoded = URL_SAFE.decode(body).unwrap();
        
        let cred = self.enclave.lock().await.issue_cred(&decoded);
        let out = URL_SAFE.encode(cred);
        let res = PingoraWebHttpResponse::text(StatusCode::OK, out);
        Ok(res)
    }
}

pub struct PublicKeyApi {
    public_key: Long
}

impl PublicKeyApi {
    pub fn init(enclave: &mut EnclaveRuntime) -> Self{
        let public_key = enclave.public_key();

        PublicKeyApi { public_key }
     }
}

#[async_trait]
impl Handler for PublicKeyApi {
    async fn handle(
        &self,
        req: PingoraHttpRequest,
    ) -> Result<PingoraWebHttpResponse, WebError> {
        let out = URL_SAFE.encode(self.public_key);
        Ok(PingoraWebHttpResponse::text(StatusCode::OK, out))
    }
}