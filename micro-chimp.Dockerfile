FROM rust:1 as rust-deps-builder

RUN cargo install cargo-build-deps

COPY Cargo.toml Cargo.lock /build
COPY src/main.rs /build/src

WORKDIR /build
RUN cargo build-deps --release



FROM node:11 as node-codegen

COPY ./docker/codegen.js /build/

RUN npm install yaml
RUN npm install snake-case

ARG SITE_NAMES_FILE=site_names.yml

COPY ${SITE_NAMES_FILE} /build

WORKDIR /build

RUN node codegen.js $SITE_NAMES_FILE



FROM rust-deps-builder as rust-builder

COPY --from=node-codegen /build/sites.rs /build/src

RUN cargo build --release



FROM scratch

COPY --from=rust-builder /build/src/target/release/micro_chimp /server/micro_chimp

CMD ./server/micro_chimp
