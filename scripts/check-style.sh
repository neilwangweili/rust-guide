#!/usr/bin/env sh

if ! [ `cargo fmt --all && cargo clippy -- -D warnings` ]; then
    echo "\n\n\033[31m Error: There are some style problems! \n\n \033[0m"
    exit 1
  else
    echo "\n\n\033[32m Code style valid. \n\n \033[0m"
fi
