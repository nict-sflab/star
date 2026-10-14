use key_manager::KeyReceiver;
use rand::{rand_core::UnwrapErr, rngs::SysRng};
use star_attestation::{Quote, PUBLIC_KEY_LEN};
use star_core::serv::enclave::Enclave as CredentialIssuer;

// Embed the request limit in the image so changing it changes MRENCLAVE.
#[cfg(not(feature = "bench-max-count"))]
include!(concat!(env!("OUT_DIR"), "/max_count.rs"));
#[cfg(feature = "bench-max-count")]
const MAX_COUNT: u32 = 1_000_000;
#[used]
static MEASURED_MAX_COUNT_BE: [u8; 4] = MAX_COUNT.to_be_bytes();

/// Issues credentials and tokens after receiving keys from a trusted key manager.
pub struct Enclave {
    issuer: Option<CredentialIssuer>,
    key_receiver: KeyReceiver,
}

impl Default for Enclave {
    fn default() -> Self {
        Self {
            issuer: None,
            key_receiver: KeyReceiver::random(),
        }
    }
}

impl Enclave {
    /// Start a fresh key transfer before production keys have been installed.
    pub fn prepare_key_transfer(&mut self) -> Result<(), String> {
        if self.issuer.is_some() {
            return Err("enclave already initialized".into());
        }
        self.key_receiver = KeyReceiver::random();
        Ok(())
    }

    pub fn transfer_public_key(&self) -> [u8; PUBLIC_KEY_LEN] {
        self.key_receiver.public_key()
    }

    pub fn attest(&self) -> Result<Vec<u8>, String> {
        star_attestation::attest(Quote::report_data_for_public_key(
            self.transfer_public_key(),
        ))
    }

    pub fn install_keys(&mut self, manager_evidence: &[u8], envelope: &[u8]) -> Result<(), String> {
        if self.issuer.is_some() {
            return Err("enclave already initialized".into());
        }
        let keys = self
            .key_receiver
            .receive_attested(manager_evidence, envelope)?;
        self.issuer = Some(
            CredentialIssuer::new(
                u32::from_be_bytes(MEASURED_MAX_COUNT_BE),
                keys.enc_key,
                keys.hpke_receiver_key,
            )
            .map_err(|err| err.to_string())?,
        );
        Ok(())
    }

    pub fn public_key(&self) -> Result<[u8; PUBLIC_KEY_LEN], String> {
        Ok(self.issuer()?.public_key())
    }

    pub fn issue_cred(&self, request: &[u8]) -> Result<Vec<u8>, String> {
        let credential = self.issuer()?.issue_cred(request, &mut UnwrapErr(SysRng));
        if credential.is_empty() {
            return Err("issue_cred failed".into());
        }
        Ok(credential)
    }

    pub fn issue_token(&self, request: &[u8], current_period: [u8; 8]) -> Result<Vec<u8>, String> {
        self.issuer()?
            .issue_token(request, current_period)
            .map_err(|_| "issue_token failed".into())
    }

    fn issuer(&self) -> Result<&CredentialIssuer, String> {
        self.issuer
            .as_ref()
            .ok_or_else(|| "enclave is not initialized".into())
    }
}

#[path = "lib_test.rs"]
#[cfg(test)]
mod tests;
