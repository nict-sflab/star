pub const DEFAULT_REGISTER_API: &str = "/star/register";
pub const DEFAULT_PUBLIC_KEY_API: &str = "/star/public_key";

use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use rand_core::OsRng;

use crate::core::ENCLAVE;
use pingora_web::{PingoraHttpRequest, PingoraWebHttpResponse, StatusCode, WebError};

pub fn issue_api(req: PingoraHttpRequest) -> Result<PingoraWebHttpResponse, WebError> {
    let body = req.body().as_ref();
    let decoded = URL_SAFE.decode(body).unwrap();

    let mut rng = OsRng;
    let enclave = ENCLAVE.get().unwrap();
    let cred = enclave.issue_cred(&decoded, &mut rng);

    let out = URL_SAFE.encode(cred);
    let res = PingoraWebHttpResponse::text(StatusCode::OK, out);
    Ok(res)
}

pub fn public_key_api(_req: PingoraHttpRequest) -> Result<PingoraWebHttpResponse, WebError> {
    let enclave = ENCLAVE.get().unwrap();
    let public_key = enclave.public_key();

    let out = URL_SAFE.encode(public_key);
    let res = PingoraWebHttpResponse::text(StatusCode::OK, out);
    Ok(res)
}
