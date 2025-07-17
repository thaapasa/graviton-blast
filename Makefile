.PHONY: check
check:
	cargo check --all-features --tests

.PHONY: clippy
clippy:
	cargo clippy --all-features --tests

.PHONY: lint
lint:
	cargo +nightly fmt
	make check
	make clippy

.PHONY: test
test:
	cargo test

.PHONY: run
run:
	cargo run
