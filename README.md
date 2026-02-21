# qrab

[![Build Status](https://github.com/lucernae/qrab/workflows/Build/badge.svg)](https://github.com/lucernae/qrab/actions/workflows/build.yml)
[![Tests](https://github.com/lucernae/qrab/workflows/Tests/badge.svg)](https://github.com/lucernae/qrab/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)](https://www.rust-lang.org)

**QR Code Generator for Terminal-Friendly URL Access**

`qrab` is a Rust CLI tool that extracts URLs from piped text and displays scannable QR codes directly in your terminal using Unicode block characters. Perfect for accessing URLs in non-graphical environments like Raspberry Pi consoles, SSH sessions, or headless servers.

## Features

- **Pipe-friendly**: Accepts text from stdin, extracts all URLs automatically
- **Interactive selection**: When multiple URLs are found, presents a selection menu
- **Terminal QR codes**: Uses Unicode half-block characters for compact, scannable QR codes
- **Theme support**: Automatic dark theme, with `--light-theme` flag for light terminals
- **Works on bare consoles**: No X11 or graphical environment needed
- **Smart deduplication**: Same URL appearing multiple times shows up once
- **Email filtering**: Ignores email addresses, focuses only on URLs

## Installation

### Using Nix Flakes (recommended)

#### NixOS Configuration

Add to your NixOS configuration (`/etc/nixos/configuration.nix` or flake):

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    qrab.url = "github:lucernae/qrab";
  };

  outputs = { nixpkgs, qrab, ... }: {
    nixosConfigurations.your-hostname = nixpkgs.lib.nixosSystem {
      modules = [
        {
          environment.systemPackages = [
            qrab.packages.x86_64-linux.default
            # or qrab.packages.aarch64-linux.default for ARM
          ];
        }
      ];
    };
  };
}
```

#### Home Manager Configuration

Add to your home-manager configuration:

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    home-manager.url = "github:nix-community/home-manager";
    qrab.url = "github:lucernae/qrab";
  };

  outputs = { nixpkgs, home-manager, qrab, ... }: {
    homeConfigurations.your-user = home-manager.lib.homeManagerConfiguration {
      modules = [
        {
          home.packages = [
            qrab.packages.x86_64-linux.default
            # or qrab.packages.aarch64-linux.default for ARM
            # or qrab.packages.x86_64-darwin.default for macOS Intel
            # or qrab.packages.aarch64-darwin.default for macOS ARM
          ];
        }
      ];
    };
  };
}
```

#### Try without installing

```bash
nix run github:lucernae/qrab -- --help
```

### Build from source with Nix

```bash
git clone https://github.com/lucernae/qrab
cd qrab
nix build
./result/bin/qrab --help
```

### Using Cargo

```bash
git clone https://github.com/lucernae/qrab
cd qrab
cargo build --release
```

### Install to PATH

```bash
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Pipe text containing URLs
echo "Check out https://example.com for details" | qrab

# From curl output
curl -s https://example.com | qrab

# From clipboard (with xclip)
xclip -o | qrab

# From file
cat urls.txt | qrab
```

### Multiple URLs

When multiple URLs are detected, qrab presents an interactive selection menu:

```bash
echo "Visit https://example.com or https://rust-lang.org" | qrab
```

Output:
```
Select a URL to generate QR code
> https://example.com
  https://rust-lang.org

QR code for: https://example.com
█████████████████████████████████
...
```

Use arrow keys to select, Enter to confirm.

### Terminal Theme Support

By default, qrab assumes a dark terminal background. For light-themed terminals, use the `--light-theme` or `--invert` flag:

```bash
# For light terminal backgrounds
echo "https://example.com" | qrab --light-theme

# Alternative flag (same effect)
echo "https://example.com" | qrab --invert
```

This inverts the QR code colors for better contrast on light backgrounds.

### Display All URLs

Use the `--all` or `-a` flag to generate QR codes for all URLs without the selection menu. QR codes are arranged in a grid layout, wrapping to new rows based on terminal width:

```bash
# Display all URLs in grid layout
echo "Visit https://example.com or https://rust-lang.org" | qrab --all

# Combine with theme flag
curl -s https://example.com | qrab --all --light-theme
```

This is useful when you want to quickly scan multiple URLs without interactive selection.

### Single URL (No Interaction)

When only one URL is found, qrab skips the menu and displays the QR code immediately:

```bash
echo "https://example.com" | qrab
```

### No Pipe

Running `qrab` without piped input shows usage help:

```bash
qrab
```

## Examples

### SSH Session URL Access

On a remote server without a browser:

```bash
# Generate deployment URL
./deployment-script.sh | grep "https://" | qrab
# Scan with phone to open the URL
```

### Raspberry Pi Console

```bash
# Get WiFi setup URL
cat /var/log/setup.log | qrab
```

### GitHub Actions Logs

```bash
gh run view 123456 --log | qrab
# Scan the artifact download URL
```

### Tailscale Login URL

Authorize Tailscale from a headless server:

```bash
# Start tailscale and pipe the login URL to qrab
sudo tailscale up 2>&1 | qrab

# The authentication URL will be extracted and displayed as a QR code
# Scan with your phone to complete login
```

This is particularly useful for:
- Headless servers without a browser
- SSH sessions where copying URLs is cumbersome
- Initial device setup on Raspberry Pi or similar devices

## Requirements

- Terminal with UTF-8 support (for Unicode block characters)
- For interactive selection with piped stdin: `/dev/tty` access (available on Linux/macOS)

## Error Handling

- **No URLs found**: Clear error message with exit code 1
- **Empty input**: Error message indicating no input received
- **No terminal available**: Selection menu fails gracefully with context

## How It Works

1. **URL Extraction**: Uses the `linkify` crate to find all URLs in text
2. **Deduplication**: Removes duplicate URLs, preserving first occurrence
3. **Selection**: Single URL → direct display; multiple → interactive menu
4. **QR Generation**: Uses `qrcode` crate with Dense1x2 Unicode rendering
5. **Terminal Display**: Inverted colors (white-on-black) for dark terminal backgrounds

## Development

See [DEVELOPMENT.md](DEVELOPMENT.md) for build instructions, testing, and contribution guidelines.

## Similar Projects & Acknowledgements

While `qrab` focuses on pipe-friendly URL extraction and terminal QR display, there are other excellent QR code generators worth mentioning:

- **[qrencode](https://fukuchi.org/works/qrencode/)** - Classic QR code generator (C-based) with terminal output support
- **[qr](https://github.com/fujiapple852/qr)** - Simple QR code generator CLI in Rust
- **[qrtool](https://github.com/sorairolake/qrtool)** - Another Rust-based QR code utility
- **[qrcp](https://github.com/claudiodangelis/qrcp)** - Transfer files via QR code (Go-based)

`qrab` differentiates itself by:
- Automatic URL extraction from piped text (no need to manually copy URLs)
- Interactive selection menu for multiple URLs
- Grid layout for displaying all URLs at once
- Specific focus on terminal-based URL workflow

## License

MIT

See [LICENSE](LICENSE) file for details.

## Repository

https://github.com/lucernae/qrab
