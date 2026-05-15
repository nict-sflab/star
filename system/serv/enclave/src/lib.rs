use rand::{rngs::OsRng, Rng};
use star_core::serv::enclave::Enclave;
use std::io;
use utils::io::{
    OP_ISSUE_CRED, OP_ISSUE_TOKEN, OP_PUBLIC_KEY,
    STATUS_BAD_REQUEST, STATUS_OK, STATUS_PANIC,
};

use hpke::HpkeError;

//const INIT_PAYLOAD_LEN: usize = 4;
const PERIOD_LEN: usize = 8;

struct EnclaveApp {
    enclave: Enclave,
    rng: OsRng,
}

impl EnclaveApp {
    fn new(enclave: Enclave) -> Self {
        Self {
            enclave,
            rng: OsRng,
        }
    }

    fn init<T: Rng>(max: u32, mut rng: T) -> Result<Self, HpkeError> {
        let mut sk = [0u8; 32];
        let mut cipher_receiver = [0u8; 32];
        rng.fill_bytes(&mut sk);
        rng.fill_bytes(&mut cipher_receiver);

        let enclave = Enclave::new(max, sk, cipher_receiver)?;

        Ok(Self::new(enclave))
    }

    fn public_key(&self, _: &[u8]) -> (u8, Vec<u8>) {
        (STATUS_OK, self.enclave.public_key().to_vec())
    }

    fn issue_cred(&mut self, payload: &[u8]) -> (u8, Vec<u8>) {
        let cred = self.enclave.issue_cred(payload, &mut self.rng);
        (STATUS_OK, cred)
    }

    fn issue_token(&self, payload: &[u8]) -> (u8, Vec<u8>) {
        if payload.len() < PERIOD_LEN {
            return (STATUS_BAD_REQUEST, b"issue_token missing period".to_vec());
        }

        let current_period = payload[..PERIOD_LEN].try_into().expect("fixed slice");
        let cipher = &payload[PERIOD_LEN..];

        match self.enclave.issue_token(cipher, current_period) {
            Ok(token) => (STATUS_OK, token),
            Err(_) => (STATUS_PANIC, b"issue_token panicked".to_vec()),
        }
    }
}

impl utils::child::Commands for EnclaveApp {
    fn handle(&mut self, op: u8, payload: &[u8]) -> (u8, Vec<u8>) {
        match op {
            OP_PUBLIC_KEY => self.public_key(payload),
            OP_ISSUE_CRED => self.issue_cred(payload),
            OP_ISSUE_TOKEN => self.issue_token(payload),
            _ => (STATUS_BAD_REQUEST, b"unknown operation".to_vec()),
        }
    }
}
use rand::{rngs::StdRng, SeedableRng};

pub fn run_stdio() -> io::Result<()> {
    let rng = StdRng::seed_from_u64(10);
    let mut app = EnclaveApp::init(3000000000, rng).unwrap();

    utils::child::run_stdio(&mut app)
}
