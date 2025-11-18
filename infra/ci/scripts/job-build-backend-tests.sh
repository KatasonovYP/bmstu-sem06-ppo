#!/bin/bash

mkdir -p ./tests

cargo +nightly nextest archive --release --archive-file ./tests/unit.tar.zst --lib
cargo +nightly nextest archive --release --archive-file ./tests/integration.tar.zst -p test_integration
cargo +nightly nextest archive --release --archive-file ./tests/e2e.tar.zst -p test_e2e
