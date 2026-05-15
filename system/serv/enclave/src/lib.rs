mod attest;
mod seal;

use rand::{rngs::OsRng, Rng};
use star_core::serv::enclave::Enclave;
use std::{fs, io, path::Path};
use utils::io::{
    OP_ATTEST, OP_ISSUE_CRED, OP_ISSUE_TOKEN, OP_PUBLIC_KEY, STATUS_BAD_REQUEST, STATUS_OK,
    STATUS_PANIC,
};

const PERIOD_LEN: usize = 8;
const MAX_TICKETS: u32 = 3_000_000_000;
const SEALED_ENC_KEY_FILE: &str = "sealed_enc_key.bin";
const SEALED_PRF_KEY_FILE: &str = "sealed_prf_key.bin";

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

    fn init<T: Rng>(max: u32, mut rng: T) -> io::Result<Self> {
        let mut sk = [0u8; 32];
        let mut cipher_receiver: [u8; 32] = [0u8; 32];
        rng.fill_bytes(&mut sk);
        rng.fill_bytes(&mut cipher_receiver);

        let sealed_enc_key = seal::seal(ENC_KEY, &sk).map_err(to_io)?;
        let sealed_prf_key = seal::seal(PRF_KEY, &cipher_receiver).map_err(to_io)?;
        fs::write(SEALED_ENC_KEY_FILE, sealed_enc_key)?;
        fs::write(SEALED_PRF_KEY_FILE, sealed_prf_key)?;

        let enclave = Enclave::new(max, sk, cipher_receiver).map_err(to_io)?;

        Ok(Self::new(enclave))
    }

    fn load(max: u32) -> io::Result<Self> {
        let sealed_enc_key = fs::read(SEALED_ENC_KEY_FILE)?;
        let sealed_prf_key = fs::read(SEALED_PRF_KEY_FILE)?;

        let sk: [u8; 32] = seal::unseal(ENC_KEY, &sealed_enc_key)
            .map_err(to_io)?
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad sealed enc key"))?;
        let cipher_receiver: [u8; 32] = seal::unseal(PRF_KEY, &sealed_prf_key)
            .map_err(to_io)?
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad sealed prf key"))?;

        let enclave = Enclave::new(max, sk, cipher_receiver).map_err(to_io)?;

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

    fn attest(&self, payload: &[u8]) -> (u8, Vec<u8>) {
        let report_data: [u8; 64] = match payload.try_into() {
            Ok(report_data) => report_data,
            Err(_) => return (STATUS_BAD_REQUEST, b"attest needs 64 bytes".to_vec()),
        };

        match attest::attest(report_data) {
            Ok(quote) => (STATUS_OK, quote),
            Err(err) => (STATUS_PANIC, err.into_bytes()),
        }
    }
}

impl utils::child::Commands for EnclaveApp {
    fn handle(&mut self, op: u8, payload: &[u8]) -> (u8, Vec<u8>) {
        match op {
            OP_PUBLIC_KEY => self.public_key(payload),
            OP_ISSUE_CRED => self.issue_cred(payload),
            OP_ISSUE_TOKEN => self.issue_token(payload),
            OP_ATTEST => self.attest(payload),
            _ => (STATUS_BAD_REQUEST, b"unknown operation".to_vec()),
        }
    }
}
use rand::{rngs::StdRng, SeedableRng};

use crate::seal::{ENC_KEY, PRF_KEY};

pub fn run_stdio() -> io::Result<()> {
    let rng = StdRng::seed_from_u64(10);
    let has_sealed_keys =
        Path::new(SEALED_ENC_KEY_FILE).exists() && Path::new(SEALED_PRF_KEY_FILE).exists();
    let mut app = if has_sealed_keys {
        EnclaveApp::load(MAX_TICKETS)?
    } else {
        EnclaveApp::init(MAX_TICKETS, rng)?
    };

    utils::child::run_stdio(&mut app)
}

fn to_io<E: std::fmt::Debug>(err: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{err:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::child::Commands;

    #[test]
    fn attest_dispatch_rejects_non_64_byte_challenge() {
        let enclave = Enclave::new(1, [1u8; 32], [2u8; 32]).expect("enclave");
        let mut app = EnclaveApp::new(enclave);

        let (status, payload) = app.handle(OP_ATTEST, b"short");

        assert_eq!(status, STATUS_BAD_REQUEST);
        assert_eq!(payload, b"attest needs 64 bytes");
    }
}
