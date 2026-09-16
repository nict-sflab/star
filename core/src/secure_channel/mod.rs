pub mod client;
pub mod server;

use core::convert::Infallible;

struct RngCompat<'a, R>(&'a mut R);

impl<R: rand::RngCore> rand_core::TryRng for RngCompat<'_, R> {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(self.0.next_u32())
    }

    fn try_next_u64(&mut self) -> Result<u64, Self::Error> {
        Ok(self.0.next_u64())
    }

    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Self::Error> {
        self.0.fill_bytes(dst);
        Ok(())
    }
}

impl<R: rand::CryptoRng + rand::RngCore> rand_core::TryCryptoRng for RngCompat<'_, R> {}

#[cfg(test)]
pub mod tests;
