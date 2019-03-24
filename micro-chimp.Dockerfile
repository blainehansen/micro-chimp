FROM blainehansen:micro-chimp-node-codegen as node-codegen

ARG SITE_NAMES_FILE=site_names.yml

COPY ${SITE_NAMES_FILE} /build

WORKDIR /build

RUN node rust-codegen.js $SITE_NAMES_FILE



FROM blainehansen:micro-chimp-rust-builder as rust-builder

COPY --from=node-codegen /build/sites.rs /build/src

RUN cargo build --release



FROM scratch
COPY --from=rust-builder /build/src/target/release/micro_chimp /server/micro_chimp
CMD ./server/micro_chimp
