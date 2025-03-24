FROM rust:1.85 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates --no-install-recommends && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/alertmanager-bridge /usr/local/bin
CMD ["alertmanager-bridge"]