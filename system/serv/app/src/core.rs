use rand_core::{OsRng, RngCore};
use star_core::serv::enclave::Enclave;
use std::env;
use std::fmt;
use std::io::{self, BufReader, ErrorKind, Read, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::{Mutex, OnceLock};

const MAGIC: &[u8; 8] = b"STARIPC1";
const OP_INIT: u8 = 0;
const OP_PUBLIC_KEY: u8 = 1;
const OP_ISSUE_CRED: u8 = 2;
const OP_ISSUE_TOKEN: u8 = 3;

const STATUS_OK: u8 = 0;
const STATUS_ENCLAVE_ERROR: u8 = 3;
const MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;

pub static ENCLAVE: OnceLock<EnclaveRuntime> = OnceLock::new();

pub enum EnclaveRuntime {
    Direct(Enclave),
    Stdio(Mutex<StdioEnclave>),
}

impl EnclaveRuntime {
    pub fn from_env(max_count: u32) -> Result<Self, EnclaveInitError> {
        match env::var("STAR_ENCLAVE_MODE")
            .unwrap_or_else(|_| "direct".to_string())
            .as_str()
        {
            "direct" | "" => Self::direct_random(max_count),
            "stdio" => Self::stdio(max_count),
            mode => Err(EnclaveInitError::BadMode(mode.to_string())),
        }
    }

    pub fn direct_random(max_count: u32) -> Result<Self, EnclaveInitError> {
        let mut rng = OsRng;
        let mut cipher_receiver = [0u8; 32];
        let mut sk = [0u8; 32];
        rng.fill_bytes(&mut cipher_receiver);
        rng.fill_bytes(&mut sk);
        Self::direct(max_count, sk, cipher_receiver)
    }

    pub fn direct(
        max_count: u32,
        sk: [u8; 32],
        cipher_receiver: [u8; 32],
    ) -> Result<Self, EnclaveInitError> {
        let enclave =
            Enclave::new(max_count, sk, cipher_receiver).map_err(|_| EnclaveInitError::Direct)?;
        Ok(Self::Direct(enclave))
    }

    pub fn stdio(max_count: u32) -> Result<Self, EnclaveInitError> {
        Ok(Self::Stdio(Mutex::new(StdioEnclave::spawn(max_count)?)))
    }

    pub fn public_key(&self) -> [u8; 32] {
        match self {
            Self::Direct(enclave) => enclave.public_key(),
            Self::Stdio(enclave) => enclave
                .lock()
                .expect("enclave stdio mutex poisoned")
                .public_key()
                .expect("enclave stdio public_key"),
        }
    }

    pub fn issue_cred<R>(&self, cipher: &[u8], rng: &mut R) -> Vec<u8>
    where
        R: rand_core::RngCore,
    {
        match self {
            Self::Direct(enclave) => enclave.issue_cred(cipher, rng),
            Self::Stdio(enclave) => enclave
                .lock()
                .expect("enclave stdio mutex poisoned")
                .issue_cred(cipher)
                .expect("enclave stdio issue_cred"),
        }
    }

    pub fn issue_token(&self, cipher: &[u8], current_period: [u8; 8]) -> Result<Vec<u8>, u8> {
        match self {
            Self::Direct(enclave) => enclave.issue_token(cipher, current_period),
            Self::Stdio(enclave) => enclave
                .lock()
                .expect("enclave stdio mutex poisoned")
                .issue_token(cipher, current_period)
                .map_err(|err| err.token_error_code()),
        }
    }
}

pub struct StdioEnclave {
    _child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

impl StdioEnclave {
    fn spawn(max_count: u32) -> Result<Self, EnclaveInitError> {
        let bin = env::var("STAR_ENCLAVE_BIN").unwrap_or_else(|_| default_enclave_bin());
        let mut child = Command::new(&bin)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|source| EnclaveInitError::Spawn {
                command: bin.clone(),
                source,
            })?;

        let stdin = child
            .stdin
            .take()
            .ok_or(EnclaveInitError::MissingPipe("stdin"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or(EnclaveInitError::MissingPipe("stdout"))?;

        let mut enclave = Self {
            _child: child,
            stdin,
            stdout: BufReader::new(stdout),
        };

        let payload = max_count.to_be_bytes();
        enclave.request(OP_INIT, &payload)?;

        Ok(enclave)
    }

    fn public_key(&mut self) -> Result<[u8; 32], EnclaveClientError> {
        let response = self.request(OP_PUBLIC_KEY, &[])?;
        response
            .as_slice()
            .try_into()
            .map_err(|_| EnclaveClientError::InvalidResponse("bad public_key length"))
    }

    fn issue_cred(&mut self, cipher: &[u8]) -> Result<Vec<u8>, EnclaveClientError> {
        self.request(OP_ISSUE_CRED, cipher)
    }

    fn issue_token(
        &mut self,
        cipher: &[u8],
        current_period: [u8; 8],
    ) -> Result<Vec<u8>, EnclaveClientError> {
        let mut payload = Vec::with_capacity(current_period.len() + cipher.len());
        payload.extend_from_slice(&current_period);
        payload.extend_from_slice(cipher);
        self.request(OP_ISSUE_TOKEN, &payload)
    }

    fn request(&mut self, op: u8, payload: &[u8]) -> Result<Vec<u8>, EnclaveClientError> {
        if payload.len() > MAX_PAYLOAD_LEN {
            return Err(EnclaveClientError::InvalidResponse("request too large"));
        }

        self.stdin.write_all(MAGIC)?;
        self.stdin.write_all(&[op])?;
        self.stdin
            .write_all(&(payload.len() as u32).to_be_bytes())?;
        self.stdin.write_all(payload)?;
        self.stdin.flush()?;

        let mut status = [0u8; 1];
        self.stdout.read_exact(&mut status)?;

        let mut len = [0u8; 4];
        self.stdout.read_exact(&mut len)?;
        let len = u32::from_be_bytes(len) as usize;
        if len > MAX_PAYLOAD_LEN {
            return Err(EnclaveClientError::InvalidResponse("response too large"));
        }

        let mut response = vec![0u8; len];
        self.stdout.read_exact(&mut response)?;

        if status[0] == STATUS_OK {
            Ok(response)
        } else {
            Err(EnclaveClientError::Remote {
                status: status[0],
                payload: response,
            })
        }
    }
}

#[derive(Debug)]
pub enum EnclaveInitError {
    BadMode(String),
    Direct,
    MissingPipe(&'static str),
    Spawn { command: String, source: io::Error },
    Client(EnclaveClientError),
}

impl fmt::Display for EnclaveInitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadMode(mode) => write!(f, "unknown STAR_ENCLAVE_MODE {mode:?}"),
            Self::Direct => write!(f, "failed to initialize direct enclave"),
            Self::MissingPipe(pipe) => write!(f, "enclave child missing {pipe} pipe"),
            Self::Spawn { command, source } => {
                write!(f, "failed to spawn enclave command {command:?}: {source}")
            }
            Self::Client(err) => write!(f, "failed to initialize enclave over stdio: {err}"),
        }
    }
}

impl std::error::Error for EnclaveInitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Spawn { source, .. } => Some(source),
            Self::Client(err) => Some(err),
            _ => None,
        }
    }
}

impl From<EnclaveClientError> for EnclaveInitError {
    fn from(err: EnclaveClientError) -> Self {
        Self::Client(err)
    }
}

#[derive(Debug)]
pub enum EnclaveClientError {
    Io(io::Error),
    InvalidResponse(&'static str),
    Remote { status: u8, payload: Vec<u8> },
}

impl EnclaveClientError {
    fn token_error_code(&self) -> u8 {
        match self {
            Self::Remote {
                status: STATUS_ENCLAVE_ERROR,
                payload,
            } => payload.first().copied().unwrap_or(STATUS_ENCLAVE_ERROR),
            _ => u8::MAX,
        }
    }
}

impl fmt::Display for EnclaveClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{err}"),
            Self::InvalidResponse(message) => write!(f, "{message}"),
            Self::Remote { status, payload } => match std::str::from_utf8(payload) {
                Ok(message) => write!(f, "remote enclave error {status}: {message}"),
                Err(_) => write!(f, "remote enclave error {status}: {} bytes", payload.len()),
            },
        }
    }
}

impl std::error::Error for EnclaveClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for EnclaveClientError {
    fn from(err: io::Error) -> Self {
        if err.kind() == ErrorKind::BrokenPipe || err.kind() == ErrorKind::UnexpectedEof {
            return Self::InvalidResponse("enclave child closed stdio");
        }
        Self::Io(err)
    }
}

fn default_enclave_bin() -> String {
    let profile = if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    };
    let manifest_dir = option_env!("CARGO_MANIFEST_DIR").unwrap_or(".");

    PathBuf::from(manifest_dir)
        .join("../enclave/target")
        .join(profile)
        .join("enclave")
        .to_string_lossy()
        .into_owned()
}
