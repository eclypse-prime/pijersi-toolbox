#! /bin/bash
cargo build --release
target/release/pijersi-toolbox new -e 3 -o positions/positions --split 100
for i in {0..99}
do
target/release/pijersi-toolbox load positions/positions_$i positions -s 1 -o responses/responses_$i
done