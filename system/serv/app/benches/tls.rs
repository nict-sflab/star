mod common;

use common::scale_out::{OPS_PER_SERVER, SERVER_COUNTS, ScaleOutServers, run_worker};
use common::tls::{self, CertificateKind, MemoryStream};
use criterion::{Criterion, Throughput, black_box, criterion_group};
use openssl::ssl::{
    ErrorCode, HandshakeError, MidHandshakeSslStream, SslAcceptor, SslMethod, SslVersion,
};
use std::time::{Duration, Instant};

const BENCH_CASE_NAME: &str = "openssl_tls13_first_flight_to_server_finished";

fn expected_client_finished_wait(handshake: &MidHandshakeSslStream<MemoryStream<&[u8]>>) -> bool {
    handshake.error().code() == ErrorCode::WANT_READ
        && handshake.get_ref().consumed_all()
        && !handshake.get_ref().written().is_empty()
        && handshake.ssl().version_str() == "TLSv1.3"
        && !handshake.ssl().session_reused()
}

struct FirstFlightHarness {
    acceptor: SslAcceptor,
    client_hello: Vec<u8>,
}

impl FirstFlightHarness {
    fn new() -> Self {
        let mut acceptor =
            SslAcceptor::mozilla_intermediate(SslMethod::tls()).expect("build TLS acceptor");
        tls::configure_server(&mut acceptor, CertificateKind::Chain);
        acceptor
            .set_min_proto_version(Some(SslVersion::TLS1_3))
            .expect("require TLS 1.3");
        acceptor
            .set_max_proto_version(Some(SslVersion::TLS1_3))
            .expect("limit TLS to 1.3");
        acceptor.set_num_tickets(0).expect("disable session tickets");
        let acceptor = acceptor.build();

        let mut connector = tls::client_connector();
        connector
            .set_min_proto_version(Some(SslVersion::TLS1_3))
            .expect("require client TLS 1.3");
        connector
            .set_max_proto_version(Some(SslVersion::TLS1_3))
            .expect("limit client TLS to 1.3");

        // The client is used only for setup and validation, never during measurement.
        let (client_hello, mut client) = tls::prepare_client_hello(connector.build());
        let server = match acceptor.accept(MemoryStream::recording(client_hello.as_slice())) {
            Err(HandshakeError::WouldBlock(handshake)) => handshake,
            _ => panic!("server must wait for the client's Finished"),
        };
        assert!(
            expected_client_finished_wait(&server),
            "server did not reach the expected TLS 1.3 WANT_READ state"
        );
        // The recorded first flight must be enough for the client to verify the
        // server certificate and reach its own Finished (not merely ServerHello/HRR).
        client.get_mut().set_input(server.get_ref().written().to_vec());
        let completed_client = client
            .handshake()
            .expect("server first flight must complete the client's TLS 1.3 handshake");
        assert_eq!(completed_client.ssl().version_str(), "TLSv1.3");
        assert!(!completed_client.ssl().session_reused(), "TLS resumption detected");
        assert!(
            completed_client.ssl().peer_certificate().is_some(),
            "certificate authentication was skipped"
        );
        assert!(
            completed_client.get_ref().written().len() > client_hello.len(),
            "client did not produce its Finished message"
        );

        Self { acceptor, client_hello }
    }

    fn generates_server_first_flight(&self) -> bool {
        // Each accept() creates a fresh server SSL session.
        match self.acceptor.accept(MemoryStream::recording(black_box(self.client_hello.as_slice()))) {
            Err(HandshakeError::WouldBlock(handshake)) => expected_client_finished_wait(&handshake),
            Ok(_) | Err(HandshakeError::Failure(_)) => false,
            Err(HandshakeError::SetupFailure(error)) => {
                panic!("TLS server setup failed: {error}")
            }
        }
    }

    fn measure(&self, iters: u64) -> Duration {
        let mut total = Duration::ZERO;
        for _ in 0..iters {
            let start = Instant::now();
            let completed = self.generates_server_first_flight();
            total += start.elapsed();
            assert!(completed, "unexpected TLS server first-flight outcome");
        }
        total
    }
}

fn run_scale_out_worker(core_id: usize) {
    run_worker(core_id, FirstFlightHarness::new, |server, operations| {
        for _ in 0..operations {
            assert!(server.generates_server_first_flight());
        }
    });
}

fn bench_tls_server_first_flight(c: &mut Criterion) {
    let mut harness = None;
    let mut group = c.benchmark_group("tls13_server_first_flight");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(4));
    group.throughput(Throughput::Elements(1));
    group.bench_function(BENCH_CASE_NAME, |b| {
        let server = harness.get_or_insert_with(FirstFlightHarness::new);
        b.iter_custom(|iters| server.measure(iters));
    });
    group.finish();

    let mut group = c.benchmark_group("tls13_server_first_flight_scale_out_throughput");
    group.sample_size(20);
    group.warm_up_time(Duration::from_secs(1));
    group.measurement_time(Duration::from_secs(4));
    for &server_count in SERVER_COUNTS {
        let mut servers = None;
        group.throughput(Throughput::Elements(u64::from(server_count) * OPS_PER_SERVER));
        group.bench_function(format!("{BENCH_CASE_NAME}_{server_count}_servers"), |b| {
            let servers = servers.get_or_insert_with(|| {
                ScaleOutServers::new(server_count, |_, _, core| vec![core.to_string()])
            });
            b.iter_custom(|batches| servers.measure(batches));
        });
        if let Some(servers) = servers {
            servers.finish();
        }
    }
    group.finish();
}

criterion_group!(benches, bench_tls_server_first_flight);

fn main() {
    let mut args = std::env::args();
    if args.nth(1).as_deref() == Some("--scale-out-worker") {
        let core_id = args.next().expect("server CPU core").parse().expect("CPU core");
        run_scale_out_worker(core_id);
        return;
    }
    benches();
    Criterion::default().configure_from_args().final_summary();
}
