//! Language-specific text configuration for the TUI.
//!
//! This module provides language detection and configuration for proper text
//! rendering, especially for languages that require special handling like
//! Armenian (hy).

/// Returns true if the given character is an Armenian letter.
///
/// Armenian characters are in the Unicode range U+0530 to U+058F (Armenian
/// alphabet) and U+FB13 to U+FB17 (Armenian ligatures).
pub fn is_armenian_char(c: char) -> bool {
    matches!(c,
        // Armenian alphabet (U+0530 to U+058F)
        '\u{0530}'..='\u{058F}' |
        // Armenian ligatures (U+FB13 to U+FB17)
        '\u{FB13}'..='\u{FB17}'
    )
}

/// Returns true if the given string contains any Armenian characters.
pub fn contains_armenian(text: &str) -> bool {
    text.chars().any(is_armenian_char)
}

/// Returns true if the given locale string represents Armenian.
///
/// This matches locales like "hy", "hy-AM", "hy-RU", etc.
pub fn is_armenian_locale(locale: &str) -> bool {
    locale.starts_with("hy")
}

/// Detects if the given text or locale indicates Armenian content.
///
/// Returns true if either the text contains Armenian characters or the
/// locale is Armenian.
pub fn is_armenian(text: &str, locale: Option<&str>) -> bool {
    if contains_armenian(text) {
        return true;
    }
    if let Some(loc) = locale {
        if is_armenian_locale(loc) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_armenian_char() {
        // Armenian letters
        assert!(is_armenian_char('Ա')); // U+0531 - Armenian capital letter Ayb
        assert!(is_armenian_char('դ')); // U+0564 - Armenian small letter da
        assert!(is_armenian_char('։')); // U+0589 - Armenian full stop
        
        // Non-Armenian
        assert!(!is_armenian_char('A'));
        assert!(!is_armenian_char('а')); // Cyrillic
        assert!(!is_armenian_char('α')); // Greek
    }

    #[test]
    fn test_contains_armenian() {
        assert!(contains_armenian("Ողորմություն")); // "Forgiveness" in Armenian
        assert!(contains_armenian("Hello Անուն"));
        assert!(!contains_armenian("Hello World"));
    }

    #[test]
    fn test_is_armenian_locale() {
        assert!(is_armenian_locale("hy"));
        assert!(is_armenian_locale("hy-AM"));
        assert!(is_armenian_locale("hy-RU"));
        assert!(!is_armenian_locale("en"));
        assert!(!is_armenian_locale("en-US"));
        assert!(!is_armenian_locale("ru"));
    }

    #[test]
    fn test_is_armenian() {
        // Armenian text
        assert!(is_armenian("Ողորմություն", None));
        
        // Armenian locale
        assert!(is_armenian("Hello", Some("hy-AM")));
        
        // Neither
        assert!(!is_armenian("Hello", Some("en-US")));
    }
}
