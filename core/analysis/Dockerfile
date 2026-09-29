FROM rust:bookworm

ARG BINSEC_COMMIT=9749bd8aed7ee0a27959158b1e53e8789cb0e5ab

RUN apt-get update && apt-get install -y --no-install-recommends \
    binutils git libgmp-dev m4 make opam pkg-config python3 python3-venv z3 \
    && rm -rf /var/lib/apt/lists/*

RUN python3 -m venv /opt/xonsh \
    && /opt/xonsh/bin/pip install --no-cache-dir xonsh \
    && ln -s /opt/xonsh/bin/xonsh /usr/local/bin/xonsh

RUN rustup target add x86_64-fortanix-unknown-sgx

RUN opam init --disable-sandboxing -y \
    && opam switch create binsec 4.14.3 -y \
    && eval "$(opam env --switch=binsec)" \
    && opam install -y unisim_archisec \
    && git clone https://github.com/binsec/binsec.git /tmp/binsec \
    && cd /tmp/binsec \
    && git checkout "$BINSEC_COMMIT" \
    && make \
    && make install \
    && ln -s "$(opam var --switch=binsec bin)/binsec" /usr/local/bin/binsec \
    && rm -rf /tmp/binsec

WORKDIR /workspace
COPY star /workspace/star
COPY constant-time-utils /workspace/constant-time-utils
COPY timecop-rs /workspace/timecop-rs

WORKDIR /workspace/star/core

CMD ["xonsh", "analysis/run.xonsh"]
