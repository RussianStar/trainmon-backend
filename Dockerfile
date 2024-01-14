ARG RUST_VERSION=1.75

FROM rust:${RUST_VERSION} as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/simple-api /usr/local/bin/
CMD ["simple-api"]
