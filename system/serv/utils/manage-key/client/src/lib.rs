use base64::{Engine as _, engine::general_purpose::URL_SAFE};
mod export;
pub use export::KeyManagerExport;

pub const DEFAULT_KEY_MANAGER_PUBLIC_KEY_API: &str = "/star/key-manager/public_key";
pub const DEFAULT_KEY_MANAGER_ATTESTATION_API: &str = "/star/key-manager/attestation";
pub const DEFAULT_KEY_MANAGER_EXPORT_API: &str = "/star/key-manager/export";
pub const STAR_KEY_MANAGER_URL: &str = "STAR_KEY_MANAGER_URL";
pub use star_attestation::PUBLIC_KEY_LEN as ENCLAVE_PUBLIC_KEY_LEN;

#[derive(Debug, thiserror::Error)]
pub enum KeyManagerError {
    #[error("key-manager returned public key with invalid length {len}")]
    InvalidPublicKeyLength { len: usize },
    #[error("service error: {0}")]
    Service(String),
}

pub struct RemoteKeyManagerRuntime {
    base_url: String,
    agent: ureq::Agent,
}

impl RemoteKeyManagerRuntime {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            agent: ureq::AgentBuilder::new().build(),
        }
    }

    pub fn from_env() -> Option<Self> {
        std::env::var(STAR_KEY_MANAGER_URL)
            .ok()
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .map(Self::new)
    }

    pub fn public_key(&self) -> Result<[u8; ENCLAVE_PUBLIC_KEY_LEN], KeyManagerError> {
        let response = self.get_bytes(DEFAULT_KEY_MANAGER_PUBLIC_KEY_API)?;
        let len = response.len();

        response
            .as_slice()
            .try_into()
            .map_err(|_| KeyManagerError::InvalidPublicKeyLength { len })
    }

    pub fn public_key_attestation(&self) -> Result<Vec<u8>, KeyManagerError> {
        self.get_bytes(DEFAULT_KEY_MANAGER_ATTESTATION_API)
    }

    pub fn export_keys_for_recipient_quote(
        &self,
        recipient_quote: &[u8],
    ) -> Result<KeyManagerExport, KeyManagerError> {
        let response = self
            .agent
            .post(&format!(
                "{}{}",
                self.base_url, DEFAULT_KEY_MANAGER_EXPORT_API
            ))
            .set("content-type", "text/plain")
            .send_string(&URL_SAFE.encode(recipient_quote))
            .map_err(|err| KeyManagerError::Service(err.to_string()))?;
        let body = response
            .into_string()
            .map_err(|err| KeyManagerError::Service(err.to_string()))?;
        let response = URL_SAFE
            .decode(body.trim())
            .map_err(|err| KeyManagerError::Service(err.to_string()))?;
        KeyManagerExport::decode(&response)
            .map_err(|err| KeyManagerError::Service(format!("decode export response: {err}")))
    }

    fn get_bytes(&self, path: &str) -> Result<Vec<u8>, KeyManagerError> {
        let response = self
            .agent
            .get(&format!("{}{}", self.base_url, path))
            .call()
            .map_err(|err| KeyManagerError::Service(err.to_string()))?;
        let body = response
            .into_string()
            .map_err(|err| KeyManagerError::Service(err.to_string()))?;
        URL_SAFE
            .decode(body.trim())
            .map_err(|err| KeyManagerError::Service(err.to_string()))
    }
}
