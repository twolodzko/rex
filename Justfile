test: lint unit-test integration-test

unit-test:
    cargo test

lint:
    cargo clippy

build-release:
    cargo build -r
    cp ./target/release/rex .

install:
    cargo install --path .

integration-test: build-release
    bats test.bats
