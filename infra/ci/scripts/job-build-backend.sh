#!/bin/bash

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

TEST_FILE=./target/nextest/default/junit.xml

cargo nextest run --nff --no-capture --archive-file ./tests/unit.tar.zst > /dev/null 2>&1 || true
xmlstarlet ed -L -P  -s '//testcase' -t elem -n skipped -v '' -d '//failure' ${TEST_FILE}
mv ${TEST_FILE} ./allure-results/unit.xml

cargo nextest run --nff --no-capture --archive-file ./tests/integration.tar.zst > /dev/null 2>&1 || true
xmlstarlet ed -L -P  -s '//testcase' -t elem -n skipped -v '' -d '//failure' ${TEST_FILE}
mv ${TEST_FILE} ./allure-results/integration.xml

cargo nextest run --nff --no-capture --archive-file ./tests/e2e.tar.zst > /dev/null 2>&1 || true
xmlstarlet ed -L -P  -s '//testcase' -t elem -n skipped -v '' -d '//failure' ${TEST_FILE}
mv ${TEST_FILE} ./allure-results/e2e.xml
