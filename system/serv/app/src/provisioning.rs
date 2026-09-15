use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use key_manager_client::RemoteKeyManagerRuntime;
use key_manager_service::KeyManagerRuntime;
use std::error::Error;

use crate::enclave::{EnclaveError, EnclaveRuntime, EnclaveRuntimes};
use star_attestation::dcap;

const FILTER_ENCLAVE_POOL_SIZE: u8 = 10;

pub(crate) struct ProvisionedEnclaves {
    pub(crate) public_key: [u8; 32],
    pub(crate) filter_enclave: EnclaveRuntimes,
    pub(crate) issue_enclave: EnclaveRuntime,
    key_manager_quote: Vec<u8>,
    filter_quote: Vec<u8>,
    issue_quote: Vec<u8>,
    filter_evidence_for_key_manager: Vec<u8>,
    issue_evidence_for_key_manager: Vec<u8>,
    key_manager_evidence: Vec<u8>,
}

impl ProvisionedEnclaves {
    pub(crate) fn provision() -> Result<Self, Box<dyn Error>> {
        let key_manager =
            RemoteKeyManagerRuntime::from_env().ok_or("STAR_KEY_MANAGER_URL is required")?;
        let filter_enclave = EnclaveRuntimes::from_env(FILTER_ENCLAVE_POOL_SIZE)?;
        let issue_enclave = EnclaveRuntime::from_env()?;

        let public_key = key_manager.public_key()?;
        let key_manager_quote = key_manager.public_key_attestation()?;
        let filter_quote = filter_enclave.transfer_key_attestation()?;
        let issue_quote = issue_enclave.transfer_key_attestation()?;
        let filter_export = key_manager.export_keys_for_recipient_quote(&filter_quote)?;
        let issue_export = key_manager.export_keys_for_recipient_quote(&issue_quote)?;
        let key_manager_evidence =
            dcap::raw_evidence_for_verifier(&key_manager_quote).map_err(EnclaveError::Dcap)?;

        filter_enclave.install_keys(&key_manager_evidence, &filter_export.envelope)?;
        issue_enclave.install_keys(&key_manager_evidence, &issue_export.envelope)?;

        Ok(Self {
            public_key,
            filter_enclave,
            issue_enclave,
            key_manager_quote,
            filter_quote,
            issue_quote,
            filter_evidence_for_key_manager: filter_export.recipient_evidence,
            issue_evidence_for_key_manager: issue_export.recipient_evidence,
            key_manager_evidence,
        })
    }

    pub(crate) fn emit_startup_attestation(&self) {
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
            URL_SAFE.encode(&self.filter_quote)
        );
        println!(
            "STAR_ISSUE_ENCLAVE_TRANSFER_QUOTE_BASE64={}",
            URL_SAFE.encode(&self.issue_quote)
        );
        println!(
            "STAR_FILTER_ENCLAVE_DCAP_EVIDENCE_FOR_KEY_MANAGER_BASE64={}",
            URL_SAFE.encode(&self.filter_evidence_for_key_manager)
        );
        println!(
            "STAR_ISSUE_ENCLAVE_DCAP_EVIDENCE_FOR_KEY_MANAGER_BASE64={}",
            URL_SAFE.encode(&self.issue_evidence_for_key_manager)
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

pub fn install_key_manager_keys_for_pool(
    key_manager: &KeyManagerRuntime,
    recipient: &EnclaveRuntimes,
) -> Result<(), Box<dyn Error>> {
    install_keys_remotely(key_manager, recipient)
}

pub fn install_key_manager_keys_for_enclave(
    key_manager: &KeyManagerRuntime,
    recipient: &EnclaveRuntime,
) -> Result<(), Box<dyn Error>> {
    install_keys_remotely(key_manager, recipient)
}

trait KeyRecipient {
    fn transfer_key_attestation(&self) -> Result<Vec<u8>, EnclaveError>;
    fn install_keys(&self, manager_evidence: &[u8], envelope: &[u8]) -> Result<(), EnclaveError>;
}

impl KeyRecipient for EnclaveRuntimes {
    fn transfer_key_attestation(&self) -> Result<Vec<u8>, EnclaveError> {
        self.transfer_key_attestation()
    }

    fn install_keys(&self, manager_evidence: &[u8], envelope: &[u8]) -> Result<(), EnclaveError> {
        self.install_keys(manager_evidence, envelope)
    }
}

impl KeyRecipient for EnclaveRuntime {
    fn transfer_key_attestation(&self) -> Result<Vec<u8>, EnclaveError> {
        self.transfer_key_attestation()
    }

    fn install_keys(&self, manager_evidence: &[u8], envelope: &[u8]) -> Result<(), EnclaveError> {
        self.install_keys(manager_evidence, envelope)
    }
}

fn install_keys_remotely(
    key_manager: &KeyManagerRuntime,
    recipient: &impl KeyRecipient,
) -> Result<(), Box<dyn Error>> {
    let key_manager_quote = key_manager.public_key_attestation()?;
    let recipient_quote = recipient.transfer_key_attestation()?;
    let recipient_evidence =
        dcap::raw_evidence_for_verifier(&recipient_quote).map_err(EnclaveError::Dcap)?;
    let key_manager_evidence =
        dcap::raw_evidence_for_verifier(&key_manager_quote).map_err(EnclaveError::Dcap)?;
    let keys = key_manager.export_keys(&recipient_evidence)?;

    recipient.install_keys(&key_manager_evidence, &keys)?;
    Ok(())
}

pub fn benchmark_key_bundle() -> Result<Vec<u8>, Box<dyn Error>> {
    let key_manager = KeyManagerRuntime::from_env_with_sealed_keys(&[])?;
    Ok(key_manager.sealed_keys().to_vec())
}
