# Stocks Tracker

## Поднять локальные базы

из корня проекта

```bash
docker compose up postgres redis -d
```

## Обновление миграций

```bash
source ./.env
sea-orm-cli migrate refresh -u "$APP_POSTGRES_CONNECTION_STRING" -d apps/migration
sea-orm-cli generate entity -u "$APP_POSTGRES_CONNECTION_STRING" -o libs/adapters/src/postgres/schema --with-serde both --serde-skip-deserializing-primary-key --model-extra-derives 'Default' --seaography
```

## генерация клиента для e2e тестов

```bash
openapi-generator-cli generate -i http://localhost:3000/api/v1 -g rust -o ./libs/http_client_stocks_tracker
```

## Работа с секретами

Установка зависемостей:

```bash
helm plugin install https://github.com/jkroepke/helm-secrets --version v4.6.0
brew install age
brew install sops
```

Генерация ключа (только для новых проектов!)

получить [ключ](https://gitlab.com/the-qsb/stocks-tracker/-/settings/ci_cd#js-cicd-variables-settings) и положить его в корень проекта с именем key.txt

```bash
age-keygen -o key.txt
```

запускать из корня проекта

```bash
export SOPS_AGE_KEY_FILE=$(pwd)/key.txt
export SOPS_AGE_RECIPIENTS=age18yxwsntjasphxzgtymdnrz2ggs2zd86rnyfpaeh2pk6gz42asgyqxd8r8a
EDITOR="code --wait" helm secrets edit path/to/sops/secret
```

## пример раскатки приложения с рук

важно, что можно поменять только параметры, без пересборки в CI код останется прежним

```bash
CI_COMMIT_SHORT_SHA=73302d69 STAGE=stress helmwave up --build --yml --tpl infra/helm/helmwave.app.yml.tpl
```
