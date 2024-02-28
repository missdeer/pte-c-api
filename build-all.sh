#!/bin/sh
export RUSTFLAGS="-C panic=abort"
cargo build --release --target=aarch64-apple-darwin
cargo build --release --target=x86_64-apple-darwin
lipo -create -output libpte.a target/x86_64-apple-darwin/release/libpte.a target/aarch64-apple-darwin/release/libpte.a
cargo build --release --target=x86_64-pc-windows-gnu
cargo build --release --target=x86_64-pc-windows-msvc
