FROM rust:bookworm

RUN apt-get update && apt-get install -y --no-install-recommends \
    clang cmake curl libssl-dev openssl pkg-config protobuf-compiler zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-fortanix-unknown-sgx \
    && cargo install fortanix-sgx-tools --locked

WORKDIR /workspace
COPY star /workspace/star
COPY constant-time-utils /workspace/constant-time-utils
COPY timecop-rs /workspace/timecop-rs
COPY simple-filter-box /workspace/simple-filter-box
COPY fast-rust-sgx-caller /workspace/fast-rust-sgx-caller

WORKDIR /workspace/star/system/serv
ENV STAR_KEY_MANAGER_ADDR=127.0.0.1:8090 \
    STAR_KEY_MANAGER_URL=http://127.0.0.1:8090 \
    STAR_API_ADDR=0.0.0.0:18080 \
    STAR_PROXY_ADDR=0.0.0.0:18081 \
    PCCS_URL=https://pccs.phala.network \
    STAR_ALLOW_DEBUG_ENCLAVES=1 \
    STAR_ALLOW_ADVISORY_ENCLAVES=1 \
    STAR_TRUST_SAME_SIGNER_ENCLAVES=1 \
    TLS_CERT_PATH=/workspace/star/system/serv/app/examples/certs/server.crt \
    TLS_KEY_PATH=/workspace/star/system/serv/app/examples/certs/server.key

RUN cargo build --release --manifest-path utils/manage-key/service/Cargo.toml \
    && cargo build --release --manifest-path app/Cargo.toml --example main \
    && cd app/examples && bash ./keygen.sh

CMD ["bash", "-c", "./target/release/key-manager-service & until curl -fs http://127.0.0.1:8090/star/key-manager/public_key >/dev/null; do sleep 1; done; exec ./target/release/examples/main"]
