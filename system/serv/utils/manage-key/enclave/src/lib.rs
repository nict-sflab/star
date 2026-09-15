mod key_bundle;
mod key_receiver;
mod policy;
mod seal;

pub use key_bundle::KeyBundle;
pub use key_receiver::KeyReceiver;
use rand::rngs::OsRng;
use star_attestation::{Evidence, Quote, QuotePolicy, PUBLIC_KEY_LEN};

pub struct KeyManager {
    keys: KeyBundle,
    recipient_policy: QuotePolicy,
}

impl KeyManager {
    /// The host persists the sealed bundle and supplies it again on restart.
    pub fn restore(sealed: &[u8]) -> Result<(Self, Vec<u8>), String> {
        let (keys, sealed) = if sealed.is_empty() {
            let keys = KeyBundle::random(&mut OsRng);
            let sealed = keys.seal()?;
            (keys, sealed)
        } else {
            (KeyBundle::unseal(sealed)?, sealed.to_vec())
        };
        Ok((
            Self {
                keys,
                recipient_policy: policy::recipient_policy()?,
            },
            sealed,
        ))
    }

    pub fn public_key(&self) -> [u8; PUBLIC_KEY_LEN] {
        self.keys.public_key()
    }

    pub fn attest(&self) -> Result<Vec<u8>, String> {
        star_attestation::attest(Quote::report_data_for_public_key(self.public_key()))
    }

    pub fn export(&self, evidence: &[u8]) -> Result<Vec<u8>, String> {
        let recipient = Evidence::decode(evidence)?.verify(&self.recipient_policy)?;
        self.keys
            .export(recipient.public_key()?, &mut OsRng)
            .map_err(|err| format!("export keys: {err:?}"))
    }
}

#[path = "lib_test.rs"]
#[cfg(test)]
mod tests;
