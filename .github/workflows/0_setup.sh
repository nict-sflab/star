sudo rm ./output/*

git reset
git checkout main

git pull

git status | tee result/durty.txt
screenfetch | tee result/machine.txt

