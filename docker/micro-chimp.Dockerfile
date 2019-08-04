FROM blainehansen/micro-chimp:rust as rust-builder
COPY ./sites.rs ./src/
RUN cargo build --release

FROM scratch
COPY --from=rust-builder /home/rust/src/target/x86_64-unknown-linux-musl/release/micro-chimp /server/micro-chimp
EXPOSE 5050
CMD ["/server/micro-chimp"]
