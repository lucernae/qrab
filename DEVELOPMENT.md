# Development Guide

This document explains how to build, test, and develop qrab.

## Prerequisites

### Option 1: Nix (Recommended)

The project uses Nix flakes for a reproducible development environment.

```bash
# Enter development shell
nix develop

# This provides:
# - Rust stable toolchain (latest)
# - cargo, rustc, rustfmt, clippy
# - rust-analyzer for IDE support
```

### Option 2: Manual Setup

- Rust 1.70+ (2021 edition)
- cargo

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Quick Start with Makefile

The project includes a Makefile for common development tasks:

```bash
# Show all available targets
make help

# Run tests
make test

# Build debug binary
make build

# Build release binary
make release

# Run full CI pipeline (tests + clippy + format check + build)
make ci

# Clean build artifacts
make clean
```

**Note:** All Makefile targets use `nix develop` internally, ensuring consistent environment.

## Project Structure

```
qrab/
├── src/
│   ├── main.rs      # Entry point, stdin handling, orchestration
│   ├── lib.rs       # Module exports for testability
│   ├── extract.rs   # URL extraction (linkify wrapper)
│   ├── select.rs    # Interactive URL selection (dialoguer + /dev/tty)
│   ├── qr.rs        # QR code generation (Dense1x2 unicode rendering)
│   └── layout.rs    # Grid layout for multiple QR codes
├── tests/           # Integration tests
├── Cargo.toml       # Dependencies and package metadata
├── flake.nix        # Nix development environment
└── PLAN.md          # Original implementation plan
```

## Building

### Development Build

```bash
cargo build
```

Binary location: `target/debug/qrab`

### Release Build

```bash
cargo build --release
```

Binary location: `target/release/qrab` (optimized)

### With Nix

```bash
nix develop --command cargo build
```

## Testing

### Run All Tests

```bash
cargo test
```

This runs:
- Unit tests in `src/extract.rs` (6 tests)
- Unit tests in `src/qr.rs` (2 tests)
- Unit tests in `src/select.rs` (2 tests)
- Integration tests in `tests/` directory

### Run Specific Test Suite

```bash
# Only unit tests in extract module
cargo test extract::tests

# Only integration tests
cargo test --test integration

# Run a specific test
cargo test test_single_url
```

### Test with Output

```bash
# Show println/eprintln output from tests
cargo test -- --nocapture

# Show test names as they run
cargo test -- --test-threads=1 --nocapture
```

### Unit Tests Overview

#### `src/extract.rs` Tests

- `no_urls` - Empty result when no URLs in text
- `single_url` - Extracts one URL correctly
- `multiple_urls` - Extracts multiple URLs in order
- `deduplicates` - Removes duplicate URLs
- `excludes_emails` - Ignores email addresses
- `url_with_trailing_punctuation` - Handles URLs with trailing periods

#### `src/qr.rs` Tests

- `renders_valid_url` - QR code generation succeeds
- `renders_short_text` - Handles short input strings
- `renders_light_theme` - Light theme rendering works
- `theme_default_is_dark` - Default theme is dark

#### `src/select.rs` Tests

- `zero_urls_returns_error` - Error when no URLs provided
- `single_url_returns_directly` - Bypasses menu for single URL

#### `src/layout.rs` Tests

- `merge_single_qr` - Single QR code merging works
- `merge_two_qr_codes` - Side-by-side QR code merging
- `merge_different_heights` - Handles QR codes of different heights
- `empty_qr_codes` - Empty input handling

### Integration Tests

Located in `tests/integration.rs`, these test the full pipeline:

```bash
cargo test --test integration
```

Tests include:
- Piping single URL to qrab
- Piping multiple URLs (non-interactive tests only)
- Error cases (no URLs, empty input)

### Manual Integration Testing

#### Test Single URL

```bash
echo "https://example.com" | cargo run
```

Expected: QR code displayed immediately, no menu.

#### Test Multiple URLs

```bash
echo "Visit https://example.com or https://rust-lang.org" | cargo run
```

Expected: Interactive menu appears. Use arrow keys to select.

#### Test No URLs

```bash
echo "Hello world" | cargo run
```

Expected: Error message "No URLs found in the input text", exit code 1.

#### Test Empty Input

```bash
echo "" | cargo run
```

Expected: Error message "No input received on stdin", exit code 1.

#### Test No Pipe

```bash
cargo run
```

Expected: Usage message, exit code 1.

#### Test from curl

```bash
curl -s https://example.com | cargo run
```

Expected: QR code for one of the URLs found in the HTML.

#### Test Theme Support

```bash
# Test dark theme (default)
echo "https://example.com" | cargo run

# Test light theme
echo "https://example.com" | cargo run -- --light-theme

# Test invert flag (alias)
echo "https://example.com" | cargo run -- --invert
```

Expected: QR codes with inverted colors for light theme.

#### Test --all Flag (Grid Layout)

```bash
# Test with 2 URLs (should display side-by-side)
echo "Visit https://example.com or https://rust-lang.org" | cargo run -- --all

# Test with many URLs (should wrap to multiple rows)
echo "https://example.com https://rust-lang.org https://github.com https://crates.io" | cargo run -- --all

# Combine with theme flag
echo "https://example.com https://rust-lang.org" | cargo run -- --all --light-theme
```

Expected: Multiple QR codes arranged in a grid, wrapping based on terminal width.

## Linting and Formatting

### Run Clippy

```bash
cargo clippy
```

All code should pass clippy with zero warnings.

### Check Formatting

```bash
cargo fmt --check
```

### Auto-format Code

```bash
cargo fmt
```

### Strict Clippy

```bash
cargo clippy -- -W clippy::pedantic
```

## Code Coverage

Using `cargo-tarpaulin`:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage
```

Open `coverage/index.html` to view the report.

## Benchmarking

For performance testing:

```bash
# Create a large input file
for i in {1..1000}; do echo "https://example.com/$i"; done > urls.txt

# Time the execution
time cat urls.txt | cargo run --release
```

## Dependencies

### Core Dependencies

- **linkify** (0.10) - URL extraction from plain text
- **qrcode** (0.14) - QR code encoding and rendering
- **dialoguer** (0.12) - Interactive terminal prompts
- **console** (0.16) - Terminal abstraction
- **anyhow** (1.x) - Error handling
- **clap** (4.x) - Command-line argument parsing

### Updating Dependencies

```bash
# Check for outdated dependencies
cargo outdated

# Update to latest compatible versions
cargo update

# Update Cargo.lock
cargo update --aggressive
```

## Common Development Tasks

### Add a New Dependency

1. Edit `Cargo.toml`:
   ```toml
   [dependencies]
   new-crate = "1.0"
   ```

2. Run `cargo build` to download and compile.

### Add a New Module

1. Create `src/newmodule.rs`
2. Add to `src/lib.rs`:
   ```rust
   pub mod newmodule;
   ```
3. Import in `src/main.rs`:
   ```rust
   mod newmodule;
   ```

### Debug a Test

```bash
# Run specific test with debug output
RUST_LOG=debug cargo test test_name -- --nocapture
```

### Profile Performance

```bash
# Build with debug symbols for profiling
cargo build --release --profile profiling

# Run with perf (Linux)
perf record ./target/release/qrab < urls.txt
perf report
```

## Troubleshooting

### Tests Fail in CI But Pass Locally

- Check Rust version: `cargo --version`
- Clean build: `cargo clean && cargo build`
- Update dependencies: `cargo update`

### `/dev/tty` Not Available

The selection menu requires `/dev/tty` for interactive input when stdin is piped. This is standard on Linux/macOS but may not work in:
- Docker containers without TTY allocation
- CI environments without interactive terminal
- Some SSH configurations

For single-URL inputs, no TTY is needed.

### QR Code Not Scanning

- Ensure terminal font supports Unicode block characters
- Try inverting terminal colors (light theme vs dark theme)
- Increase terminal font size
- Ensure good lighting and camera focus

## Continuous Integration

The project can use GitHub Actions for CI:

```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test
      - run: cargo clippy -- -D warnings
      - run: cargo fmt --check
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make changes and add tests
4. Run tests: `cargo test`
5. Run linting: `cargo clippy && cargo fmt`
6. Commit with descriptive message
7. Push and create a pull request

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` (if exists)
3. Run full test suite: `cargo test`
4. Build release binary: `cargo build --release`
5. Tag release: `git tag v0.1.0`
6. Push tag: `git push origin v0.1.0`
7. Publish to crates.io: `cargo publish`

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/)
- [linkify docs](https://docs.rs/linkify/)
- [qrcode docs](https://docs.rs/qrcode/)
- [dialoguer docs](https://docs.rs/dialoguer/)
