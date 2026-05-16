mod attest;
mod seal;

use rand::{rngs::OsRng, RngCore};
use star_core::serv::enclave::Enclave;
use std::io;
#[cfg(target_env = "sgx")]
use std::slice;
#[cfg(target_env = "sgx")]
use std::sync::{Mutex, OnceLock};

const PERIOD_LEN: usize = 8;
const MAX_TICKETS: u32 = 3_000_000_000;
const MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;

const OP_INIT: u8 = 0;
const OP_PUBLIC_KEY: u8 = 1;
const OP_ISSUE_CRED: u8 = 2;
const OP_ISSUE_TOKEN: u8 = 3;
const OP_ATTEST: u8 = 4;

const STATUS_OK: u8 = 0;
const STATUS_NOT_INITIALIZED: u8 = 1;
const STATUS_BAD_REQUEST: u8 = 2;
const STATUS_PANIC: u8 = 4;
const STATUS_BUFFER_TOO_SMALL: u8 = 5;

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

pub fn run() {}

#[cfg(target_env = "sgx")]
static LIBRARY_APP: OnceLock<Mutex<EnclaveApp>> = OnceLock::new();

#[cfg(target_env = "sgx")]
#[allow(improper_ctypes_definitions)]
#[no_mangle]
// Matches the Fortanix SGX library entry ABI: p1 selects the operation, p2-p5 carry buffers.
pub extern "C" fn entry(p1: u64, p2: u64, p3: u64, _ignore: u64, p4: u64, p5: u64) -> (u64, u64) {
    library_entry(p1, p2, p3, p4, p5)
}

#[cfg(target_env = "sgx")]
fn library_entry(
    op: u64,
    input_ptr: u64,
    input_len: u64,
    output_ptr: u64,
    output_len: u64,
) -> (u64, u64) {
    let op = match u8::try_from(op) {
        Ok(op) => op,
        Err(_) => return (STATUS_BAD_REQUEST as u64, 0),
    };

    let input = match unsafe { user_slice(input_ptr, input_len) } {
        Ok(input) => input,
        Err(status) => return (status as u64, 0),
    };
    let output = match unsafe { user_slice_mut(output_ptr, output_len) } {
        Ok(output) => output,
        Err(status) => return (status as u64, 0),
    };

    let app = LIBRARY_APP.get_or_init(|| Mutex::new(EnclaveApp::new(MAX_TICKETS)));
    let (status, response) = match app.lock() {
        Ok(mut app) => app.handle(op, input),
        Err(_) => (STATUS_PANIC, b"enclave app lock poisoned".to_vec()),
    };

    if response.len() > output.len() {
        return (STATUS_BUFFER_TOO_SMALL as u64, response.len() as u64);
    }

    output[..response.len()].copy_from_slice(&response);
    (status as u64, response.len() as u64)
}

#[cfg(target_env = "sgx")]
unsafe fn user_slice<'a>(ptr: u64, len: u64) -> Result<&'a [u8], u8> {
    let len = checked_user_len(len)?;
    if len == 0 {
        return Ok(&[]);
    }

    let ptr = ptr as *const u8;
    if ptr.is_null() || !is_user_range(ptr, len) {
        return Err(STATUS_BAD_REQUEST);
    }

    Ok(unsafe { slice::from_raw_parts(ptr, len) })
}

#[cfg(target_env = "sgx")]
unsafe fn user_slice_mut<'a>(ptr: u64, len: u64) -> Result<&'a mut [u8], u8> {
    let len = checked_user_len(len)?;
    if len == 0 {
        return Ok(&mut []);
    }

    let ptr = ptr as *mut u8;
    if ptr.is_null() || !is_user_range(ptr, len) {
        return Err(STATUS_BAD_REQUEST);
    }

    Ok(unsafe { slice::from_raw_parts_mut(ptr, len) })
}

#[cfg(target_env = "sgx")]
fn checked_user_len(len: u64) -> Result<usize, u8> {
    let len = usize::try_from(len).map_err(|_| STATUS_BAD_REQUEST)?;
    if len > MAX_PAYLOAD_LEN {
        return Err(STATUS_BAD_REQUEST);
    }
    Ok(len)
}

#[cfg(target_env = "sgx")]
fn is_user_range(ptr: *const u8, len: usize) -> bool {
    if len == 0 {
        return true;
    }

    let start = ptr as usize;
    let Some(end) = start.checked_add(len - 1) else {
        return false;
    };

    let base = image_base();
    let enclave_size = unsafe { ENCLAVE_SIZE };
    let Some(enclave_end) = base.checked_add(enclave_size.saturating_sub(1)) else {
        return false;
    };

    end < base || start > enclave_end
}

#[cfg(target_env = "sgx")]
fn image_base() -> usize {
    let base: usize;
    unsafe {
        std::arch::asm!(
            "lea IMAGE_BASE(%rip), {}",
            lateout(reg) base,
            options(att_syntax, nostack, preserves_flags, nomem, pure),
        );
    }
    base
}

#[cfg(target_env = "sgx")]
unsafe extern "C" {
    static ENCLAVE_SIZE: usize;
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
