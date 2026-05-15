use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use tokio::sync::Mutex;
use utils::io::{OP_ISSUE_CRED, OP_ISSUE_TOKEN, OP_PUBLIC_KEY};
use utils::parent::SubprocessError;

pub struct EnclaveRuntimes {
    inner: Mutex<Vec<EnclaveRuntime>>,
}

#[derive(Debug)]
pub enum EnclaveError {
    InvalidPublicKeyLength { len: usize },
    Subprocess(SubprocessError),
}

impl fmt::Display for EnclaveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPublicKeyLength { len } => {
                write!(f, "enclave returned public key with invalid length {len}")
            }
            Self::Subprocess(err) => write!(f, "{err}"),
        }
    }
}

impl Error for EnclaveError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidPublicKeyLength { .. } => None,
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
    pub fn init(name: &str, num: u8) -> Result<Self, EnclaveError> {
        let mut in_use = vec![];

        for _ in 0..num {
            let enclave = EnclaveRuntime::init(name)?;
            in_use.push(enclave);
        }

        Ok(EnclaveRuntimes {
            inner: Mutex::new(in_use),
        })
    }
    pub fn from_env(num: u8) -> Result<Self, EnclaveError> {
        let name = default_enclave_bin();
        EnclaveRuntimes::init(&name, num)
    }
    pub async fn pick(&self) -> Option<EnclaveRuntime> {
        self.inner.lock().await.pop()
    }

    pub async fn release(&self, enclave: EnclaveRuntime) {
        self.inner.lock().await.push(enclave);
    }

    pub fn try_pick(&self) -> Option<EnclaveRuntime> {
        self.inner.try_lock().ok()?.pop()
    }

    pub fn try_release(&self, enclave: EnclaveRuntime) -> bool {
        match self.inner.try_lock() {
            Ok(mut guard) => {
                guard.push(enclave);
                true
            }
            Err(_) => false,
        }
    }
}

pub struct EnclaveRuntime {
    process: utils::parent::Subprocess,
}

impl EnclaveRuntime {
    pub fn init(name: &str) -> Result<Self, EnclaveError> {
        let process = utils::parent::Subprocess::init(name)?;
        Ok(EnclaveRuntime { process })
    }

    pub fn from_env() -> Result<Self, EnclaveError> {
        let name = default_enclave_bin();
        Self::init(&name)
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

fn default_enclave_bin() -> String {
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let manifest_dir = match option_env!("CARGO_MANIFEST_DIR") {
        Some(dir) => dir,
        None => ".",
    };

    PathBuf::from(manifest_dir)
        .join("../enclave/target")
        .join(profile)
        .join("enclave")
        .to_string_lossy()
        .into_owned()
}
