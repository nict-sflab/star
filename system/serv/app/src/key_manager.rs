use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use star_attestation::{LOCAL_REPORT_LEN, QVE_TARGET_INFO_LEN, append_blob, take_blob};

use crate::enclave::{ENCLAVE_PUBLIC_KEY_LEN, EnclaveError};
use crate::env_or_default;
use crate::sgx::{LibraryRuntime, build_sgxs};

const STAR_KEY_MANAGER_SGXS: &str = "STAR_KEY_MANAGER_SGXS";
const STAR_KEY_MANAGER_HEAP_SIZE: &str = "STAR_KEY_MANAGER_HEAP_SIZE";
const STAR_KEY_MANAGER_STACK_SIZE: &str = "STAR_KEY_MANAGER_STACK_SIZE";
const STAR_KEY_MANAGER_THREADS: &str = "STAR_KEY_MANAGER_THREADS";
const STAR_KEY_MANAGER_SEALED_KEYS: &str = "STAR_KEY_MANAGER_SEALED_KEYS";
const DEFAULT_KEY_MANAGER_SEALED_KEYS: &str = "key-manager.keys.sealed";
pub const STAR_KEY_MANAGER_URL: &str = "STAR_KEY_MANAGER_URL";
pub const DEFAULT_KEY_MANAGER_PUBLIC_KEY_API: &str = "/star/key-manager/public_key";
pub const DEFAULT_KEY_MANAGER_ATTESTATION_API: &str = "/star/key-manager/attestation";
pub const DEFAULT_KEY_MANAGER_EXPORT_API: &str = "/star/key-manager/export";

const DEFAULT_KEY_MANAGER_HEAP_SIZE: &str = "33554432";
const DEFAULT_KEY_MANAGER_STACK_SIZE: &str = "131072";

const OP_INIT: u64 = 0;
const OP_PUBLIC_KEY: u64 = 1;
const OP_ATTEST: u64 = 4;
const OP_EXPORT_KEYS: u64 = 5;
const OP_QVE_TARGET_INFO: u64 = 6;
const OP_LOCAL_ATTEST: u64 = 7;

const PUBLIC_KEY_RESPONSE_CAPACITY: usize = 32;
const ATTEST_RESPONSE_CAPACITY: usize = 32;
const EXPORT_KEYS_RESPONSE_CAPACITY: usize = 256;
const QVE_TARGET_INFO_RESPONSE_CAPACITY: usize = QVE_TARGET_INFO_LEN;
const LOCAL_ATTEST_RESPONSE_CAPACITY: usize = LOCAL_REPORT_LEN;
const INIT_RESPONSE_CAPACITY: usize = 1024;

#[derive(Debug)]
pub enum KeyManagerError {
    Enclave(EnclaveError),
    InvalidPublicKeyLength { len: usize },
}

impl fmt::Display for KeyManagerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Enclave(err) => write!(f, "{err}"),
            Self::InvalidPublicKeyLength { len } => {
                write!(
                    f,
                    "key-manager returned public key with invalid length {len}"
                )
            }
        }
    }
}

impl Error for KeyManagerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Enclave(err) => Some(err),
            Self::InvalidPublicKeyLength { .. } => None,
        }
    }
}

impl From<EnclaveError> for KeyManagerError {
    fn from(err: EnclaveError) -> Self {
        Self::Enclave(err)
    }
}

pub struct KeyManagerRuntime {
    library: LibraryRuntime,
    sealed_keys: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyManagerExport {
    pub key_manager_quote: Vec<u8>,
    pub recipient_evidence: Vec<u8>,
    pub envelope: Vec<u8>,
}

pub struct RemoteKeyManagerRuntime {
    base_url: String,
    agent: ureq::Agent,
}

impl KeyManagerRuntime {
    pub fn from_env() -> Result<Self, EnclaveError> {
        let sgxs = default_key_manager_sgxs()?;
        let library = LibraryRuntime::load(&sgxs)?;
        let sealed_keys_path = sealed_keys_path();
        let init_payload = read_optional_sealed_keys(&sealed_keys_path)?;
        let sealed_keys = library.call(
            OP_INIT,
            init_payload.as_deref().unwrap_or(&[]),
            initial_response_capacity(OP_INIT),
        )?;

        if init_payload.as_deref() != Some(sealed_keys.as_slice()) {
            write_sealed_keys(&sealed_keys_path, &sealed_keys)?;
        }

        Ok(KeyManagerRuntime {
            library,
            sealed_keys,
        })
    }

    pub fn from_env_with_sealed_keys(sealed_keys: &[u8]) -> Result<Self, EnclaveError> {
        let sgxs = default_key_manager_sgxs()?;
        let library = LibraryRuntime::load(&sgxs)?;
        let sealed_keys = library.call(OP_INIT, sealed_keys, initial_response_capacity(OP_INIT))?;

        Ok(KeyManagerRuntime {
            library,
            sealed_keys,
        })
    }

    pub fn sealed_keys(&self) -> &[u8] {
        &self.sealed_keys
    }

    pub fn public_key(&self) -> Result<[u8; ENCLAVE_PUBLIC_KEY_LEN], KeyManagerError> {
        let response = self.call(OP_PUBLIC_KEY, &[])?;
        let len = response.len();

        response
            .as_slice()
            .try_into()
            .map_err(|_| KeyManagerError::InvalidPublicKeyLength { len })
    }

    pub fn public_key_attestation(&self) -> Result<Vec<u8>, EnclaveError> {
        self.call(OP_ATTEST, &[])
    }

    pub fn local_public_key_attestation(
        &self,
        verifier_target_info: &[u8],
    ) -> Result<Vec<u8>, EnclaveError> {
        self.call(OP_LOCAL_ATTEST, verifier_target_info)
    }

    pub fn export_keys(&self, recipient_evidence: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        self.call(OP_EXPORT_KEYS, recipient_evidence)
    }

    pub fn qve_target_info(&self) -> Result<Vec<u8>, EnclaveError> {
        let response = self.call(OP_QVE_TARGET_INFO, &[])?;
        if response.len() != QVE_TARGET_INFO_LEN {
            return Err(EnclaveError::Qve(format!(
                "key-manager returned targetinfo with invalid length {}",
                response.len()
            )));
        }
        Ok(response)
    }

    fn call(&self, op: u64, payload: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        self.library
            .call(op, payload, initial_response_capacity(op))
    }
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

    pub fn public_key_attestation(&self) -> Result<Vec<u8>, EnclaveError> {
        self.get_bytes(DEFAULT_KEY_MANAGER_ATTESTATION_API)
    }

    pub fn export_keys_for_recipient_quote(
        &self,
        recipient_quote: &[u8],
    ) -> Result<KeyManagerExport, EnclaveError> {
        let response = self.post_bytes(DEFAULT_KEY_MANAGER_EXPORT_API, recipient_quote)?;
        decode_key_manager_export_response(&response).map_err(|err| {
            EnclaveError::KeyManagerService(format!("decode export response: {err}"))
        })
    }

    fn endpoint(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn get_bytes(&self, path: &str) -> Result<Vec<u8>, EnclaveError> {
        let response = self
            .agent
            .get(&self.endpoint(path))
            .call()
            .map_err(http_error)?;
        decode_base64_response(response)
    }

    fn post_bytes(&self, path: &str, body: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        let encoded = URL_SAFE.encode(body);
        let response = self
            .agent
            .post(&self.endpoint(path))
            .set("content-type", "text/plain")
            .send_string(&encoded)
            .map_err(http_error)?;
        decode_base64_response(response)
    }
}

pub fn encode_key_manager_export_response(export: &KeyManagerExport) -> Vec<u8> {
    let mut out = Vec::new();
    append_blob(&mut out, &export.key_manager_quote);
    append_blob(&mut out, &export.recipient_evidence);
    append_blob(&mut out, &export.envelope);
    out
}

pub fn decode_key_manager_export_response(payload: &[u8]) -> Result<KeyManagerExport, String> {
    let mut remaining = payload;
    let key_manager_quote = take_blob(&mut remaining)?.to_vec();
    let recipient_evidence = take_blob(&mut remaining)?.to_vec();
    let envelope = take_blob(&mut remaining)?.to_vec();

    if !remaining.is_empty() {
        return Err("trailing bytes after key-manager export response".to_string());
    }

    Ok(KeyManagerExport {
        key_manager_quote,
        recipient_evidence,
        envelope,
    })
}

fn decode_base64_response(response: ureq::Response) -> Result<Vec<u8>, EnclaveError> {
    let body = response
        .into_string()
        .map_err(|err| EnclaveError::KeyManagerService(format!("read response body: {err}")))?;
    URL_SAFE
        .decode(body.trim())
        .map_err(|err| EnclaveError::KeyManagerService(format!("decode base64 response: {err}")))
}

fn http_error(err: ureq::Error) -> EnclaveError {
    match err {
        ureq::Error::Status(code, response) => {
            let body = response
                .into_string()
                .unwrap_or_else(|err| format!("failed to read error body: {err}"));
            EnclaveError::KeyManagerService(format!("HTTP {code}: {body}"))
        }
        ureq::Error::Transport(err) => EnclaveError::KeyManagerService(err.to_string()),
    }
}

fn initial_response_capacity(op: u64) -> usize {
    match op {
        OP_INIT => INIT_RESPONSE_CAPACITY,
        OP_PUBLIC_KEY => PUBLIC_KEY_RESPONSE_CAPACITY,
        OP_ATTEST => ATTEST_RESPONSE_CAPACITY,
        OP_EXPORT_KEYS => EXPORT_KEYS_RESPONSE_CAPACITY,
        OP_QVE_TARGET_INFO => QVE_TARGET_INFO_RESPONSE_CAPACITY,
        OP_LOCAL_ATTEST => LOCAL_ATTEST_RESPONSE_CAPACITY,
        _ => 1024,
    }
}

fn sealed_keys_path() -> PathBuf {
    std::env::var_os(STAR_KEY_MANAGER_SEALED_KEYS)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(DEFAULT_KEY_MANAGER_SEALED_KEYS))
}

fn read_optional_sealed_keys(path: &Path) -> Result<Option<Vec<u8>>, EnclaveError> {
    match fs::read(path) {
        Ok(sealed_keys) => Ok(Some(sealed_keys)),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(EnclaveError::Storage(err)),
    }
}

fn write_sealed_keys(path: &Path, sealed_keys: &[u8]) -> Result<(), EnclaveError> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(EnclaveError::Storage)?;
    }

    fs::write(path, sealed_keys).map_err(EnclaveError::Storage)
}

fn default_key_manager_sgxs() -> Result<PathBuf, EnclaveError> {
    if let Some(path) = std::env::var_os(STAR_KEY_MANAGER_SGXS) {
        return Ok(PathBuf::from(path));
    }

    let manifest_dir = match option_env!("CARGO_MANIFEST_DIR") {
        Some(dir) => dir,
        None => ".",
    };

    let key_manager_dir = PathBuf::from(manifest_dir).join("../manage-key");
    let manifest_path = key_manager_dir.join("Cargo.toml");

    build_key_manager_sgxs(&manifest_path)
}

fn build_key_manager_sgxs(manifest_path: &Path) -> Result<PathBuf, EnclaveError> {
    build_sgxs(
        manifest_path,
        "key-manager",
        env_or_default(STAR_KEY_MANAGER_HEAP_SIZE, DEFAULT_KEY_MANAGER_HEAP_SIZE),
        env_or_default(STAR_KEY_MANAGER_STACK_SIZE, DEFAULT_KEY_MANAGER_STACK_SIZE),
        default_key_manager_threads(),
    )
}

fn default_key_manager_threads() -> String {
    std::env::var(STAR_KEY_MANAGER_THREADS).unwrap_or_else(|_| "1".to_string())
}

#[path = "key_manager_test.rs"]
#[cfg(test)]
mod tests;
