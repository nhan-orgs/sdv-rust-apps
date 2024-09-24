#!/bin/bash

# When running the container, you should modify $APP_ARCH and $APP_NAME to fit your needs.

ARCH=$(uname -m)

case "$ARCH" in
    x86_64)
        ARCH_NAME="amd64"
        ;;
    aarch64)
        ARCH_NAME="arm64"
        ;;
esac

APP_NAME=${APP_NAME:-"sdv-rust-app"}
APP_ARCH=$ARCH_NAME

echo "Building the $APP_NAME app for $APP_ARCH platform"
cargo new $APP_NAME --bin
cp /app/input/main.rs /app/$APP_NAME/src/main.rs
cat /app/input/main.rs
cat /app/$APP_NAME/src/main.rs
cd /app/$APP_NAME/
cargo add tokio tonic
cargo add --path ../sdv-rust-lib/

if [ "$APP_ARCH" = "amd64" ]; then
    cargo build --release --target x86_64-unknown-linux-musl
    mv target/x86_64-unknown-linux-musl/release/${APP_NAME} /app/output/

elif [ "$APP_ARCH" = "arm64" ]; then
    cargo build --release --target aarch64-unknown-linux-musl
    mv target/aarch64-unknown-linux-musl/release/${APP_NAME} /app/output/
fi

