FROM rust:latest as builder

WORKDIR /usr/src/app

COPY migrations/ migrations/
COPY src/ src/
COPY Cargo.toml Cargo.toml

RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/importer /usr/local/bin/importer
CMD ["importer"]
