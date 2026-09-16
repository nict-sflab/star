use alloc::vec::Vec;
use hmac::digest::Update;
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
        assert!(
            bytes.len() >= 40,
            "token bytes must contain 32-byte token + 8-byte period"
        );
        let token: Long = bytes[0..32].try_into().expect("fixed-length slice");

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
