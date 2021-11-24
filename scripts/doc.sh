#!/usr/bin/env sh

cargo doc
rm -rf ./documents/resource
cp -rf ./target/doc ./documents/resource
