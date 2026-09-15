use std::env;

pub mod api;
pub mod enclave;
pub mod filter;
mod provisioning;
mod proxy;
mod server;
mod sgx;
pub mod tls;

pub use key_manager_service::KeyManagerRuntime;
pub use provisioning::{
    benchmark_key_bundle, install_key_manager_keys_for_enclave, install_key_manager_keys_for_pool,
};
pub use server::{build_server, Shard};

#[cfg(test)]
pub(crate) use server::build_server_without_startup_attestation;

pub(crate) fn env_or_default(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

#[cfg(test)]
mod tests;
