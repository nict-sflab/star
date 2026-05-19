# STAR SGX Attestation Tools

This crate verifies remote SGX attestation quotes produced by the service. The
service prints the quote on startup as URL-safe base64:

```text
STAR_ATTESTATION_QUOTE_BASE64=<quote>
```

The quote `report_data` is `public_key || 32 zero bytes`, so verification binds
the enclave measurement to the enclave public key.

## Commands

Inspect measurements and the attested public key:

```bash
cargo run --manifest-path tools/Cargo.toml -- inspect < quote.txt
```

Verify a quote:

```bash
cargo run --manifest-path tools/Cargo.toml -- verify \
  --mrsigner <32-byte hex MRSIGNER> \
  --mrenclave <32-byte hex MRENCLAVE> \
  --public-key <32-byte hex public key> \
  --allow-debug \
  --allow-advisory \
  < quote.txt
```

`--mrenclave` and `--public-key` are optional if signer-only policy and
displaying the attested key are acceptable. `--allow-debug` must only be used for
development enclaves. `--allow-advisory` accepts Intel DCAP QVL results such as
`CONFIG_NEEDED`, `OUT_OF_DATE`, and `SW_HARDENING_NEEDED`; omit it when the
policy requires an up-to-date `OK` result.

The verifier loads Intel DCAP QVL from `--qvl`, `SGX_DCAP_QVL`, or
`libsgx_dcap_quoteverify.so`. The quote provider path can be set with `--qpl` or
`SGX_DCAP_QPL`.
