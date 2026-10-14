
use std::convert::Infallible;

use rand::{Rng, SeedableRng, TryCryptoRng, TryRng};
use rand_chacha::ChaCha20Rng;

pub(crate) struct PoisonedRng(ChaCha20Rng);

impl PoisonedRng {
    pub(crate) fn new() -> Self {
        let mut seed = [0u8; 32];
        timecop::poison(seed.as_slice());
        Self(ChaCha20Rng::from_seed(seed))
    }
}

impl TryRng for PoisonedRng {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        let mut bytes = [0u8; 4];
        self.try_fill_bytes(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        let mut bytes = [0u8; 8];
        self.try_fill_bytes(&mut bytes)?;
        Ok(u64::from_le_bytes(bytes))
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Self::Error> {
        self.0.fill_bytes(dest);
        timecop::poison(dest);
        Ok(())
    }
}

impl TryCryptoRng for PoisonedRng {}
