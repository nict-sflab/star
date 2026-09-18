use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use key_manager_client::RemoteKeyManagerRuntime;
use key_manager_service::KeyManagerRuntime;
use std::error::Error;
use std::sync::Arc;

use crate::enclave::{EnclaveError, EnclaveRuntime};
use star_attestation::Evidence;

pub(crate) struct ProvisionedEnclave {
    pub(crate) public_key: [u8; 32],
    pub(crate) enclave: Arc<EnclaveRuntime>,
    key_manager_quote: Vec<u8>,
    enclave_quote: Vec<u8>,
    enclave_evidence_for_key_manager: Vec<u8>,
    key_manager_evidence: Vec<u8>,
}

impl ProvisionedEnclave {
    pub(crate) fn provision() -> Result<Self, Box<dyn Error>> {
        let key_manager =
            RemoteKeyManagerRuntime::from_env().ok_or("STAR_KEY_MANAGER_URL is required")?;
        let enclave = Arc::new(EnclaveRuntime::from_env()?);

        let public_key = key_manager.public_key()?;
        let key_manager_quote = key_manager.public_key_attestation()?;
        let enclave_quote = enclave.transfer_key_attestation()?;
        let enclave_export = key_manager.export_keys_for_recipient_quote(&enclave_quote)?;
        let key_manager_evidence = evidence_for_verifier(&key_manager_quote)?;

        enclave.install_keys(&key_manager_evidence, &enclave_export.envelope)?;

        Ok(Self {
            public_key,
            enclave,
            key_manager_quote,
            enclave_quote,
            enclave_evidence_for_key_manager: enclave_export.recipient_evidence,
            key_manager_evidence,
        })
    }

    pub(crate) fn emit_startup_attestation(&self) {
        // Preserve the existing log keys; both roles now use the same enclave.
        println!(
            "STAR_ATTESTATION_PUBLIC_KEY_BASE64={}",
            URL_SAFE.encode(self.public_key)
        );
        println!(
            "STAR_ATTESTATION_QUOTE_BASE64={}",
            URL_SAFE.encode(&self.key_manager_quote)
        );
        println!(
            "STAR_KEY_MANAGER_QUOTE_BASE64={}",
            URL_SAFE.encode(&self.key_manager_quote)
        );
        println!(
            "STAR_FILTER_ENCLAVE_TRANSFER_QUOTE_BASE64={}",
            URL_SAFE.encode(&self.enclave_quote)
        );
        println!(
            "STAR_ISSUE_ENCLAVE_TRANSFER_QUOTE_BASE64={}",
            URL_SAFE.encode(&self.enclave_quote)
        );
        println!(
            "STAR_FILTER_ENCLAVE_DCAP_EVIDENCE_FOR_KEY_MANAGER_BASE64={}",
            URL_SAFE.encode(&self.enclave_evidence_for_key_manager)
        );
        println!(
            "STAR_ISSUE_ENCLAVE_DCAP_EVIDENCE_FOR_KEY_MANAGER_BASE64={}",
            URL_SAFE.encode(&self.enclave_evidence_for_key_manager)
        );
        println!(
            "STAR_KEY_MANAGER_DCAP_EVIDENCE_FOR_FILTER_ENCLAVE_BASE64={}",
            URL_SAFE.encode(&self.key_manager_evidence)
        );
        println!(
            "STAR_KEY_MANAGER_DCAP_EVIDENCE_FOR_ISSUE_ENCLAVE_BASE64={}",
            URL_SAFE.encode(&self.key_manager_evidence)
        );
    }
}

pub fn install_key_manager_keys_for_enclave(
    key_manager: &KeyManagerRuntime,
    recipient: &EnclaveRuntime,
) -> Result<(), Box<dyn Error>> {
    let key_manager_quote = key_manager.public_key_attestation()?;
    let recipient_quote = recipient.transfer_key_attestation()?;
    let recipient_evidence = evidence_for_verifier(&recipient_quote)?;
    let key_manager_evidence = evidence_for_verifier(&key_manager_quote)?;
    let keys = key_manager.export_keys(&recipient_evidence)?;

    recipient.install_keys(&key_manager_evidence, &keys)?;
    Ok(())
}

pub fn benchmark_key_bundle() -> Result<Vec<u8>, Box<dyn Error>> {
    let key_manager = KeyManagerRuntime::from_env_with_sealed_keys(&[])?;
    Ok(key_manager.sealed_keys().to_vec())
}

fn evidence_for_verifier(quote: &[u8]) -> Result<Vec<u8>, EnclaveError> {
    let runtime = tokio::runtime::Runtime::new()
        .map_err(|err| EnclaveError::Dcap(format!("create Tokio runtime: {err}")))?;

    runtime
        .block_on(Evidence::encode_for_verifier(quote))
        .map_err(EnclaveError::Dcap)
}
