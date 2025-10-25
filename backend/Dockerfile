FROM cr.yandex/crpav8o4hj057l1dbgqa/the-qsb/stocks-tracker/alpine:3.21 AS runtime

ARG APP=api

WORKDIR /app

RUN addgroup -S appuser && adduser -S appuser -G appuser

COPY --chown=appuser:appuser config/app.default.yaml /app/config/
COPY --chown=appuser:appuser bin/${APP} /app/app

USER appuser

CMD ["/app/app"]
