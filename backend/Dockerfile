# Используем готовый образ для сборки с MUSL
FROM clux/muslrust:stable as builder

WORKDIR /app

# Копируем код
COPY . .

# Собираем приложение
RUN cargo build --release

# Финальный образ
FROM alpine:3.18

WORKDIR /app
COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/stocks_tracker /app/stocks_tracker

# Удостоверимся, что файл исполняемый
RUN chmod +x /app/stocks_tracker

CMD ["/app/stocks_tracker"]
