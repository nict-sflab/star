# star
STAR: Server-side TEE-based Anonymous and Rate-limiting

## Reproducing the SGX app test

Run the release test from the app crate on an Intel SGX machine:

```sh
cd system/serv/app
cargo test --release
```

