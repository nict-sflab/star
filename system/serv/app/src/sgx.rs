use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use aesm_client::AesmClient;
use enclave_runner::{EnclaveBuilder, Library};
use enclave_runner_sgx::EnclaveBuilder as SgxEnclaveBuilder;
use fast_rust_sgx_caller::Caller;
use sgx_isa::{Attributes, Miscselect, Sigstruct};
use sgxs::loader::{Load, Mapping};
use sgxs::sgxs::SgxsRead;
use sgxs_loaders::{Tcs, isgx::Device as SgxDevice};

use crate::enclave::EnclaveError;

pub(crate) const SGX_TARGET: &str = "x86_64-fortanix-unknown-sgx";
const MAX_PAYLOAD_LEN: usize = 16 * 1024 * 1024;
const STATUS_OK: u64 = 0;
const STATUS_BUFFER_TOO_SMALL: u64 = 5;

struct SharedLibrary(Library);

// The SGX runner captures an Arc<EnclaveState>, but its erased closure does
// not expose Send/Sync. The runtime mutex serializes every enclave entry.
unsafe impl Send for SharedLibrary {}
unsafe impl Sync for SharedLibrary {}

// Retain the official builder's signature and launch-token handling. Both
// callers enter the same mapping, with a separate TCS for provisioning I/O.
struct SplitLoader<'a> {
    device: SgxDevice,
    caller: &'a mut Option<Caller<Tcs>>,
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
            "STAR needs two TCSes: one for provisioning and one for direct calls; rebuild the .sgxs image"
        );
        let provisioning = mapping.tcss.pop().unwrap();
        *self.caller = Some(Caller::new(mapping.tcss.pop().unwrap())?);
        Ok(Mapping {
            info: mapping.info,
            tcss: vec![provisioning],
        })
    }
}

pub(crate) struct LibraryRuntime {
    state: Mutex<LibraryState>,
}

struct LibraryState {
    library: SharedLibrary,
    caller: Caller<Tcs>,
    stopped: bool,
}

impl LibraryRuntime {
    pub(crate) fn load(sgxs: &Path) -> Result<Self, EnclaveError> {
        let device = SgxDevice::new()
            .map_err(EnclaveError::Build)?
            .einittoken_provider(AesmClient::new())
            .build();
        let mut caller = None;
        let mut loader = SplitLoader {
            device,
            caller: &mut caller,
        };
        let library = EnclaveBuilder::new(SgxEnclaveBuilder::new(sgxs))
            .build(&mut loader)
            .map_err(|err| EnclaveError::Library(err.to_string()))?;
        Ok(Self {
            state: Mutex::new(LibraryState {
                library: SharedLibrary(library),
                caller: caller.unwrap(),
                stopped: false,
            }),
        })
    }

    pub(crate) fn call(
        &self,
        op: u64,
        payload: &[u8],
        capacity: usize,
    ) -> Result<Vec<u8>, EnclaveError> {
        self.invoke(op, payload, capacity, false)
    }

    pub(crate) fn call_provisioning(
        &self,
        op: u64,
        payload: &[u8],
        capacity: usize,
    ) -> Result<Vec<u8>, EnclaveError> {
        self.invoke(op, payload, capacity, true)
    }

    fn invoke(
        &self,
        op: u64,
        payload: &[u8],
        capacity: usize,
        provisioning: bool,
    ) -> Result<Vec<u8>, EnclaveError> {
        // There is one request worker per server. This uncontended mutex also
        // makes the public API safe if called from other threads.
        let mut state = self
            .state
            .lock()
            .map_err(|e| EnclaveError::Library(e.to_string()))?;
        if state.stopped {
            return Err(EnclaveError::Library(
                "enclave stopped after a library failure".into(),
            ));
        }
        call_library(op, payload, capacity, |args| {
            // Fail closed on entry errors or panics; never replay a failed entry.
            state.stopped = true;
            let result = if provisioning {
                let library = &state.library;
                // The official runner creates a Tokio runtime. A scoped thread
                // lets provisioning work inside an existing async runtime too.
                std::thread::scope(|scope| {
                    scope
                        .spawn(|| {
                            // Both buffers live until this synchronous entry returns.
                            unsafe { library.0.call(args[0], args[1], args[2], args[3], args[4]) }
                                .map_err(|e| EnclaveError::Library(e.to_string()))
                        })
                        .join()
                        .unwrap_or_else(|_| {
                            Err(EnclaveError::Library("provisioning thread panicked".into()))
                        })
                })
            } else {
                // STAR copies the request, bounds output writes, and retains no
                // host pointers. The mutex excludes all other enclave entries.
                unsafe {
                    state
                        .caller
                        .call(args[0], args[1], args[2], args[3], args[4])
                }
                .map_err(|e| EnclaveError::Library(e.to_string()))
            };
            state.stopped = result.is_err();
            result
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
