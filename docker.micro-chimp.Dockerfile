FROM blainehansen/micro-chimp:codegen as codegen

ARG SITE_NAMES_FILE=site_names.yml

COPY ${SITE_NAMES_FILE} /generated/

WORKDIR /generated

RUN node docker.codegen.js $(basename $SITE_NAMES_FILE)



FROM blainehansen/micro-chimp:rust as rust-builder

COPY --from=codegen /generated/sites.rs ./src/

# RUN cargo build --release
RUN cargo build



FROM scratch

# COPY --from=rust-builder /home/rust/src/target/x86_64-unknown-linux-musl/release/micro-chimp /server/micro-chimp
COPY --from=rust-builder /home/rust/src/target/x86_64-unknown-linux-musl/debug/micro-chimp /server/micro-chimp

EXPOSE 5050

CMD ["/server/micro-chimp"]
