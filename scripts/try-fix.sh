#!/usr/bin/env sh

cargo fix
cargo +nightly clippy --fix -Z unstable-options
