# qrab - Terminal QR Code Generator for Piped URLs

## Context

qrab is a Rust CLI tool for accessing URLs in non-graphical environments (e.g., Raspberry Pi console). When text is piped to it, it extracts URLs, lets the user pick one via an interactive menu, and displays a scannable QR code using Unicode block characters directly in the terminal. The project is currently a blank slate (just "Hello, qrab!").

## Dependencies

| Crate | Purpose |
|-------|---------|
| `linkify` | URL extraction from text (handles edge cases far better than regex) |
| `qrcode` | QR code encoding + built-in `Dense1x2` Unicode renderer |
| `dialoguer` | Interactive selection menu with `/dev/tty` support for piped stdin |
| `console` | Terminal abstraction (used by dialoguer, also used directly for `Term`) |
| `anyhow` | Ergonomic error handling for CLI |

No CLI framework (clap) needed yet -- the tool has one mode: pipe stdin, get QR.

## File Structure

```
src/
  main.rs      -- Entry point: stdin detection, reading, orchestration
  lib.rs       -- Re-exports modules for testability
  extract.rs   -- URL extraction (wraps linkify, deduplicates, preserves order)
  select.rs    -- Interactive URL selection (dialoguer + /dev/tty fallback)
  qr.rs        -- QR code generation and Dense1x2 unicode terminal rendering
```

## Implementation Steps

### Step 1: Add dependencies to `Cargo.toml`
- Add linkify, qrcode, dialoguer, console, anyhow

### Step 2: Create `src/lib.rs`
- Declare `pub mod extract; pub mod qr; pub mod select;`

### Step 3: Implement `src/extract.rs`
- `extract_urls(text: &str) -> Vec<String>` using linkify
- Filter to `LinkKind::Url` only (skip emails)
- Deduplicate while preserving first-occurrence order
- Unit tests: zero URLs, one URL, multiple, duplicates, emails excluded

### Step 4: Implement `src/qr.rs`
- `render_qr(data: &str) -> Result<String>` using qrcode crate
- Use `Dense1x2` renderer with inverted colors (dark bg terminals)
- Unit tests: valid URL produces output, contains Unicode blocks

### Step 5: Implement `src/select.rs`
- `select_url(urls: &[String]) -> Result<String>`
- 0 URLs → error, 1 URL → return directly (no interaction needed), 2+ → interactive menu
- When stdin is piped, open `/dev/tty` for interactive input (standard Unix pattern)
- Use `Term::stderr()` when stdin is already a terminal

### Step 6: Implement `src/main.rs`
- Detect if stdin is a terminal → print usage and exit
- Read all stdin, extract URLs, select one, render QR
- Print QR label to stderr, QR code to stdout

## Key Design Decisions

- **`/dev/tty` fallback**: When stdin is piped, dialoguer can't read from it. Opening `/dev/tty` directly is the standard Unix solution (same as `less`, `git`).
- **Inverted QR colors**: Terminals have dark backgrounds; swapping light/dark in the renderer produces scannable white-on-black codes.
- **Single URL bypass**: Skip the selection menu when there's only one URL — better UX and works in fully non-interactive contexts.
- **stderr for UI, stdout for QR**: Keeps output clean for potential piping/redirection.

## Verification

```bash
# Single URL - should show QR immediately
echo "Check out https://example.com" | cargo run

# Multiple URLs - should show selection menu
echo "Visit https://example.com or https://rust-lang.org" | cargo run

# No URLs - should error
echo "Hello world" | cargo run

# No pipe - should show usage
cargo run

# Scan QR with phone to verify it works

# Lint
cargo clippy && cargo fmt --check
cargo test
```
