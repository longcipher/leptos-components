//! Line numbers display
//!
//! Provides line number gutter rendering for the editor.

/// Count the number of lines in text.
#[must_use]
pub fn count_lines(text: &str) -> usize {
    if text.is_empty() {
        1
    } else {
        text.chars().filter(|&c| c == '\n').count() + 1
    }
}

/// Get the width needed for line number display.
#[must_use]
pub fn gutter_width(line_count: usize, font_size: f32) -> f32 {
    let digit_count = if line_count == 0 {
        1
    } else {
        (line_count as f32).log10().floor() as usize + 1
    };

    let char_width = font_size * 0.6;
    let content_width = char_width * digit_count as f32;

    (content_width + 24.0).max(40.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines(""), 1);
        assert_eq!(count_lines("hello"), 1);
        assert_eq!(count_lines("hello\nworld"), 2);
        assert_eq!(count_lines("a\nb\nc\n"), 4);
    }

    #[test]
    fn test_gutter_width() {
        let width = gutter_width(100, 14.0);
        assert!(width >= 40.0);
    }
}
