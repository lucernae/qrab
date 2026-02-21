use anyhow::{Context, Result};
use qrcode::render::unicode;
use qrcode::QrCode;

/// Generate a QR code for the given text and return it as a
/// unicode string suitable for terminal display.
pub fn render_qr(data: &str) -> Result<String> {
    let code = QrCode::new(data.as_bytes())
        .with_context(|| format!("Failed to encode QR code for: {data}"))?;

    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();

    Ok(image)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_valid_url() {
        let result = render_qr("https://example.com").unwrap();
        assert!(!result.is_empty());
        // Should contain unicode block characters
        assert!(result.contains('█') || result.contains('▀') || result.contains('▄'));
    }

    #[test]
    fn renders_short_text() {
        let result = render_qr("hello").unwrap();
        assert!(!result.is_empty());
    }
}
