FROM nixos/nix:2.35.2

RUN nix-channel --add https://channels.nixos.org/nixos-unstable nixpkgs \
    && nix-channel --update

WORKDIR /workspace
COPY star /workspace/star
COPY constant-time-utils /workspace/constant-time-utils
COPY timecop-rs /workspace/timecop-rs

WORKDIR /workspace/star/core
RUN nix-shell analysis/shell.nix --run true

CMD ["nix-shell", "analysis/shell.nix", "--run", "xonsh analysis/run.xonsh"]
