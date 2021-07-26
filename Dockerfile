FROM rust as builder

WORKDIR /miktik
COPY . .

# RUN cargo install --path .

# RUN cargo build --release 
RUN cargo build --release --target armv7-unknown-linux-gnueabihf


# FROM scratch
# FROM alpine:latest --platform=linux/arm/v7
# FROM ubuntu:latest
FROM --platform=linux/arm/v7 ubuntu:latest

WORKDIR /miktik

# COPY --from=builder /miktik/config ./config
# COPY --from=builder /miktik/templates ./templates
COPY --from=builder /miktik/target/release/metrics .

# COPY ./target/release_linux/metrics .
COPY ./config ./config
COPY ./templates ./templates

# RUN apk update
# # RUN apk add --no-cache openssl

# RUN apk add openssl
# RUN apk add bash


RUN apt update
RUN apt install openssl -y

ENTRYPOINT [ "./metrics" ]
# ENTRYPOINT [ "/bin/bash" ]