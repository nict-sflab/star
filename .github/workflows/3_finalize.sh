#!/usr/bin/env bash
set -euo pipefail

git add ./output
if git diff --cached --quiet; then
  exit 0
fi

COMMIT_ID=$(git rev-parse --short HEAD)
git commit -m "bench & test: ${COMMIT_ID}"

git push
