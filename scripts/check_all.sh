#!/usr/bin/env sh

cargo clean
cargo test --color=always --message-format=json-diagnostic-rendered-ansi --no-run --package rust-guide --test integration_test
grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
cargo fmt --all
cargo clippy -- -D warnings
