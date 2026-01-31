FROM rust:alpine3.23 as builder

WORKDIR /usr/src/app

RUN apk add --no-cache musl-dev gcc openssl-dev openssl-libs-static pkgconfig

COPY migrations/ migrations/
COPY src/ src/
COPY Cargo.toml Cargo.toml

RUN rustup target add x86_64-unknown-linux-musl

ENV OPENSSL_STATIC=yes
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/importer /usr/local/bin/importer

CMD ["importer"]
