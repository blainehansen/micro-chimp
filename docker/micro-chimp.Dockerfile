FROM blainehansen/micro-chimp:codegen as codegen
COPY sites_manifest.yml .
RUN npx ts-node codegen.ts

FROM blainehansen/micro-chimp:rust as rust-builder
COPY --from=codegen /home/sites.rs ./src/
RUN cargo build --release

FROM scratch
COPY --from=rust-builder /home/rust/src/target/x86_64-unknown-linux-musl/release/micro-chimp /server/micro-chimp
EXPOSE 5050
CMD ["/server/micro-chimp"]
