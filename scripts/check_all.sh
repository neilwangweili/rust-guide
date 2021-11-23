#!/usr/bin/env sh

cargo clean
cargo +nightly build
cargo +nightly test -- --test-threads=1
grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
cargo fmt --all
cargo clippy -- -D warnings
