//! The same worker synchronization and wall-clock throughput measurement for TLS and STAR.

use std::io::{self, Read, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::time::{Duration, Instant};

pub const SERVER_COUNTS: &[u32] = &[1, 2, 4, 8, 16];
pub const OPS_PER_SERVER: u64 = 64;

struct ServerProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: ChildStdout,
}

pub struct ScaleOutServers {
    servers: Vec<ServerProcess>,
}

impl ScaleOutServers {
    pub fn new(server_count: u32, worker_args: impl Fn(usize, u32, usize) -> Vec<String>) -> Self {
        let cores = core_affinity::get_core_ids().expect("get benchmark CPU cores");
        assert!(
            cores.len() >= server_count as usize,
            "scale-out benchmark requires at least {server_count} CPU cores"
        );
        let executable = std::env::current_exe().expect("get benchmark executable");
        let servers = cores
            .into_iter()
            .take(server_count as usize)
            .enumerate()
            .map(|(index, core)| {
                let mut child = Command::new(&executable)
                    .arg("--scale-out-worker")
                    .args(worker_args(index, server_count, core.id))
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .expect("spawn scale-out benchmark server");
                let mut server = ServerProcess {
                    stdin: child.stdin.take().expect("server stdin"),
                    stdout: child.stdout.take().expect("server stdout"),
                    child,
                };
                let mut ready = [0];
                server.stdout.read_exact(&mut ready).expect("server setup");
                assert_eq!(ready, [b'R']);
                server
            })
            .collect::<Vec<_>>();
        Self { servers }
    }

    pub fn measure(&mut self, batches: u64) -> Duration {
        let operations = batches
            .checked_mul(OPS_PER_SERVER)
            .expect("scale-out benchmark operation count overflow");
        for server in &mut self.servers {
            server
                .stdin
                .write_all(&operations.to_le_bytes())
                .expect("send server operation count");
        }
        for server in &mut self.servers {
            let mut ready = [0];
            server.stdout.read_exact(&mut ready).expect("server ready");
            assert_eq!(ready, [b'R']);
        }
        // Include start commands and all worker completions, just like STAR's original benchmark.
        let timer = Instant::now();
        for server in &mut self.servers {
            server.stdin.write_all(&[b'S']).expect("start server");
        }
        for server in &mut self.servers {
            let mut done = [0];
            server.stdout.read_exact(&mut done).expect("server result");
            assert_eq!(done, [b'D']);
        }
        timer.elapsed()
    }

    pub fn finish(self) {
        for mut server in self.servers {
            drop(server.stdin);
            assert!(server.child.wait().expect("server exit").success());
        }
    }
}

/// Pins the worker before setup, then processes only after the parent's start signal.
pub fn run_worker<S>(
    core_id: usize,
    setup: impl FnOnce() -> S,
    mut process: impl FnMut(&S, u64),
) {
    assert!(
        core_affinity::set_for_current(core_affinity::CoreId { id: core_id }),
        "pin scale-out benchmark server"
    );
    let server = setup();
    let input = io::stdin();
    let output = io::stdout();
    let mut stdin = input.lock();
    let mut stdout = output.lock();
    stdout.write_all(&[b'R']).expect("signal server setup");
    stdout.flush().expect("flush server setup");
    loop {
        let mut count = [0; 8];
        match stdin.read_exact(&mut count) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(error) => panic!("read server operation count: {error}"),
        }
        let operations = u64::from_le_bytes(count);
        stdout.write_all(&[b'R']).expect("signal server ready");
        stdout.flush().expect("flush server ready");
        let mut start = [0];
        stdin.read_exact(&mut start).expect("wait for server start");
        assert_eq!(start, [b'S']);
        process(&server, operations);
        stdout.write_all(&[b'D']).expect("signal server completion");
        stdout.flush().expect("flush server completion");
    }
}
