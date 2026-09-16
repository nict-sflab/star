git checkout main

rm ./bench
git status | tee bench/durty.txt
screenfetch | tee bench/machine.txt

git pull


COMMIT_ID=$(git rev-parse --short HEAD)

echo "bench mark testing ..."
sudo docker compose -f compose.bench.yaml up --build core
sudo docker compose -f compose.bench.yaml up --build serv --exit-code-from serv
sudo docker compose -f compose.bench.yaml up --build sidechannel
sudo docker compose -f compose.bench.yaml down --rmi all --remove-orphans

git add ./bench
git commit -m "bench: ${COMMIT_ID}"

git push
