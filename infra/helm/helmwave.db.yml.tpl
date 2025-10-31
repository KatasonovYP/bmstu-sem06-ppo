project: stocks-tracker-{{ requiredEnv "STAGE" }}
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

.stocks-tracker-values: &stocks-tracker-values
  - ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-values.yaml
  - src: ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-secrets.yaml
    renderer: sops

.charts:
  - env
  - migration
  - app
  - routing

releases:

{{- get ".charts" }}
{{ range $chart := . }}

  - name: stocks-tracker-{{ requiredEnv "STAGE" }}-{{ $chart }}
    namespace: stocks-tracker
    <<: *options
    chart: ./infra/helm/charts/{{ $chart }}
    values:
      - *stocks-tracker-values
      - ./infra/helm/values/{{ $chart }}-{{ requiredEnv "STAGE" }}-values.yaml

{{ end }}
{{- end }}
