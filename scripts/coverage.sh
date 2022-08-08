#!/usr/bin/env sh

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -C panic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"
cargo clean
cargo +nightly build
cargo +nightly test -- --test-threads=10
grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing --ignore "**/lib.rs" -o ./target/debug/coverage/
data=$(cat ./target/debug/coverage/coverage.json | sed 's/,"/\n/g' | grep -v IAM_Server | sed 's/":"/=/g;s/"$//g;s/^{"//g;s/"}//g')
declare $data >/dev/null 2>&1
if [ $message != '100%' ]; then
  echo "\n\n\033[31m Error: Coverage is not 100%! Current coverage: $message. \n\n \033[0m"
  exit 1
else
  echo "\n\n\033[32m Line coverage 100%!\n\n \033[0m"
fi
