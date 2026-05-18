use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use aesm_client::AesmClient;
use enclave_runner::{EnclaveBuilder, Library};
use enclave_runner_sgx::EnclaveBuilder as SgxEnclaveBuilder;
use sgxs_loaders::isgx::Device as SgxDevice;
use tokio::sync::Mutex;

const SGX_TARGET: &str = "x86_64-fortanix-unknown-sgx";
const STAR_ENCLAVE_SGXS: &str = "STAR_ENCLAVE_SGXS";
const STAR_ENCLAVE_HEAP_SIZE: &str = "STAR_ENCLAVE_HEAP_SIZE";
const STAR_ENCLAVE_STACK_SIZE: &str = "STAR_ENCLAVE_STACK_SIZE";
const STAR_ENCLAVE_THREADS: &str = "STAR_ENCLAVE_THREADS";
const DEFAULT_ENCLAVE_HEAP_SIZE: &str = "33554432";
const DEFAULT_ENCLAVE_STACK_SIZE: &str = "131072";
const SEALED_ENC_KEY_FILE: &str = "sealed_enc_key.bin";
const SEALED_PRF_KEY_FILE: &str = "sealed_prf_key.bin";
const MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;
pub const ATTESTATION_REPORT_DATA_LEN: usize = 64;
pub const ENCLAVE_PUBLIC_KEY_LEN: usize = 32;

const OP_INIT: u64 = 0;
const OP_PUBLIC_KEY: u64 = 1;
const OP_ISSUE_CRED: u64 = 2;
const OP_ISSUE_TOKEN: u64 = 3;
const OP_ATTEST: u64 = 4;

const STATUS_OK: u64 = 0;
const STATUS_BUFFER_TOO_SMALL: u64 = 5;

const INIT_RESPONSE_CAPACITY: usize = 4096;
const PUBLIC_KEY_RESPONSE_CAPACITY: usize = 32;
const ISSUE_CRED_RESPONSE_CAPACITY: usize = 128;
const ISSUE_TOKEN_RESPONSE_CAPACITY: usize = 32;
const ATTEST_RESPONSE_CAPACITY: usize = 8192;

pub struct EnclaveRuntimes {
    inner: Mutex<Vec<EnclaveRuntime>>,
}

#[derive(Debug)]
pub enum EnclaveError {
    Build(io::Error),
    BuildFailed(String),
    InvalidPublicKeyLength { len: usize },
    InvalidSealedKeys(String),
    Library(String),
    Remote { status: u64, payload: Vec<u8> },
    ResponseTooLarge { len: usize },
    Storage(io::Error),
}

impl fmt::Display for EnclaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Build(err) => write!(f, "enclave build failed to start: {err}"),
            Self::BuildFailed(status) => write!(f, "enclave build failed: {status}"),
            Self::InvalidPublicKeyLength { len } => {
                write!(f, "enclave returned public key with invalid length {len}")
            }
            Self::InvalidSealedKeys(err) => write!(f, "invalid sealed keys: {err}"),
            Self::Library(err) => write!(f, "enclave library call failed: {err}"),
            Self::Remote { status, payload } => write!(
                f,
                "enclave returned status {status} with {} bytes",
                payload.len()
            ),
            Self::ResponseTooLarge { len } => write!(f, "enclave response is too large: {len}"),
            Self::Storage(err) => write!(f, "sealed key storage failed: {err}"),
        }
    }
}

impl Error for EnclaveError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Build(err) => Some(err),
            Self::BuildFailed(_) => None,
            Self::InvalidPublicKeyLength { .. } => None,
            Self::InvalidSealedKeys(_) => None,
            Self::Library(_) => None,
            Self::Remote { .. } => None,
            Self::ResponseTooLarge { .. } => None,
            Self::Storage(err) => Some(err),
        }
    }
}

impl EnclaveRuntimes {
    pub fn init(num: u8) -> Result<Self, EnclaveError> {
        let sgxs = default_enclave_sgxs()?;
        let mut in_use = vec![];

        for _ in 0..num {
            let enclave = EnclaveRuntime::init_library(&sgxs)?;
            in_use.push(enclave);
        }

        Ok(EnclaveRuntimes {
            inner: Mutex::new(in_use),
        })
    }

    pub fn from_env(num: u8) -> Result<Self, EnclaveError> {
        EnclaveRuntimes::init(num)
    }

    pub async fn pick(&mut self) -> Option<EnclaveRuntime> {
        self.inner.lock().await.pop()
    }

    pub async fn release(&mut self, enclave: EnclaveRuntime) {
        self.inner.lock().await.push(enclave);
    }
}

pub struct EnclaveRuntime {
    library: LibraryRuntime,
}

impl EnclaveRuntime {
    pub fn from_env() -> Result<Self, EnclaveError> {
        let sgxs = default_enclave_sgxs()?;
        Self::init_library(&sgxs)
    }

    fn init_library(sgxs: &PathBuf) -> Result<Self, EnclaveError> {
        let library = LibraryRuntime::load(sgxs)?;
        let mut runtime = EnclaveRuntime { library };
        runtime.initialize()?;
        Ok(runtime)
    }

    fn initialize(&mut self) -> Result<(), EnclaveError> {
        let key_paths = sealed_key_paths();
        let init_payload = read_sealed_keys(&key_paths)?;
        let response = self.call(OP_INIT, &init_payload)?;

        if init_payload.is_empty() {
            let (sealed_enc_key, sealed_prf_key) = decode_sealed_keys(&response)?;
            write_sealed_keys(&key_paths, sealed_enc_key, sealed_prf_key)?;
        }

        Ok(())
    }

    pub fn public_key(&mut self) -> Result<[u8; 32], EnclaveError> {
        let response = self.call(OP_PUBLIC_KEY, &[])?;
        let len = response.len();
        response
            .as_slice()
            .try_into()
            .map_err(|_| EnclaveError::InvalidPublicKeyLength { len })
    }

    pub fn public_key_attestation(
        &mut self,
        public_key: [u8; ENCLAVE_PUBLIC_KEY_LEN],
    ) -> Result<Vec<u8>, EnclaveError> {
        self.attest(report_data_for_public_key(public_key))
    }

    pub fn issue_cred(&mut self, cipher: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        self.call(OP_ISSUE_CRED, cipher)
    }

    pub fn issue_token(
        &mut self,
        cipher: &[u8],
        current_period: [u8; 8],
    ) -> Result<Vec<u8>, EnclaveError> {
        let mut payload = Vec::with_capacity(current_period.len() + cipher.len());
        payload.extend_from_slice(&current_period);
        payload.extend_from_slice(cipher);
        self.call(OP_ISSUE_TOKEN, &payload)
    }

    pub fn attest(
        &mut self,
        report_data: [u8; ATTESTATION_REPORT_DATA_LEN],
    ) -> Result<Vec<u8>, EnclaveError> {
        self.call(OP_ATTEST, &report_data)
    }

    fn call(&mut self, op: u64, payload: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        self.library.call(op, payload)
    }
}

pub fn report_data_for_public_key(
    public_key: [u8; ENCLAVE_PUBLIC_KEY_LEN],
) -> [u8; ATTESTATION_REPORT_DATA_LEN] {
    let mut report_data = [0u8; ATTESTATION_REPORT_DATA_LEN];
    report_data[..ENCLAVE_PUBLIC_KEY_LEN].copy_from_slice(&public_key);
    report_data
}

struct LibraryRuntime {
    library: Library,
}

// enclave_runner::Library internally serializes TCS access and is safe to move
// with the runtime object that owns it.
unsafe impl Send for LibraryRuntime {}

impl LibraryRuntime {
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

    fn call(&mut self, op: u64, payload: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        let mut output = vec![0u8; initial_response_capacity(op, payload.len())];

        loop {
            let (status, len) = unsafe {
                self.library.call(
                    op,
                    payload.as_ptr() as u64,
                    payload.len() as u64,
                    output.as_mut_ptr() as u64,
                    output.len() as u64,
                )
            }
            .map_err(|err| EnclaveError::Library(err.to_string()))?;

            let len = usize::try_from(len)
                .map_err(|_| EnclaveError::ResponseTooLarge { len: usize::MAX })?;

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
}

fn default_enclave_sgxs() -> Result<PathBuf, EnclaveError> {
    if let Some(path) = std::env::var_os(STAR_ENCLAVE_SGXS) {
        return Ok(PathBuf::from(path));
    }

    let manifest_dir = match option_env!("CARGO_MANIFEST_DIR") {
        Some(dir) => dir,
        None => ".",
    };

    let enclave_dir = PathBuf::from(manifest_dir).join("../enclave");
    let manifest_path = enclave_dir.join("Cargo.toml");
    build_enclave_sgxs(&manifest_path)?;

    Ok(enclave_dir
        .join("target")
        .join(SGX_TARGET)
        .join("release")
        .join("enclave.sgxs"))
}

fn build_enclave_sgxs(manifest_path: &Path) -> Result<(), EnclaveError> {
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

    let target_dir = manifest_dir.join("target").join(SGX_TARGET).join("release");
    let elf_path = target_dir.join("enclave");
    let sgxs_path = target_dir.join("enclave.sgxs");
    run_command(
        Command::new("ftxsgx-elf2sgxs")
            .arg(&elf_path)
            .arg("--library")
            .arg("--debug")
            .arg("--heap-size")
            .arg(env_or_default(
                STAR_ENCLAVE_HEAP_SIZE,
                DEFAULT_ENCLAVE_HEAP_SIZE,
            ))
            .arg("--stack-size")
            .arg(env_or_default(
                STAR_ENCLAVE_STACK_SIZE,
                DEFAULT_ENCLAVE_STACK_SIZE,
            ))
            .arg("--threads")
            .arg(default_enclave_threads())
            .arg("--output")
            .arg(&sgxs_path),
    )
}

fn env_or_default(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}

fn default_enclave_threads() -> String {
    std::env::var(STAR_ENCLAVE_THREADS).unwrap_or_else(|_| {
        std::thread::available_parallelism()
            .map(|threads| threads.get())
            .unwrap_or(1)
            .to_string()
    })
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

fn initial_response_capacity(op: u64, payload_len: usize) -> usize {
    match op {
        OP_INIT if payload_len == 0 => INIT_RESPONSE_CAPACITY,
        OP_INIT => 0,
        OP_PUBLIC_KEY => PUBLIC_KEY_RESPONSE_CAPACITY,
        OP_ISSUE_CRED => ISSUE_CRED_RESPONSE_CAPACITY,
        OP_ISSUE_TOKEN => ISSUE_TOKEN_RESPONSE_CAPACITY,
        OP_ATTEST => ATTEST_RESPONSE_CAPACITY,
        _ => 1024,
    }
}

struct SealedKeyPaths {
    enc: PathBuf,
    prf: PathBuf,
}

#[cfg(test)]
fn sealed_key_paths() -> SealedKeyPaths {
    let dir = std::env::temp_dir().join(format!("star-app-sgx-test-{}", std::process::id()));
    SealedKeyPaths {
        enc: dir.join(SEALED_ENC_KEY_FILE),
        prf: dir.join(SEALED_PRF_KEY_FILE),
    }
}

#[cfg(not(test))]
fn sealed_key_paths() -> SealedKeyPaths {
    let enc = std::env::var_os("STAR_SEALED_ENC_KEY_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(SEALED_ENC_KEY_FILE));
    let prf = std::env::var_os("STAR_SEALED_PRF_KEY_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(SEALED_PRF_KEY_FILE));

    SealedKeyPaths { enc, prf }
}

fn read_sealed_keys(paths: &SealedKeyPaths) -> Result<Vec<u8>, EnclaveError> {
    let enc = read_optional(&paths.enc)?;
    let prf = read_optional(&paths.prf)?;

    match (enc, prf) {
        (Some(enc), Some(prf)) => Ok(encode_sealed_keys(&enc, &prf)),
        _ => Ok(Vec::new()),
    }
}

fn read_optional(path: &PathBuf) -> Result<Option<Vec<u8>>, EnclaveError> {
    match fs::read(path) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(EnclaveError::Storage(err)),
    }
}

fn write_sealed_keys(
    paths: &SealedKeyPaths,
    sealed_enc_key: &[u8],
    sealed_prf_key: &[u8],
) -> Result<(), EnclaveError> {
    if let Some(parent) = paths.enc.parent() {
        fs::create_dir_all(parent).map_err(EnclaveError::Storage)?;
    }
    if let Some(parent) = paths.prf.parent() {
        fs::create_dir_all(parent).map_err(EnclaveError::Storage)?;
    }

    fs::write(&paths.enc, sealed_enc_key).map_err(EnclaveError::Storage)?;
    fs::write(&paths.prf, sealed_prf_key).map_err(EnclaveError::Storage)
}

fn encode_sealed_keys(sealed_enc_key: &[u8], sealed_prf_key: &[u8]) -> Vec<u8> {
    let mut payload = Vec::with_capacity(8 + sealed_enc_key.len() + sealed_prf_key.len());
    append_blob(&mut payload, sealed_enc_key);
    append_blob(&mut payload, sealed_prf_key);
    payload
}

fn append_blob(payload: &mut Vec<u8>, blob: &[u8]) {
    payload.extend_from_slice(&(blob.len() as u32).to_be_bytes());
    payload.extend_from_slice(blob);
}

fn decode_sealed_keys(payload: &[u8]) -> Result<(&[u8], &[u8]), EnclaveError> {
    let mut remaining = payload;
    let sealed_enc_key = take_blob(&mut remaining)?;
    let sealed_prf_key = take_blob(&mut remaining)?;
    if !remaining.is_empty() {
        return Err(EnclaveError::InvalidSealedKeys(
            "trailing bytes in init response".to_string(),
        ));
    }

    Ok((sealed_enc_key, sealed_prf_key))
}

fn take_blob<'a>(payload: &mut &'a [u8]) -> Result<&'a [u8], EnclaveError> {
    let len: [u8; 4] = payload
        .get(..4)
        .ok_or_else(|| EnclaveError::InvalidSealedKeys("missing blob length".to_string()))?
        .try_into()
        .expect("fixed slice");
    let len = u32::from_be_bytes(len) as usize;
    let blob = payload
        .get(4..4 + len)
        .ok_or_else(|| EnclaveError::InvalidSealedKeys("truncated blob".to_string()))?;
    *payload = &payload[4 + len..];
    Ok(blob)
}
