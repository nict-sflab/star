mod attest;
mod peer_sync;
mod seal;

pub use peer_sync::{OP_PEER_SYNC_BEGIN, OP_PEER_SYNC_EXPORT, OP_PEER_SYNC_IMPORT};

use rand::{rngs::OsRng, Rng};
use star_core::serv::enclave::Enclave;
use std::{fs, io, path::Path};
use utils::io::{
    OP_ISSUE_CRED, OP_ISSUE_TOKEN, OP_PUBLIC_KEY, STATUS_BAD_REQUEST, STATUS_OK, STATUS_PANIC,
};

const PERIOD_LEN: usize = 8;
const MAX_TICKETS: u32 = 3_000_000_000;
const SEALED_ENC_KEY_FILE: &str = "sealed_enc_key.bin";
const SEALED_PRF_KEY_FILE: &str = "sealed_prf_key.bin";

struct EnclaveApp {
    enclave: Enclave,
    secrets: peer_sync::PeerSecrets,
    max_tickets: u32,
    rng: OsRng,
    peer_sync: peer_sync::PeerSync,
}

impl EnclaveApp {
    fn new(max_tickets: u32, enclave: Enclave, secrets: peer_sync::PeerSecrets) -> Self {
        Self {
            enclave,
            secrets,
            max_tickets,
            rng: OsRng,
            peer_sync: peer_sync::PeerSync::new(),
        }
    }

    fn init<T: Rng>(max: u32, mut rng: T) -> io::Result<Self> {
        let mut sk = [0u8; 32];
        let mut cipher_receiver: [u8; 32] = [0u8; 32];
        rng.fill_bytes(&mut sk);
        rng.fill_bytes(&mut cipher_receiver);

        let secrets = peer_sync::PeerSecrets {
            token_key: sk,
            hpke_receiver_key: cipher_receiver,
        };
        write_sealed_secrets(&secrets)?;

        let enclave =
            Enclave::new(max, secrets.token_key, secrets.hpke_receiver_key).map_err(to_io)?;

        Ok(Self::new(max, enclave, secrets))
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

        let secrets = peer_sync::PeerSecrets {
            token_key: sk,
            hpke_receiver_key: cipher_receiver,
        };
        let enclave =
            Enclave::new(max, secrets.token_key, secrets.hpke_receiver_key).map_err(to_io)?;

        Ok(Self::new(max, enclave, secrets))
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

    #[allow(dead_code)]
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

    fn peer_sync_begin(&mut self, payload: &[u8]) -> (u8, Vec<u8>) {
        if !payload.is_empty() {
            return (
                STATUS_BAD_REQUEST,
                b"peer_sync_begin does not accept a payload".to_vec(),
            );
        }

        match self.peer_sync.begin(&mut self.rng) {
            Ok(challenge) => (STATUS_OK, challenge),
            Err(err) => peer_sync_error(err),
        }
    }

    fn peer_sync_export(&mut self, payload: &[u8]) -> (u8, Vec<u8>) {
        match self
            .peer_sync
            .export(self.secrets.clone(), payload, &mut self.rng)
        {
            Ok(transfer) => (STATUS_OK, transfer),
            Err(err) => peer_sync_error(err),
        }
    }

    fn peer_sync_import(&mut self, payload: &[u8]) -> (u8, Vec<u8>) {
        let secrets = match self.peer_sync.import(payload) {
            Ok(secrets) => secrets,
            Err(err) => return peer_sync_error(err),
        };

        match self.install_peer_secrets(secrets) {
            Ok(()) => (STATUS_OK, b"peer secrets installed".to_vec()),
            Err(err) => (STATUS_PANIC, err.to_string().into_bytes()),
        }
    }

    fn install_peer_secrets(&mut self, secrets: peer_sync::PeerSecrets) -> io::Result<()> {
        let enclave = Enclave::new(
            self.max_tickets,
            secrets.token_key,
            secrets.hpke_receiver_key,
        )
        .map_err(to_io)?;

        write_sealed_secrets(&secrets)?;

        self.enclave = enclave;
        self.secrets = secrets;
        Ok(())
    }
}

impl utils::child::Commands for EnclaveApp {
    fn handle(&mut self, op: u8, payload: &[u8]) -> (u8, Vec<u8>) {
        match op {
            OP_PUBLIC_KEY => self.public_key(payload),
            OP_ISSUE_CRED => self.issue_cred(payload),
            OP_ISSUE_TOKEN => self.issue_token(payload),
            peer_sync::OP_PEER_SYNC_BEGIN => self.peer_sync_begin(payload),
            peer_sync::OP_PEER_SYNC_EXPORT => self.peer_sync_export(payload),
            peer_sync::OP_PEER_SYNC_IMPORT => self.peer_sync_import(payload),
            _ => (STATUS_BAD_REQUEST, b"unknown operation".to_vec()),
        }
    }
}

use crate::seal::{ENC_KEY, PRF_KEY};

pub fn run_stdio() -> io::Result<()> {
    let has_sealed_keys =
        Path::new(SEALED_ENC_KEY_FILE).exists() && Path::new(SEALED_PRF_KEY_FILE).exists();
    let mut app = if has_sealed_keys {
        EnclaveApp::load(MAX_TICKETS)?
    } else {
        EnclaveApp::init(MAX_TICKETS, OsRng)?
    };

    utils::child::run_stdio(&mut app)
}

fn write_sealed_secrets(secrets: &peer_sync::PeerSecrets) -> io::Result<()> {
    let sealed_enc_key = seal::seal(ENC_KEY, &secrets.token_key).map_err(to_io)?;
    let sealed_prf_key = seal::seal(PRF_KEY, &secrets.hpke_receiver_key).map_err(to_io)?;
    fs::write(SEALED_ENC_KEY_FILE, sealed_enc_key)?;
    fs::write(SEALED_PRF_KEY_FILE, sealed_prf_key)?;
    Ok(())
}

fn peer_sync_error(err: peer_sync::PeerSyncError) -> (u8, Vec<u8>) {
    let status = if err.is_bad_request() {
        STATUS_BAD_REQUEST
    } else {
        STATUS_PANIC
    };
    (status, err.to_string().into_bytes())
}

fn to_io<E: std::fmt::Debug>(err: E) -> io::Error {
    io::Error::new(io::ErrorKind::Other, format!("{err:?}"))
}
