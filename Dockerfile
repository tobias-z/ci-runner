FROM rust:1.65.0-buster as builder
COPY . .
RUN cargo install --path .

FROM docker:20-dind

WORKDIR /ci-runner
RUN mkdir /ci-runner/bin
ENV PATH=${PATH}:/ci-runner/bin

COPY --from=builder /usr/local/cargo/bin/ci-runner /ci-runner/bin/ci-runner

ENTRYPOINT [ "ci-runner" ]
