# STAR

STAR: Scalable Anonymous Rate-Limiting with a Server-Side TEE

STAR is an experimental implementation of anonymous rate limiting using a server-side TEE.

It allows a service to enforce per-user access limits without linking authentication requests to the same user. 
STAR is designed around two security goals: rate-limiting and unlinkability. 

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

