#! /bin/bash
echo "cargo run --bin client set Genshin 114514"
cargo run --bin client set Genshin 114514
echo "cargo run --bin client get Genshin"
cargo run --bin client get Genshin
echo "cargo run --bin client del Genshin"
cargo run --bin client del Genshin
echo "cargo run --bin client get Genshin"
cargo run --bin client get Genshin
echo "cargo run --bin client ping"
cargo run --bin client ping