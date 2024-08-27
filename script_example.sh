#! /bin/bash
cargo build --release
target/release/pijersi-toolbox -e 3 -s 1 -o out1 new
target/release/pijersi-toolbox -e 3 -s 1 -o out2 load-positions out1
target/release/pijersi-toolbox -e 3 -s 1 -o out3 load-responses out2