rm ./bench/*

git reset
git checkout main

git pull

git status | tee bench/durty.txt
screenfetch | tee bench/machine.txt

git clone http://github.com/akakou/ecdaa-rs
cd ecdaa-rs
nix-shell -p cargo --command "cargo bench"
cd ..

git clone https://github.com/cloudflareresearch/arc.git
cd arc
nix-shell -p cargo --command "cargo bench --features suite_ristretto255 --bench arc_bench -- arc"
cd ..

echo "bench mark testing ..."
sudo docker compose -f compose.bench.yaml up --build core
sudo docker compose -f compose.bench.yaml up --build serv --exit-code-from serv
sudo docker compose -f compose.bench.yaml up --build sidechannel
sudo docker compose -f compose.bench.yaml down --rmi all --remove-orphans

git add ./bench

COMMIT_ID=$(git rev-parse --short HEAD)
git commit -m "bench: ${COMMIT_ID}"

git push
