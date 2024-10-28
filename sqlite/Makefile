# Makefile for Rust project CI pipeline

# Define the cargo command for convenience
CARGO := cargo

# Target for checking code (linting, warnings)
check:
	$(CARGO) clippy --all-targets -- -D warnings

# Target for formatting code according to Rust's standards
format:
	$(CARGO) fmt -- --check

# Target for running tests
test:
	$(CARGO) test --all

# Target for building an optimized release binary
release:
	$(CARGO) build --release
