project: stocks-tracker-{{ requiredEnv "STAGE" }}{{ requiredEnv "TEST_ID" }}
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
{{- with readFile "./infra/helm/vars.yaml" | fromYaml | get "charts" }}
{{- range $chart := . }}
  - name: stocks-tracker-{{ requiredEnv "STAGE" }}{{ requiredEnv "TEST_ID" }}-{{ $chart }}
    namespace: stocks-tracker
    <<: *options
    chart: ./infra/helm/charts/{{ $chart }}
    values:
      - ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-values.yaml
      - src: ./infra/helm/values/stocks-tracker-{{ requiredEnv "STAGE" }}-secrets.yaml
        renderer: sops
{{- end }}
{{- end }}
