FROM rust:alpine3.16 as base
RUN apk update \
    && apk add \
        git \
        gcc \
        g++ \
        openssl \
        openssl-dev \
        pkgconfig

COPY . /src

RUN rustup update 1.64 && rustup default 1.64

RUN cd /src \
    && sed -i -e "s/openssl.*=.*//" Cargo.toml \
    && RUSTFLAGS="-C target-feature=-crt-static" cargo build --release

FROM alpine:3.17 as tool

RUN apk update && apk add libgcc

ENTRYPOINT [ "routez" ]

COPY --from=base /src/target/release/routez /usr/local/bin
