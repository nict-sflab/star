use std::{sync::Arc, time::Duration};

use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use rand_core::OsRng;
use simple_filter_box::{core::Entry, dashmap::DashMapFilterBox};
use star_app::{
    enclave::{EnclaveRuntime, EnclaveRuntimes},
    filter::StarFilter,
};
use star_core::{client::User, secure_channel::client::SecureChannelClient};
use tokio::{runtime::Builder, sync::Mutex, task::JoinSet};

type HashMapStarFilter = StarFilter<DashMapFilterBox>;
type SharedStarFilter = Arc<Mutex<HashMapStarFilter>>;

const MAX_COUNT: u32 = 1_000_000;
const PARALLEL_OPS_PER_WORKER: usize = 64;
const FIXED_PREFIX_MAX: u32 = 4;
const FIXED_CONCURRENCY: usize = FIXED_PREFIX_MAX as usize;

fn fixed_prefixes() -> Vec<u32> {
    (0..FIXED_PREFIX_MAX).collect()
}

fn build_filter() -> HashMapStarFilter {
    let filter_box = DashMapFilterBox::new();
    let enclave = EnclaveRuntimes::from_env(10).expect("filter enclave pool");

    StarFilter::new(filter_box, fixed_prefixes(), FIXED_PREFIX_MAX, enclave)
}

fn build_route_only_filter() -> HashMapStarFilter {
    build_filter()
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

fn token_from_counter(counter: u64) -> Entry {
    let mut token = [0u8; 32];
    token[0..8].copy_from_slice(&counter.to_be_bytes());
    token
}

fn bench_issue_token_and_route(c: &mut Criterion) {
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let filter = Arc::new(Mutex::new(build_route_only_filter()));
    let request = Arc::new(build_request());

    let mut group = c.benchmark_group("star_filter_issue_token_and_route");
    group.throughput(Throughput::Elements(1));
    group.bench_function("fixed_per_op", |b| {
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

async fn run_check_duplication_batch(filter: SharedStarFilter, batch_start: u64) {
    let mut joins = JoinSet::new();

    for worker in 0..FIXED_CONCURRENCY {
        let filter = Arc::clone(&filter);
        let worker_start = batch_start + (worker * PARALLEL_OPS_PER_WORKER) as u64;
        joins.spawn(async move {
            for offset in 0..PARALLEL_OPS_PER_WORKER {
                let idx = worker_start + offset as u64;
                let token = token_from_counter(idx);
                let route = (idx as u32) % FIXED_PREFIX_MAX;
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
        let route = (idx as u32) % FIXED_PREFIX_MAX;
        let result = filter
            .lock()
            .await
            .check_duplication(black_box(&token), black_box(route))
            .await;
        let _ = black_box(result);
    }
}

fn bench_check_duplication_parallel(c: &mut Criterion) {
    let rt = Builder::new_multi_thread()
        .worker_threads(FIXED_CONCURRENCY)
        .enable_all()
        .build()
        .expect("tokio runtime");
    let filter = Arc::new(Mutex::new(build_filter()));

    let mut batch_start = 0u64;
    let batch_ops = (FIXED_CONCURRENCY * PARALLEL_OPS_PER_WORKER) as u64;

    let mut group = c.benchmark_group("star_filter_check_duplication_parallel");
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(4));
    group.throughput(Throughput::Elements(batch_ops));

    group.bench_function("fixed_batch_256_ops_per_iter", |b| {
        let filter = Arc::clone(&filter);
        b.to_async(&rt).iter(|| {
            let filter = Arc::clone(&filter);
            let start = batch_start;
            batch_start = batch_start.wrapping_add(batch_ops);

            async move {
                run_check_duplication_batch(filter, start).await;
            }
        });
    });

    group.finish();
}

fn bench_check_duplication_single_thread(c: &mut Criterion) {
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let batch_filter = Arc::new(Mutex::new(build_filter()));
    let per_op_filter = Arc::new(Mutex::new(build_filter()));

    let mut batch_start = 0u64;
    let batch_ops = FIXED_CONCURRENCY * PARALLEL_OPS_PER_WORKER;
    let mut per_op_start = 0u64;

    let mut group = c.benchmark_group("star_filter_check_duplication_single_thread");
    group.sample_size(40);
    group.measurement_time(Duration::from_secs(4));

    group.throughput(Throughput::Elements(batch_ops as u64));
    group.bench_function("fixed_batch_256_ops_per_iter", |b| {
        let filter = Arc::clone(&batch_filter);
        b.to_async(&rt).iter(|| {
            let filter = Arc::clone(&filter);
            let start = batch_start;
            batch_start = batch_start.wrapping_add(batch_ops as u64);

            async move {
                run_check_duplication_single_batch(filter, start, batch_ops).await;
            }
        });
    });

    group.throughput(Throughput::Elements(1));
    group.bench_function("fixed_per_op", |b| {
        let filter = Arc::clone(&per_op_filter);
        b.to_async(&rt).iter(|| {
            let filter = Arc::clone(&filter);
            let start = per_op_start;
            per_op_start = per_op_start.wrapping_add(1);

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
    bench_check_duplication_parallel,
    bench_check_duplication_single_thread
);
criterion_main!(benches);
