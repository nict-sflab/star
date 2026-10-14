use std::io;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;

use crate::protocol;

#[derive(Debug, thiserror::Error)]
pub enum EnclaveError {
    #[error("enclave build failed to start: {0}")]
    Build(io::Error),
    #[error("enclave build failed: {0}")]
    BuildFailed(String),
    #[error("enclave runner error: {0}")]
    Runner(String),
    #[error("enclave stream error: {0}")]
    Io(#[from] io::Error),
    #[error("enclave remote error: status {status}: {}", String::from_utf8_lossy(.payload))]
    Remote { status: u8, payload: Vec<u8> },
}

pub(crate) const SGX_TARGET: &str = "x86_64-fortanix-unknown-sgx";

pub(crate) struct CommandRuntime {
    child: Mutex<Child>,
}

impl CommandRuntime {
    pub(crate) fn load(sgxs: &Path) -> Result<Self, EnclaveError> {
        let runner =
            std::env::var_os("STAR_KEY_MANAGER_RUNNER").unwrap_or_else(|| "ftxsgx-runner".into());
        Self::spawn(Command::new(runner).arg(sgxs))
    }

    fn spawn(command: &mut Command) -> Result<Self, EnclaveError> {
        let child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(|err| EnclaveError::Runner(format!("start runner: {err}")))?;
        Ok(Self {
            child: Mutex::new(child),
        })
    }

    pub(crate) fn call(&self, op: u8, payload: &[u8]) -> Result<Vec<u8>, EnclaveError> {
        let mut child = self
            .child
            .lock()
            .map_err(|err| EnclaveError::Runner(err.to_string()))?;
        // Hold the lock for the whole exchange so concurrent calls cannot mix frames.
        protocol::write(child.stdin.as_mut().expect("piped stdin"), op, payload)?;
        let (status, payload) = protocol::read(child.stdout.as_mut().expect("piped stdout"))?
            .ok_or_else(|| {
                EnclaveError::Runner("runner closed stdout; see stderr for details".into())
            })?;
        match status {
            protocol::OK => Ok(payload),
            protocol::ERROR => Err(EnclaveError::Remote { status, payload }),
            _ => Err(EnclaveError::Runner(format!(
                "invalid response status: {status}"
            ))),
        }
    }
}

impl Drop for CommandRuntime {
    fn drop(&mut self) {
        let child = self.child.get_mut().unwrap_or_else(|err| err.into_inner());
        // Close the input and reap the child, even if it is stuck in a usercall.
        drop(child.stdin.take());
        let _ = child.kill();
        let _ = child.wait();
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

    let mut candidates = Vec::new();
    if let Some(target_dir) = std::env::var_os("CARGO_TARGET_DIR") {
        candidates.push(PathBuf::from(target_dir));
    }
    candidates.extend(manifest_dir.ancestors().map(|path| path.join("target")));
    let target_dir = candidates
        .into_iter()
        .map(|path| path.join(SGX_TARGET).join("release"))
        .find(|path| path.join(binary_name).is_file())
        .ok_or_else(|| {
            EnclaveError::BuildFailed(format!(
                "cargo built {binary_name}, but the SGX ELF was not found"
            ))
        })?;
    let elf_path = target_dir.join(binary_name);
    let sgxs_path = target_dir.join(format!("{binary_name}.sgxs"));

    run_command(
        Command::new("ftxsgx-elf2sgxs")
            .arg(&elf_path)
            .arg("--debug")
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

