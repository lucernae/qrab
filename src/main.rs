mod extract;
mod qr;
mod select;

use anyhow::{bail, Context, Result};
use std::io::{self, IsTerminal, Read};

fn main() -> Result<()> {
    if io::stdin().is_terminal() {
        eprintln!("Usage: echo 'text with URLs' | qrab");
        eprintln!("       curl -s https://example.com | qrab");
        eprintln!();
        eprintln!("qrab extracts URLs from piped text and displays a QR code.");
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

    let chosen = select::select_url(&urls)?;

    let qr_string = qr::render_qr(&chosen)?;

    eprintln!("QR code for: {chosen}");
    println!("{qr_string}");

    Ok(())
}
