FROM clux/muslrust:stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin api
RUN mv /app/target/$CARGO_BUILD_TARGET/release/api .

FROM alpine:3.21 AS runtime
RUN addgroup -S myuser && adduser -S myuser -G myuser
COPY --from=builder /app/api /app/stocks_tracker

RUN chmod +x /app/stocks_tracker
USER myuser
CMD ["/app/stocks_tracker"]
