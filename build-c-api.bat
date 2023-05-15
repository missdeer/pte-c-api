set "RUSTFLAGS=-C panic=abort"
cargo build --release --manifest-path Cargo.toml
cargo build --release --manifest-path Cargo.toml  --target=x86_64-pc-windows-gnu
