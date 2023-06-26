
release: lint
    cargo build -r
    cp ./target/release/rex .

lint:
    cargo clippy

install:
    cargo install --path .
