use std::error::Error;
use std::fmt;
use std::vec;
use std::path::PathBuf;
use tokio::sync::Mutex;
use utils::io::{OP_ISSUE_CRED, OP_ISSUE_TOKEN, OP_PUBLIC_KEY};
use utils::parent::SubprocessError;

const CARGO_NAME : &str = "cargo";

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
        Ok(EnclaveRuntime { process })
    }

    pub fn from_env() -> Result<Self, EnclaveError> {
        let args = default_enclave_bin();
        Self::init(&args)
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

    println!("{:?}", args);

    if cfg!(debug_assertions) {
    } else {
        args.append(
            vec![
                "--release".to_string(),
                "--target".to_string(),
                "x86_64-fortanix-unknown-sgx".to_string()
            ].as_mut()
        )
    }

    args
}
