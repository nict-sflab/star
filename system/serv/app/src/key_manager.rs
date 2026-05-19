use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::Command;

use aesm_client::AesmClient;
use enclave_runner::{EnclaveBuilder, Library};
use enclave_runner_sgx::EnclaveBuilder as SgxEnclaveBuilder;
use sgxs_loaders::isgx::Device as SgxDevice;
use star_attestation::QVE_TARGET_INFO_LEN;

use crate::enclave::{ENCLAVE_PUBLIC_KEY_LEN, EnclaveError};

const SGX_TARGET: &str = "x86_64-fortanix-unknown-sgx";
const STAR_KEY_MANAGER_SGXS: &str = "STAR_KEY_MANAGER_SGXS";
const STAR_KEY_MANAGER_HEAP_SIZE: &str = "STAR_KEY_MANAGER_HEAP_SIZE";
const STAR_KEY_MANAGER_STACK_SIZE: &str = "STAR_KEY_MANAGER_STACK_SIZE";
const STAR_KEY_MANAGER_THREADS: &str = "STAR_KEY_MANAGER_THREADS";

const DEFAULT_KEY_MANAGER_HEAP_SIZE: &str = "33554432";
const DEFAULT_KEY_MANAGER_STACK_SIZE: &str = "131072";

const MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;

const OP_INIT: u64 = 0;
const OP_PUBLIC_KEY: u64 = 1;
const OP_ATTEST: u64 = 4;
const OP_EXPORT_KEYS: u64 = 5;
const OP_QVE_TARGET_INFO: u64 = 6;

const STATUS_OK: u64 = 0;
const STATUS_BUFFER_TOO_SMALL: u64 = 5;

const PUBLIC_KEY_RESPONSE_CAPACITY: usize = 32;
const ATTEST_RESPONSE_CAPACITY: usize = 32;
const EXPORT_KEYS_RESPONSE_CAPACITY: usize = 256;
const QVE_TARGET_INFO_RESPONSE_CAPACITY: usize = QVE_TARGET_INFO_LEN;

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
    library: KeyManagerLibraryRuntime,
}

impl KeyManagerRuntime {
    pub fn from_env() -> Result<Self, EnclaveError> {
        let sgxs = default_key_manager_sgxs()?;
        let library = KeyManagerLibraryRuntime::load(&sgxs)?;
        let runtime = KeyManagerRuntime { library };
        runtime.initialize()?;
        Ok(runtime)
    }

    fn initialize(&self) -> Result<(), EnclaveError> {
        self.call(OP_INIT, &[])?;
        Ok(())
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
        self.library.call(op, payload)
    }
}

struct KeyManagerLibraryRuntime {
    library: Library,
}

unsafe impl Send for KeyManagerLibraryRuntime {}
unsafe impl Sync for KeyManagerLibraryRuntime {}

impl KeyManagerLibraryRuntime {
    fn load(sgxs: &PathBuf) -> Result<Self, EnclaveError> {
        let mut device = SgxDevice::new()
            .map_err(EnclaveError::Build)?
            .einittoken_provider(AesmClient::new())
            .build();

        let sgx_builder = SgxEnclaveBuilder::new(sgxs);

        let library = EnclaveBuilder::new(sgx_builder)
            .build(&mut device)
            .map_err(|err| EnclaveError::Library(err.to_string()))?;

        Ok(Self { library })
    }

    fn call(&self, op: u64, payload: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        std::thread::scope(|scope| {
            let handle = scope.spawn(|| call_library(&self.library, op, payload));
            match handle.join() {
                Ok(result) => result,
                Err(panic) => std::panic::resume_unwind(panic),
            }
        })
    }
}

fn call_library(library: &Library, op: u64, payload: &[u8]) -> Result<Vec<u8>, EnclaveError> {
    let mut output = vec![0u8; initial_response_capacity(op)];

    loop {
        let (status, len) = unsafe {
            library.call(
                op,
                payload.as_ptr() as u64,
                payload.len() as u64,
                output.as_mut_ptr() as u64,
                output.len() as u64,
            )
        }
        .map_err(|err| EnclaveError::Library(err.to_string()))?;

        let len =
            usize::try_from(len).map_err(|_| EnclaveError::ResponseTooLarge { len: usize::MAX })?;

        if status == STATUS_BUFFER_TOO_SMALL {
            if len > MAX_PAYLOAD_LEN {
                return Err(EnclaveError::ResponseTooLarge { len });
            }

            output.resize(len, 0);
            continue;
        }

        if len > output.len() {
            return Err(EnclaveError::ResponseTooLarge { len });
        }

        output.truncate(len);

        if status == STATUS_OK {
            return Ok(output);
        }

        return Err(EnclaveError::Remote {
            status,
            payload: output,
        });
    }
}

fn initial_response_capacity(op: u64) -> usize {
    match op {
        OP_INIT => 0,
        OP_PUBLIC_KEY => PUBLIC_KEY_RESPONSE_CAPACITY,
        OP_ATTEST => ATTEST_RESPONSE_CAPACITY,
        OP_EXPORT_KEYS => EXPORT_KEYS_RESPONSE_CAPACITY,
        OP_QVE_TARGET_INFO => QVE_TARGET_INFO_RESPONSE_CAPACITY,
        _ => 1024,
    }
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
    let manifest_dir = manifest_path.parent().ok_or_else(|| {
        EnclaveError::BuildFailed(format!(
            "missing parent directory for {}",
            manifest_path.display()
        ))
    })?;

    run_command(
        Command::new("cargo")
            .current_dir(manifest_dir)
            .arg("build")
            .arg("--quiet")
            .arg("--release")
            .arg("--target")
            .arg(SGX_TARGET)
            .arg("--manifest-path")
            .arg(manifest_path),
    )?;

    let target_dir = sgx_release_target_dir(manifest_dir, "key-manager")?;
    let elf_path = target_dir.join("key-manager");
    let sgxs_path = target_dir.join("key-manager.sgxs");

    run_command(
        Command::new("ftxsgx-elf2sgxs")
            .arg(&elf_path)
            .arg("--library")
            .arg("--debug")
            .arg("--heap-size")
            .arg(env_or_default(
                STAR_KEY_MANAGER_HEAP_SIZE,
                DEFAULT_KEY_MANAGER_HEAP_SIZE,
            ))
            .arg("--stack-size")
            .arg(env_or_default(
                STAR_KEY_MANAGER_STACK_SIZE,
                DEFAULT_KEY_MANAGER_STACK_SIZE,
            ))
            .arg("--threads")
            .arg(default_key_manager_threads())
            .arg("--output")
            .arg(&sgxs_path),
    )?;

    Ok(sgxs_path)
}

fn sgx_release_target_dir(manifest_dir: &Path, binary_name: &str) -> Result<PathBuf, EnclaveError> {
    let mut candidates = Vec::new();

    if let Some(target_dir) = std::env::var_os("CARGO_TARGET_DIR") {
        candidates.push(PathBuf::from(target_dir));
    }

    if let Some(parent) = manifest_dir.parent() {
        candidates.push(parent.join("target"));
    }

    candidates.push(manifest_dir.join("target"));

    for candidate in candidates {
        let target_dir = candidate.join(SGX_TARGET).join("release");
        if target_dir.join(binary_name).is_file() {
            return Ok(target_dir);
        }
    }

    Err(EnclaveError::BuildFailed(format!(
        "cargo built {binary_name}, but the SGX ELF was not found"
    )))
}

fn env_or_default(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}

fn default_key_manager_threads() -> String {
    std::env::var(STAR_KEY_MANAGER_THREADS).unwrap_or_else(|_| "1".to_string())
}

fn run_command(command: &mut Command) -> Result<(), EnclaveError> {
    let output = command.output().map_err(EnclaveError::Build)?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    Err(EnclaveError::BuildFailed(format!(
        "{}\nstdout:\n{}\nstderr:\n{}",
        output.status, stdout, stderr
    )))
}
