#!/usr/bin/env bash
set -euo pipefail

# Always run from the repository root so target paths resolve consistently.
script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd -- "${script_dir}/.." && pwd)"
cd "${repo_root}"

jobs="${CARGO_BUILD_JOBS:-$(nproc)}"
echo "cargo build -j ${jobs} --bin analysis --features timecop --release"
cargo build -j "${jobs}" --bin analysis --features timecop --release

echo "valgrind ./target/release/analysis"
valgrind ./target/release/analysis
