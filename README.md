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

### Test (requires SGX hardware)

```sh
docker compose -f compose.test.yaml up --build --exit-code-from tests
docker compose -f compose.test.yaml down
```

## Disclaimer

This is a research-grade Proof of Concept (PoC) and does not guarantee complete security when deployed.   
For example, the implementation of a trusted time source, which would be required in a production environment, has been omitted.
