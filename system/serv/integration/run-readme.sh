#!/usr/bin/env bash
set -Eeuo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
ENV_FILE="$ROOT/integration/readme-env.sh"
LOG_DIR="$ROOT/integration/logs"
RUN_ID=$(date +%Y%m%d-%H%M%S)
LOG_FILE="$LOG_DIR/readme-integration-$RUN_ID.log"
APP_LOG="$LOG_DIR/star-app-attest-$RUN_ID.log"
KEY_MANAGER_LOG="$LOG_DIR/key-manager-$RUN_ID.log"
INSPECT_LOG="$LOG_DIR/inspect-$RUN_ID.log"
VERIFY_LOG="$LOG_DIR/verify-$RUN_ID.log"

mkdir -p "$LOG_DIR"
cd "$ROOT"

# shellcheck source=integration/readme-env.sh
. "$ENV_FILE"

exec > >(tee -a "$LOG_FILE") 2>&1

AESM_PROXY_PID=
KEY_MANAGER_PID=
APP_PID=

cleanup() {
    stop_pid "$APP_PID"
    stop_pid "$KEY_MANAGER_PID"
    stop_pid "$AESM_PROXY_PID"
}

stop_pid() {
    local pid="${1:-}"
    if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
        kill "$pid" 2>/dev/null || true
        wait "$pid" 2>/dev/null || true
    fi
}

trap cleanup EXIT

section() {
    printf '\n== %s ==\n' "$1"
}

run() {
    section "$1"
    shift
    printf '+'
    printf ' %q' "$@"
    printf '\n'
    "$@"
}

run_with_stdin() {
    local title="$1"
    local stdin_file="$2"
    shift 2
    section "$title"
    printf '+'
    printf ' %q' "$@"
    printf ' < %q\n' "$stdin_file"
    "$@" < "$stdin_file"
}

wait_for_file_text() {
    local file="$1"
    local text="$2"
    local seconds="$3"
    local i
    for i in $(seq 1 "$seconds"); do
        if grep -q "$text" "$file" 2>/dev/null; then
            return 0
        fi
        if [ -n "${APP_PID:-}" ] && ! kill -0 "$APP_PID" 2>/dev/null; then
            return 1
        fi
        sleep 1
    done
    return 1
}

wait_for_http() {
    local url="$1"
    local seconds="$2"
    local i
    for i in $(seq 1 "$seconds"); do
        if curl -fsS "$url" >/dev/null 2>&1; then
            return 0
        fi
        if [ -n "${KEY_MANAGER_PID:-}" ] && ! kill -0 "$KEY_MANAGER_PID" 2>/dev/null; then
            return 1
        fi
        sleep 1
    done
    return 1
}

start_aesm_proxy() {
    section "Start AESM TCP proxy"
    if nc -z 127.0.0.1 5555 >/dev/null 2>&1; then
        echo "AESM proxy already listens on 127.0.0.1:5555"
        return 0
    fi

    perl -MIO::Socket::INET -MIO::Socket::UNIX -e '
        use strict;
        use warnings;
        my $listen = $ENV{AESM_PROXY} || "127.0.0.1:5555";
        my ($host, $port) = split /:/, $listen, 2;
        my $socket_path = $ENV{AESM_SOCKET} || "/run/aesmd/aesm.socket";
        my $server = IO::Socket::INET->new(
            LocalAddr => $host,
            LocalPort => $port,
            Listen => 16,
            ReuseAddr => 1,
            Proto => "tcp",
        ) or die "listen $listen: $!";
        while (my $client = $server->accept) {
            my $pid = fork();
            die "fork: $!" unless defined $pid;
            if ($pid == 0) {
                close $server;
                eval {
                    while (1) {
                        read_exact($client, 4, my $len_buf) or last;
                        my $len = unpack("L", $len_buf);
                        die "AESM frame too large: $len" if $len > 16 * 1024 * 1024;
                        read_exact($client, $len, my $body) or last;
                        my $unix = IO::Socket::UNIX->new(Type => SOCK_STREAM(), Peer => $socket_path)
                            or die "connect $socket_path: $!";
                        print {$unix} $len_buf, $body;
                        read_exact($unix, 4, my $res_len_buf) or die "AESM closed early";
                        my $res_len = unpack("L", $res_len_buf);
                        die "AESM response too large: $res_len" if $res_len > 16 * 1024 * 1024;
                        read_exact($unix, $res_len, my $res_body) or die "short AESM response";
                        print {$client} $res_len_buf, $res_body;
                        close $unix;
                    }
                };
                warn "$@\n" if $@;
                close $client;
                exit 0;
            }
            close $client;
            while (waitpid(-1, 1) > 0) {}
        }
        sub read_exact {
            my ($fh, $want, undef) = @_;
            my $buf = "";
            while (length($buf) < $want) {
                my $n = sysread($fh, my $chunk, $want - length($buf));
                return 0 unless defined($n) && $n > 0;
                $buf .= $chunk;
            }
            $_[2] = $buf;
            return 1;
        }
    ' &
    AESM_PROXY_PID=$!
    sleep 1
    if ! kill -0 "$AESM_PROXY_PID" 2>/dev/null; then
        wait "$AESM_PROXY_PID"
    fi
    echo "AESM proxy pid: $AESM_PROXY_PID"
}

start_key_manager() {
    section "Run Key-Manager Separately"
    cargo run --manifest-path utils/manage-key/service/Cargo.toml --release > "$KEY_MANAGER_LOG" 2>&1 &
    KEY_MANAGER_PID=$!
    echo "key-manager pid: $KEY_MANAGER_PID"
    if ! wait_for_http "$STAR_KEY_MANAGER_URL/star/key-manager/public_key" 900; then
        sed -n '1,160p' "$KEY_MANAGER_LOG"
        return 1
    fi
}

start_star_app() {
    section "Start With Attestation"
    cargo run --manifest-path app/Cargo.toml --release --bin star_app > "$APP_LOG" 2>&1 &
    APP_PID=$!
    echo "star_app pid: $APP_PID"
    if ! wait_for_file_text "$APP_LOG" "STAR_ISSUE_ENCLAVE_DCAP_EVIDENCE_FOR_KEY_MANAGER_BASE64=" 900; then
        sed -n '1,200p' "$APP_LOG"
        return 1
    fi
    stop_pid "$APP_PID"
    APP_PID=
    sed -n '1,80p' "$APP_LOG"
}

extract_field() {
    local name="$1"
    awk -v key="$name" '$1 == key { print $NF; exit }' "$INSPECT_LOG"
}

section "Environment"
    echo "env file: $ENV_FILE"
    echo "main log: $LOG_FILE"
    echo "app log: $APP_LOG"
    echo "key-manager log: $KEY_MANAGER_LOG"
    echo "inspect log: $INSPECT_LOG"
    echo "verify log: $VERIFY_LOG"
    env | sort | grep -E '^(AESM|PCCS|SGX|STAR|TLS|NO_PROXY|PKG_CONFIG|OPENSSL|LD_LIBRARY_PATH|PATH|CC|AR|RANLIB)='

start_aesm_proxy

start_key_manager

run "README Test" cargo test --release

run "README Reproduce SGX Benchmarks" cargo bench
run "README Benchmark extension" cargo bench -p star_app --bench extension
run "README Benchmark filter" cargo bench -p star_app --bench filter
run "README Benchmark storage" cargo bench -p star_app --bench storage
run "README Benchmark tls" cargo bench -p star_app --bench tls

start_star_app

run_with_stdin "README Inspect A Quote" "$APP_LOG" \
    cargo run --manifest-path tools/verifier/Cargo.toml --release -- inspect \
    > "$INSPECT_LOG"
cat "$INSPECT_LOG"

PUBLIC_KEY=$(extract_field "public")
MRENCLAVE=$(extract_field "mrenclave:")
MRSIGNER=$(extract_field "mrsigner")

if [ -z "$PUBLIC_KEY" ] || [ -z "$MRENCLAVE" ] || [ -z "$MRSIGNER" ]; then
    echo "failed to extract verifier measurements"
    exit 1
fi

run_with_stdin "README Verify A Quote" "$APP_LOG" \
    cargo run --manifest-path tools/verifier/Cargo.toml --release -- verify \
        --mrsigner "$MRSIGNER" \
        --mrenclave "$MRENCLAVE" \
        --public-key "$PUBLIC_KEY" \
        --allow-debug \
        --allow-advisory \
        --pccs-url "$PCCS_URL" \
    > "$VERIFY_LOG"
cat "$VERIFY_LOG"

section "Done"
echo "main log: $LOG_FILE"
echo "app log: $APP_LOG"
echo "key-manager log: $KEY_MANAGER_LOG"
echo "inspect log: $INSPECT_LOG"
echo "verify log: $VERIFY_LOG"
