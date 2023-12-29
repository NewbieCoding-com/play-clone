FROM --platform=linux/arm64 messense/rust-musl-cross:aarch64-musl

USER root

WORKDIR /app
COPY . .

RUN cargo dev

RUN musl-strip target/aarch64-unknown-linux-musl/release/play

FROM --platform=linux/arm64 gcr.io/distroless/static-debian11

COPY --from=0 /app/target/aarch64-unknown-linux-musl/release/play .

ENTRYPOINT ["/play"]