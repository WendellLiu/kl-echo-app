FROM rust:1.47.0 as builder
WORKDIR /usr/src/echoapp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/echoapp /usr/local/bin/echoapp

ENV RUST_LOG=info
CMD ["echoapp"]
