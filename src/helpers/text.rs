//! Text processing utilities
//!
//! Provides efficient text analysis and manipulation functions.

/// Count the number of lines in a string.
///
/// Returns at least 1 for an empty string (representing a single empty line).
///
/// # Examples
///
/// ```
/// use longcipher_leptos_components::helpers::count_lines;
///
/// assert_eq!(count_lines(""), 1);
/// assert_eq!(count_lines("hello"), 1);
/// assert_eq!(count_lines("hello\nworld"), 2);
/// ```
#[must_use]
pub fn count_lines(text: &str) -> usize {
    if text.is_empty() {
        1
    } else {
        text.chars().filter(|&c| c == '\n').count() + 1
    }
}

/// Calculate basic text statistics.
///
/// Returns a tuple of (words, characters, `characters_no_spaces`, lines).
#[must_use]
pub fn text_stats(text: &str) -> (usize, usize, usize, usize) {
    if text.is_empty() {
        return (0, 0, 0, 1);
    }

    let mut words = 0;
    let mut chars = 0;
    let mut chars_no_spaces = 0;
    let mut in_word = false;

    for ch in text.chars() {
        chars += 1;

        if ch.is_whitespace() {
            if in_word {
                in_word = false;
            }
        } else {
            chars_no_spaces += 1;
            if !in_word {
                in_word = true;
                words += 1;
            }
        }
    }

    let lines = count_lines(text);

    (words, chars, chars_no_spaces, lines)
}

/// Get the line and column position from a character offset.
///
/// Both line and column are 0-indexed.
#[must_use]
#[allow(clippy::explicit_counter_loop)]
pub fn offset_to_position(text: &str, offset: usize) -> (usize, usize) {
    let mut line = 0;
    let mut col = 0;
    let mut current_offset = 0;

    for ch in text.chars() {
        if current_offset >= offset {
            break;
        }
        current_offset += 1;

        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    (line, col)
}

/// Get the character offset from a line and column position.
///
/// Both line and column are 0-indexed. Returns `None` if the position is invalid.
#[must_use]
pub fn position_to_offset(text: &str, line: usize, col: usize) -> Option<usize> {
    let mut current_line = 0;
    let mut current_col = 0;
    let mut offset = 0;

    for ch in text.chars() {
        if current_line == line && current_col == col {
            return Some(offset);
        }

        offset += 1;

        if ch == '\n' {
            if current_line == line {
                // Column is beyond line length
                return None;
            }
            current_line += 1;
            current_col = 0;
        } else {
            current_col += 1;
        }
    }

    // Handle position at end of text
    if current_line == line && current_col == col {
        return Some(offset);
    }

    None
}

/// Get the start and end offsets of a specific line (0-indexed).
///
/// Returns `(start, end)` where `end` is exclusive.
#[must_use]
pub fn line_range(text: &str, line: usize) -> Option<(usize, usize)> {
    let mut current_line = 0;
    let mut line_start = 0;
    let mut offset = 0;

    for ch in text.chars() {
        if current_line == line {
            if ch == '\n' {
                return Some((line_start, offset));
            }
        } else if ch == '\n' {
            current_line += 1;
            line_start = offset + 1;
        }
        offset += 1;
    }

    // Handle last line (no trailing newline)
    if current_line == line {
        return Some((line_start, offset));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines(""), 1);
        assert_eq!(count_lines("hello"), 1);
        assert_eq!(count_lines("hello\n"), 2);
        assert_eq!(count_lines("hello\nworld"), 2);
        assert_eq!(count_lines("a\nb\nc"), 3);
        assert_eq!(count_lines("\n\n\n"), 4);
    }

    #[test]
    fn test_text_stats() {
        let (words, chars, chars_no_spaces, lines) = text_stats("Hello, World!");
        assert_eq!(words, 2);
        assert_eq!(chars, 13);
        assert_eq!(chars_no_spaces, 12);
        assert_eq!(lines, 1);
    }

    #[test]
    fn test_offset_to_position() {
        let text = "hello\nworld";
        assert_eq!(offset_to_position(text, 0), (0, 0));
        assert_eq!(offset_to_position(text, 5), (0, 5));
        assert_eq!(offset_to_position(text, 6), (1, 0));
        assert_eq!(offset_to_position(text, 11), (1, 5));
    }

    #[test]
    fn test_position_to_offset() {
        let text = "hello\nworld";
        assert_eq!(position_to_offset(text, 0, 0), Some(0));
        assert_eq!(position_to_offset(text, 0, 5), Some(5));
        assert_eq!(position_to_offset(text, 1, 0), Some(6));
        assert_eq!(position_to_offset(text, 1, 5), Some(11));
        assert_eq!(position_to_offset(text, 2, 0), None);
    }

    #[test]
    fn test_line_range() {
        let text = "hello\nworld\nfoo";
        assert_eq!(line_range(text, 0), Some((0, 5)));
        assert_eq!(line_range(text, 1), Some((6, 11)));
        assert_eq!(line_range(text, 2), Some((12, 15)));
        assert_eq!(line_range(text, 3), None);
    }
}
