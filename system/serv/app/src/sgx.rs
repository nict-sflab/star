use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Condvar, Mutex, RwLock};

use aesm_client::AesmClient;
use enclave_runner::{EnclaveBuilder, Library};
use enclave_runner_sgx::EnclaveBuilder as SgxEnclaveBuilder;
use fast_rust_sgx_caller::{Caller, Usercall, default_usercall};
use sgx_isa::{Attributes, Miscselect, Sigstruct};
use sgxs::loader::{Load, Mapping, Tcs as _};
use sgxs::sgxs::SgxsRead;
use sgxs_loaders::{Tcs, isgx::Device as SgxDevice};

use crate::enclave::EnclaveError;

mod events;
use events::Events;

pub(crate) const SGX_TARGET: &str = "x86_64-fortanix-unknown-sgx";

const MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;
const STATUS_OK: u64 = 0;
const STATUS_BUFFER_TOO_SMALL: u64 = 5;

struct SharedLibrary(Library);

// The official SGX library captures an Arc<EnclaveState>; calls are additionally
// serialized by `gate`. Library's erased closure does not expose Send/Sync.
unsafe impl Send for SharedLibrary {}
unsafe impl Sync for SharedLibrary {}

// Keep the official builder's signature, attributes and launch-token handling.
// Give it one TCS for provisioning/I/O and retain the others for direct entry.
struct SplitLoader<'a> {
    device: SgxDevice,
    callers: &'a mut Vec<(usize, Caller<Tcs>)>,
    worker_count: usize,
}

impl Load for SplitLoader<'_> {
    type MappingInfo = <SgxDevice as Load>::MappingInfo;
    type Tcs = Tcs;

    fn load<R: SgxsRead>(
        &mut self,
        reader: &mut R,
        signature: &Sigstruct,
        attributes: Attributes,
        miscselect: Miscselect,
    ) -> anyhow::Result<Mapping<Self>> {
        let mut mapping = self
            .device
            .load(reader, signature, attributes, miscselect)?;
        anyhow::ensure!(
            mapping.tcss.len() >= 2,
            "STAR needs at least two TCSes: one for provisioning and one for direct calls; rebuild the .sgxs image"
        );
        let provisioning = mapping.tcss.pop().unwrap();
        for tcs in mapping.tcss.into_iter().take(self.worker_count.max(1)) {
            self.callers
                .push((tcs.address() as usize, Caller::new(tcs)?));
        }
        Ok(Mapping {
            info: mapping.info,
            tcss: vec![provisioning],
        })
    }
}

pub(crate) struct LibraryRuntime {
    library: SharedLibrary,
    // Provisioning must not overlap direct calls: its runner has separate
    // usercall services, and its writes must not contend with enclave readers.
    gate: RwLock<()>,
    callers: Mutex<Vec<(usize, Caller<Tcs>)>>,
    available: Condvar,
    stopped: AtomicBool,
    events: Events,
}

// Return a TCS even when response decoding fails. If entry fails or unwinds,
// stop the entire enclave and wake both pool waiters and usercall waiters.
struct CallerLease<'a> {
    runtime: &'a LibraryRuntime,
    caller: Option<(usize, Caller<Tcs>)>,
    healthy: bool,
}

impl Drop for CallerLease<'_> {
    fn drop(&mut self) {
        if !self.healthy {
            self.runtime.stop();
        }
        let mut callers = self
            .runtime
            .callers
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        callers.push(self.caller.take().unwrap());
        self.runtime.available.notify_one();
    }
}

impl LibraryRuntime {
    pub(crate) fn load(sgxs: &Path, worker_count: usize) -> Result<Self, EnclaveError> {
        let device = SgxDevice::new()
            .map_err(EnclaveError::Build)?
            .einittoken_provider(AesmClient::new())
            .build();
        let mut callers = Vec::new();
        let mut loader = SplitLoader {
            device,
            callers: &mut callers,
            worker_count,
        };
        let library = EnclaveBuilder::new(SgxEnclaveBuilder::new(sgxs))
            .build(&mut loader)
            .map_err(|err| EnclaveError::Library(err.to_string()))?;
        let events = Events::new(callers.iter().map(|(address, _)| *address));
        Ok(Self {
            library: SharedLibrary(library),
            gate: RwLock::new(()),
            callers: Mutex::new(callers),
            available: Condvar::new(),
            stopped: AtomicBool::new(false),
            events,
        })
    }

    fn stop(&self) {
        // Hold the pool mutex to avoid losing a notification between checking
        // stopped and sleeping on available.
        let _callers = self.callers.lock().unwrap_or_else(|e| e.into_inner());
        self.stopped.store(true, Ordering::Release);
        self.available.notify_all();
        self.events.stop();
    }

    fn check_running(&self) -> Result<(), EnclaveError> {
        if self.stopped.load(Ordering::Acquire) {
            Err(EnclaveError::Library(
                "enclave stopped after a library failure".into(),
            ))
        } else {
            Ok(())
        }
    }

    pub(crate) fn call(
        &self,
        op: u64,
        payload: &[u8],
        initial_capacity: usize,
    ) -> Result<Vec<u8>, EnclaveError> {
        let _gate = self
            .gate
            .read()
            .map_err(|e| EnclaveError::Library(e.to_string()))?;
        let mut callers = self
            .callers
            .lock()
            .map_err(|e| EnclaveError::Library(e.to_string()))?;
        let caller = loop {
            self.check_running()?;
            if let Some(caller) = callers.pop() {
                break caller;
            }
            callers = self
                .available
                .wait(callers)
                .map_err(|e| EnclaveError::Library(e.to_string()))?;
        };
        drop(callers);
        let mut lease = CallerLease {
            runtime: self,
            caller: Some(caller),
            healthy: true,
        };
        call_library(op, payload, initial_capacity, |args| {
            self.check_running()?;
            lease.healthy = false;
            let (address, caller) = lease.caller.as_mut().unwrap();
            // STAR's entry copies the request, writes only within the supplied
            // output, and retains no host pointers. Both buffers live through
            // this synchronous call. Never replay an entry after a usercall error.
            let result = unsafe {
                caller.call_with(args, |number, args| {
                    if self.stopped.load(Ordering::Acquire) {
                        return Err(fast_rust_sgx_caller::Error::Stopped);
                    }
                    if number == Usercall::wait as u64 {
                        self.events.wait(*address, args[0], args[1])
                    } else if number == Usercall::send as u64 {
                        self.events.send(args[0], args[1] as usize)
                    } else {
                        default_usercall(number, args)
                    }
                })
            };
            lease.healthy = result.is_ok();
            result.map_err(|e| EnclaveError::Library(e.to_string()))
        })
    }

    pub(crate) fn call_provisioning(
        &self,
        op: u64,
        payload: &[u8],
        initial_capacity: usize,
    ) -> Result<Vec<u8>, EnclaveError> {
        let _gate = self
            .gate
            .write()
            .map_err(|e| EnclaveError::Library(e.to_string()))?;
        self.check_running()?;
        // The official runner creates a Tokio runtime. Run provisioning on a
        // scoped OS thread so this also works inside the app's async runtime.
        std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    call_library(op, payload, initial_capacity, |args| {
                        // The gate excludes all other calls and the buffers stay
                        // alive until this scoped thread has joined.
                        unsafe {
                            self.library
                                .0
                                .call(args[0], args[1], args[2], args[3], args[4])
                        }
                        .map_err(|err| {
                            self.stop();
                            EnclaveError::Library(err.to_string())
                        })
                    })
                })
                .join()
                .unwrap_or_else(|_| {
                    self.stop();
                    Err(EnclaveError::Library("provisioning thread panicked".into()))
                })
        })
    }
}

pub(crate) fn build_sgxs(
    manifest_path: &Path,
    binary_name: &str,
    heap_size: String,
    stack_size: String,
    threads: String,
) -> Result<PathBuf, EnclaveError> {
    let manifest_dir = manifest_path.parent().ok_or_else(|| {
        EnclaveError::BuildFailed(format!(
            "missing parent directory for {}",
            manifest_path.display()
        ))
    })?;

    run_command(
        Command::new("cargo")
            .current_dir(manifest_dir)
            .arg("build")
            .arg("--quiet")
            .arg("--release")
            .arg("--target")
            .arg(SGX_TARGET)
            .arg("--manifest-path")
            .arg(manifest_path),
    )?;

    let target_dir = release_target_dir(manifest_dir, binary_name)?;
    let elf_path = target_dir.join(binary_name);
    let sgxs_path = target_dir.join(format!("{binary_name}.sgxs"));

    run_command(
        Command::new("ftxsgx-elf2sgxs")
            .arg(&elf_path)
            .arg("--library")
            //.arg("--debug")
            .arg("--heap-size")
            .arg(heap_size)
            .arg("--stack-size")
            .arg(stack_size)
            .arg("--threads")
            .arg(threads)
            .arg("--output")
            .arg(&sgxs_path),
    )?;

    Ok(sgxs_path)
}

fn release_target_dir(manifest_dir: &Path, binary_name: &str) -> Result<PathBuf, EnclaveError> {
    let mut candidates = Vec::new();

    if let Some(target_dir) = std::env::var_os("CARGO_TARGET_DIR") {
        candidates.push(PathBuf::from(target_dir));
    }

    let mut current = Some(manifest_dir);
    while let Some(path) = current {
        candidates.push(path.join("target"));
        current = path.parent();
    }

    for candidate in candidates {
        let target_dir = candidate.join(SGX_TARGET).join("release");
        if target_dir.join(binary_name).is_file() {
            return Ok(target_dir);
        }
    }

    Err(EnclaveError::BuildFailed(format!(
        "cargo built {binary_name}, but the SGX ELF was not found"
    )))
}

fn run_command(command: &mut Command) -> Result<(), EnclaveError> {
    let output = command.output().map_err(EnclaveError::Build)?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    Err(EnclaveError::BuildFailed(format!(
        "{}\nstdout:\n{}\nstderr:\n{}",
        output.status, stdout, stderr
    )))
}

fn call_library(
    op: u64,
    payload: &[u8],
    initial_capacity: usize,
    mut call: impl FnMut([u64; 5]) -> Result<(u64, u64), EnclaveError>,
) -> Result<Vec<u8>, EnclaveError> {
    let mut output = vec![0u8; initial_capacity];

    loop {
        let (status, len) = call([
            op,
            payload.as_ptr() as u64,
            payload.len() as u64,
            output.as_mut_ptr() as u64,
            output.len() as u64,
        ])?;

        let len =
            usize::try_from(len).map_err(|_| EnclaveError::ResponseTooLarge { len: usize::MAX })?;

        if status == STATUS_BUFFER_TOO_SMALL {
            if len <= output.len() || len > MAX_PAYLOAD_LEN {
                return Err(EnclaveError::ResponseTooLarge { len });
            }

            output.resize(len, 0);
            continue;
        }

        if len > output.len() {
            return Err(EnclaveError::ResponseTooLarge { len });
        }

        output.truncate(len);

        if status == STATUS_OK {
            return Ok(output);
        }

        return Err(EnclaveError::Remote {
            status,
            payload: output,
        });
    }
}

#[cfg(test)]
mod tests;
