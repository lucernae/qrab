use linkify::{LinkFinder, LinkKind};
use std::collections::HashSet;

/// Extract all URLs from the given text.
/// Returns deduplicated URLs preserving first-occurrence order.
pub fn extract_urls(text: &str) -> Vec<String> {
    let finder = LinkFinder::new();
    let mut seen = HashSet::new();
    finder
        .links(text)
        .filter(|link| *link.kind() == LinkKind::Url)
        .filter_map(|link| {
            let url = link.as_str().to_string();
            if seen.insert(url.clone()) {
                Some(url)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_urls() {
        let result = extract_urls("hello world, no links here");
        assert!(result.is_empty());
    }

    #[test]
    fn single_url() {
        let result = extract_urls("check out https://example.com for details");
        assert_eq!(result, vec!["https://example.com"]);
    }

    #[test]
    fn multiple_urls() {
        let result = extract_urls("visit https://example.com or https://rust-lang.org for more");
        assert_eq!(result, vec!["https://example.com", "https://rust-lang.org"]);
    }

    #[test]
    fn deduplicates() {
        let result = extract_urls("https://example.com and again https://example.com");
        assert_eq!(result, vec!["https://example.com"]);
    }

    #[test]
    fn excludes_emails() {
        let result = extract_urls("email user@example.com or visit https://example.com");
        assert_eq!(result, vec!["https://example.com"]);
    }

    #[test]
    fn url_with_trailing_punctuation() {
        let result = extract_urls("see https://example.com.");
        assert_eq!(result, vec!["https://example.com"]);
    }
}
