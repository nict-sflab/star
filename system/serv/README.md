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

The service starts a separate `key-manager` enclave. The key-manager owns the
STAR credential key and HPKE receiver key, attests the service public key, and
only exports the key bundle to serving enclaves after checking their remote
attestation evidence inside the key-manager enclave. Each serving enclave checks
the key-manager evidence inside the serving enclave before accepting the
encrypted key bundle.

Trusted peer policy is evaluated inside the enclaves, not by the host. Key
sharing pins exact peer `MRENCLAVE` values; `MRSIGNER` is not accepted as the
peer trust boundary. Set these variables before building the `.sgxs` files so
the values are embedded in the enclave binaries:

```bash
export STAR_TRUSTED_KEY_MANAGER_MRENCLAVE=<32-byte hex MRENCLAVE>
export STAR_TRUSTED_ENCLAVE_MRENCLAVE=<32-byte hex MRENCLAVE>

# Development enclaves only.
export STAR_ALLOW_DEBUG_KEY_MANAGER=1
export STAR_ALLOW_DEBUG_ENCLAVE=1
```

The host prints the key-manager and serving-enclave remote attestation quotes to
stdout before key transfer, plus the QvE evidence used for enclave-to-enclave
key sharing:

```text
STAR_ATTESTATION_PUBLIC_KEY_BASE64=<public key>
STAR_ATTESTATION_QUOTE_BASE64=<key-manager quote>
STAR_KEY_MANAGER_QUOTE_BASE64=<key-manager quote>
STAR_FILTER_ENCLAVE_TRANSFER_QUOTE_BASE64=<serving enclave transfer quote>
STAR_ISSUE_ENCLAVE_TRANSFER_QUOTE_BASE64=<serving enclave transfer quote>
STAR_FILTER_ENCLAVE_QVE_EVIDENCE_FOR_KEY_MANAGER_BASE64=<QvE evidence>
STAR_ISSUE_ENCLAVE_QVE_EVIDENCE_FOR_KEY_MANAGER_BASE64=<QvE evidence>
STAR_KEY_MANAGER_QVE_EVIDENCE_FOR_FILTER_ENCLAVE_BASE64=<QvE evidence>
STAR_KEY_MANAGER_QVE_EVIDENCE_FOR_ISSUE_ENCLAVE_BASE64=<QvE evidence>
```

The key-manager quote binds the service public key by setting SGX `report_data`
to `public_key || 32 zero bytes`. Serving enclave transfer quotes bind their
ephemeral transfer public keys the same way. The host carries quotes and
ciphertexts, but the policy decision is made in enclave code. For key sharing,
the host asks Intel DCAP QVL/QvE to produce QvE evidence targeted to the verifier
enclave. The verifier enclave checks the QvE report MAC, QvE report-data hash,
QvE identity, DCAP result, collateral freshness, exact peer `MRENCLAVE`, debug
policy, and `report_data` public-key binding before using the attested HPKE key.

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
