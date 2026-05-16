mod attest;
mod seal;

use rand::{rngs::OsRng, RngCore};
use star_core::serv::enclave::Enclave;
use std::io;
use utils::io::{
    OP_ATTEST, OP_INIT, OP_ISSUE_CRED, OP_ISSUE_TOKEN, OP_PUBLIC_KEY, STATUS_BAD_REQUEST,
    STATUS_NOT_INITIALIZED, STATUS_OK, STATUS_PANIC,
};

const PERIOD_LEN: usize = 8;
const MAX_TICKETS: u32 = 3_000_000_000;

struct EnclaveApp {
    enclave: Option<Enclave>,
    rng: OsRng,
    max: u32,
}

impl EnclaveApp {
    fn new(max: u32) -> Self {
        Self {
            enclave: None,
            rng: OsRng,
            max,
        }
    }

    #[cfg(test)]
    fn with_enclave(enclave: Enclave) -> Self {
        Self {
            enclave: Some(enclave),
            rng: OsRng,
            max: MAX_TICKETS,
        }
    }

    fn init_fresh(&mut self) -> io::Result<Vec<u8>> {
        let mut sk = [0u8; 32];
        let mut cipher_receiver: [u8; 32] = [0u8; 32];
        self.rng.fill_bytes(&mut sk);
        self.rng.fill_bytes(&mut cipher_receiver);

        let sealed_enc_key = seal::seal(ENC_KEY, &sk).map_err(to_io)?;
        let sealed_prf_key = seal::seal(PRF_KEY, &cipher_receiver).map_err(to_io)?;

        let enclave = Enclave::new(self.max, sk, cipher_receiver).map_err(to_io)?;
        self.enclave = Some(enclave);

        Ok(encode_sealed_keys(&sealed_enc_key, &sealed_prf_key))
    }

    fn init_sealed(&mut self, payload: &[u8]) -> io::Result<()> {
        let (sealed_enc_key, sealed_prf_key) = decode_sealed_keys(payload)?;

        let sk: [u8; 32] = seal::unseal(ENC_KEY, sealed_enc_key)
            .map_err(to_io)?
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad sealed enc key"))?;
        let cipher_receiver: [u8; 32] = seal::unseal(PRF_KEY, sealed_prf_key)
            .map_err(to_io)?
            .try_into()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad sealed prf key"))?;

        let enclave = Enclave::new(self.max, sk, cipher_receiver).map_err(to_io)?;
        self.enclave = Some(enclave);

        Ok(())
    }

    fn init(&mut self, payload: &[u8]) -> (u8, Vec<u8>) {
        if self.enclave.is_some() {
            return (STATUS_BAD_REQUEST, b"enclave already initialized".to_vec());
        }

        let result = if payload.is_empty() {
            self.init_fresh()
        } else {
            self.init_sealed(payload).map(|_| Vec::new())
        };

        match result {
            Ok(payload) => (STATUS_OK, payload),
            Err(err) => (STATUS_PANIC, err.to_string().into_bytes()),
        }
    }

    fn enclave(&self) -> Result<&Enclave, (u8, Vec<u8>)> {
        self.enclave.as_ref().ok_or_else(|| {
            (
                STATUS_NOT_INITIALIZED,
                b"enclave is not initialized".to_vec(),
            )
        })
    }

    fn public_key(&self, _: &[u8]) -> (u8, Vec<u8>) {
        match self.enclave() {
            Ok(enclave) => (STATUS_OK, enclave.public_key().to_vec()),
            Err(err) => err,
        }
    }

    fn issue_cred(&mut self, payload: &[u8]) -> (u8, Vec<u8>) {
        let Some(enclave) = self.enclave.as_ref() else {
            return (
                STATUS_NOT_INITIALIZED,
                b"enclave is not initialized".to_vec(),
            );
        };

        let cred = enclave.issue_cred(payload, &mut self.rng);
        (STATUS_OK, cred)
    }

    fn issue_token(&self, payload: &[u8]) -> (u8, Vec<u8>) {
        if payload.len() < PERIOD_LEN {
            return (STATUS_BAD_REQUEST, b"issue_token missing period".to_vec());
        }

        let current_period = payload[..PERIOD_LEN].try_into().expect("fixed slice");
        let cipher = &payload[PERIOD_LEN..];

        let enclave = match self.enclave() {
            Ok(enclave) => enclave,
            Err(err) => return err,
        };

        match enclave.issue_token(cipher, current_period) {
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
            OP_INIT => self.init(payload),
            OP_PUBLIC_KEY => self.public_key(payload),
            OP_ISSUE_CRED => self.issue_cred(payload),
            OP_ISSUE_TOKEN => self.issue_token(payload),
            OP_ATTEST => self.attest(payload),
            _ => (STATUS_BAD_REQUEST, b"unknown operation".to_vec()),
        }
    }
}
use crate::seal::{ENC_KEY, PRF_KEY};

pub fn run_stdio() -> io::Result<()> {
    let mut app = EnclaveApp::new(MAX_TICKETS);
    utils::child::run_stdio(&mut app)
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

fn decode_sealed_keys(payload: &[u8]) -> io::Result<(&[u8], &[u8])> {
    let mut remaining = payload;
    let sealed_enc_key = take_blob(&mut remaining)?;
    let sealed_prf_key = take_blob(&mut remaining)?;
    if !remaining.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "trailing sealed key bytes",
        ));
    }

    Ok((sealed_enc_key, sealed_prf_key))
}

fn take_blob<'a>(payload: &mut &'a [u8]) -> io::Result<&'a [u8]> {
    let len: [u8; 4] = payload
        .get(..4)
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "missing blob length"))?
        .try_into()
        .expect("fixed slice");
    let len = u32::from_be_bytes(len) as usize;
    let blob = payload
        .get(4..4 + len)
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "truncated blob"))?;
    *payload = &payload[4 + len..];
    Ok(blob)
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
        let mut app = EnclaveApp::with_enclave(enclave);

        let (status, payload) = app.handle(OP_ATTEST, b"short");

        assert_eq!(status, STATUS_BAD_REQUEST);
        assert_eq!(payload, b"attest needs 64 bytes");
    }

    #[test]
    fn sealed_key_payload_round_trips() {
        let encoded = encode_sealed_keys(b"enc", b"prf");
        let (enc, prf) = decode_sealed_keys(&encoded).expect("decode");

        assert_eq!(enc, b"enc");
        assert_eq!(prf, b"prf");
    }
}
