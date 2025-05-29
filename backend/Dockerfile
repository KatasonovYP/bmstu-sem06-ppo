FROM clux/muslrust:stable AS builder
# FROM clux/muslrust:stable AS chef
USER root
# RUN cargo install cargo-chef
WORKDIR /app

# FROM chef AS planner
# COPY . .
# RUN cargo chef prepare --recipe-path recipe.json

# FROM chef AS builder
# COPY --from=planner /app/recipe.json recipe.json
# RUN cargo chef cook --release --recipe-path recipe.json

ARG APP=api
COPY . .
RUN cargo build --release -p ${APP}
RUN mv /app/target/$CARGO_BUILD_TARGET/release/${APP} .

FROM alpine:3.21 AS runtime
ARG APP=api
RUN addgroup -S appuser && adduser -S appuser -G appuser
WORKDIR /app
RUN mkdir -p /app/config
COPY config/app.default.yaml /app/config/
COPY --from=builder /app/${APP} /app/app
RUN chmod +x /app/app
USER appuser
CMD ["./app"]
