use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rand::{rngs::OsRng, RngCore};
use star_core::client::User;
use star_core::secure_channel::client::SecureChannelClient;
use star_core::serv::enclave::Enclave;
use std::sync::Arc;
use std::time::Instant;

const PERIOD: u64 = 1;
const MAX_TOKENS_PER_CREDENTIAL: u32 = 3;

struct BenchFixture {
    enclave: Enclave,
    envelope: Vec<u8>,
    current_period: [u8; 8],
}

fn build_fixture() -> BenchFixture {
    let mut rng = OsRng;

    let mut token_sk = [0u8; 32];
    let mut hpke_sk = [0u8; 32];
    rng.fill_bytes(&mut token_sk);
    rng.fill_bytes(&mut hpke_sk);

    let enclave = Enclave::new(MAX_TOKENS_PER_CREDENTIAL, token_sk, hpke_sk)
        .expect("failed to initialize enclave");

    let sender =
        SecureChannelClient::new(enclave.public_key()).expect("failed to initialize sender");
    let mut user = User::new(MAX_TOKENS_PER_CREDENTIAL as u64, sender);

    let (cipher, ck) = user.request_credential(&mut rng);
    let credential_bytes = enclave.issue_cred(&cipher, &mut rng);
    user.receive_credential(&credential_bytes, &ck);

    let envelope = user
        .request_auth(0, PERIOD, &mut rng)
        .expect("failed to build token request");

    BenchFixture {
        enclave,
        envelope,
        current_period: PERIOD.to_be_bytes(),
    }
}

fn bench_issue_token_throughput(c: &mut Criterion) {
    let fixture = build_fixture();
    let fixture_parallel = Arc::new(build_fixture());

    let mut group = c.benchmark_group("enclave.issue_token");
    group.throughput(Throughput::Elements(1));
    group.bench_function("single_request", |b| {
        b.iter(|| {
            let token = fixture
                .enclave
                .issue_token(
                    black_box(fixture.envelope.as_slice()),
                    black_box(fixture.current_period),
                )
                .expect("token issuance failed");
            black_box(token);
        });
    });
    group.bench_function("4_threads_parallel", |b| {
        let fixture_parallel = Arc::clone(&fixture_parallel);
        b.iter_custom(|iters| {
            const THREADS: u64 = 4;

            let per_thread = iters / THREADS;
            let remainder = iters % THREADS;
            let start = Instant::now();

            std::thread::scope(|scope| {
                let mut handles = Vec::with_capacity(THREADS as usize);

                for idx in 0..THREADS {
                    let runs = per_thread + u64::from(idx < remainder);
                    let fixture = Arc::clone(&fixture_parallel);
                    handles.push(scope.spawn(move || {
                        for _ in 0..runs {
                            let token = fixture
                                .enclave
                                .issue_token(
                                    black_box(fixture.envelope.as_slice()),
                                    black_box(fixture.current_period),
                                )
                                .expect("token issuance failed");
                            black_box(token);
                        }
                    }));
                }

                for handle in handles {
                    handle.join().expect("worker thread panicked");
                }
            });

            start.elapsed()
        });
    });
    group.finish();
}

criterion_group!(benches, bench_issue_token_throughput);
criterion_main!(benches);
