project: stocks-tracker-testing
version: 0.42.2

.options: &options
  create_namespace: true
  wait: true
  timeout: 1m
  max_history: 3
  atomic: true
  context: the-qsb/stocks-tracker:main

releases:

  - name: patroni-it-{{ requiredEnv "CI_ENVIRONMENT_NAME" }}
    namespace: patroni
    <<: *options
    chart: ./infra/helm/charts/patroni
    values:
      - ./infra/helm/values/stocks-tracker-migration-it-values.yaml
      - src: ./infra/helm/values/stocks-tracker-migration-it-secrets.yaml
        renderer: sops
      - ./infra/helm/values/patroni-it-values.yaml

  - name: stocks-tracker-env-it-{{ requiredEnv "CI_ENVIRONMENT_NAME" }}
    namespace: stocks-tracker
    <<: *options
    chart: ./infra/helm/charts/stocks-tracker-env
    values:
      - ./infra/helm/values/stocks-tracker-migration-it-values.yaml
      - src: ./infra/helm/values/stocks-tracker-migration-it-secrets.yaml
        renderer: sops

  - name: stocks-tracker-migration-it-{{ requiredEnv "CI_ENVIRONMENT_NAME" }}
    namespace: stocks-tracker
    <<: *options
    chart: ./infra/helm/charts/stocks-tracker-migration
    depends_on:
      - patroni-it-{{ requiredEnv "CI_ENVIRONMENT_NAME" }}@patroni
      - stocks-tracker-env-it-{{ requiredEnv "CI_ENVIRONMENT_NAME" }}@stocks-tracker
    values:
      - ./infra/helm/values/stocks-tracker-migration-it-values.yaml
      - src: ./infra/helm/values/stocks-tracker-migration-it-secrets.yaml
        renderer: sops
