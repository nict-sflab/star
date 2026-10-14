#!/usr/bin/env bash
set -euo pipefail

mkdir -p output
trap 'sudo docker compose -f compose.eval.yaml down --rmi all --remove-orphans' EXIT

sudo docker compose -f compose.eval.yaml up --build serv --exit-code-from serv
sudo docker compose -f compose.eval.yaml up --build core --exit-code-from core
sudo docker compose -f compose.eval.yaml up --build sidechannel --exit-code-from sidechannel
sudo docker compose -f compose.eval.yaml up ecdaa-rs --exit-code-from ecdaa-rs
sudo docker compose -f compose.eval.yaml up arc --exit-code-from arc

