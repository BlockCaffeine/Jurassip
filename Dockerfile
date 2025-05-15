# syntax=docker/dockerfile:1
FROM rust:latest as builder
WORKDIR /app

# Install system dependencies for libudev
RUN apt-get update && apt-get install -y libudev-dev pkg-config protobuf-compiler

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/Jurassip /app/Jurassip
COPY proto/ proto/
EXPOSE 50051
CMD ["/app/Jurassip"]
