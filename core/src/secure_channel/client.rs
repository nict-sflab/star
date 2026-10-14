use alloc::vec::Vec;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use hpke_ng::{
    ChaCha20Poly1305 as HpkeChaCha20Poly1305, DhKemX25519HkdfSha256, HkdfSha256, Hpke,
    HpkeError, Kem,
};
use rand::CryptoRng;

type HpkeSuite = Hpke<DhKemX25519HkdfSha256, HkdfSha256, HpkeChaCha20Poly1305>;

// HPKE sender living outside the enclave.
pub struct SecureChannelClient {
    receiver_pk: <DhKemX25519HkdfSha256 as Kem>::PublicKey,
}

impl SecureChannelClient {
    /// Restore a sender from the receiver's 32-byte X25519 public key.
    pub fn new(receiver_pk_bytes: [u8; 32]) -> Result<Self, HpkeError> {
        let receiver_pk = DhKemX25519HkdfSha256::pk_from_bytes(&receiver_pk_bytes)?;
        Ok(Self { receiver_pk })
    }

    /// Encrypt arbitrary data using HPKE (X25519 + ChaCha20-Poly1305).
    /// Returns a single envelope: aad(32) || info(32) || enc(32) || ciphertext.
    pub fn send_hpke<R>(&self, plaintext: &[u8], rng: &mut R) -> Result<Vec<u8>, HpkeError>
    where
        R: CryptoRng,
    {
        let mut aad = [0u8; 32];
        let mut info = [0u8; 32];
        rng.fill_bytes(&mut aad);
        rng.fill_bytes(&mut info);

        let (enc, ciphertext) =
            HpkeSuite::seal_base(rng, &self.receiver_pk, &info, &aad, plaintext)?;

        let enc_bytes = enc.as_ref();
        let mut envelope =
            Vec::with_capacity(aad.len() + info.len() + enc_bytes.len() + ciphertext.len());
        envelope.extend_from_slice(&aad);
        envelope.extend_from_slice(&info);
        envelope.extend_from_slice(enc_bytes);
        envelope.extend_from_slice(&ciphertext);
        Ok(envelope)
    }

    /// Send a freshly generated 32-byte shared key using HPKE.
    /// Returns (hpke_envelope, shared_key).
    pub fn send_shared_key<R>(&self, rng: &mut R) -> Result<(Vec<u8>, [u8; 32]), HpkeError>
    where
        R: CryptoRng,
    {
        let mut shared_key = [0u8; 32];
        rng.fill_bytes(&mut shared_key);
        let envelope = self.send_hpke(&shared_key, rng)?;
        Ok((envelope, shared_key))
    }

    /// Decrypt a ChaCha20-Poly1305 message encrypted with the shared key.
    /// Expected format: nonce(12) || ciphertext.
    pub fn decrypt_with_shared_key(
        &self,
        cipher: &[u8],
        shared_key: &[u8; 32],
    ) -> Result<Vec<u8>, chacha20poly1305::aead::Error> {
        const NONCE_LEN: usize = 12;
        if cipher.len() < NONCE_LEN {
            return Err(chacha20poly1305::aead::Error);
        }

        let (nonce_bytes, ciphertext) = cipher.split_at(NONCE_LEN);
        let nonce = Nonce::from_slice(nonce_bytes);
        let aead =
            ChaCha20Poly1305::new_from_slice(shared_key).expect("shared key must be 32 bytes");
        aead.decrypt(nonce, ciphertext)
    }
}
