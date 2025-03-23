FROM rust:1.85 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /app/target/release/alertmanager-bridge /usr/local/bin
CMD ["alertmanager-bridge"]