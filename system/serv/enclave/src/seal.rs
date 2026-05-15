use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes128Gcm, Nonce,
};
use rand::random;
use serde::{Deserialize, Serialize};
use sgx_isa::{Attributes, ErrorCode, Keyname, Keypolicy, Keyrequest, Miscselect, Report};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SealError {
    #[error("SGX EGETKEY failed: {0:?}")]
    Sgx(ErrorCode),

    #[error("current enclave attributes do not match sealed metadata")]
    InvalidEnclaveAttributes,

    #[error("AES-GCM encryption/decryption failed")]
    Crypto,

    #[error("serialization failed: {0}")]
    Serialize(#[from] Box<bincode::ErrorKind>),
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

fn seal_key(label: [u8; 16]) -> Result<([u8; 16], SealData), SealError> {
    let report = Report::for_self();

    let seal_data = SealData {
        rand: random(),
        isvsvn: report.isvsvn,
        cpusvn: report.cpusvn,
        attributes: report.attributes,
        miscselect: report.miscselect,
    };

    let key = egetkey(label, &seal_data)?;
    Ok((key, seal_data))
}

fn unseal_key(label: [u8; 16], seal_data: &SealData) -> Result<[u8; 16], SealError> {
    let report = Report::for_self();

    if report.attributes != seal_data.attributes || report.miscselect != seal_data.miscselect {
        return Err(SealError::InvalidEnclaveAttributes);
    }

    egetkey(label, seal_data)
}

pub fn seal(label: [u8; 16], plaintext: &[u8]) -> Result<Vec<u8>, SealError> {
    let (key_bytes, seal_data) = seal_key(label)?;

    let cipher = Aes128Gcm::new_from_slice(&key_bytes).map_err(|_| SealError::Crypto)?;
    let nonce_bytes: [u8; 12] = random();
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

pub fn unseal(label: [u8; 16], sealed: &[u8]) -> Result<Vec<u8>, SealError> {
    let blob: SealedBlob = bincode::deserialize(sealed)?;

    let key_bytes = unseal_key(label, &blob.seal_data)?;

    let cipher = Aes128Gcm::new_from_slice(&key_bytes).map_err(|_| SealError::Crypto)?;
    let nonce = Nonce::from_slice(&blob.nonce);

    let plaintext = cipher
        .decrypt(nonce, blob.ciphertext.as_ref())
        .map_err(|_| SealError::Crypto)?;

    Ok(plaintext)
}
