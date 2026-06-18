all: fmt run

run *args:
    RUST_LOG=debug cargo run -- {{args}}

build:
    cargo build

check:
    cargo check

clippy:
    cargo clippy

fmt:
    cargo fmt

test:
    RUST_LOG=debug cargo test -- --show-output



