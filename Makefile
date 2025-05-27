build:
	cargo build --release

clean:
	rm -rf target

test:
	cargo test

run:
	cargo run

lint:
	cargo clippy

format:
	cargo fmt

.PHONY: build clean test run lint format