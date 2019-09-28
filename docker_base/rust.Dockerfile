FROM ekidd/rust-musl-builder:latest

RUN cargo install cargo-build-deps

COPY Cargo.toml Cargo.lock ./
COPY src/main.rs ./src/

RUN sudo chown -R rust:rust /home/rust

RUN cargo build-deps
# RUN cargo build-deps --release
