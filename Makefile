CARGO := cargo

.PHONY: check-all clippy fmt

check-all: clippy fmt

clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

fmt:
	$(CARGO) fmt --all
