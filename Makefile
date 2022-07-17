LOG_LEVEL ?= cargo-cm=trace

.PHONY: clean
clean:
	rm -rf target

.PHONY: build
build:
	cargo build --release

.PHONY: test
test:
	cargo test

.PHONY: lint
lint:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo fmt --all -- --check

