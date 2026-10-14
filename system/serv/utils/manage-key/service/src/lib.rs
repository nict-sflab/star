#[path = "../../protocol.rs"]
mod protocol;
mod sgx;

pub use sgx::EnclaveError;
use std::fs;
use std::io;
use std::path::PathBuf;

use key_manager_client::ENCLAVE_PUBLIC_KEY_LEN;

use crate::protocol::{ATTEST, EXPORT_KEYS, INIT, PUBLIC_KEY};
use crate::sgx::{CommandRuntime, build_sgxs};

const STAR_KEY_MANAGER_SGXS: &str = "STAR_KEY_MANAGER_SGXS";
const STAR_KEY_MANAGER_HEAP_SIZE: &str = "STAR_KEY_MANAGER_HEAP_SIZE";
const STAR_KEY_MANAGER_STACK_SIZE: &str = "STAR_KEY_MANAGER_STACK_SIZE";
const STAR_KEY_MANAGER_THREADS: &str = "STAR_KEY_MANAGER_THREADS";
const STAR_KEY_MANAGER_SEALED_KEYS: &str = "STAR_KEY_MANAGER_SEALED_KEYS";
const DEFAULT_KEY_MANAGER_SEALED_KEYS: &str = "key-manager.keys.sealed";

const DEFAULT_KEY_MANAGER_HEAP_SIZE: &str = "33554432";
const DEFAULT_KEY_MANAGER_STACK_SIZE: &str = "131072";

pub struct KeyManagerRuntime {
    runner: CommandRuntime,
    sealed_keys: Vec<u8>,
}

impl KeyManagerRuntime {
    pub fn from_env() -> Result<Self, EnclaveError> {
        let sgxs = default_key_manager_sgxs()?;
        let runner = CommandRuntime::load(&sgxs)?;
        let sealed_keys_path = std::env::var_os(STAR_KEY_MANAGER_SEALED_KEYS)
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_KEY_MANAGER_SEALED_KEYS));
        let init_payload = match fs::read(&sealed_keys_path) {
            Ok(keys) => Some(keys),
            Err(err) if err.kind() == io::ErrorKind::NotFound => None,
            Err(err) => return Err(EnclaveError::Runner(format!("read sealed keys: {err}"))),
        };
        let sealed_keys = runner.call(INIT, init_payload.as_deref().unwrap_or(&[]))?;

        if init_payload.as_deref() != Some(sealed_keys.as_slice()) {
            if let Some(parent) = sealed_keys_path
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
            {
                fs::create_dir_all(parent).map_err(|err| {
                    EnclaveError::Runner(format!("create sealed keys dir: {err}"))
                })?;
            }
            fs::write(&sealed_keys_path, &sealed_keys)
                .map_err(|err| EnclaveError::Runner(format!("write sealed keys: {err}")))?;
        }

        Ok(KeyManagerRuntime {
            runner,
            sealed_keys,
        })
    }

    pub fn from_env_with_sealed_keys(sealed_keys: &[u8]) -> Result<Self, EnclaveError> {
        let sgxs = default_key_manager_sgxs()?;
        let runner = CommandRuntime::load(&sgxs)?;
        let sealed_keys = runner.call(INIT, sealed_keys)?;

        Ok(KeyManagerRuntime {
            runner,
            sealed_keys,
        })
    }

    pub fn sealed_keys(&self) -> &[u8] {
        &self.sealed_keys
    }

    pub fn public_key(&self) -> Result<[u8; ENCLAVE_PUBLIC_KEY_LEN], EnclaveError> {
        let response = self.runner.call(PUBLIC_KEY, &[])?;
        let len = response.len();

        response
            .as_slice()
            .try_into()
            .map_err(|_| EnclaveError::Runner(format!("invalid public key length: {len}")))
    }

    pub fn public_key_attestation(&self) -> Result<Vec<u8>, EnclaveError> {
        self.runner.call(ATTEST, &[])
    }

    pub fn export_keys(&self, recipient_evidence: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        self.runner.call(EXPORT_KEYS, recipient_evidence)
    }
}

fn default_key_manager_sgxs() -> Result<PathBuf, EnclaveError> {
    if let Some(path) = std::env::var_os(STAR_KEY_MANAGER_SGXS) {
        return Ok(PathBuf::from(path));
    }

    let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../enclave/Cargo.toml");

    build_sgxs(
        &manifest_path,
        "key-manager",
        env_or_default(STAR_KEY_MANAGER_HEAP_SIZE, DEFAULT_KEY_MANAGER_HEAP_SIZE),
        env_or_default(STAR_KEY_MANAGER_STACK_SIZE, DEFAULT_KEY_MANAGER_STACK_SIZE),
        env_or_default(STAR_KEY_MANAGER_THREADS, "1"),
    )
}

fn env_or_default(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}
