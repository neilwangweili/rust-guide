#!/usr/bin/env sh

cargo test
cargo fmt --all
cargo clippy -- -D warnings
