# Stocks Tracker

## Refresh migrations

```bash
source ./.env
sea-orm-cli migrate refresh -u "$APP_POSTGRES_CONNECTION_STRING" -d apps/migration
sea-orm-cli generate entity -u "$APP_POSTGRES_CONNECTION_STRING" -o libs/adapters/src/postgres/schema --with-serde both --serde-skip-deserializing-primary-key --model-extra-derives 'Default' --seaography
```
