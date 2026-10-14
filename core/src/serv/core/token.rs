use alloc::vec::Vec;
use sha2::Digest;

use crate::serv::core::bytes::{Long, Short};

pub struct Token {
    pub token: Long,
}

impl Token {
    pub fn new(token: Long) -> Self {
        Self { token }
    }

    pub fn random(uid: Long, period: Short, cnt: Short) -> Self {
        use sha2::Sha256;

        // calc token ← HASH(cnt||t||uid)
        let mut h = Sha256::new();

        Digest::update(&mut h, cnt);
        Digest::update(&mut h, period);
        Digest::update(&mut h, uid);

        let token: Long = h.finalize().into();

        Self { token }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let token: Long = bytes.try_into().expect("token must be exactly 32 bytes");

        Self { token }
    }

    pub fn route(&self, max: u32) -> u32 {
        let mut rem: u64 = 0;

        for &byte in &self.token {
            rem = (rem * 256 + byte as u64) % max as u64;
        }

        rem as u32
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.token);
        bytes
    }
}

#[cfg(test)]
mod tests {
    use super::Token;

    #[test]
    fn token_bytes_round_trip() {
        let token = Token::new([0x42; 32]);
        let serialized = token.to_bytes();

        assert_eq!(serialized.len(), 32);
        assert_eq!(Token::from_bytes(&serialized).token, token.token);
    }

    #[test]
    #[should_panic(expected = "token must be exactly 32 bytes")]
    fn token_from_bytes_rejects_short_input() {
        Token::from_bytes(&[0u8; 31]);
    }

    #[test]
    #[should_panic(expected = "token must be exactly 32 bytes")]
    fn token_from_bytes_rejects_extra_bytes() {
        Token::from_bytes(&[0u8; 40]);
    }
}
