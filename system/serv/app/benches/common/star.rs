use rand::{rand_core::UnwrapErr, rngs::SysRng};
use simple_filter_box::hashmap::HashMapFilterBox;
use star_app::{
    KeyManagerRuntime, enclave::EnclaveRuntime, filter::StarFilter,
    install_key_manager_keys_for_enclave,
};
use star_core::{client::User, secure_channel::client::SecureChannelClient};
use std::sync::Arc;

use super::aesm_proxy::AesmProxyGuard;

pub type BenchFilter = StarFilter<HashMapFilterBox>;
pub const MAX_COUNT_PER_CREDENTIAL: u64 = 1_000_000;

pub struct KeyedEnclaves {
    enclave: Arc<EnclaveRuntime>,
}

impl KeyedEnclaves {
    pub fn new() -> Self {
        let _aesm_proxy = AesmProxyGuard::start();
        let key_manager = KeyManagerRuntime::from_env_with_sealed_keys(&[]).expect("key manager");
        let enclave = Arc::new(EnclaveRuntime::from_env().expect("shard enclave"));
        install_key_manager_keys_for_enclave(&key_manager, &enclave)
            .expect("install shard enclave benchmark keys");
        Self { enclave }
    }

    pub fn into_filter(
        self,
        prefixes: Vec<u32>,
        prefix_max: u32,
    ) -> (BenchFilter, Arc<EnclaveRuntime>) {
        (
            StarFilter::new(
                HashMapFilterBox::new(),
                prefixes,
                prefix_max,
                Arc::clone(&self.enclave),
            ),
            self.enclave,
        )
    }
}

pub fn register_user(enclave: &EnclaveRuntime, max_count: u64, rng: &mut UnwrapErr<SysRng>) -> User {
    let sender =
        SecureChannelClient::new(enclave.public_key().expect("public key")).expect("sender init");
    let mut user = User::new(max_count, sender);
    let (request, shared_key) = user.request_credential(rng);
    let credential = enclave.issue_cred(&request).expect("issue credential");
    user.receive_credential(&credential, &shared_key);
    user
}
