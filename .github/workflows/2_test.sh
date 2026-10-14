#!/usr/bin/env bash
set -euo pipefail

trap 'sudo docker compose -f compose.test.yaml down --rmi all --remove-orphans' EXIT
sudo docker compose -f compose.test.yaml up --build --exit-code-from tests tests

