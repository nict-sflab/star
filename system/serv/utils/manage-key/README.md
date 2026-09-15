# Key manager

The host service exposes the existing HTTP API and persists the sealed key bundle.
The service starts `ftxsgx-runner <image.sgxs>` as a child process and exchanges
length-delimited binary messages over stdin/stdout. The enclave's `main()` reads
requests and writes responses. Stdout is reserved for this protocol; runner
diagnostics go to the inherited stderr. There is no custom stream router.
The runner stays alive across requests and is terminated and reaped when its
`KeyManagerRuntime` is dropped.

Key generation, unsealing, recipient verification, and key export stay inside
SGX. The host only handles sealed keys, public attestation data, and encrypted
exports. `KeyManagerRuntime` remains available to callers and benchmarks.

## Running

With the SGX toolchain, device and AESM proxy configured, and `ftxsgx-runner`
(from `fortanix-sgx-tools`) available on `PATH`:

```sh
cargo run --release --manifest-path service/Cargo.toml
```

Unless `STAR_KEY_MANAGER_SGXS` points to an existing image, the service builds the
enclave and converts it using `ftxsgx-elf2sgxs` without `--library`.
The existing `STAR_KEY_MANAGER_*` settings still apply.
Set `STAR_KEY_MANAGER_RUNNER` to use a runner executable at a different path.
This setting is an executable path, not a shell command. The runner receives the
SGXS path as its only argument and must forward stdin/stdout to the enclave.
See the [Fortanix deployment guide](https://edp.fortanix.com/docs/tasks/deployment/)
for the standard runner.

## Updating from the library enclave

Rebuild the key-manager SGXS image; an old `--library` image is incompatible.
For `system/serv/integration/run-all.sh`, set `STAR_README_REBUILD=1` to rebuild
cached artifacts.

The executable has a different MRENCLAVE measurement. Existing key bundles are
sealed to MRENCLAVE and cannot be unsealed by the new image. Retain the old image
and sealed bundle if you need those keys. For a fresh development instance, set
`STAR_KEY_MANAGER_SEALED_KEYS` to a new path; this creates new keys. Update any
pinned key-manager measurement in clients when deploying the new image.
