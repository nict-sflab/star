pub const DEFAULT_REGISTER_API: &str = "/star/register";
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use rand_core::OsRng;

use crate::core::ENCLAVE;
use pingora_web::{PingoraHttpRequest, PingoraWebHttpResponse, StatusCode, WebError};


pub fn issue_api(req: PingoraHttpRequest) -> Result<PingoraWebHttpResponse, WebError> {
    let body = req.body().as_ref();
    let decoded = URL_SAFE.decode(body).unwrap();

    // Don’t share an RNG across requests; create one per request.
    let mut rng = OsRng;
    let enclave = ENCLAVE.get().unwrap();
    let cred = enclave.issue_cred(&decoded, &mut rng);

    let out = URL_SAFE.encode(cred);
    Ok(PingoraWebHttpResponse::text(StatusCode::OK, out))
}
