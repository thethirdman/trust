#!/bin/bash

dico=$1
app=$2
bin=$3
ref=$4
refbin=$5

shuf -n 300 $dico | sed "s/[ \t]*[0-9]\+$//" | sed "s/.*/approx 1 \0/" > test.txt

echo "Test our version"
cat test.txt | $app $bin > log
echo "Test ref version"
cat test.txt | $ref $refbin > reflog
echo "Differences: "
cmp log reflog

echo "Time our version"
cat test.txt | time $app $bin > /dev/null

echo "Time ref version"
cat test.txt | time $ref $refbin > /dev/null
