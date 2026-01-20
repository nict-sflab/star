use hpke::{
    aead::ChaCha20Poly1305 as HpkeChaCha20Poly1305,
    kdf::HkdfSha256,
    kem::X25519HkdfSha256,
    setup_sender, Deserializable, OpModeS, Serializable,
};
use rand::{rngs::OsRng, RngCore};
use constant_time_utils::time::current_period;

use crate::serv::core::{bytes::Short, cred::Credential};

/// Client-side holder for credentials and token requests.
pub struct User {
    credential: Option<Credential>,
    max: u64,
}

impl User {
    /// Create a user allowed to make an unlimited number of token requests.
    /// Create a user capped at `max` token requests for a credential.
    pub fn new(max: u64) -> Self {
        Self {
            credential: None,
            max,
        }
    }

    /// Store a freshly issued credential.
    pub fn receive_credential(&mut self, credential_bytes: &[u8]) {
        let credential = Credential::from_bytes(credential_bytes);
        self.credential = Some(credential);
    }

    /// Present the credential by building a token request (credential + count + period).
    pub fn request_auth_plain(&mut self, counter: u64) -> Vec<u8> {
        if counter >= self.max {
            panic!("exceeded max token requests for this credential");
        }

        let credential = self
            .credential
            .as_ref()
            .expect("credential not received yet");

        let count: Short = counter.to_be_bytes();
        let period: Short = current_period();
        let credential_bytes = credential.to_bytes();

        let mut token_request =
            Vec::with_capacity(credential_bytes.len() + count.len() + period.len());
        token_request.extend_from_slice(&credential_bytes);
        token_request.extend_from_slice(&count);
        token_request.extend_from_slice(&period);
        token_request
    }

    /// Present the credential over HPKE to the enclave.
    /// Returns an encrypted payload ready for `Enclave::issue_token`.
    pub fn request_auth(
        &mut self,
        counter: u64,
        receiver_public_key: &[u8],
    ) -> Result<Vec<u8>, hpke::HpkeError> {
        let token_request = self.request_auth_plain(counter);
        let pk = <X25519HkdfSha256 as hpke::kem::Kem>::PublicKey::from_bytes(
            receiver_public_key,
        )?;

        let mut aad = [0u8; 32];
        let mut info = [0u8; 32];
        let mut rng = OsRng;
        rng.fill_bytes(&mut aad);
        rng.fill_bytes(&mut info);

        let (enc, mut sender_ctx) =
            setup_sender::<HpkeChaCha20Poly1305, HkdfSha256, X25519HkdfSha256, _>(
                &OpModeS::Base,
                &pk,
                &info,
                &mut rng,
            )?;

        let ciphertext = sender_ctx.seal(&token_request, &aad)?;

        let mut envelope =
            Vec::with_capacity(aad.len() + info.len() + enc.to_bytes().len() + ciphertext.len());
        envelope.extend_from_slice(&aad);
        envelope.extend_from_slice(&info);
        envelope.extend_from_slice(&enc.to_bytes());
        envelope.extend_from_slice(&ciphertext);
        Ok(envelope)
    }
}
