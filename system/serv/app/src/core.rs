use star_core::serv::enclave::Enclave;
use std::sync::OnceLock;

pub static ENCLAVE: OnceLock<Enclave> = OnceLock::new();
