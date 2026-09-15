#[cfg(target_env = "sgx")]
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
#[cfg(target_env = "sgx")]
use hkdf::Hkdf;
#[cfg(target_env = "sgx")]
use rand::{rngs::OsRng, RngCore};
use serde::{Deserialize, Serialize};
use sgx_isa::{Attributes, Miscselect};
#[cfg(target_env = "sgx")]
use sgx_isa::{ErrorCode, Keyname, Keypolicy, Keyrequest, Report};
#[cfg(target_env = "sgx")]
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

    #[cfg(target_env = "sgx")]
    #[error("ChaCha20-Poly1305 encryption/decryption failed")]
    Crypto,

    #[error("serialization failed: {0}")]
    Serialize(#[from] Box<bincode::ErrorKind>),

    #[cfg(not(target_env = "sgx"))]
    #[error("SGX sealing is only available inside the Fortanix SGX target")]
    UnsupportedTarget,
}

/// Metadata required to reproduce this enclave's sealing key.
#[derive(Debug, Serialize, Deserialize)]
struct SealData {
    rand: [u8; 16],
    isvsvn: u16,
    cpusvn: [u8; 16],
    attributes: Attributes,
    miscselect: Miscselect,
}

#[cfg(target_env = "sgx")]
impl SealData {
    fn derive_key(&self, label: [u8; 16]) -> Result<[u8; 32], SealError> {
        let report = Report::for_self();
        if report.attributes != self.attributes || report.miscselect != self.miscselect {
            return Err(SealError::InvalidEnclaveAttributes);
        }

        let mut keyid = [0u8; 32];
        keyid[..16].copy_from_slice(&label);
        keyid[16..].copy_from_slice(&self.rand);
        let request = Keyrequest {
            keyname: Keyname::Seal as u16,
            keypolicy: Keypolicy::MRENCLAVE,
            isvsvn: self.isvsvn,
            cpusvn: self.cpusvn,
            attributemask: [!0u64; 2],
            keyid,
            miscmask: !0u32,
            ..Default::default()
        };
        let sgx_key = request.egetkey().map_err(SealError::Sgx)?;
        let mut key = [0u8; 32];
        Hkdf::<Sha256>::new(Some(&self.rand), &sgx_key)
            .expand(b"sgx sealing chacha20poly1305 aead key", &mut key)
            .expect("32 bytes fit within the HKDF-SHA256 output limit");
        Ok(key)
    }
}

/// Encrypted payload and the metadata needed to unseal it.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SealedBlob {
    seal_data: SealData,
    nonce: [u8; 12],
    ciphertext: Vec<u8>,
}

impl SealedBlob {
    #[cfg(target_env = "sgx")]
    pub(crate) fn seal(label: [u8; 16], plaintext: &[u8]) -> Result<Self, SealError> {
        let report = Report::for_self();
        let mut rand = [0u8; 16];
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut rand);
        OsRng.fill_bytes(&mut nonce);
        let seal_data = SealData {
            rand,
            isvsvn: report.isvsvn,
            cpusvn: report.cpusvn,
            attributes: report.attributes,
            miscselect: report.miscselect,
        };
        let key = seal_data.derive_key(label)?;
        let ciphertext = ChaCha20Poly1305::new(&key.into())
            .encrypt(&Nonce::from(nonce), plaintext)
            .map_err(|_| SealError::Crypto)?;
        Ok(Self {
            seal_data,
            nonce,
            ciphertext,
        })
    }

    #[cfg(target_env = "sgx")]
    pub(crate) fn unseal(&self, label: [u8; 16]) -> Result<Vec<u8>, SealError> {
        let key = self.seal_data.derive_key(label)?;
        ChaCha20Poly1305::new(&key.into())
            .decrypt(&Nonce::from(self.nonce), self.ciphertext.as_ref())
            .map_err(|_| SealError::Crypto)
    }

    #[cfg(not(target_env = "sgx"))]
    pub(crate) fn seal(_: [u8; 16], _: &[u8]) -> Result<Self, SealError> {
        Err(SealError::UnsupportedTarget)
    }

    #[cfg(not(target_env = "sgx"))]
    pub(crate) fn unseal(&self, _: [u8; 16]) -> Result<Vec<u8>, SealError> {
        Err(SealError::UnsupportedTarget)
    }

    pub(crate) fn encode(&self) -> Result<Vec<u8>, SealError> {
        Ok(bincode::serialize(self)?)
    }

    pub(crate) fn decode(bytes: &[u8]) -> Result<Self, SealError> {
        Ok(bincode::deserialize(bytes)?)
    }
}

#[path = "seal_test.rs"]
#[cfg(test)]
mod tests;
