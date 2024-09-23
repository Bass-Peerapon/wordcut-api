FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

FROM chef AS builder
COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release
RUN mv ./target/release/wordcut-api ./app

COPY ./dict.txt /app/dict.txt
COPY ./thai_cluster_rules.txt /app/thai_cluster_rules.txt
COPY ./tnc_freq.txt /app/tnc_freq.txt

FROM debian:stable-slim AS runtime
WORKDIR /app
COPY --from=builder /app/app /usr/local/bin/
COPY --from=builder /app/dict.txt /app/dict.txt
COPY --from=builder /app/thai_cluster_rules.txt /app/thai_cluster_rules.txt
COPY --from=builder /app/tnc_freq.txt /app/tnc_freq.txt

ENTRYPOINT ["/usr/local/bin/app"]
