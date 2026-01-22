use crate::secure_channel::server::SecureChannelServer;
use crate::serv::core::cred::Credential;
use crate::serv::core::token::Token;
use crate::serv::core::bytes::{Long, Short};

use alloc::vec::Vec;
use hpke::HpkeError;
use rand::{rngs::OsRng, RngCore};

// HPKE receiver living inside the enclave.
pub struct Enclave {
    cipher_receiver: SecureChannelServer,
    max_tickets: Short,
    sk: Long,
}

impl Enclave {
    /// Restore a receiver from an existing 32-byte X25519 secret key.
    pub fn new(max_tickets: u32, sk_bytes: Long, cipher_receiver: Long) -> Result<Self, HpkeError> {
        let cipher_receiver = SecureChannelServer::new(cipher_receiver)?;

        let max_tickets: Short = (max_tickets as u64).to_be_bytes();
        Ok(Self { cipher_receiver, max_tickets, sk: sk_bytes })
    }
    
    /// Generate a fresh receiver keypair using curve25519-dalek.
    pub fn default() -> Self {
        let mut rng = OsRng;

        let mut cipher_receiver = [0u8; 32];
        rng.fill_bytes(&mut cipher_receiver);

        let mut sk = [0u8; 32];
        rng.fill_bytes(&mut sk);

        Self::new(0, sk, cipher_receiver).expect("failed to create enclave")

    }

    /// Public key for advertising to senders.
    pub fn public_key(&self) -> [u8; 32] {
        self.cipher_receiver.public_key()
    }

    pub fn issue_token(&self, cipher: &[u8]) -> Result<Vec<u8>, u8> {
        const AAD_LEN: usize = 32;
        const INFO_LEN: usize = 32;
        const ENC_LEN: usize = 32;

        let min_len = AAD_LEN + INFO_LEN + ENC_LEN;
        assert!(cipher.len() >= min_len, "HPKE envelope too short");

        let aad = &cipher[..AAD_LEN];
        let info = &cipher[AAD_LEN..AAD_LEN + INFO_LEN];
        let enc = &cipher[AAD_LEN + INFO_LEN..AAD_LEN + INFO_LEN + ENC_LEN];
        let ciphertext = &cipher[AAD_LEN + INFO_LEN + ENC_LEN..];

        let plain = match self.cipher_receiver.receive_hpke(enc, info, aad, ciphertext) {
            Ok(p) => p,
            Err(_) => panic!("decryption failed"),
        };

        let result = self.issue_token_plain(&plain);
        Ok(result)
    }

    /// Issue a token from an already decrypted request (credential 64 bytes + count 8 bytes + period 8 bytes).
    /// This is mainly for examples/tests; production callers should use `issue_token` with HPKE-protected payloads.
    pub fn issue_token_plain(&self, request: &[u8]) -> Vec<u8> {
        if request.len() < 80 {
            return alloc::vec![0u8];
        }

        let cred = Credential::from_bytes(&request[0..64]);
        let count: Short = request[64..72].try_into().expect("fixed-length slice");
        let period: Short = request[72..80].try_into().expect("fixed-length slice");

        if !cred.is_valid(period, count, self.max_tickets, self.sk) {
            return alloc::vec![0u8];
        }

        let token = Token::random(
            cred.uid,
            period,
            count,
            self.sk,
        );

        token.to_bytes()
    }


    pub fn issue_cred(&self, cipher: &[u8]) -> Vec<u8> {
        let cred = Credential::random(self.sk);
        let plain = cred.to_bytes();

        // The HPKE envelope is aad(32) || info(32) || enc(32) || ciphertext.
        // respond_cipher expects the full envelope to recover the shared key.
        self.cipher_receiver.respond_cipher(&plain, cipher)
    }

    pub fn issue_cred_plain(&self) -> Vec<u8> {
        let cred = Credential::random(self.sk);
        cred.to_bytes()
    }

}
