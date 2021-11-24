#!/usr/bin/env sh

sh ./scripts/coverage.sh && sh ./scripts/check-style.sh && cargo doc
