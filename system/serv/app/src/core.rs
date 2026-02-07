use std::sync::{OnceLock};
use star_core::serv::enclave::Enclave;

pub static ENCLAVE: OnceLock<Enclave> = OnceLock::new();
