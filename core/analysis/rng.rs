
use rand::{rngs::OsRng, CryptoRng, Error, RngCore, SeedableRng};
use rand_chacha::ChaCha20Rng;

pub(crate) struct PoisonedRng(ChaCha20Rng);

impl PoisonedRng {
    pub(crate) fn new() -> Self {
        let mut seed = [0u8; 32];
        timecop::poison(seed.as_slice());
        Self(ChaCha20Rng::from_seed(seed))
    }
}

impl RngCore for PoisonedRng {
    fn next_u32(&mut self) -> u32 {
        let mut bytes = [0u8; 4];
        self.fill_bytes(&mut bytes);
        u32::from_le_bytes(bytes)
    }

    fn next_u64(&mut self) -> u64 {
        let mut bytes = [0u8; 8];
        self.fill_bytes(&mut bytes);
        u64::from_le_bytes(bytes)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.fill_bytes(dest);
        timecop::poison(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for PoisonedRng {}
