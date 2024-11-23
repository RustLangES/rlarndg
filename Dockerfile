
FROM rust:1.82-slim-bullseye AS backend-builder

RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev pkg-config

WORKDIR /build

COPY Cargo.* .
COPY src/ src/
COPY migrations/ migrations/
COPY .sqlx/ .sqlx/

RUN SQLX_OFFLINE=true cargo build --release

FROM denoland/deno:2.1.1 AS frontend-builder

WORKDIR /build

COPY frontend .

RUN deno install
RUN deno task build

FROM nginx:1-bookworm

RUN apt-get update && apt-get install -y \
	libssl-dev \
    libc6 \
    libgcc-s1 \
    libstdc++6 \
    ca-certificates

WORKDIR /app

COPY --from=backend-builder /build/target/release/rlarndg /app/backend
COPY --from=frontend-builder /build/dist/ /app/frontend/
COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY sources.json /app/sources.json

ARG STRIPE_SECRET
ARG DATABASE_URL

RUN echo "STRIPE_SECRET=${STRIPE_SECRET}" > /app/.env && \
	echo "DATABASE_URL=${DATABASE_URL}" >> /app/.env

RUN chmod 777 ./backend

CMD ["/bin/sh", "-c", "service nginx start && ./backend --source sources.json"]