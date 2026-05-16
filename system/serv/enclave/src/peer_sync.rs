use aes_gcm::{
    aead::{Aead, KeyInit, Payload},
    Aes128Gcm, Nonce,
};
use rand::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::seal;

pub const OP_PEER_SYNC_BEGIN: u8 = 0x40;
pub const OP_PEER_SYNC_EXPORT: u8 = 0x41;
pub const OP_PEER_SYNC_IMPORT: u8 = 0x42;

const VERSION: u8 = 1;
const MAX_PENDING_SESSIONS: usize = 16;
const CHALLENGE_DOMAIN: &[u8] = b"star.enclave.peer-sync.challenge.v1";
const TRANSFER_DOMAIN: &[u8] = b"star.enclave.peer-sync.transfer.v1";

#[derive(Debug, Error)]
pub enum PeerSyncError {
    #[error("SGX peer key derivation failed: {0}")]
    Seal(#[from] seal::SealError),

    #[error("peer sync message serialization failed: {0}")]
    Serialize(#[from] Box<bincode::ErrorKind>),

    #[error("unsupported peer sync version {0}")]
    UnsupportedVersion(u8),

    #[error("peer authentication failed")]
    Authentication,

    #[error("peer sync session was not found")]
    UnknownSession,

    #[error("peer sync encryption or decryption failed")]
    Crypto,
}

impl PeerSyncError {
    pub fn is_bad_request(&self) -> bool {
        matches!(
            self,
            Self::Serialize(_)
                | Self::UnsupportedVersion(_)
                | Self::Authentication
                | Self::UnknownSession
                | Self::Crypto
        )
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PeerSecrets {
    pub token_key: [u8; 32],
    pub hpke_receiver_key: [u8; 32],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerSyncChallenge {
    version: u8,
    session_id: [u8; 16],
    challenge: [u8; 32],
    proof_nonce: [u8; 12],
    proof_tag: [u8; 16],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerSyncTransfer {
    version: u8,
    session_id: [u8; 16],
    nonce: [u8; 12],
    ciphertext: Vec<u8>,
}

#[derive(Default)]
pub struct PeerSync {
    pending: Vec<PeerSyncChallenge>,
}

impl PeerSync {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn begin<R>(&mut self, rng: &mut R) -> Result<Vec<u8>, PeerSyncError>
    where
        R: CryptoRng + RngCore,
    {
        let peer_key = seal::peer_sync_key()?;
        self.begin_with_key(&peer_key, rng)
    }

    pub fn export<R>(
        &self,
        secrets: PeerSecrets,
        challenge: &[u8],
        rng: &mut R,
    ) -> Result<Vec<u8>, PeerSyncError>
    where
        R: CryptoRng + RngCore,
    {
        let peer_key = seal::peer_sync_key()?;
        Self::export_with_key(&peer_key, secrets, challenge, rng)
    }

    pub fn import(&mut self, transfer: &[u8]) -> Result<PeerSecrets, PeerSyncError> {
        let peer_key = seal::peer_sync_key()?;
        self.import_with_key(&peer_key, transfer)
    }

    fn begin_with_key<R>(
        &mut self,
        peer_key: &[u8; 16],
        rng: &mut R,
    ) -> Result<Vec<u8>, PeerSyncError>
    where
        R: CryptoRng + RngCore,
    {
        let mut challenge = PeerSyncChallenge {
            version: VERSION,
            session_id: [0u8; 16],
            challenge: [0u8; 32],
            proof_nonce: [0u8; 12],
            proof_tag: [0u8; 16],
        };
        rng.fill_bytes(&mut challenge.session_id);
        rng.fill_bytes(&mut challenge.challenge);
        rng.fill_bytes(&mut challenge.proof_nonce);

        let aad = challenge_aad(&challenge);
        let proof = encrypt(peer_key, &challenge.proof_nonce, &[], &aad)?;
        challenge.proof_tag = proof.try_into().map_err(|_| PeerSyncError::Crypto)?;

        if self.pending.len() >= MAX_PENDING_SESSIONS {
            self.pending.remove(0);
        }
        self.pending.push(challenge.clone());

        Ok(bincode::serialize(&challenge)?)
    }

    fn export_with_key<R>(
        peer_key: &[u8; 16],
        secrets: PeerSecrets,
        challenge: &[u8],
        rng: &mut R,
    ) -> Result<Vec<u8>, PeerSyncError>
    where
        R: CryptoRng + RngCore,
    {
        let challenge: PeerSyncChallenge = bincode::deserialize(challenge)?;
        verify_version(challenge.version)?;
        verify_challenge(peer_key, &challenge)?;

        let plaintext = bincode::serialize(&secrets)?;
        let mut nonce = [0u8; 12];
        rng.fill_bytes(&mut nonce);
        let aad = transfer_aad(&challenge)?;
        let ciphertext = encrypt(peer_key, &nonce, &plaintext, &aad)?;

        let transfer = PeerSyncTransfer {
            version: VERSION,
            session_id: challenge.session_id,
            nonce,
            ciphertext,
        };
        Ok(bincode::serialize(&transfer)?)
    }

    fn import_with_key(
        &mut self,
        peer_key: &[u8; 16],
        transfer: &[u8],
    ) -> Result<PeerSecrets, PeerSyncError> {
        let transfer: PeerSyncTransfer = bincode::deserialize(transfer)?;
        verify_version(transfer.version)?;

        let pending_index = self
            .pending
            .iter()
            .position(|challenge| challenge.session_id == transfer.session_id)
            .ok_or(PeerSyncError::UnknownSession)?;
        let challenge = self.pending[pending_index].clone();

        let aad = transfer_aad(&challenge)?;
        let plaintext = decrypt(peer_key, &transfer.nonce, &transfer.ciphertext, &aad)?;
        let secrets = bincode::deserialize(&plaintext)?;

        self.pending.remove(pending_index);
        Ok(secrets)
    }
}

fn verify_version(version: u8) -> Result<(), PeerSyncError> {
    if version == VERSION {
        Ok(())
    } else {
        Err(PeerSyncError::UnsupportedVersion(version))
    }
}

fn verify_challenge(
    peer_key: &[u8; 16],
    challenge: &PeerSyncChallenge,
) -> Result<(), PeerSyncError> {
    let aad = challenge_aad(challenge);
    let proof = decrypt(peer_key, &challenge.proof_nonce, &challenge.proof_tag, &aad)?;
    if proof.is_empty() {
        Ok(())
    } else {
        Err(PeerSyncError::Authentication)
    }
}

fn challenge_aad(challenge: &PeerSyncChallenge) -> Vec<u8> {
    let mut aad = Vec::with_capacity(
        CHALLENGE_DOMAIN.len() + 1 + challenge.session_id.len() + challenge.challenge.len(),
    );
    aad.extend_from_slice(CHALLENGE_DOMAIN);
    aad.push(challenge.version);
    aad.extend_from_slice(&challenge.session_id);
    aad.extend_from_slice(&challenge.challenge);
    aad
}

fn transfer_aad(challenge: &PeerSyncChallenge) -> Result<Vec<u8>, PeerSyncError> {
    let encoded = bincode::serialize(challenge)?;
    let mut aad = Vec::with_capacity(TRANSFER_DOMAIN.len() + encoded.len());
    aad.extend_from_slice(TRANSFER_DOMAIN);
    aad.extend_from_slice(&encoded);
    Ok(aad)
}

fn encrypt(
    key: &[u8; 16],
    nonce: &[u8; 12],
    plaintext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, PeerSyncError> {
    let cipher = Aes128Gcm::new_from_slice(key).map_err(|_| PeerSyncError::Crypto)?;
    cipher
        .encrypt(
            Nonce::from_slice(nonce),
            Payload {
                msg: plaintext,
                aad,
            },
        )
        .map_err(|_| PeerSyncError::Crypto)
}

fn decrypt(
    key: &[u8; 16],
    nonce: &[u8; 12],
    ciphertext: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, PeerSyncError> {
    let cipher = Aes128Gcm::new_from_slice(key).map_err(|_| PeerSyncError::Crypto)?;
    cipher
        .decrypt(
            Nonce::from_slice(nonce),
            Payload {
                msg: ciphertext,
                aad,
            },
        )
        .map_err(|_| PeerSyncError::Authentication)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    fn secrets(byte: u8) -> PeerSecrets {
        PeerSecrets {
            token_key: [byte; 32],
            hpke_receiver_key: [byte.wrapping_add(1); 32],
        }
    }

    #[test]
    fn authenticated_transfer_round_trip() {
        let key = [7u8; 16];
        let mut recipient_rng = StdRng::seed_from_u64(11);
        let mut donor_rng = StdRng::seed_from_u64(12);
        let donor_secrets = secrets(9);

        let mut recipient = PeerSync::new();
        let challenge = recipient
            .begin_with_key(&key, &mut recipient_rng)
            .expect("begin peer sync");
        let transfer =
            PeerSync::export_with_key(&key, donor_secrets.clone(), &challenge, &mut donor_rng)
                .expect("export secrets");
        let imported = recipient
            .import_with_key(&key, &transfer)
            .expect("import secrets");

        assert_eq!(imported.token_key, donor_secrets.token_key);
        assert_eq!(imported.hpke_receiver_key, donor_secrets.hpke_receiver_key);
    }

    #[test]
    fn export_rejects_unauthenticated_challenge() {
        let key = [7u8; 16];
        let wrong_key = [8u8; 16];
        let mut recipient_rng = StdRng::seed_from_u64(21);
        let mut donor_rng = StdRng::seed_from_u64(22);
        let mut recipient = PeerSync::new();

        let challenge = recipient
            .begin_with_key(&wrong_key, &mut recipient_rng)
            .expect("begin peer sync");
        let err = PeerSync::export_with_key(&key, secrets(1), &challenge, &mut donor_rng)
            .expect_err("challenge must not authenticate");

        assert!(matches!(err, PeerSyncError::Authentication));
    }

    #[test]
    fn import_consumes_pending_session() {
        let key = [7u8; 16];
        let mut recipient_rng = StdRng::seed_from_u64(31);
        let mut donor_rng = StdRng::seed_from_u64(32);
        let mut recipient = PeerSync::new();

        let challenge = recipient
            .begin_with_key(&key, &mut recipient_rng)
            .expect("begin peer sync");
        let transfer = PeerSync::export_with_key(&key, secrets(2), &challenge, &mut donor_rng)
            .expect("export secrets");

        recipient
            .import_with_key(&key, &transfer)
            .expect("first import");
        let err = match recipient.import_with_key(&key, &transfer) {
            Ok(_) => panic!("session should be consumed"),
            Err(err) => err,
        };

        assert!(matches!(err, PeerSyncError::UnknownSession));
    }
}
