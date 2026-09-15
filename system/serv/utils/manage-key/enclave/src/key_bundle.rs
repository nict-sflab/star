use crate::seal::SealedBlob;
use hpke::{
    aead::ChaCha20Poly1305 as HpkeChaCha20Poly1305,
    kdf::HkdfSha256,
    kem::{Kem as _, X25519HkdfSha256},
    setup_sender, Deserializable, HpkeError, OpModeS, Serializable,
};
use rand::{rngs::OsRng, RngCore};
use star_attestation::PUBLIC_KEY_LEN;
const KEY_BUNDLE_SEAL_LABEL: [u8; 16] = *b"key-bundle      ";
pub struct KeyBundle {
    pub enc_key: [u8; 32],
    pub hpke_receiver_key: [u8; 32],
}

impl KeyBundle {
    pub(crate) fn random(rng: &mut OsRng) -> Self {
        let mut enc_key = [0u8; 32];
        let mut hpke_receiver_key = [0u8; 32];
        rng.fill_bytes(&mut enc_key);
        rng.fill_bytes(&mut hpke_receiver_key);
        Self {
            enc_key,
            hpke_receiver_key,
        }
    }

    pub(crate) fn public_key(&self) -> [u8; PUBLIC_KEY_LEN] {
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

    pub(crate) fn export(
        &self,
        recipient_public_key: [u8; PUBLIC_KEY_LEN],
        rng: &mut OsRng,
    ) -> Result<Vec<u8>, HpkeError> {
        let plaintext = self.encode();
        let sender_sk =
            <X25519HkdfSha256 as hpke::kem::Kem>::PrivateKey::from_bytes(&self.hpke_receiver_key)?;
        let sender_pk =
            <X25519HkdfSha256 as hpke::kem::Kem>::PublicKey::from_bytes(&self.public_key())?;
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

        let ciphertext = ctx.seal(&plaintext, &aad)?;
        let enc = enc.to_bytes();

        let mut envelope =
            Vec::with_capacity(aad.len() + info.len() + enc.len() + ciphertext.len());
        envelope.extend_from_slice(&aad);
        envelope.extend_from_slice(&info);
        envelope.extend_from_slice(enc.as_ref());
        envelope.extend_from_slice(&ciphertext);
        Ok(envelope)
    }

    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(64);
        out.extend_from_slice(&self.enc_key);
        out.extend_from_slice(&self.hpke_receiver_key);
        out
    }

    pub(crate) fn decode(payload: &[u8]) -> Result<Self, String> {
        if payload.len() != 64 {
            return Err("key bundle must be 64 bytes".into());
        }

        Ok(KeyBundle {
            enc_key: payload[..32].try_into().unwrap(),
            hpke_receiver_key: payload[32..64].try_into().unwrap(),
        })
    }

    pub(crate) fn seal(&self) -> Result<Vec<u8>, String> {
        SealedBlob::seal(KEY_BUNDLE_SEAL_LABEL, &self.encode())
            .and_then(|blob| blob.encode())
            .map_err(|err| format!("seal key bundle: {err}"))
    }

    pub(crate) fn unseal(sealed: &[u8]) -> Result<Self, String> {
        let plaintext = SealedBlob::decode(sealed)
            .and_then(|blob| blob.unseal(KEY_BUNDLE_SEAL_LABEL))
            .map_err(|err| format!("unseal key bundle: {err}"))?;
        Self::decode(&plaintext)
    }
}
