use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::vec;
use tokio::sync::Mutex;
use utils::io::{OP_INIT, OP_ISSUE_CRED, OP_ISSUE_TOKEN, OP_PUBLIC_KEY};
use utils::parent::SubprocessError;

const CARGO_NAME: &str = "cargo";
const SEALED_ENC_KEY_FILE: &str = "sealed_enc_key.bin";
const SEALED_PRF_KEY_FILE: &str = "sealed_prf_key.bin";

pub struct EnclaveRuntimes {
    inner: Mutex<Vec<EnclaveRuntime>>,
}

#[derive(Debug)]
pub enum EnclaveError {
    InvalidPublicKeyLength { len: usize },
    InvalidSealedKeys(String),
    Storage(io::Error),
    Subprocess(SubprocessError),
}

impl fmt::Display for EnclaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPublicKeyLength { len } => {
                write!(f, "enclave returned public key with invalid length {len}")
            }
            Self::InvalidSealedKeys(err) => write!(f, "invalid sealed keys: {err}"),
            Self::Storage(err) => write!(f, "sealed key storage failed: {err}"),
            Self::Subprocess(err) => write!(f, "{err}"),
        }
    }
}

impl Error for EnclaveError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidPublicKeyLength { .. } => None,
            Self::InvalidSealedKeys(_) => None,
            Self::Storage(err) => Some(err),
            Self::Subprocess(err) => Some(err),
        }
    }
}

impl From<SubprocessError> for EnclaveError {
    fn from(err: SubprocessError) -> Self {
        Self::Subprocess(err)
    }
}

impl EnclaveRuntimes {
    pub fn init(args: &[String], num: u8) -> Result<Self, EnclaveError> {
        let mut in_use = vec![];

        for _ in 0..num {
            let enclave = EnclaveRuntime::init(args)?;
            in_use.push(enclave);
        }

        Ok(EnclaveRuntimes {
            inner: Mutex::new(in_use),
        })
    }
    pub fn from_env(num: u8) -> Result<Self, EnclaveError> {
        let args = default_enclave_bin();
        EnclaveRuntimes::init(&args, num)
    }

    pub async fn pick(&mut self) -> Option<EnclaveRuntime> {
        self.inner.lock().await.pop()
    }

    pub async fn release(&mut self, enclave: EnclaveRuntime) {
        self.inner.lock().await.push(enclave);
    }
}

pub struct EnclaveRuntime {
    process: utils::parent::Subprocess,
}

impl EnclaveRuntime {
    pub fn init(args: &[String]) -> Result<Self, EnclaveError> {
        let process = utils::parent::Subprocess::init(CARGO_NAME, args)?;
        let mut runtime = EnclaveRuntime { process };
        runtime.initialize()?;
        Ok(runtime)
    }

    pub fn from_env() -> Result<Self, EnclaveError> {
        let args = default_enclave_bin();
        Self::init(&args)
    }

    fn initialize(&mut self) -> Result<(), EnclaveError> {
        let key_paths = sealed_key_paths();
        let init_payload = read_sealed_keys(&key_paths)?;
        let response = self.process.call(OP_INIT, &init_payload)?;

        if init_payload.is_empty() {
            let (sealed_enc_key, sealed_prf_key) = decode_sealed_keys(&response)?;
            write_sealed_keys(&key_paths, sealed_enc_key, sealed_prf_key)?;
        }

        Ok(())
    }

    pub fn public_key(&mut self) -> Result<[u8; 32], EnclaveError> {
        let response = self.process.call(OP_PUBLIC_KEY, &[])?;
        let len = response.len();
        response
            .as_slice()
            .try_into()
            .map_err(|_| EnclaveError::InvalidPublicKeyLength { len })
    }

    pub fn issue_cred(&mut self, cipher: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        self.process.call(OP_ISSUE_CRED, cipher).map_err(Into::into)
    }

    pub fn issue_token(
        &mut self,
        cipher: &[u8],
        current_period: [u8; 8],
    ) -> Result<Vec<u8>, EnclaveError> {
        let mut payload = Vec::with_capacity(current_period.len() + cipher.len());
        payload.extend_from_slice(&current_period);
        payload.extend_from_slice(cipher);
        self.process
            .call(OP_ISSUE_TOKEN, &payload)
            .map_err(Into::into)
    }
}

fn default_enclave_bin() -> vec::Vec<String> {
    let manifest_dir = match option_env!("CARGO_MANIFEST_DIR") {
        Some(dir) => dir,
        None => ".",
    };

    let manifest_path = PathBuf::from(manifest_dir)
        .join("../enclave/Cargo.toml")
        .to_string_lossy()
        .into_owned();

    let mut args = vec![
        "run".to_string(),
        "--quiet".to_string(),
        "--manifest-path".to_string(),
        manifest_path,
    ];

    if cfg!(debug_assertions) {
    } else {
        args.append(
            vec![
                "--release".to_string(),
                "--target".to_string(),
                "x86_64-fortanix-unknown-sgx".to_string(),
            ]
            .as_mut(),
        )
    }

    println!("{:?}", args);

    args
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
