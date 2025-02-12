FROM rust:1.84-slim-bullseye AS base

FROM base AS chef
WORKDIR /app

RUN cargo install cargo-chef
RUN cargo init --name solution

COPY Cargo.toml Cargo.lock ./
RUN cargo chef prepare --recipe-path recipe.json

FROM base AS planner
WORKDIR /app

RUN cargo install cargo-chef
RUN cargo init --name solution

COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


FROM base AS builder
WORKDIR /app

COPY .sqlx .sqlx
COPY migrations migrations
COPY src src
COPY Cargo.toml Cargo.toml

COPY --from=planner /app/target target
COPY --from=planner /app/Cargo.lock Cargo.lock

RUN cargo build --release


FROM gcr.io/distroless/cc-debian12 AS final
WORKDIR /app
COPY --from=builder /app/target/release/solution solution
ENTRYPOINT [ "/app/solution" ]

