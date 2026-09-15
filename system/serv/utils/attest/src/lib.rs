#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

mod attest;
mod codec;
mod evidence;
mod measurement;
mod policy;
mod quote;
mod report;
mod verify;

pub use attest::attest;
pub use evidence::Evidence;
pub use measurement::Measurement;
pub use policy::QuotePolicy;
pub use quote::Quote;

pub const CHALLENGE_LEN: usize = 64;
pub const MEASUREMENT_LEN: usize = 32;
pub const PUBLIC_KEY_LEN: usize = 32;

pub type Result<T> = core::result::Result<T, alloc::string::String>;

#[cfg(all(feature = "host-dcap", any(unix, windows)))]
mod dcap;

#[path = "lib_test.rs"]
#[cfg(test)]
mod tests;
