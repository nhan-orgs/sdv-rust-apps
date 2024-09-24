
FROM boschvn/sdv-rust-lib:latest AS lib

FROM rust:1.81-bookworm AS builder
WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl aarch64-unknown-linux-musl && mkdir input output 
COPY --from=lib /sdv-rust-lib /app/sdv-rust-lib
COPY sample-compile/src/main.rs /app/input/main.rs

RUN cargo add tokio tonic && cargo add --path ../sdv-rust-lib/

COPY build.sh /build.sh
RUN chmod +x /build.sh
ENTRYPOINT [ "/build.sh" ]