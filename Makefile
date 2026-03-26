CARGO := cargo

.PHONY: check

check:
	$(CARGO) clippy --all-targets --all-features -- -D warnings
	$(CARGO) fmt --all
