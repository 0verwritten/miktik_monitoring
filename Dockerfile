FROM rust as builder

WORKDIR /miktik
COPY . .

# RUN cargo install --path .
RUN cargo build --release

# FROM scratch
# FROM alpine:latest
FROM ubuntu:latest

WORKDIR /miktik

# COPY --from=builder /miktik/config ./config
# COPY --from=builder /miktik/templates ./templates
COPY --from=builder /miktik/target/release/metrics .

# COPY ./target/release_linux/metrics .
COPY ./config ./config
COPY ./templates ./templates

# RUN apk update
# RUN apk add --no-cache openssl

# RUN apk add openssl
# RUN apk add bash


RUN apt update
RUN apt install openssl -y

ENTRYPOINT [ "./metrics" ]
# ENTRYPOINT [ "/bin/bash" ]