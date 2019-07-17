FROM blainehansen/micro-chimp:codegen as codegen

ARG SITE_NAMES_FILE=site_names.yml

COPY ${SITE_NAMES_FILE} /generated/

WORKDIR /generated

RUN node codegen.js $(basename $SITE_NAMES_FILE)



FROM blainehansen/micro-chimp:rust as rust-builder

COPY --from=codegen /generated/sites.rs /build/src/

RUN cargo build --release



FROM scratch

COPY --from=rust-builder /build/target/release/micro-chimp /server/micro-chimp

CMD ./server/micro-chimp
