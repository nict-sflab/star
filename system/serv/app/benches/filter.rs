use std::{
    env,
    sync::{Arc, Mutex as StdMutex},
    time::Duration,
};

use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use rand_core::OsRng;
use simple_filter_box::{core::Entry, dashmap::DashMapFilterBox};
use star_app::{
    enclave::{EnclaveRuntime, EnclaveRuntimes},
    filter::StarFilter,
};
use star_core::{client::User, secure_channel::client::SecureChannelClient};
use tokio::{
    runtime::{Builder, Runtime},
    sync::Mutex,
    task::JoinSet,
};

type HashMapStarFilter = StarFilter<DashMapFilterBox>;
type SharedStarFilter = Arc<Mutex<HashMapStarFilter>>;

const MAX_COUNT: u32 = 1_000_000;
const PARALLEL_OPS_PER_WORKER: usize = 64;
const FIXED_PREFIX_MAX: u32 = 4;
const FIXED_CONCURRENCY: usize = FIXED_PREFIX_MAX as usize;
const DEFAULT_FILTER_ENCLAVES: u8 = 2;
const CHECK_DUPLICATION_BATCH_OPS: usize = FIXED_CONCURRENCY * PARALLEL_OPS_PER_WORKER;
const ISSUE_TOKEN_GROUP_NAME: &str = "star_filter_issue_token_and_route";
const CHECK_DUPLICATION_PARALLEL_GROUP_NAME: &str = "star_filter_check_duplication_parallel";
const CHECK_DUPLICATION_SINGLE_THREAD_GROUP_NAME: &str =
    "star_filter_check_duplication_single_thread";
const DIAGNOSTICS_GROUP_NAME: &str = "star_filter_diagnostics";
const FIXED_BATCH_BENCH_NAME: &str = "fixed_batch_256_ops_per_iter";
const FIXED_PER_OP_BENCH_NAME: &str = "fixed_per_op";

fn current_thread_runtime() -> Runtime {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime")
}

fn multi_thread_runtime(worker_threads: usize) -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(worker_threads)
        .enable_all()
        .build()
        .expect("tokio runtime")
}

fn fixed_prefixes() -> Vec<u32> {
    (0..FIXED_PREFIX_MAX).collect()
}

fn filter_enclave_count() -> u8 {
    env::var("STAR_FILTER_BENCH_ENCLAVES")
        .ok()
        .and_then(|value| value.parse().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_FILTER_ENCLAVES)
}

fn build_filter() -> HashMapStarFilter {
    let filter_box = DashMapFilterBox::new();
    let enclave = EnclaveRuntimes::from_env(filter_enclave_count()).expect("filter enclave pool");

    StarFilter::new(filter_box, fixed_prefixes(), FIXED_PREFIX_MAX, enclave)
}

fn build_request() -> Vec<u8> {
    let mut enclave = EnclaveRuntime::from_env().expect("issue enclave");
    let sender =
        SecureChannelClient::new(enclave.public_key().expect("public key")).expect("sender init");
    let mut user = User::new(MAX_COUNT as u64, sender);
    let mut rng = OsRng;

    let (cred_request, shared_key) = user.request_credential(&mut rng);
    let cred_cipher = enclave.issue_cred(&cred_request).expect("issue credential");
    user.receive_credential(&cred_cipher, &shared_key);

    let period = u64::from_be_bytes(constant_time_utils::time::current_period());
    user.request_auth(0, period, &mut rng)
        .expect("token request")
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
    token[0..8].copy_from_slice(&counter.to_be_bytes());
    token
}

fn bench_issue_token_and_route(c: &mut Criterion) {
    let rt = current_thread_runtime();
    let filter = Arc::new(Mutex::new(build_filter()));
    let request = Arc::new(build_request());

    let mut group = c.benchmark_group(ISSUE_TOKEN_GROUP_NAME);
    group.throughput(Throughput::Elements(1));
    group.bench_function(FIXED_PER_OP_BENCH_NAME, |b| {
        let filter = Arc::clone(&filter);
        let request = Arc::clone(&request);
        b.to_async(&rt).iter(|| {
            let filter = Arc::clone(&filter);
            let request = Arc::clone(&request);
            async move {
                let result = filter
                    .lock()
                    .await
                    .issue_token_and_route(black_box(request.as_slice()))
                    .await
                    .expect("issue token and route");
                black_box(result);
            }
        })
    });
    group.finish();
}

fn bench_diagnostics(c: &mut Criterion) {
    if env::var_os("STAR_FILTER_BENCH_DIAGNOSTICS").is_none() {
        return;
    }

    let rt = current_thread_runtime();
    let request = Arc::new(build_request());

    let mut group = c.benchmark_group(DIAGNOSTICS_GROUP_NAME);
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(1));

    let public_key_enclave = Arc::new(StdMutex::new(
        EnclaveRuntime::from_env().expect("public key enclave"),
    ));
    group.throughput(Throughput::Elements(1));
    group.bench_function("public_key_ecall", |b| {
        let enclave = Arc::clone(&public_key_enclave);
        b.iter(|| {
            let key = enclave
                .lock()
                .expect("enclave mutex")
                .public_key()
                .expect("public key");
            black_box(key);
        });
    });

    let issue_token_enclave = Arc::new(StdMutex::new(
        EnclaveRuntime::from_env().expect("issue token enclave"),
    ));
    group.throughput(Throughput::Elements(1));
    group.bench_function("issue_token_direct_ecall", |b| {
        let enclave = Arc::clone(&issue_token_enclave);
        let request = Arc::clone(&request);
        b.iter(|| {
            let token = enclave
                .lock()
                .expect("enclave mutex")
                .issue_token(
                    black_box(request.as_slice()),
                    constant_time_utils::time::current_period(),
                )
                .expect("issue token");
            black_box(token);
        });
    });

    group.throughput(Throughput::Elements(1));
    group.bench_function("spawn_blocking_noop", |b| {
        b.to_async(&rt).iter(|| async {
            let value = tokio::task::spawn_blocking(|| 1u64)
                .await
                .expect("spawn_blocking join");
            black_box(value);
        });
    });

    group.finish();
}

async fn run_check_duplication_batch(filter: SharedStarFilter, batch_start: u64) {
    let mut joins = JoinSet::new();

    for worker in 0..FIXED_CONCURRENCY {
        let filter = Arc::clone(&filter);
        let worker_start = batch_start + (worker * PARALLEL_OPS_PER_WORKER) as u64;
        joins.spawn(async move {
            for offset in 0..PARALLEL_OPS_PER_WORKER {
                let idx = worker_start + offset as u64;
                let token = token_from_counter(idx);
                let route = route_for_token_index(idx);
                let result = filter
                    .lock()
                    .await
                    .check_duplication(black_box(&token), black_box(route))
                    .await;
                let _ = black_box(result);
            }
        });
    }

    while let Some(joined) = joins.join_next().await {
        joined.expect("join worker");
    }
}

async fn run_check_duplication_single_batch(
    filter: SharedStarFilter,
    batch_start: u64,
    batch_ops: usize,
) {
    for offset in 0..batch_ops {
        let idx = batch_start + offset as u64;
        let token = token_from_counter(idx);
        let route = route_for_token_index(idx);
        let result = filter
            .lock()
            .await
            .check_duplication(black_box(&token), black_box(route))
            .await;
        let _ = black_box(result);
    }
}

fn bench_check_duplication_parallel(c: &mut Criterion) {
    let rt = multi_thread_runtime(FIXED_CONCURRENCY);
    let filter = Arc::new(Mutex::new(build_filter()));

    let mut batch_start = 0u64;
    let batch_ops = CHECK_DUPLICATION_BATCH_OPS as u64;

    let mut group = c.benchmark_group(CHECK_DUPLICATION_PARALLEL_GROUP_NAME);
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(4));
    group.throughput(Throughput::Elements(batch_ops));

    group.bench_function(FIXED_BATCH_BENCH_NAME, |b| {
        let filter = Arc::clone(&filter);
        b.to_async(&rt).iter(|| {
            let filter = Arc::clone(&filter);
            let start = advance_batch(&mut batch_start, batch_ops);

            async move {
                run_check_duplication_batch(filter, start).await;
            }
        });
    });

    group.finish();
}

fn bench_check_duplication_single_thread(c: &mut Criterion) {
    let rt = current_thread_runtime();
    let batch_filter = Arc::new(Mutex::new(build_filter()));
    let per_op_filter = Arc::new(Mutex::new(build_filter()));

    let mut batch_start = 0u64;
    let batch_ops = CHECK_DUPLICATION_BATCH_OPS;
    let mut per_op_start = 0u64;

    let mut group = c.benchmark_group(CHECK_DUPLICATION_SINGLE_THREAD_GROUP_NAME);
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(4));

    group.throughput(Throughput::Elements(batch_ops as u64));
    group.bench_function(FIXED_BATCH_BENCH_NAME, |b| {
        let filter = Arc::clone(&batch_filter);
        b.to_async(&rt).iter(|| {
            let filter = Arc::clone(&filter);
            let start = advance_batch(&mut batch_start, batch_ops as u64);

            async move {
                run_check_duplication_single_batch(filter, start, batch_ops).await;
            }
        });
    });

    group.throughput(Throughput::Elements(1));
    group.bench_function(FIXED_PER_OP_BENCH_NAME, |b| {
        let filter = Arc::clone(&per_op_filter);
        b.to_async(&rt).iter(|| {
            let filter = Arc::clone(&filter);
            let start = advance_batch(&mut per_op_start, 1);

            async move {
                run_check_duplication_single_batch(filter, start, 1).await;
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_issue_token_and_route,
    bench_diagnostics,
    bench_check_duplication_parallel,
    bench_check_duplication_single_thread
);
criterion_main!(benches);
