project: stocks-tracker-{{ requiredEnv "STAGE" }}
version: 0.42.2

.options: &options
  create_namespace: true
  wait: true
  timeout: 1m
  max_history: 3
  atomic: true
  context: the-qsb/stocks-tracker:main

releases:

  - name: patroni-{{ requiredEnv "STAGE" }}
    namespace: patroni
    <<: *options
    chart: ./infra/helm/charts/patroni
    values:
      - ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-values.yaml
      - src: ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-secrets.yaml
        renderer: sops
      - ./infra/helm/values/patroni-{{ requiredEnv "STAGE" }}-values.yaml

  - name: stocks-tracker-{{ requiredEnv "STAGE" }}-env
    namespace: stocks-tracker
    <<: *options
    chart: ./infra/helm/charts/stocks-tracker-env
    values:
      - ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-values.yaml
      - src: ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-secrets.yaml
        renderer: sops

  - name: stocks-tracker-{{ requiredEnv "STAGE" }}-migration
    namespace: stocks-tracker
    <<: *options
    chart: ./infra/helm/charts/stocks-tracker-migration
    depends_on:
      - stocks-tracker-{{ requiredEnv "STAGE" }}-env@stocks-tracker
      - patroni-{{ requiredEnv "STAGE" }}@patroni
    values:
      - ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-values.yaml
      - src: ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-secrets.yaml
        renderer: sops

  - name: stocks-tracker-{{ requiredEnv "STAGE" }}-app
    namespace: stocks-tracker
    <<: *options
    chart: ./infra/helm/charts/stocks-tracker-app
    depends_on:
      - stocks-tracker-{{ requiredEnv "STAGE" }}-migration@stocks-tracker
    values:
      - ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-values.yaml
      - src: ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-secrets.yaml
        renderer: sops
