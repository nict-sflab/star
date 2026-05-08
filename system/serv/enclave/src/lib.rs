use rand::{rngs::OsRng, RngCore};
use star_core::serv::enclave::Enclave;
use std::io::{self, ErrorKind, Read, Write};
use std::panic::{self, AssertUnwindSafe};

pub const MAGIC: &[u8; 8] = b"STARIPC1";
pub const OP_INIT: u8 = 0;
pub const OP_PUBLIC_KEY: u8 = 1;
pub const OP_ISSUE_CRED: u8 = 2;
pub const OP_ISSUE_TOKEN: u8 = 3;

pub const STATUS_OK: u8 = 0;
pub const STATUS_NOT_INITIALIZED: u8 = 1;
pub const STATUS_BAD_REQUEST: u8 = 2;
pub const STATUS_ENCLAVE_ERROR: u8 = 3;
pub const STATUS_PANIC: u8 = 4;

const INIT_PAYLOAD_LEN: usize = 4;
const PERIOD_LEN: usize = 8;
const MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;

struct EnclaveApp {
    enclave: Option<Enclave>,
    rng: OsRng,
}

impl EnclaveApp {
    fn new() -> Self {
        Self {
            enclave: None,
            rng: OsRng,
        }
    }

    fn handle(&mut self, op: u8, payload: &[u8]) -> (u8, Vec<u8>) {
        match op {
            OP_INIT => self.init(payload),
            OP_PUBLIC_KEY => self.public_key(payload),
            OP_ISSUE_CRED => self.issue_cred(payload),
            OP_ISSUE_TOKEN => self.issue_token(payload),
            _ => (STATUS_BAD_REQUEST, b"unknown operation".to_vec()),
        }
    }

    fn init(&mut self, payload: &[u8]) -> (u8, Vec<u8>) {
        if payload.len() != INIT_PAYLOAD_LEN {
            return (STATUS_BAD_REQUEST, b"bad init payload".to_vec());
        }

        let max_count = u32::from_be_bytes(payload.try_into().expect("fixed slice"));
        let mut sk = [0u8; 32];
        let mut cipher_receiver = [0u8; 32];
        self.rng.fill_bytes(&mut sk);
        self.rng.fill_bytes(&mut cipher_receiver);

        match Enclave::new(max_count, sk, cipher_receiver) {
            Ok(enclave) => {
                self.enclave = Some(enclave);
                (STATUS_OK, Vec::new())
            }
            Err(_) => (
                STATUS_ENCLAVE_ERROR,
                b"failed to initialize enclave".to_vec(),
            ),
        }
    }

    fn public_key(&self, payload: &[u8]) -> (u8, Vec<u8>) {
        if !payload.is_empty() {
            return (STATUS_BAD_REQUEST, b"public_key takes no payload".to_vec());
        }

        match self.enclave.as_ref() {
            Some(enclave) => (STATUS_OK, enclave.public_key().to_vec()),
            None => (
                STATUS_NOT_INITIALIZED,
                b"enclave is not initialized".to_vec(),
            ),
        }
    }

    fn issue_cred(&mut self, payload: &[u8]) -> (u8, Vec<u8>) {
        let Some(enclave) = self.enclave.as_ref() else {
            return (
                STATUS_NOT_INITIALIZED,
                b"enclave is not initialized".to_vec(),
            );
        };

        match panic::catch_unwind(AssertUnwindSafe(|| {
            enclave.issue_cred(payload, &mut self.rng)
        })) {
            Ok(cred) => (STATUS_OK, cred),
            Err(_) => (STATUS_PANIC, b"issue_cred panicked".to_vec()),
        }
    }

    fn issue_token(&self, payload: &[u8]) -> (u8, Vec<u8>) {
        let Some(enclave) = self.enclave.as_ref() else {
            return (
                STATUS_NOT_INITIALIZED,
                b"enclave is not initialized".to_vec(),
            );
        };
        if payload.len() < PERIOD_LEN {
            return (STATUS_BAD_REQUEST, b"issue_token missing period".to_vec());
        }

        let current_period = payload[..PERIOD_LEN].try_into().expect("fixed slice");
        let cipher = &payload[PERIOD_LEN..];

        match panic::catch_unwind(AssertUnwindSafe(|| {
            enclave.issue_token(cipher, current_period)
        })) {
            Ok(Ok(token)) => (STATUS_OK, token),
            Ok(Err(code)) => (STATUS_ENCLAVE_ERROR, vec![code]),
            Err(_) => (STATUS_PANIC, b"issue_token panicked".to_vec()),
        }
    }
}

pub fn run_stdio() -> io::Result<()> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();
    let mut app = EnclaveApp::new();

    loop {
        let Some((op, payload)) = read_request(&mut stdin)? else {
            return Ok(());
        };
        let (status, response) = app.handle(op, &payload);
        write_response(&mut stdout, status, &response)?;
    }
}

fn read_request(input: &mut impl Read) -> io::Result<Option<(u8, Vec<u8>)>> {
    let mut magic = [0u8; MAGIC.len()];
    match input.read_exact(&mut magic) {
        Ok(()) => {}
        Err(err) if err.kind() == ErrorKind::UnexpectedEof => return Ok(None),
        Err(err) => return Err(err),
    }

    if &magic != MAGIC {
        return Err(io::Error::new(ErrorKind::InvalidData, "bad request magic"));
    }

    let mut op = [0u8; 1];
    input.read_exact(&mut op)?;

    let mut len = [0u8; 4];
    input.read_exact(&mut len)?;
    let len = u32::from_be_bytes(len) as usize;
    if len > MAX_PAYLOAD_LEN {
        return Err(io::Error::new(ErrorKind::InvalidData, "payload too large"));
    }

    let mut payload = vec![0u8; len];
    input.read_exact(&mut payload)?;
    Ok(Some((op[0], payload)))
}

fn write_response(output: &mut impl Write, status: u8, payload: &[u8]) -> io::Result<()> {
    output.write_all(&[status])?;
    output.write_all(&(payload.len() as u32).to_be_bytes())?;
    output.write_all(payload)?;
    output.flush()
}
