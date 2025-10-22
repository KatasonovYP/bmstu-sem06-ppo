FROM alpine:3.21 AS runtime
ARG APP=api
RUN addgroup -S appuser && adduser -S appuser -G appuser
WORKDIR /app
COPY config/app.default.yaml /app/config/
COPY bin/${APP} /app/app
RUN chmod +x /app/app
USER appuser
CMD ["./app"]
