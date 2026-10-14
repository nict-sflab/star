mod common;

use std::{
    env,
    sync::{Arc, Mutex},
    time::Duration,
};

use common::star::{BenchFilter, KeyedEnclaves, MAX_COUNT_PER_CREDENTIAL, register_user};
use criterion::{BatchSize, Criterion, Throughput, black_box, criterion_group, criterion_main};
use rand::{rand_core::UnwrapErr, rngs::SysRng};
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

fn pin_benchmark_thread() {
    let core = core_affinity::get_core_ids()
        .and_then(|cores| cores.into_iter().next())
        .expect("benchmark CPU core");
    assert!(core_affinity::set_for_current(core), "pin benchmark thread");
}

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
    rng: UnwrapErr<SysRng>,
}

impl TokenRequestGenerator {
    fn new(issue_enclave: Arc<EnclaveRuntime>) -> Self {
        let mut rng = UnwrapErr(SysRng);
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
    pin_benchmark_thread();
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
    pin_benchmark_thread();
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

criterion_group!(
    benches,
    bench_issue_token_and_route,
    bench_check_duplication,
);
criterion_main!(benches);
