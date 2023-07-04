FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
RUN apt-get update && apt-get install -y musl-tools musl-dev
RUN update-ca-certificates
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS app-builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --profile dist --recipe-path recipe.json
COPY . .
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --profile dist --bin alertmanager-uptime-kuma-proxy --target x86_64-unknown-linux-musl

# taken from https://medium.com/@lizrice/non-privileged-containers-based-on-the-scratch-image-a80105d6d341
FROM ubuntu:latest as user-creator
RUN groupadd -g 578 kah \
        && useradd -u 578 -g 578 kah 

FROM scratch AS runtime
COPY --from=user-creator /etc/passwd /etc/passwd

USER kah
COPY --from=app-builder --chown=kah:kah /app/target/x86_64-unknown-linux-musl/dist/alertmanager-uptime-kuma-proxy /app

ENTRYPOINT ["/app"]
