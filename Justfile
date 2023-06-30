test: lint
    cargo test

lint:
    cargo clippy

release:
    cargo build -r
    cp ./target/release/rex .

install:
    cargo install --path .
