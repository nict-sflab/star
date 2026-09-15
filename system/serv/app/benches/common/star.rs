use rand_core::OsRng;
use simple_filter_box::hashmap::HashMapFilterBox;
use star_app::{
    KeyManagerRuntime,
    enclave::{EnclaveRuntime, EnclaveRuntimes},
    filter::StarFilter,
    install_key_manager_keys_for_enclave, install_key_manager_keys_for_pool,
};
use star_core::{client::User, secure_channel::client::SecureChannelClient};

use super::aesm_proxy::AesmProxyGuard;

pub type BenchFilter = StarFilter<DashMapFilterBox>;
pub const MAX_COUNT_PER_CREDENTIAL: u64 = 1_000_000;

pub struct KeyedEnclaves {
    filter: EnclaveRuntimes,
    issue: EnclaveRuntime,
}

impl KeyedEnclaves {
    pub fn new(filter_pool_size: u8) -> Self {
        let _aesm_proxy = AesmProxyGuard::start();
        let key_manager = KeyManagerRuntime::from_env_with_sealed_keys(&[]).expect("key manager");
        let filter = EnclaveRuntimes::from_env(filter_pool_size).expect("filter enclave pool");
        let issue = EnclaveRuntime::from_env().expect("issue enclave");

        install_key_manager_keys_for_pool(&key_manager, &filter)
            .expect("install filter enclave benchmark keys");
        install_key_manager_keys_for_enclave(&key_manager, &issue)
            .expect("install issue enclave benchmark keys");

        Self { filter, issue }
    }

    pub fn into_filter(
        self,
        prefixes: Vec<u32>,
        prefix_max: u32,
    ) -> (BenchFilter, EnclaveRuntime) {
        (
            StarFilter::new(HashMapFilterBox::new(), prefixes, prefix_max, self.filter),
            self.issue,
        )
    }
}

pub fn register_user(enclave: &EnclaveRuntime, max_count: u64, rng: &mut OsRng) -> User {
    let sender =
        SecureChannelClient::new(enclave.public_key().expect("public key")).expect("sender init");
    let mut user = User::new(max_count, sender);
    let (request, shared_key) = user.request_credential(rng);
    let credential = enclave.issue_cred(&request).expect("issue credential");
    user.receive_credential(&credential, &shared_key);
    user
}
