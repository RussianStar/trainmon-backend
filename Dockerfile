# Stage 1: Build
FROM rust:1.54 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Stage 2: Deploy
FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/simple-api /usr/local/bin/
CMD ["simple-api"]