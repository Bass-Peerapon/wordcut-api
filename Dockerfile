# Stage 1: Cargo chef for caching and compiling Rust code
FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

# Stage 2: Cargo chef planner
FROM chef AS planner
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
RUN cargo chef prepare

# Stage 3: Cargo chef builder
FROM chef AS builder
# Install Python and pip dependencies for PyO3
RUN apt-get update && \
  apt-get install -y python3 python3-pip && \
  pip3 install --no-cache-dir --break-system-packages pythainlp

COPY --from=planner /app/recipe.json .
RUN cargo chef cook --release
COPY . .
RUN cargo build --release
RUN mv ./target/release/wordcut-api ./app

COPY ./dict.txt /app/dict.txt
COPY ./thai_cluster_rules.txt /app/thai_cluster_rules.txt

# Stage 4: Runtime
FROM debian:stable-slim AS runtime
WORKDIR /app
# Install Python in the runtime stage as well
RUN apt-get update && apt-get install -y python3 python3-pip && \
  pip3 install --no-cache-dir --break-system-packages pythainlp

COPY --from=builder /app/app /usr/local/bin/
COPY --from=builder /app/dict.txt /app/dict.txt
COPY --from=builder /app/thai_cluster_rules.txt /app/thai_cluster_rules.txt

ENTRYPOINT ["/usr/local/bin/app"]

