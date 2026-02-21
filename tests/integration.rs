use qrab::{extract, qr, select};

#[test]
fn test_extract_single_url() {
    let text = "Check out https://example.com for more info";
    let urls = extract::extract_urls(text);
    assert_eq!(urls.len(), 1);
    assert_eq!(urls[0], "https://example.com");
}

#[test]
fn test_extract_multiple_urls() {
    let text = "Visit https://example.com or https://rust-lang.org for details";
    let urls = extract::extract_urls(text);
    assert_eq!(urls.len(), 2);
    assert_eq!(urls[0], "https://example.com");
    assert_eq!(urls[1], "https://rust-lang.org");
}

#[test]
fn test_extract_no_urls() {
    let text = "This is just plain text without any links";
    let urls = extract::extract_urls(text);
    assert!(urls.is_empty());
}

#[test]
fn test_extract_url_in_html() {
    let html = r#"<a href="https://example.com">Link</a>"#;
    let urls = extract::extract_urls(html);
    assert_eq!(urls.len(), 1);
    assert_eq!(urls[0], "https://example.com");
}

#[test]
fn test_extract_mixed_content() {
    let text = r#"
        Email me at user@example.com or visit
        https://example.com and also check
        http://another-site.org for updates.
        Duplicate: https://example.com again
    "#;
    let urls = extract::extract_urls(text);
    // Should find 2 unique URLs (excluding email)
    assert_eq!(urls.len(), 2);
    assert!(urls.contains(&"https://example.com".to_string()));
    assert!(urls.contains(&"http://another-site.org".to_string()));
}

#[test]
fn test_select_single_url_no_interaction() {
    let urls = vec!["https://example.com".to_string()];
    let result = select::select_url(&urls);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "https://example.com");
}

#[test]
fn test_select_zero_urls_error() {
    let urls: Vec<String> = vec![];
    let result = select::select_url(&urls);
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("No URLs found"));
}

#[test]
fn test_qr_render_valid_url() {
    let result = qr::render_qr("https://example.com", qr::Theme::Dark);
    assert!(result.is_ok());
    let qr_code = result.unwrap();

    // Should contain Unicode block characters
    assert!(
        qr_code.contains('█') || qr_code.contains('▀') || qr_code.contains('▄'),
        "QR code should contain Unicode block characters"
    );

    // Should have multiple lines
    assert!(qr_code.lines().count() > 5, "QR code should be multi-line");
}

#[test]
fn test_qr_render_short_text() {
    let result = qr::render_qr("hello", qr::Theme::Dark);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
}

#[test]
fn test_qr_render_long_url() {
    let long_url = "https://example.com/very/long/path/with/many/segments/and/query?param1=value1&param2=value2&param3=value3";
    let result = qr::render_qr(long_url, qr::Theme::Dark);
    assert!(result.is_ok());
    assert!(!result.unwrap().is_empty());
}

#[test]
fn test_full_pipeline_single_url() {
    // Simulate the full flow: extract -> select -> render
    let input = "Check this out: https://rust-lang.org";

    // Extract
    let urls = extract::extract_urls(input);
    assert_eq!(urls.len(), 1);

    // Select (should return immediately for single URL)
    let chosen = select::select_url(&urls).unwrap();
    assert_eq!(chosen, "https://rust-lang.org");

    // Render
    let qr_code = qr::render_qr(&chosen, qr::Theme::Dark).unwrap();
    assert!(!qr_code.is_empty());
}

#[test]
fn test_full_pipeline_multiple_urls_first_choice() {
    let input = "Visit https://example.com or https://rust-lang.org";

    let urls = extract::extract_urls(input);
    assert_eq!(urls.len(), 2);

    // For automated testing, we can't interact with the menu,
    // but we can verify that both URLs are valid for QR generation
    for url in &urls {
        let qr_code = qr::render_qr(url, qr::Theme::Dark).unwrap();
        assert!(!qr_code.is_empty());
    }
}

#[test]
fn test_extract_urls_from_markdown() {
    let markdown = r#"
# Documentation

Check out these resources:
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- Visit https://crates.io for packages

Email: support@rust-lang.org
"#;
    let urls = extract::extract_urls(markdown);
    assert_eq!(urls.len(), 3);
    assert!(urls.contains(&"https://doc.rust-lang.org/book/".to_string()));
    assert!(urls.contains(&"https://doc.rust-lang.org/cargo/".to_string()));
    assert!(urls.contains(&"https://crates.io".to_string()));
}

#[test]
fn test_extract_urls_preserve_order() {
    let text = "First https://first.com then https://second.com and https://third.com";
    let urls = extract::extract_urls(text);
    assert_eq!(urls[0], "https://first.com");
    assert_eq!(urls[1], "https://second.com");
    assert_eq!(urls[2], "https://third.com");
}

#[test]
fn test_qr_special_characters() {
    // URLs with special characters that need proper encoding
    let urls = vec![
        "https://example.com/path?query=value&other=123",
        "https://example.com/path#fragment",
        "https://example.com/path%20with%20spaces",
    ];

    for url in urls {
        let result = qr::render_qr(url, qr::Theme::Dark);
        assert!(result.is_ok(), "Failed to encode URL: {}", url);
    }
}

#[test]
fn test_extract_international_urls() {
    let text = "Visit https://例え.jp and https://münchen.de";
    let urls = extract::extract_urls(text);
    assert_eq!(urls.len(), 2);
}

#[test]
fn test_qr_render_light_theme() {
    let result = qr::render_qr("https://example.com", qr::Theme::Light);
    assert!(result.is_ok());
    let qr_code = result.unwrap();
    assert!(!qr_code.is_empty());
    assert!(qr_code.contains('█') || qr_code.contains('▀') || qr_code.contains('▄'));
}

#[test]
fn test_qr_render_dark_theme() {
    let result = qr::render_qr("https://example.com", qr::Theme::Dark);
    assert!(result.is_ok());
    let qr_code = result.unwrap();
    assert!(!qr_code.is_empty());
}

#[test]
fn test_theme_default() {
    assert_eq!(qr::Theme::default(), qr::Theme::Dark);
}
