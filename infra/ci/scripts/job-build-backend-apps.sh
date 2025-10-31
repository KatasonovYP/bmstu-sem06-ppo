#!/bin/bash

cd ./backend || exit 0

cargo +nightly build --release --all-targets --timings

mkdir -p ./bin

find "./target/${CARGO_BUILD_TARGET}/release" \
    -maxdepth 1 -type f -print0 -executable \
    | xargs -0 -I '{}' mv {} ./bin

# выгружаем таминги загрузки
mv "./target/cargo-timings" ./backend
