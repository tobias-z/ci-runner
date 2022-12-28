FROM rust:1.65.0-buster as builder
COPY . .
RUN cargo install --path .

FROM cruizba/ubuntu-dind

# RUN apk add --no-cache bash ca-certificates file iptables libc6-compat libgcc libstdc++ wget && \
#     update-ca-certificates && \
#     ln -s /lib/libc.musl-x86_64.so.1 /lib/ld-linux-x86-64.so.2

WORKDIR /ci-runner
RUN mkdir /ci-runner/bin
ENV PATH=${PATH}:/ci-runner/bin

COPY --from=builder /usr/local/cargo/bin/ci-runner /ci-runner/bin/ci-runner

ENTRYPOINT [ "ci-runner" ]
