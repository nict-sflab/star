git checkout bench
git pull

COMMIT_ID=$(git rev-parse --short HEAD)

echo "bench mark testing ..."
git add .
git commit -m "bench: ${COMMIT_ID}"

git push
