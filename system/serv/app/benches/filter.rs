mod common;

use std::{
    env,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::star::{BenchFilter, KeyedEnclaves, MAX_COUNT_PER_CREDENTIAL, register_user};
use criterion::{BatchSize, Criterion, Throughput, black_box, criterion_group, criterion_main};
use rand_core::OsRng;
use simple_filter_box::{core::Entry, hashmap::HashMapFilterBox};
use star_app::{
    KeyManagerRuntime, enclave::EnclaveRuntime, filter::StarFilter,
    install_key_manager_keys_for_enclave,
};
use star_core::client::User;
use tokio::runtime::{Builder, Runtime};

const FIXED_PREFIX_MAX: u32 = 2;
const ISSUE_TOKEN_GROUP_NAME: &str = "star_filter_issue_token_and_route";
const CHECK_DUPLICATION_GROUP_NAME: &str = "star_filter_check_duplication";
const DIAGNOSTICS_GROUP_NAME: &str = "star_filter_diagnostics";
const LATENCY_BENCH_NAME: &str = "latency";

fn current_thread_runtime() -> Runtime {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
}

fn build_filter(prefixes: Vec<u32>) -> BenchFilter {
    StarFilter::new(
        HashMapFilterBox::new(),
        prefixes,
        FIXED_PREFIX_MAX,
        Arc::new(EnclaveRuntime::from_env().expect("shard enclave")),
    )
}

struct TokenRequestGenerator {
    issue_enclave: Arc<EnclaveRuntime>,
    user: User,
    counter: u64,
    period: u64,
    rng: OsRng,
}

impl TokenRequestGenerator {
    fn new(issue_enclave: Arc<EnclaveRuntime>) -> Self {
        let mut rng = OsRng;
        let user = register_user(&issue_enclave, MAX_COUNT_PER_CREDENTIAL, &mut rng);

        Self {
            issue_enclave,
            user,
            counter: 0,
            period: u64::from_be_bytes(constant_time_utils::time::current_period()),
            rng,
        }
    }

    fn next_request(&mut self) -> Vec<u8> {
        let current_period = u64::from_be_bytes(constant_time_utils::time::current_period());
        if current_period != self.period || self.counter >= MAX_COUNT_PER_CREDENTIAL {
            self.period = current_period;
            self.user = register_user(&self.issue_enclave, MAX_COUNT_PER_CREDENTIAL, &mut self.rng);
            self.counter = 0;
        }

        let request = self
            .user
            .request_auth(self.counter, self.period, &mut self.rng)
            .expect("token request");
        self.counter += 1;
        request
    }
}

fn route_for_token_index(idx: u64) -> u32 {
    (idx as u32) % FIXED_PREFIX_MAX
}

fn advance_batch(counter: &mut u64, batch_ops: u64) -> u64 {
    let start = *counter;
    *counter = (*counter).wrapping_add(batch_ops);
    start
}

fn token_from_counter(counter: u64) -> Entry {
    let mut token = [0u8; 32];
    token[..8].copy_from_slice(&counter.to_be_bytes());
    token
}

fn bench_issue_token_and_route(c: &mut Criterion) {
    let latency_rt = current_thread_runtime();
    let (filter, issue_enclave) =
        KeyedEnclaves::new().into_filter((0..FIXED_PREFIX_MAX).collect(), FIXED_PREFIX_MAX);
    let filter = Arc::new(filter);
    let request_generator = Arc::new(Mutex::new(TokenRequestGenerator::new(issue_enclave)));

    let mut group = c.benchmark_group(ISSUE_TOKEN_GROUP_NAME);

    group.throughput(Throughput::Elements(1));
    group.bench_function(LATENCY_BENCH_NAME, |b| {
        let filter = Arc::clone(&filter);
        let request_generator = Arc::clone(&request_generator);

        b.to_async(&latency_rt).iter_batched(
            || {
                request_generator
                    .lock()
                    .expect("request generator mutex")
                    .next_request()
            },
            |request| {
                let filter = Arc::clone(&filter);
                async move {
                    black_box(
                        filter
                            .issue_token_and_route(black_box(request.as_slice()))
                            .await
                            .expect("issue token and route"),
                    );
                }
            },
            BatchSize::PerIteration,
        )
    });

    group.finish();
}

fn bench_check_duplication(c: &mut Criterion) {
    let latency_filter = Arc::new(build_filter((0..FIXED_PREFIX_MAX).collect()));
    let mut latency_counter = 0u64;

    let mut group = c.benchmark_group(CHECK_DUPLICATION_GROUP_NAME);
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(4));

    group.throughput(Throughput::Elements(1));
    group.bench_function(LATENCY_BENCH_NAME, |b| {
        let filter = Arc::clone(&latency_filter);
        b.iter_batched(
            || {
                let idx = advance_batch(&mut latency_counter, 1);
                (token_from_counter(idx), route_for_token_index(idx))
            },
            |(token, route)| {
                black_box(
                    filter
                        .check_duplication(black_box(&token), black_box(route))
                        .expect("unique token on its shard"),
                );
            },
            BatchSize::PerIteration,
        )
    });

    group.finish();
}

fn bench_diagnostics(c: &mut Criterion) {
    if env::var_os("STAR_FILTER_BENCH_DIAGNOSTICS").is_none() {
        return;
    }

    let _aesm_proxy = common::aesm_proxy::AesmProxyGuard::start();
    let rt = current_thread_runtime();
    let key_manager = KeyManagerRuntime::from_env_with_sealed_keys(&[]).expect("key manager");

    let request_enclave = EnclaveRuntime::from_env().expect("request enclave");
    install_key_manager_keys_for_enclave(&key_manager, &request_enclave)
        .expect("install request enclave benchmark keys");
    let mut rng = OsRng;
    let mut user = register_user(&request_enclave, MAX_COUNT_PER_CREDENTIAL, &mut rng);
    let request = user
        .request_auth(
            0,
            u64::from_be_bytes(constant_time_utils::time::current_period()),
            &mut rng,
        )
        .expect("token request");

    let public_key_enclave = EnclaveRuntime::from_env().expect("public key enclave");
    install_key_manager_keys_for_enclave(&key_manager, &public_key_enclave)
        .expect("install public key enclave benchmark keys");

    let issue_token_enclave = EnclaveRuntime::from_env().expect("issue token enclave");
    install_key_manager_keys_for_enclave(&key_manager, &issue_token_enclave)
        .expect("install issue token enclave benchmark keys");

    let mut group = c.benchmark_group(DIAGNOSTICS_GROUP_NAME);
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(1));
    group.throughput(Throughput::Elements(1));

    group.bench_function("public_key_ecall", |b| {
        b.iter(|| black_box(public_key_enclave.public_key().expect("public key")));
    });

    group.bench_function("issue_token_direct_ecall", |b| {
        b.iter(|| {
            black_box(
                issue_token_enclave
                    .issue_token(
                        black_box(request.as_slice()),
                        constant_time_utils::time::current_period(),
                    )
                    .expect("issue token"),
            );
        });
    });

    group.bench_function("spawn_blocking_noop", |b| {
        b.to_async(&rt).iter(|| async {
            black_box(
                tokio::task::spawn_blocking(|| 1u64)
                    .await
                    .expect("spawn_blocking join"),
            );
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_issue_token_and_route,
    bench_check_duplication,
    bench_diagnostics
);
criterion_main!(benches);
