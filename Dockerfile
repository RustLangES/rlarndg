FROM lukemathwalker/cargo-chef:latest-rust-1.82-slim-bullseye AS chef
WORKDIR /app
ARG DATABASE_URL="postgres://user:password@localhost/dbname"

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN apt-get update && apt-get install -y --no-install-recommends curl && apt-get clean && rm -rf /var/lib/apt/lists/*

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN SQLX_OFFLINE=true cargo build --release

# Runtime
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update && apt-get clean && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/app /usr/local/bin/app
ENTRYPOINT ["/usr/local/bin/app"]
