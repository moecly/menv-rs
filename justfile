all: fmt run

run:
    RUST_LOG=debug cargo run

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



