use alloc::vec::Vec;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use hpke_ng::{
    kem::dh::{DiffieHellman, X25519},
    ChaCha20Poly1305 as HpkeChaCha20Poly1305, DhKemX25519HkdfSha256, HkdfSha256, Hpke,
    HpkeError, Kem,
};
use rand::{CryptoRng, RngCore};

use super::RngCompat;

type HpkeSuite = Hpke<DhKemX25519HkdfSha256, HkdfSha256, HpkeChaCha20Poly1305>;

// HPKE receiver living inside the enclave.
pub struct SecureChannelServer {
    pub sk: <DhKemX25519HkdfSha256 as Kem>::PrivateKey,
    pub pk: [u8; 32],
}

impl SecureChannelServer {
    /// Restore a receiver from an existing 32-byte X25519 secret key.
    pub fn new(sk_bytes: [u8; 32]) -> Result<Self, HpkeError> {
        let sk = DhKemX25519HkdfSha256::sk_from_bytes(&sk_bytes)?;

        // hpke-ng caches the matching public key in its private-key wrapper for
        // decapsulation. Derive the advertised public key once at construction.
        let raw_sk = X25519::sk_from_bytes(&sk_bytes)?;
        let raw_pk = X25519::sk_to_pk(&raw_sk);
        let pk_bytes = X25519::pk_to_bytes(&raw_pk);
        let pk = pk_bytes
            .as_slice()
            .try_into()
            .expect("X25519 public key must be 32 bytes");

        Ok(Self { sk, pk })
    }

    /// Generate a fresh receiver keypair using hpke-ng.
    pub fn random<R>(rng: &mut R) -> Self
    where
        R: CryptoRng + RngCore,
    {
        let mut compat_rng = RngCompat(rng);
        let (sk, pk) = DhKemX25519HkdfSha256::generate(&mut compat_rng)
            .expect("X25519 key generation must succeed");
        let pk = pk
            .as_ref()
            .try_into()
            .expect("X25519 public key must be 32 bytes");

        Self { sk, pk }
    }

    /// Public key for advertising to senders.
    pub fn public_key(&self) -> [u8; 32] {
        self.pk
    }

    /// Decrypt an HPKE ciphertext (X25519 + ChaCha20-Poly1305).
    pub fn receive_hpke(
        &self,
        enc: &[u8],
        info: &[u8],
        aad: &[u8],
        ciphertext: &[u8],
    ) -> Result<Vec<u8>, HpkeError> {
        let enc = DhKemX25519HkdfSha256::enc_from_bytes(enc)?;
        HpkeSuite::open_base(&enc, &self.sk, info, aad, ciphertext)
    }

    pub fn respond_cipher<R>(&self, response: &[u8], cipher: &[u8], rng: &mut R) -> Vec<u8>
    where
        R: RngCore,
    {
        let Some((aad, info, enc, ciphertext)) = split_hpke_envelope(cipher) else {
            return Vec::new();
        };

        #[cfg(feature = "timecop")]
        timecop::poison(ciphertext);

        let key = match self.receive_hpke(enc, info, aad, ciphertext) {
            Ok(key) => key,
            Err(_) => return Vec::new(),
        };

        #[cfg(feature = "timecop")]
        timecop::poison(key.as_slice());

        let mut nonce_bytes = [0u8; 12];
        rng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let aead = match ChaCha20Poly1305::new_from_slice(&key) {
            Ok(aead) => aead,
            Err(_) => return Vec::new(),
        };
        let ciphertext = match aead.encrypt(nonce, response) {
            Ok(ciphertext) => ciphertext,
            Err(_) => return Vec::new(),
        };

        let mut result = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext);

        result
    }
}

pub(crate) fn split_hpke_envelope(cipher: &[u8]) -> Option<(&[u8], &[u8], &[u8], &[u8])> {
    const AAD_LEN: usize = 32;
    const INFO_LEN: usize = 32;
    const ENC_LEN: usize = 32;

    let min_len = AAD_LEN + INFO_LEN + ENC_LEN;
    if cipher.len() < min_len {
        return None;
    }

    let aad = &cipher[..AAD_LEN];
    let info = &cipher[AAD_LEN..AAD_LEN + INFO_LEN];
    let enc = &cipher[AAD_LEN + INFO_LEN..AAD_LEN + INFO_LEN + ENC_LEN];
    let ciphertext = &cipher[AAD_LEN + INFO_LEN + ENC_LEN..];

    Some((aad, info, enc, ciphertext))
}
