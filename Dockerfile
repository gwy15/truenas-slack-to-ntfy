# build
FROM rust:slim-bookworm AS builder
WORKDIR /code
COPY . .
RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo b --release --target x86_64-unknown-linux-musl

# 
FROM debian:bookworm-slim
WORKDIR /code
COPY --from=builder /code/target/x86_64-unknown-linux-musl/release/truenas-slack-to-ntfy .
ENTRYPOINT [ "./truenas-slack-to-ntfy" ]
CMD []
