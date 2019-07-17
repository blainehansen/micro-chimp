FROM rust:1

RUN cargo install cargo-build-deps

COPY Cargo.toml Cargo.lock /build/
COPY src/main.rs /build/src/

WORKDIR /build
RUN cargo build-deps --release
