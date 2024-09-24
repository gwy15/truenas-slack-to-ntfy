# build
FROM rust:slim-bookworm as builder
WORKDIR /code
COPY . .
RUN cargo b --release --target x86_64-unknown-linux-musl

# 
FROM debian:bookworm-slim
WORKDIR /code
COPY --from=builder /code/target/release/truenas-slack-to-ntfy .
ENTRYPOINT [ "./truenas-slack-to-ntfy" ]
CMD []
