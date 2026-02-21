mod extract;
mod layout;
mod qr;
mod select;

use anyhow::{bail, Context, Result};
use clap::Parser;
use layout::QrGrid;
use std::io::{self, IsTerminal, Read};

/// Extract URLs from piped text and display QR codes in the terminal
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Use light terminal theme (dark QR on light background)
    #[arg(long, conflicts_with = "invert")]
    light_theme: bool,

    /// Invert QR code colors (alias for --light-theme)
    #[arg(long, conflicts_with = "light_theme")]
    invert: bool,

    /// Display QR codes for all URLs found (no selection menu)
    #[arg(long, short)]
    all: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if io::stdin().is_terminal() {
        eprintln!("Usage: echo 'text with URLs' | qrab [OPTIONS]");
        eprintln!("       curl -s https://example.com | qrab");
        eprintln!();
        eprintln!("qrab extracts URLs from piped text and displays a QR code.");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  -a, --all            Display all URLs (no selection menu)");
        eprintln!("      --light-theme    Use light terminal theme");
        eprintln!("      --invert         Invert colors (same as --light-theme)");
        eprintln!("  -h, --help           Print help");
        eprintln!("  -V, --version        Print version");
        std::process::exit(1);
    }

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .context("Failed to read from stdin")?;

    if input.trim().is_empty() {
        bail!("No input received on stdin");
    }

    let urls = extract::extract_urls(&input);

    if urls.is_empty() {
        bail!("No URLs found in the input text");
    }

    // Determine theme based on flags
    let theme = if args.light_theme || args.invert {
        qr::Theme::Light
    } else {
        qr::Theme::Dark
    };

    if args.all {
        // Display all URLs in grid layout
        eprintln!("Found {} URL(s):", urls.len());
        for url in &urls {
            eprintln!("  - {}", url);
        }
        eprintln!();

        // Generate QR codes for all URLs
        let qr_codes: Result<Vec<String>> = urls
            .iter()
            .map(|url| qr::render_qr(url, theme))
            .collect();

        let qr_codes = qr_codes?;

        // Layout in grid
        let grid = QrGrid::new(qr_codes);
        println!("{}", grid.render());
    } else {
        // Original behavior: select one URL
        let chosen = select::select_url(&urls)?;
        let qr_string = qr::render_qr(&chosen, theme)?;

        eprintln!("QR code for: {chosen}");
        println!("{qr_string}");
    }

    Ok(())
}
