run:
	cargo run -- parse TESTME.md

test:
	cargo test

credits:
	cargo run -- credits

help:
	cargo run -- help

fmt:
	cargo fmt

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

precommit:
	cargo fmt
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test

clean:
	cargo clean

build:
	cargo build