
FROM boschvn/sdv-rust-lib:latest AS lib

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

FROM chef AS planner
WORKDIR /sdv-rust-app

COPY --from=lib /sdv-rust-lib ./sdv-rust-lib
RUN cargo init /sdv-rust-app
COPY Cargo.toml /sdv-rust-app/Cargo.toml
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS cook 
WORKDIR /app

COPY --from=lib /sdv-rust-lib ./sdv-rust-lib
COPY --from=planner /sdv-rust-app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.81-slim-bookworm
WORKDIR /sdv-rust-app

RUN apt-get install g++ \
    && mkdir output \
    && cargo init /sdv-rust-app

COPY --from=lib /sdv-rust-lib ./sdv-rust-lib
COPY --from=cook  /app/target /sdv-rust-app/target
COPY --from=cook /usr/local/cargo /usr/local/cargo 
COPY Cargo.toml /sdv-rust-app/Cargo.toml
COPY build.sh /build.sh

RUN chmod +x /build.sh

ENTRYPOINT [ "/build.sh" ]