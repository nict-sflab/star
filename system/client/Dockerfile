FROM nixos/nix:2.35.2

ENV OPENSSL_NO_VENDOR 1

RUN nix-channel --add https://channels.nixos.org/nixos-26.05 nixpkgs \
    && nix-channel --update

WORKDIR /workspace
COPY star /workspace/star
COPY constant-time-utils /workspace/constant-time-utils
COPY timecop-rs /workspace/timecop-rs


WORKDIR /workspace/star/system/client
RUN nix-shell -p rustc cargo pkg-config openssl cmake gcc gnumake \
    --run 'cargo build --release'

CMD ["./target/release/star-client-proxy"]
