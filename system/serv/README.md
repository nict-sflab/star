# STAR Service SGX Attestation

## Test

Run all service crates from this directory:

```bash
cargo test --release
```

The app test suite includes a real SGX quote test. It starts an in-process AESM
TCP proxy on `127.0.0.1:5555` when that port is free, forwards requests to
`/run/aesmd/aesm.socket`, obtains a quote for the enclave public key, and verifies
it through the same policy path used by `tools verify`.

This workspace intentionally excludes `rust-sgx-example`; it is only reference
material.

## Start With Attestation

The service attests the enclave public key at startup and prints both values to
stdout:

```text
STAR_ATTESTATION_PUBLIC_KEY_BASE64=<public key>
STAR_ATTESTATION_QUOTE_BASE64=<quote>
```

The quote binds the enclave public key by setting SGX `report_data` to
`public_key || 32 zero bytes`.

Example:

```bash
. /etc/set-environment

AESM_PROXY=127.0.0.1:5555 \
STAR_ENCLAVE_SGXS=/path/to/enclave.sgxs \
cargo run --manifest-path app/Cargo.toml --release --bin star_app \
  > /tmp/star-app-attest.log 2>&1
```

`AESM_PROXY` must point to an AESM TCP proxy reachable from inside the enclave.
The local AESM service itself uses `/run/aesmd/aesm.socket`.

## Inspect A Quote

The verifier reads a raw quote, a base64 quote, or a startup log from stdin:

```bash
cargo run --manifest-path tools/Cargo.toml --release -- inspect \
  < /tmp/star-app-attest.log
```

This prints quote size, attested public key, `MRENCLAVE`, `MRSIGNER`, and the
debug flag.

## Verify A Quote

Use measurements from `inspect`:

```bash
. /etc/set-environment

cargo run --manifest-path tools/Cargo.toml --release -- verify \
  --mrsigner <32-byte hex MRSIGNER> \
  --mrenclave <32-byte hex MRENCLAVE> \
  --public-key <32-byte hex public key> \
  --allow-debug \
  < /tmp/star-app-attest.log
```

For development enclaves, `--allow-debug` is required when the quote debug flag
is true. If Intel DCAP QVL returns an advisory result such as `OUT_OF_DATE`,
verification fails by default; pass `--allow-advisory` only when that is accepted
by the local policy.

The verifier loads QVL from `--qvl`, `SGX_DCAP_QVL`, or
`libsgx_dcap_quoteverify.so`. The quote provider path can be set with `--qpl` or
`SGX_DCAP_QPL`.
