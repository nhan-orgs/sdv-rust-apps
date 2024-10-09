#!/bin/bash

APP_NAME=${APP_NAME:-"sdv-rust-app"}

cp /app/input/main.rs /sdv-rust-app/src/main.rs
cargo build --release 
mv target/release/sdv-rust-app /app/output/${APP_NAME}
