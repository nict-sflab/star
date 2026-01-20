use rand::RngCore;

use crate::enclave::core::bytes::{Long, Short};

pub struct Credential {
    pub uid: Long,
    pub code: Long,
}

impl Credential {
    pub fn new(
        uid: Long,
        code: Long,
    ) -> Self {
        Self {
            uid,
            code,
        }
    }

    pub fn random(sk: Long) -> Self {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;
        
        // random uid
        let mut rng = rand::thread_rng();
        let mut uid = [0u8; 32];
        rng.fill_bytes(&mut uid);

        // calc code ← MAC(uid, sk)
        let mut mac = Hmac::<Sha256>::new_from_slice(&sk)
            .expect("HMAC can take key of any size");
        mac.update(&uid);
        let code: Long = mac.finalize().into_bytes().into();

        Self {
            uid,
            code,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let uid: Long = bytes[0..32].try_into().expect("fixed-length slice");
        let code: Long = bytes[32..64].try_into().expect("fixed-length slice");

        Self {
            uid,
            code,
        }
    }

    pub fn is_valid(&self, period: Short, count: Short, max_count: Short, sk: Long) -> bool {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        let current_period = constant_time_utils::time::current_period();
        let is_current = period == current_period;
        let is_count_valid = constant_time_utils::cmp::ct_lt(&count, &max_count);

        // Recalculate code and compare
        let mut mac = Hmac::<Sha256>::new_from_slice(&sk)
            .expect("HMAC can take key of any size");
        mac.update(&self.uid);
        let recalculated_code: Long = mac.finalize().into_bytes().into();
        let is_code_valid = recalculated_code == self.code; 
        
        is_current && is_count_valid && is_code_valid
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.uid);
        bytes.extend_from_slice(&self.code);
        bytes
    }
}
