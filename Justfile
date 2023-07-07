test: lint unit-test integration-test

unit-test:
    cargo test

lint:
    cargo clippy

release:
    cargo build -r
    cp ./target/release/rex .

install:
    cargo install --path .

integration-test: release
    bats test.bats
