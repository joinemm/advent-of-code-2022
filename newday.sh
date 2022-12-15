#!/bin/sh

DAY=$1
export DAY
cat src/template.rs | envsubst > src/days/day${DAY}.rs
touch "input/2022/day${DAY}_example.txt"
cargo aoc input -d ${DAY}
echo "Good luck with day ${DAY}!"