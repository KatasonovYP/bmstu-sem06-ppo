#!/bin/bash    

sccache --start-server
cd ./backend || exit 0
    
cargo +nightly build --release --all-targets

mkdir -p ./bin

find "./target/${CARGO_BUILD_TARGET}/release" \
    -maxdepth 1 -type f -print0 -executable \
    | xargs -0 -I '{}' mv {} ./bin

# пакуем исполняемые файлы тестов в архивы, чтобы передать их в следующие задачи
mkdir -p ./tests

cargo nextest archive --release --archive-file ./tests/unit.tar.zst --lib
cargo nextest archive --release --archive-file ./tests/integration.tar.zst -p test_integration
cargo nextest archive --release --archive-file ./tests/e2e.tar.zst -p test_e2e

# тут костыль с заполнением тестов тегом skipped
mkdir -p ./allure-results

cargo nextest run --release --lib --nff --no-capture || true
xmlstarlet ed -L -P  -s '//testcase' -t elem -n skipped -v '' -d '//failure' ./target/nextest/default/junit.xml
mv ./target/nextest/default/junit.xml ./allure-results/unit.xml

cargo nextest run --release --test "*it*" --nff --no-capture || true
xmlstarlet ed -L -P  -s '//testcase' -t elem -n skipped -v '' -d '//failure' ./target/nextest/default/junit.xml
mv ./target/nextest/default/junit.xml ./allure-results/integration.xml

cargo nextest run --release --test "*e2e*" --nff --no-capture || true
xmlstarlet ed -L -P  -s '//testcase' -t elem -n skipped -v '' -d '//failure' ./target/nextest/default/junit.xml
mv ./target/nextest/default/junit.xml ./allure-results/e2e.xml