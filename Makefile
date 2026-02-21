.PHONY: help test build clean check fmt clippy run install dev all

# Default target
all: test build

help:
	@echo "qrab - Makefile targets"
	@echo ""
	@echo "Development:"
	@echo "  make test      - Run all tests"
	@echo "  make build     - Build debug binary"
	@echo "  make release   - Build release binary"
	@echo "  make dev       - Enter Nix development shell"
	@echo ""
	@echo "Quality:"
	@echo "  make check     - Run tests, clippy, and fmt check"
	@echo "  make clippy    - Run clippy linter"
	@echo "  make fmt       - Format code with rustfmt"
	@echo "  make fmt-check - Check code formatting"
	@echo ""
	@echo "Utilities:"
	@echo "  make run       - Run the application (pipe input required)"
	@echo "  make clean     - Remove build artifacts"
	@echo "  make install   - Install to ~/.cargo/bin"
	@echo ""
	@echo "Nix:"
	@echo "  make nix-build - Build with nix flake"
	@echo "  make nix-check - Check nix flake"
	@echo ""
	@echo "CI/CD:"
	@echo "  make ci        - Run full CI pipeline (check + test + build)"

# Run all tests
test:
	@echo "Running tests..."
	nix develop --command bash -c "cargo test --verbose"

# Build debug binary
build:
	@echo "Building debug binary..."
	nix develop --command bash -c "cargo build"
	@echo "Binary: target/debug/qrab"

# Build release binary
release:
	@echo "Building release binary..."
	nix develop --command bash -c "cargo build --release"
	@echo "Binary: target/release/qrab"

# Run all quality checks
check: fmt-check clippy test

# Run clippy
clippy:
	@echo "Running clippy..."
	nix develop --command bash -c "cargo clippy -- -D warnings"

# Format code
fmt:
	@echo "Formatting code..."
	nix develop --command bash -c "cargo fmt"

# Check formatting
fmt-check:
	@echo "Checking code formatting..."
	nix develop --command bash -c "cargo fmt --check"

# Run the application (example)
run:
	@echo "Running qrab (pipe input to use)..."
	@echo "Example: echo 'https://example.com' | make run"
	nix develop --command bash -c "cargo run"

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	nix develop --command bash -c "cargo clean"
	rm -rf result result-*

# Install to ~/.cargo/bin
install:
	@echo "Installing to ~/.cargo/bin..."
	nix develop --command bash -c "cargo install --path ."

# Enter development shell
dev:
	@echo "Entering Nix development shell..."
	nix develop

# Nix flake build
nix-build:
	@echo "Building with Nix flake..."
	nix build
	@echo "Binary: result/bin/qrab"

# Nix flake check
nix-check:
	@echo "Checking Nix flake..."
	nix flake check

# Full CI pipeline
ci: check test build
	@echo "CI pipeline completed successfully!"
