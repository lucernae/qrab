use anyhow::{Context, Result};
use qrcode::render::unicode;
use qrcode::QrCode;

/// Terminal theme for QR code rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    /// Dark terminal background (default)
    #[default]
    Dark,
    /// Light terminal background
    Light,
}

/// Generate a QR code for the given text and return it as a
/// unicode string suitable for terminal display.
///
/// The theme parameter controls color inversion:
/// - Dark theme: white QR code on dark background (default)
/// - Light theme: dark QR code on light background
pub fn render_qr(data: &str, theme: Theme) -> Result<String> {
    let code = QrCode::new(data.as_bytes())
        .with_context(|| format!("Failed to encode QR code for: {data}"))?;

    let image = match theme {
        Theme::Dark => {
            // Dark terminal: white QR code on dark background
            code.render::<unicode::Dense1x2>()
                .dark_color(unicode::Dense1x2::Light)
                .light_color(unicode::Dense1x2::Dark)
                .build()
        }
        Theme::Light => {
            // Light terminal: dark QR code on light background
            code.render::<unicode::Dense1x2>()
                .dark_color(unicode::Dense1x2::Dark)
                .light_color(unicode::Dense1x2::Light)
                .build()
        }
    };

    Ok(image)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_valid_url() {
        let result = render_qr("https://example.com", Theme::Dark).unwrap();
        assert!(!result.is_empty());
        // Should contain unicode block characters
        assert!(result.contains('█') || result.contains('▀') || result.contains('▄'));
    }

    #[test]
    fn renders_short_text() {
        let result = render_qr("hello", Theme::Dark).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn renders_light_theme() {
        let result = render_qr("https://example.com", Theme::Light).unwrap();
        assert!(!result.is_empty());
        // Should still contain unicode block characters
        assert!(result.contains('█') || result.contains('▀') || result.contains('▄'));
    }

    #[test]
    fn theme_default_is_dark() {
        assert_eq!(Theme::default(), Theme::Dark);
    }
}
