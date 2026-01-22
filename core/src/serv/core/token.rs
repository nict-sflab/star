use alloc::vec::Vec;

use crate::serv::core::bytes::{Long, Short};

pub struct Token {
    pub token: Long,
}

impl Token {
    pub fn new(
        token: Long,
    ) -> Self {
        Self {
            token,
        }
    }

    pub fn random(uid: Long, period: Short, cnt: Short, sk: Long) -> Self {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        // calc token ← PRF(cnt||t||uid, sk)
        let mut prf = Hmac::<Sha256>::new_from_slice(&sk)
            .expect("HMAC can take key of any size");
        prf.update(&cnt);
        prf.update(&period);
        prf.update(&uid);
        let token: Long = prf.finalize().into_bytes().into();

        Self {
            token,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        assert!(bytes.len() >= 40, "token bytes must contain 32-byte token + 8-byte period");
        let token: Long = bytes[0..32].try_into().expect("fixed-length slice");

        Self {
            token,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.token);
        bytes
    }
}
