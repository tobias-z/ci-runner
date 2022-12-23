FROM rust:1.65.0-buster as builder
COPY . .
RUN cargo install --path .

FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/ci-runner /usr/local/bin/ci-runner

ENTRYPOINT [ "ci-runner" ]
