#![cfg_attr(not(target_env = "sgx"), allow(dead_code))]

mod attest;

use hpke::{
    Deserializable, HpkeError, OpModeS, Serializable,
    aead::ChaCha20Poly1305 as HpkeChaCha20Poly1305,
    kdf::HkdfSha256,
    kem::{Kem as _, X25519HkdfSha256},
    setup_sender,
};
use rand::{RngCore, rngs::OsRng};
#[cfg(target_env = "sgx")]
use sgx_isa::{AttributesFlags, Report, Targetinfo};
#[cfg(target_env = "sgx")]
use star_attestation::parse_measurement_hex;
use star_attestation::{
    PUBLIC_KEY_LEN, QuotePolicy, report_data_for_public_key, verify_qve_evidence_public_key,
};
#[cfg(target_env = "sgx")]
use std::slice;
#[cfg(target_env = "sgx")]
use std::sync::{OnceLock, RwLock};

const MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;

const OP_INIT: u8 = 0;
const OP_PUBLIC_KEY: u8 = 1;
const OP_ATTEST: u8 = 4;
const OP_EXPORT_KEYS: u8 = 5;
const OP_QVE_TARGET_INFO: u8 = 6;

const STATUS_OK: u8 = 0;
const STATUS_BAD_REQUEST: u8 = 2;
const STATUS_PANIC: u8 = 4;
const STATUS_BUFFER_TOO_SMALL: u8 = 5;

struct KeyManagerApp {
    keys: KeyBundle,
}

impl KeyManagerApp {
    fn new() -> Self {
        Self {
            keys: KeyBundle::random(&mut OsRng),
        }
    }

    fn init(&mut self, payload: &[u8]) -> (u8, Vec<u8>) {
        if payload.is_empty() {
            return (STATUS_OK, Vec::new());
        }

        match decode_key_bundle(payload) {
            Ok(keys) => {
                self.keys = keys;
                (STATUS_OK, Vec::new())
            }
            Err(err) => (STATUS_BAD_REQUEST, err.into_bytes()),
        }
    }

    fn public_key(&self, _: &[u8]) -> (u8, Vec<u8>) {
        (STATUS_OK, self.keys.public_key().to_vec())
    }

    fn attest(&self, payload: &[u8]) -> (u8, Vec<u8>) {
        let report_data = if payload.is_empty() {
            report_data_for_public_key(self.keys.public_key())
        } else {
            match payload.try_into() {
                Ok(report_data) => report_data,
                Err(_) => return (STATUS_BAD_REQUEST, b"attest needs 64 bytes".to_vec()),
            }
        };

        match attest::attest(report_data) {
            Ok(quote) => (STATUS_OK, quote),
            Err(err) => (STATUS_PANIC, err.into_bytes()),
        }
    }

    fn export_keys(&self, payload: &[u8]) -> (u8, Vec<u8>) {
        let recipient_policy = match trusted_recipient_policy() {
            Ok(policy) => policy,
            Err(err) => return (STATUS_BAD_REQUEST, err.into_bytes()),
        };

        let recipient_public_key = match verify_qve_evidence_public_key(payload, &recipient_policy)
        {
            Ok(public_key) => public_key,
            Err(err) => return (STATUS_BAD_REQUEST, err.into_bytes()),
        };

        let plaintext = encode_key_bundle(self.keys);
        match self
            .keys
            .seal_auth(&plaintext, recipient_public_key, &mut OsRng)
        {
            Ok(envelope) => (STATUS_OK, envelope),
            Err(err) => (STATUS_PANIC, format!("{err:?}").into_bytes()),
        }
    }

    fn qve_target_info(&self, _: &[u8]) -> (u8, Vec<u8>) {
        match qve_target_info() {
            Ok(target_info) => (STATUS_OK, target_info),
            Err(err) => (STATUS_PANIC, err.into_bytes()),
        }
    }

    fn handle(&mut self, op: u8, payload: &[u8]) -> (u8, Vec<u8>) {
        match op {
            OP_INIT => self.init(payload),
            OP_PUBLIC_KEY => self.public_key(payload),
            OP_ATTEST => self.attest(payload),
            OP_EXPORT_KEYS => self.export_keys(payload),
            OP_QVE_TARGET_INFO => self.qve_target_info(payload),
            _ => (STATUS_BAD_REQUEST, b"unknown operation".to_vec()),
        }
    }
}

#[cfg(target_env = "sgx")]
fn qve_target_info() -> Result<Vec<u8>, String> {
    let target_info = Targetinfo::from(Report::for_self());
    Ok(AsRef::<[u8]>::as_ref(&target_info).to_vec())
}

#[cfg(not(target_env = "sgx"))]
fn qve_target_info() -> Result<Vec<u8>, String> {
    Err("QvE targetinfo is only available inside SGX".into())
}

#[cfg(target_env = "sgx")]
fn trusted_recipient_policy() -> Result<QuotePolicy, String> {
    let self_report = Report::for_self();
    Ok(QuotePolicy {
        expected_mrenclave: required_embedded_measurement(
            option_env!("STAR_TRUSTED_ENCLAVE_MRENCLAVE"),
            "STAR_TRUSTED_ENCLAVE_MRENCLAVE",
        )?,
        allow_debug: embedded_allow_debug(
            option_env!("STAR_ALLOW_DEBUG_ENCLAVE"),
            self_report
                .attributes
                .flags
                .contains(AttributesFlags::DEBUG),
        )?,
    })
}

#[cfg(not(target_env = "sgx"))]
fn trusted_recipient_policy() -> Result<QuotePolicy, String> {
    Err("recipient policy is only available inside SGX".into())
}

#[cfg(target_env = "sgx")]
fn required_embedded_measurement(value: Option<&str>, name: &str) -> Result<[u8; 32], String> {
    let value = value.ok_or_else(|| format!("{name} must be embedded for MRENCLAVE trust"))?;
    parse_measurement_hex(value, name)
}

#[cfg(target_env = "sgx")]
fn embedded_allow_debug(specific: Option<&str>, default: bool) -> Result<bool, String> {
    let value = specific.or(option_env!("STAR_ALLOW_DEBUG_ENCLAVES"));
    match value {
        Some("1") | Some("true") | Some("TRUE") | Some("yes") | Some("YES") => Ok(true),
        Some("0") | Some("false") | Some("FALSE") | Some("no") | Some("NO") => Ok(false),
        Some(_) => Err("invalid embedded debug policy".into()),
        None => Ok(default),
    }
}

#[derive(Clone, Copy)]
struct KeyBundle {
    enc_key: [u8; 32],
    hpke_receiver_key: [u8; 32],
}

impl KeyBundle {
    fn random(rng: &mut OsRng) -> Self {
        let mut enc_key = [0u8; 32];
        let mut hpke_receiver_key = [0u8; 32];
        rng.fill_bytes(&mut enc_key);
        rng.fill_bytes(&mut hpke_receiver_key);
        Self {
            enc_key,
            hpke_receiver_key,
        }
    }

    fn public_key(&self) -> [u8; PUBLIC_KEY_LEN] {
        let sk =
            <X25519HkdfSha256 as hpke::kem::Kem>::PrivateKey::from_bytes(&self.hpke_receiver_key)
                .expect("stored X25519 private key must be valid");
        let pk = X25519HkdfSha256::sk_to_pk(&sk);
        let bytes = pk.to_bytes();
        let bytes_ref: &[u8] = bytes.as_ref();
        bytes_ref
            .try_into()
            .expect("X25519 public key must be 32 bytes")
    }

    fn seal_auth(
        &self,
        plaintext: &[u8],
        recipient_public_key: [u8; PUBLIC_KEY_LEN],
        rng: &mut OsRng,
    ) -> Result<Vec<u8>, HpkeError> {
        seal_auth_envelope(
            plaintext,
            self.hpke_receiver_key,
            self.public_key(),
            recipient_public_key,
            rng,
        )
    }
}

fn seal_auth_envelope(
    plaintext: &[u8],
    sender_secret_key: [u8; 32],
    sender_public_key: [u8; 32],
    recipient_public_key: [u8; PUBLIC_KEY_LEN],
    rng: &mut OsRng,
) -> Result<Vec<u8>, HpkeError> {
    let sender_sk =
        <X25519HkdfSha256 as hpke::kem::Kem>::PrivateKey::from_bytes(&sender_secret_key)?;
    let sender_pk =
        <X25519HkdfSha256 as hpke::kem::Kem>::PublicKey::from_bytes(&sender_public_key)?;
    let recipient_pk =
        <X25519HkdfSha256 as hpke::kem::Kem>::PublicKey::from_bytes(&recipient_public_key)?;

    let mut aad = [0u8; 32];
    let mut info = [0u8; 32];
    rng.fill_bytes(&mut aad);
    rng.fill_bytes(&mut info);

    let (enc, mut ctx) = setup_sender::<HpkeChaCha20Poly1305, HkdfSha256, X25519HkdfSha256, _>(
        &OpModeS::Auth((sender_sk, sender_pk)),
        &recipient_pk,
        &info,
        rng,
    )?;

    let ciphertext = ctx.seal(plaintext, &aad)?;
    let enc = enc.to_bytes();

    let mut envelope = Vec::with_capacity(aad.len() + info.len() + enc.len() + ciphertext.len());
    envelope.extend_from_slice(&aad);
    envelope.extend_from_slice(&info);
    envelope.extend_from_slice(enc.as_ref());
    envelope.extend_from_slice(&ciphertext);
    Ok(envelope)
}

fn encode_key_bundle(bundle: KeyBundle) -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    out.extend_from_slice(&bundle.enc_key);
    out.extend_from_slice(&bundle.hpke_receiver_key);
    out
}

fn decode_key_bundle(payload: &[u8]) -> Result<KeyBundle, String> {
    if payload.len() != 64 {
        return Err("key bundle must be 64 bytes".into());
    }

    Ok(KeyBundle {
        enc_key: payload[..32]
            .try_into()
            .map_err(|_| "bad enc key length".to_string())?,
        hpke_receiver_key: payload[32..64]
            .try_into()
            .map_err(|_| "bad hpke key length".to_string())?,
    })
}

pub fn run() {}

#[cfg(target_env = "sgx")]
static LIBRARY_APP: OnceLock<RwLock<KeyManagerApp>> = OnceLock::new();

#[cfg(target_env = "sgx")]
#[allow(improper_ctypes_definitions)]
#[no_mangle]
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

    let app = LIBRARY_APP.get_or_init(|| RwLock::new(KeyManagerApp::new()));
    let (status, response) = match app.write() {
        Ok(mut app) => app.handle(op, input),
        Err(_) => (STATUS_PANIC, b"key-manager app lock poisoned".to_vec()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_bundle_round_trips() {
        let bundle = KeyBundle {
            enc_key: [1u8; 32],
            hpke_receiver_key: [2u8; 32],
        };

        let encoded = encode_key_bundle(bundle);
        let decoded = decode_key_bundle(&encoded).expect("decode");

        assert_eq!(decoded.enc_key, bundle.enc_key);
        assert_eq!(decoded.hpke_receiver_key, bundle.hpke_receiver_key);
    }

    #[cfg(not(target_env = "sgx"))]
    #[test]
    fn public_key_attest_reports_unsupported_outside_sgx() {
        let app = KeyManagerApp::new();
        let (status, payload) = app.attest(&[]);

        assert_eq!(status, STATUS_PANIC);
        assert!(String::from_utf8(payload).unwrap().contains("Fortanix SGX"));
    }
}
