#!/bin/bash

OUTPUT=$(rust-code-analysis-cli -m -p ./backend -X './backend/libs/http_client_stocks_tracker/*' --pr -O json | jq '[.. | select(.metrics?.cyclomatic?.max? > 10) | { kind, name: (.name // "anonymous"), max_complexity: .metrics.cyclomatic.max, start_line, end_line } ]' | grep -v '\[\]')

if [[ "$OUTPUT" == "" ]]; then
  echo "${OUTPUT}"
  exit 0
else
  echo "${OUTPUT}"
  exit 1
fi
