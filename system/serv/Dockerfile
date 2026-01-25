FROM rust:1-bullseye
RUN rustup default nightly
RUN rustup target add x86_64-fortanix-unknown-sgx --toolchain nightly
RUN apt-get update -y && apt-get -y install pkg-config libssl-dev protobuf-compiler
RUN cargo install fortanix-sgx-tools sgxs-tools
