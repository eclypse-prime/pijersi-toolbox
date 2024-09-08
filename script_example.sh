#! /bin/bash
cargo build --release
target/release/pijersi-toolbox new -e 3 -o data/positions/positions --split 100
for i in {0..99}
do
target/release/pijersi-toolbox load data/positions/positions_$i positions -s 1 -o data/responses/responses_$i 1> /dev/null
done
target/release/pijersi-toolbox merge data/responses/responses responses -o data/responses/openings_3 -n 100
load data/responses/openings_3 responses -e 3 -s 1 -o data/responses/openings_2
load data/responses/openings_3 responses -e 2 -s 2 -o data/responses/openings_1
load data/responses/openings_3 responses -e 1 -s 3 -o data/responses/openings_0