#!/usr/bin/env sh

cargo clean
cargo test
cargo fmt --all
cargo clippy -- -D warnings
