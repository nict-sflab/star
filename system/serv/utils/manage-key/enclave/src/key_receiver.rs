use crate::{policy, KeyBundle};
use hpke::{
    aead::ChaCha20Poly1305,
    kdf::HkdfSha256,
    kem::{Kem, X25519HkdfSha256},
    setup_receiver, Deserializable, OpModeR, Serializable,
};
use rand_core::OsRng;
use star_attestation::{Evidence, PUBLIC_KEY_LEN};

/// Receives a key bundle from an authenticated key manager.
pub struct KeyReceiver {
    secret_key: <X25519HkdfSha256 as Kem>::PrivateKey,
}

impl KeyReceiver {
    pub fn random() -> Self {
        // hpke 0.11 still uses rand_core 0.6; keep its RNG private to key-manager.
        let (secret_key, _) = X25519HkdfSha256::gen_keypair(&mut OsRng);
        Self { secret_key }
    }

    pub fn public_key(&self) -> [u8; PUBLIC_KEY_LEN] {
        X25519HkdfSha256::sk_to_pk(&self.secret_key)
            .to_bytes()
            .into()
    }

    /// Verify the manager's attestation against the embedded policy and receive its keys.
    pub fn receive_attested(
        &self,
        manager_evidence: &[u8],
        envelope: &[u8],
    ) -> Result<KeyBundle, String> {
        let manager = Evidence::decode(manager_evidence)?.verify(&policy::key_manager_policy()?)?;
        self.receive(envelope, manager.public_key()?)
    }

    /// Receive keys from a sender whose public key has already been verified.
    pub fn receive(
        &self,
        envelope: &[u8],
        sender_public_key: [u8; PUBLIC_KEY_LEN],
    ) -> Result<KeyBundle, String> {
        // aad(32) || info(32) || enc(32) || ciphertext
        if envelope.len() < 96 {
            return Err("key-manager envelope is too short".into());
        }
        let enc = <X25519HkdfSha256 as Kem>::EncappedKey::from_bytes(&envelope[64..96])
            .map_err(|err| err.to_string())?;
        let sender = <X25519HkdfSha256 as Kem>::PublicKey::from_bytes(&sender_public_key)
            .map_err(|err| err.to_string())?;
        let mut receiver = setup_receiver::<ChaCha20Poly1305, HkdfSha256, X25519HkdfSha256>(
            &OpModeR::Auth(sender),
            &self.secret_key,
            &enc,
            &envelope[32..64],
        )
        .map_err(|err| err.to_string())?;
        let plaintext = receiver
            .open(&envelope[96..], &envelope[..32])
            .map_err(|err| format!("decrypt key-manager envelope: {err}"))?;
        KeyBundle::decode(&plaintext)
    }
}
