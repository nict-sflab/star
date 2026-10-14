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

        // h ← SHA256(t || cnt || uid), where t is the period.
        let mut h = Sha256::new();

        Digest::update(&mut h, period);
        Digest::update(&mut h, cnt);
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
    fn token_hashes_period_then_counter_then_uid() {
        let uid: [u8; 32] = core::array::from_fn(|index| index as u8);
        let period = [1, 2, 3, 4, 5, 6, 7, 8];
        let cnt = [9, 10, 11, 12, 13, 14, 15, 16];

        // Reference SHA-256 digest for period || cnt || uid.
        let expected = [
            0xae, 0x13, 0xe5, 0x4b, 0x5c, 0x9e, 0x76, 0x9a,
            0xc9, 0x9f, 0x02, 0xe2, 0x2f, 0x3a, 0xaa, 0xf7,
            0x28, 0x45, 0x42, 0x11, 0x12, 0xb9, 0x7e, 0x11,
            0x15, 0x55, 0x6f, 0xe6, 0xcd, 0x9a, 0x3d, 0x54,
        ];
        assert_eq!(Token::random(uid, period, cnt).token, expected);
    }
}
