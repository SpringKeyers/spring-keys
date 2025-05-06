.PHONY: build test watch clean check fmt install

# Default target
all: check test build

# Build the project
build:
	cargo build

# Build for release
release:
	cargo build --release

# Run all tests
test:
	cargo test

# Watch tests
watch:
	cargo watch -x test

# Clean build artifacts
clean:
	cargo clean

# Run clippy and format checks
check:
	cargo clippy
	cargo fmt -- --check

# Format code
fmt:
	cargo fmt

# Install development dependencies
setup:
	cargo install cargo-watch
	cargo install cargo-clippy

# Install the application locally
install:
	cargo install --path .

# Run specific test suite
test-suite:
	@if [ "$(suite)" ]; then \
		cargo test --test $(suite); \
	else \
		echo "Please specify a test suite with suite=<name>"; \
		echo "Example: make test-suite suite=heatmap_verification_test"; \
		exit 1; \
	fi

# Help
help:
	@echo "Available targets:"
	@echo "  make build      - Build debug version"
	@echo "  make release    - Build release version"
	@echo "  make test       - Run all tests"
	@echo "  make watch      - Watch for changes and run tests"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make check      - Run clippy and format checks"
	@echo "  make fmt        - Format code"
	@echo "  make setup      - Install development dependencies"
	@echo "  make install    - Install application locally"
	@echo "  make test-suite suite=<name> - Run specific test suite" 