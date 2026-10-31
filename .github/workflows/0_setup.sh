#!/usr/bin/env bash
set -euo pipefail

git reset
git checkout main
git pull --ff-only

mkdir -p output
sudo rm -f -- ./output/*

git status | tee output/durty.txt
if command -v screenfetch >/dev/null 2>&1; then
  screenfetch | tee output/machine.txt
else
  uname -a | tee output/machine.txt
fi
