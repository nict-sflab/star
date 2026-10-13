# STAR
STAR: Scalable Anonymous Rate-Limiting with a Server-Side TEE

STAR is an experimental implementation of anonymous rate limiting using a server-side TEE.

It allows a service to enforce per-user access limits without linking authentication requests to the same user. 
STAR is designed around two security goals: rate-limiting and unlinkability. 

[![Benchmark](https://github.com/nict-sflab/star/actions/workflows/auto-bench.yaml/badge.svg?branch=main)](https://github.com/nict-sflab/star/actions/workflows/auto-bench.yaml)

## What is included

| Path | Role |
| --- | --- |
| [`core/`](core/) | Core Library of the STAR |
| [`core/analysis/`](core/analysis/) | Side-channel checks / Timecop-style analysis |
| [`system/client/`](system/client/) | Client-side Proxy |
| [`system/serv/`](system/serv/) | Rate-Limiting Service |
| [`system/serv/app/`](system/serv/app/) | Filtering Proxy / Registration API |
| [`system/serv/app/examples/`](system/serv/app/examples/) | Web App Server and local demo |
| [`system/serv/enclave/`](system/serv/enclave/) | Auth SGX Enclave |


## Start here

### Deploy

```sh
docker compose up
```

### Benchmark

```sh
docker compose -f compose.bench.yaml up --build core
docker compose -f compose.bench.yaml up --build serv
docker compose -f compose.bench.yaml up --build sidechannel
docker compose -f compose.bench.yaml down --rmi all --remove-orphans
```

## HPKE and release builds

The secure channel and key-manager provisioning use **X25519 + HKDF-SHA256 + ChaCha20-Poly1305**. The AEAD is selected in Rust code, not through a Cargo profile.

The independent Cargo roots (`core/`, `system/client/`, and `system/serv/`) each configure release builds with `opt-level = 3`, `codegen-units = 1`, `lto = "thin"`, and `incremental = false`. This also applies to enclave images built with `cargo build --release --target x86_64-fortanix-unknown-sgx` under `system/serv/`.

SGX-specific CPU feature flags remain in `.cargo/config.toml`. Do not add `-C target-cpu=native` globally: host and SGX builds may need to run on different processors.
