.PHONY: build test clean fmt lint

build:
	soroban contract build

test:
	cargo test

clean:
	cargo clean

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets -- -D warnings
