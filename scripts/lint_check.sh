#!/usr/bin/env sh

cargo fmt --all
cargo clippy -- -D warnings
