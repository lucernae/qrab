use anyhow::{bail, Context, Result};
use console::Term;
use dialoguer::Select;
use std::io::IsTerminal;

/// Present a selection menu for multiple URLs.
/// If only one URL, return it immediately.
pub fn select_url(urls: &[String]) -> Result<String> {
    match urls.len() {
        0 => bail!("No URLs found in input"),
        1 => Ok(urls[0].clone()),
        _ => interactive_select(urls),
    }
}

fn interactive_select(urls: &[String]) -> Result<String> {
    // When stdin is piped, dialoguer cannot read from it.
    // Open /dev/tty directly to get an interactive terminal.
    let term = if !std::io::stdin().is_terminal() {
        let tty = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tty")
            .context("Cannot open /dev/tty -- are you in a terminal?")?;
        Term::read_write_pair(tty.try_clone()?, tty)
    } else {
        Term::stderr()
    };

    let selection = Select::new()
        .with_prompt("Select a URL to generate QR code")
        .items(urls)
        .default(0)
        .interact_on(&term)
        .context("URL selection cancelled")?;

    Ok(urls[selection].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_urls_returns_error() {
        let result = select_url(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn single_url_returns_directly() {
        let urls = vec!["https://example.com".to_string()];
        let result = select_url(&urls).unwrap();
        assert_eq!(result, "https://example.com");
    }
}
