rm ./bench/*

git reset
git checkout main

git pull

git status | tee bench/durty.txt
screenfetch | tee bench/machine.txt

echo "bench mark testing ..."
sudo docker compose -f compose.bench.yaml up --build serv --exit-code-from serv
sudo docker compose -f compose.bench.yaml up --build core
sudo docker compose -f compose.bench.yaml up --build sidechannel
sudo docker compose -f compose.bench.yaml up ecdaa-rs --exit-code-from ecdaa-rs
sudo docker compose -f compose.bench.yaml up arc --exit-code-from arc
sudo docker compose -f compose.bench.yaml down --rmi all --remove-orphans

git add ./bench

COMMIT_ID=$(git rev-parse --short HEAD)
git commit -m "bench: ${COMMIT_ID}"

git push
