# Тестирование. ЛР 01

## Задание

Написать unit-тесты для компонентов доступа к данным и бизнес логики выбранного проекта Требования

## Требования

### 01

>Требуемое покрытие тестами: один класс - как минимум один test suite / test class с как минимум с двумя тестами (один позитивный, другой негативный) на каждый из public-методов каждого класса основных компонентов; если проект для тестирования выполнен не в объектном стиле -- то необходимо выделить модули исходя из структуры программы

тут ссылка на примеры

### 02

>Должны быть представлены тесты на обработку исключений (когда ожидаемым результатов является Exception)

тут ссылка на пример

### 03

>Должны быть представлены тесты как в классическом (без mock \ stub) так и в “Лондонском” варианте; допустимо представить один и тот же тест в обоих вариантах для сравнения (в учебных целях, на практике это редко имеет смысл)

тут ссылка на пример

### 04

>Должна быть соблюдена структура Arrange-Act-Assert для каждого теста c использованием fixture и остальных классов\методов хелперов

./backend/libs/adapters/tests/test_active_repository.rs

### 05

>Если по какой-то причине метод не может быть вызван один в секции Act, то требуется переписать код класса

OK!

### 06

>На приватные методы писать тесты не нужны

OK!

### 07

>Должны быть представлены тесты с использованием паттерна Data Builder и Fabric(Object Mother) для генерации объектов для тестов

тут ссылка на пример

### 08

>Должен быть настроен запуск тестов из командной строки на основании локальной копии репозитория

```bash
cargo test --lib
```

### 09

>Должен быть представлен автоматически сгенерированный отчет по результатам выполнения тестов (рекомендуется использовать allure - <https://github.com/allure-framework> - кроме случаев, когда используемый язык программирования не поддерживается); генерация отчета также должна быть учтена в пункте 8

```bash
cargo +nightly test --lib -- --format=json -Z unstable-options --report-time > test-report.json

markdown-test-report test-report.json
```

```bash
cargo nextest run --lib

allure serve target/nextest/default --name stocks-tracker
```

### 10

>Должен быть предусмотрен запуск тестов в случайном порядке

```text
By default, the tests are run in alphabetical order. Use --shuffle or set
RUST_TEST_SHUFFLE to run the tests in random order. Pass the generated
"shuffle seed" to --shuffle-seed (or set RUST_TEST_SHUFFLE_SEED) to run the
tests in the same order again. Note that --shuffle and --shuffle-seed do not
affect whether the tests are run in parallel.
```

демонстрация двух запусков, где выводы отличаются

```bash
cargo +nightly test --lib -- -Z unstable-options --shuffle
cargo +nightly test --lib -- -Z unstable-options --shuffle-seed 1759094120271073000 --test-threads 1
```

### 11

>Должен быть предусмотрен запуск тестов в режиме без доступа к интернету (идея в том, что тесты только с mock должны в таком случае проходить успешно)

```txt
--offline Run without accessing the network
```

```bash
cargo test --lib --offline
```

### 12

>Проверить сколько процессов запускается на прогон всех юнит-тестов: отдельный процесс на все тесты, отдельный процесс на каждый тест-класс / тест / и т.д. - разобраться чем это конфигурируется в выбранном стэке, отразить в документации

```txt
By default, all tests are run in parallel. This can be altered with the
--test-threads flag or the RUST_TEST_THREADS environment variable when running
tests (set it to 1).
```

```bash
cargo test --lib -- --test-threads 1
cargo test --lib -- --test-threads 8
```

### 13

>Тесты должны проходить успешно

```bash
cargo test --lib
```

### 14

>Защита от регрессии, устойчивость к рефакторингу и легкость поддержки -- базовые принципы, которым стоит следовать

OK!
