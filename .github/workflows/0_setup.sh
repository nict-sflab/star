#!/usr/bin/env bash
set -euo pipefail

git reset
git checkout main
git pull --ff-only

mkdir -p output
sudo rm -f -- ./output/*

git status | tee output/durty.txt
screenfetch | tee output/machine.txt
