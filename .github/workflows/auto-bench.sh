git checkout bench

screenfetch | tee bench/machine.txt
git diff | tee bench/durty1.txt
git status | tee bench/durty2.txt

git pull


COMMIT_ID=$(git rev-parse --short HEAD)

echo "bench mark testing ..."
# docker compose -f compose.bench.yaml up --build core
# docker compose -f compose.bench.yaml up --build serv
# docker compose -f compose.bench.yaml up --build sidechannel
# docker compose -f compose.bench.yaml down --rmi all --remove-orphans

git add .
git commit -m "bench: ${COMMIT_ID}"

git push
