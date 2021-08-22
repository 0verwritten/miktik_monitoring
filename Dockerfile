# run this command beforehand: 
# docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder cargo build --release

FROM scratch

ADD target/x86_64-unknown-linux-musl/release/metrics /
COPY config /config
COPY templates /templates

EXPOSE 7878

CMD ["/metrics"]