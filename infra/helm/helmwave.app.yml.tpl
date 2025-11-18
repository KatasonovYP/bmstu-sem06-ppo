project: stocks-tracker-{{ requiredEnv "STAGE" }}{{ default "" (env "TEST_ID") }}
version: 0.42.2


repositories:
  - name: bitnami
    url: https://charts.bitnami.com/bitnami

.options: &options
  create_namespace: true
  wait: true
  timeout: 1m
  max_history: 3
  atomic: true
  context: the-qsb/stocks-tracker:main


releases:
  - name: stocks-tracker-{{ requiredEnv "STAGE" }}{{ default "" (env "TEST_ID") }}-apps
    namespace: stocks-tracker
    <<: *options
    chart: ./infra/helm/charts/apps
    values:
      - ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-values.yaml
      - src: ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-secrets.yaml
        renderer: sops
