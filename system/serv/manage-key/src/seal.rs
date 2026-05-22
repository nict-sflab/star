use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use hkdf::Hkdf;
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sgx_isa::{Attributes, Miscselect};
#[cfg(target_env = "sgx")]
use sgx_isa::{ErrorCode, Keyname, Keypolicy, Keyrequest, Report};
use sha2::Sha256;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SealError {
    #[cfg(target_env = "sgx")]
    #[error("SGX EGETKEY failed: {0:?}")]
    Sgx(ErrorCode),

    #[cfg(target_env = "sgx")]
    #[error("current enclave attributes do not match sealed metadata")]
    InvalidEnclaveAttributes,

    #[error("ChaCha20-Poly1305 encryption/decryption failed")]
    Crypto,

    #[error("serialization failed: {0}")]
    Serialize(#[from] Box<bincode::ErrorKind>),

    #[cfg(not(target_env = "sgx"))]
    #[error("SGX sealing is only available inside the Fortanix SGX target")]
    UnsupportedTarget,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SealData {
    rand: [u8; 16],
    isvsvn: u16,
    cpusvn: [u8; 16],
    attributes: Attributes,
    miscselect: Miscselect,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SealedBlob {
    seal_data: SealData,
    nonce: [u8; 12],
    ciphertext: Vec<u8>,
}

pub const PRF_KEY: [u8; 16] = *b"prf-key         ";
pub const ENC_KEY: [u8; 16] = *b"enc-key         ";

fn random_bytes<const N: usize>() -> [u8; N] {
    let mut bytes = [0u8; N];
    OsRng.fill_bytes(&mut bytes);
    bytes
}

fn seal_with_key(
    key_bytes: [u8; 32],
    seal_data: SealData,
    plaintext: &[u8],
    nonce_bytes: [u8; 12],
) -> Result<Vec<u8>, SealError> {
    let cipher = aead_cipher(key_bytes)?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|_| SealError::Crypto)?;

    let blob = SealedBlob {
        seal_data,
        nonce: nonce_bytes,
        ciphertext,
    };

    Ok(bincode::serialize(&blob)?)
}

fn unseal_with_key(key_bytes: [u8; 32], blob: &SealedBlob) -> Result<Vec<u8>, SealError> {
    let cipher = aead_cipher(key_bytes)?;
    let nonce = Nonce::from_slice(&blob.nonce);

    cipher
        .decrypt(nonce, blob.ciphertext.as_ref())
        .map_err(|_| SealError::Crypto)
}

fn aead_cipher(key_bytes: [u8; 32]) -> Result<ChaCha20Poly1305, SealError> {
    ChaCha20Poly1305::new_from_slice(&key_bytes).map_err(|_| SealError::Crypto)
}

fn derive_aead_key(sgx_key: [u8; 16], seal_data: &SealData) -> Result<[u8; 32], SealError> {
    let hk = Hkdf::<Sha256>::new(Some(&seal_data.rand), &sgx_key);
    let mut key = [0u8; 32];

    hk.expand(b"sgx sealing chacha20poly1305 aead key", &mut key)
        .map_err(|_| SealError::Crypto)?;

    Ok(key)
}

#[cfg(target_env = "sgx")]
fn egetkey(label: [u8; 16], seal_data: &SealData) -> Result<[u8; 16], SealError> {
    let mut keyid = [0u8; 32];
    keyid[..16].copy_from_slice(&label);
    keyid[16..].copy_from_slice(&seal_data.rand);

    let req = Keyrequest {
        keyname: Keyname::Seal as u16,
        keypolicy: Keypolicy::MRENCLAVE,
        isvsvn: seal_data.isvsvn,
        cpusvn: seal_data.cpusvn,
        attributemask: [!0u64; 2],
        keyid,
        miscmask: !0u32,
        ..Default::default()
    };

    req.egetkey().map_err(SealError::Sgx)
}

#[cfg(target_env = "sgx")]
fn seal_key(label: [u8; 16]) -> Result<([u8; 32], SealData), SealError> {
    let report = Report::for_self();

    let seal_data = SealData {
        rand: random_bytes(),
        isvsvn: report.isvsvn,
        cpusvn: report.cpusvn,
        attributes: report.attributes,
        miscselect: report.miscselect,
    };

    let sgx_key = egetkey(label, &seal_data)?;
    let key = derive_aead_key(sgx_key, &seal_data)?;
    Ok((key, seal_data))
}

#[cfg(not(target_env = "sgx"))]
fn seal_key(_: [u8; 16]) -> Result<([u8; 32], SealData), SealError> {
    Err(SealError::UnsupportedTarget)
}

#[cfg(target_env = "sgx")]
fn unseal_key(label: [u8; 16], seal_data: &SealData) -> Result<[u8; 32], SealError> {
    let report = Report::for_self();

    if report.attributes != seal_data.attributes || report.miscselect != seal_data.miscselect {
        return Err(SealError::InvalidEnclaveAttributes);
    }

    let sgx_key = egetkey(label, seal_data)?;
    derive_aead_key(sgx_key, seal_data)
}

#[cfg(not(target_env = "sgx"))]
fn unseal_key(_: [u8; 16], _: &SealData) -> Result<[u8; 32], SealError> {
    Err(SealError::UnsupportedTarget)
}

pub fn seal(label: [u8; 16], plaintext: &[u8]) -> Result<Vec<u8>, SealError> {
    let (key_bytes, seal_data) = seal_key(label)?;
    let nonce_bytes = random_bytes();
    seal_with_key(key_bytes, seal_data, plaintext, nonce_bytes)
}

pub fn unseal(label: [u8; 16], sealed: &[u8]) -> Result<Vec<u8>, SealError> {
    let blob: SealedBlob = bincode::deserialize(sealed)?;

    let key_bytes = unseal_key(label, &blob.seal_data)?;
    unseal_with_key(key_bytes, &blob)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_seal_data() -> SealData {
        SealData {
            rand: [3u8; 16],
            isvsvn: 7,
            cpusvn: [9u8; 16],
            attributes: Attributes::default(),
            miscselect: Miscselect::default(),
        }
    }

    #[test]
    fn sealed_blob_crypto_round_trips_with_supplied_key() {
        let key = [0x42u8; 32];
        let nonce = [0x24u8; 12];
        let plaintext = b"sealed enclave payload";

        let sealed = seal_with_key(key, fake_seal_data(), plaintext, nonce).expect("seal");
        let blob: SealedBlob = bincode::deserialize(&sealed).expect("deserialize sealed blob");

        assert_eq!(blob.nonce, nonce);
        assert_ne!(blob.ciphertext.as_slice(), plaintext);
        assert_eq!(
            unseal_with_key(key, &blob).expect("unseal"),
            plaintext.as_slice()
        );
    }

    #[test]
    fn sealed_blob_rejects_modified_ciphertext() {
        let key = [0x42u8; 32];
        let sealed = seal_with_key(
            key,
            fake_seal_data(),
            b"sealed enclave payload",
            [0x24u8; 12],
        )
        .expect("seal");
        let mut blob: SealedBlob = bincode::deserialize(&sealed).expect("deserialize sealed blob");

        *blob.ciphertext.last_mut().expect("ciphertext") ^= 1;

        assert!(matches!(
            unseal_with_key(key, &blob),
            Err(SealError::Crypto)
        ));
    }

    #[cfg(not(target_env = "sgx"))]
    #[test]
    fn public_seal_reports_unsupported_outside_sgx() {
        assert!(matches!(
            seal(ENC_KEY, b"payload"),
            Err(SealError::UnsupportedTarget)
        ));
    }

    #[cfg(target_env = "sgx")]
    #[test]
    fn public_seal_round_trips_inside_sgx() {
        let plaintext = b"payload sealed by SGX";
        let sealed = seal(ENC_KEY, plaintext).expect("seal");

        assert_eq!(unseal(ENC_KEY, &sealed).expect("unseal"), plaintext);
    }

    #[cfg(target_env = "sgx")]
    #[test]
    fn public_unseal_rejects_wrong_label_inside_sgx() {
        let sealed = seal(ENC_KEY, b"payload sealed by SGX").expect("seal");

        assert!(matches!(unseal(PRF_KEY, &sealed), Err(SealError::Crypto)));
    }

    #[cfg(target_env = "sgx")]
    #[test]
    fn public_unseal_rejects_changed_enclave_metadata_inside_sgx() {
        let sealed = seal(ENC_KEY, b"payload sealed by SGX").expect("seal");
        let mut blob: SealedBlob = bincode::deserialize(&sealed).expect("deserialize sealed blob");
        blob.seal_data.miscselect =
            Miscselect::from_bits_truncate(blob.seal_data.miscselect.bits() ^ 1);
        let sealed = bincode::serialize(&blob).expect("serialize sealed blob");

        assert!(matches!(
            unseal(ENC_KEY, &sealed),
            Err(SealError::InvalidEnclaveAttributes)
        ));
    }
}
