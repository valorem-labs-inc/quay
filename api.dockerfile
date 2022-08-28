FROM lukemathwalker/cargo-chef:latest-rust-1.61.0 as chef
WORKDIR /app

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin api

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
 && apt-get install -y --no-install-recommends openssl \
 # Clean up
 && apt-get autoremove -y \
 && apt-get clean -y \
 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/api api
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./api"]