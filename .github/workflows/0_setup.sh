#!/usr/bin/env bash
set -euo pipefail

git reset
git checkout main
git pull --ff-only

mkdir -p output result
sudo rm -f -- ./output/*

git status | tee result/durty.txt
if command -v screenfetch >/dev/null 2>&1; then
  screenfetch | tee result/machine.txt
else
  uname -a | tee result/machine.txt
fi
