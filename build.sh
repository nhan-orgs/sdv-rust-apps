#!/bin/bash

APP_NAME=${APP_NAME:-"sdv-rust-app"}

cp /app/input/main.rs /$APP_NAME/src/main.rs
cargo build --release 
mv target/release/${APP_NAME} /app/output/
