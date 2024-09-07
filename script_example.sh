#! /bin/bash
cargo build --release
target/release/pijersi-toolbox new -e 3 -o data/positions/positions --split 100
for i in {0..99}
do
target/release/pijersi-toolbox load data/positions/positions_$i positions -s 1 -o data/responses/responses_$i
done