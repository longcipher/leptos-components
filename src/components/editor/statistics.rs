//! Document statistics
//!
//! Provides word count, character count, and other text metrics.

use serde::{Deserialize, Serialize};

/// Basic text statistics.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextStats {
    /// Number of words
    pub words: usize,
    /// Number of characters (including whitespace)
    pub characters: usize,
    /// Number of characters (excluding whitespace)
    pub characters_no_spaces: usize,
    /// Number of lines
    pub lines: usize,
    /// Number of paragraphs
    pub paragraphs: usize,
}

impl TextStats {
    /// Calculate statistics from text.
    #[must_use]
    pub fn from_text(text: &str) -> Self {
        if text.is_empty() {
            return Self {
                words: 0,
                characters: 0,
                characters_no_spaces: 0,
                lines: 1,
                paragraphs: 0,
            };
        }

        let mut stats = Self::default();

        // Count lines
        stats.lines = text.chars().filter(|&c| c == '\n').count() + 1;

        // Single pass for words, characters, and paragraphs
        let mut in_word = false;
        let mut in_paragraph = false;
        let mut consecutive_newlines = 0;
        let mut line_has_content = false;

        for ch in text.chars() {
            stats.characters += 1;

            if ch.is_whitespace() {
                if in_word {
                    in_word = false;
                }

                if ch == '\n' {
                    consecutive_newlines += 1;

                    if line_has_content && !in_paragraph {
                        in_paragraph = true;
                        stats.paragraphs += 1;
                    }

                    if consecutive_newlines >= 2 {
                        in_paragraph = false;
                    }

                    line_has_content = false;
                } else {
                    consecutive_newlines = 0;
                }
            } else {
                stats.characters_no_spaces += 1;
                consecutive_newlines = 0;
                line_has_content = true;

                if !in_word {
                    in_word = true;
                    stats.words += 1;
                }
            }
        }

        // Handle final paragraph
        if line_has_content && !in_paragraph {
            stats.paragraphs += 1;
        }

        stats
    }

    /// Format as a compact string for display.
    #[must_use]
    pub fn format_compact(&self) -> String {
        format!(
            "{} words | {} chars | {} lines",
            self.words, self.characters, self.lines
        )
    }
}

/// Comprehensive document statistics including markdown elements.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentStats {
    /// Basic text statistics
    pub text: TextStats,
    /// Count of headings by level (index 0 = H1, index 5 = H6)
    pub headings_by_level: [usize; 6],
    /// Total number of headings
    pub heading_count: usize,
    /// Number of links
    pub link_count: usize,
    /// Number of images
    pub image_count: usize,
    /// Number of code blocks
    pub code_block_count: usize,
    /// Number of tables
    pub table_count: usize,
    /// Number of blockquotes
    pub blockquote_count: usize,
    /// Number of list items
    pub list_item_count: usize,
    /// Estimated reading time in minutes
    pub reading_time_minutes: u32,
}

impl DocumentStats {
    /// Calculate comprehensive statistics from markdown text.
    #[must_use]
    pub fn from_text(text: &str) -> Self {
        let text_stats = TextStats::from_text(text);
        let mut stats = Self {
            text: text_stats,
            ..Default::default()
        };

        // Calculate reading time (250 WPM average)
        stats.reading_time_minutes = ((stats.text.words as f32 / 250.0).ceil() as u32).max(1);

        // Parse markdown elements
        stats.parse_markdown(text);

        stats
    }

    /// Parse markdown-specific elements.
    fn parse_markdown(&mut self, text: &str) {
        let mut in_code_block = false;

        for line in text.lines() {
            let trimmed = line.trim();

            // Code blocks
            if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
                if in_code_block {
                    in_code_block = false;
                } else {
                    in_code_block = true;
                    self.code_block_count += 1;
                }
                continue;
            }

            if in_code_block {
                continue;
            }

            // Headings
            if let Some(level) = Self::heading_level(trimmed) {
                if level <= 6 {
                    self.headings_by_level[level - 1] += 1;
                    self.heading_count += 1;
                }
            }

            // Blockquotes
            if trimmed.starts_with('>') {
                self.blockquote_count += 1;
            }

            // List items
            if trimmed.starts_with("- ")
                || trimmed.starts_with("* ")
                || trimmed.starts_with("+ ")
                || Self::is_ordered_list_item(trimmed)
            {
                self.list_item_count += 1;
            }

            // Count links and images
            self.link_count += Self::count_links(line);
            self.image_count += Self::count_images(line);

            // Tables (simplified detection)
            if trimmed.contains('|') && trimmed.starts_with('|') {
                self.table_count += 1;
            }
        }
    }

    /// Get heading level from a line.
    fn heading_level(line: &str) -> Option<usize> {
        if !line.starts_with('#') {
            return None;
        }

        let count = line.chars().take_while(|&c| c == '#').count();
        if count <= 6 {
            let after = &line[count..];
            if after.is_empty() || after.starts_with(' ') {
                return Some(count);
            }
        }

        None
    }

    /// Check if a line is an ordered list item.
    fn is_ordered_list_item(line: &str) -> bool {
        let mut chars = line.chars();
        let mut has_digit = false;

        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                has_digit = true;
            } else if c == '.' && has_digit {
                return chars.next() == Some(' ');
            } else {
                return false;
            }
        }

        false
    }

    /// Count markdown links in a line.
    fn count_links(line: &str) -> usize {
        let mut count = 0;
        let mut chars = line.char_indices().peekable();

        while let Some((i, c)) = chars.next() {
            // Skip images
            if c == '!' && line[i + 1..].starts_with('[') {
                continue;
            }

            if c == '[' {
                // Look for ](
                let rest = &line[i + 1..];
                if let Some(close) = rest.find("](") {
                    let after_close = &rest[close + 2..];
                    if after_close.contains(')') {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    /// Count markdown images in a line.
    fn count_images(line: &str) -> usize {
        let mut count = 0;
        let mut start = 0;

        while let Some(pos) = line[start..].find("![") {
            let rest = &line[start + pos + 2..];
            if let Some(close) = rest.find("](") {
                let after_close = &rest[close + 2..];
                if after_close.contains(')') {
                    count += 1;
                }
            }
            start = start + pos + 2;
        }

        count
    }

    /// Format reading time for display.
    #[must_use]
    pub fn format_reading_time(&self) -> String {
        if self.reading_time_minutes == 1 {
            "1 min read".to_string()
        } else {
            format!("{} min read", self.reading_time_minutes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_stats() {
        let stats = TextStats::from_text("Hello, World!\n\nNew paragraph.");

        assert_eq!(stats.words, 4);
        assert_eq!(stats.lines, 3);
        assert_eq!(stats.paragraphs, 2);
    }

    #[test]
    fn test_document_stats() {
        let text = r#"# Title

Some text with a [link](url).

## Section

- Item 1
- Item 2

```rust
let x = 1;
```
"#;

        let stats = DocumentStats::from_text(text);

        assert_eq!(stats.heading_count, 2);
        assert_eq!(stats.headings_by_level[0], 1); // H1
        assert_eq!(stats.headings_by_level[1], 1); // H2
        assert_eq!(stats.link_count, 1);
        assert_eq!(stats.list_item_count, 2);
        assert_eq!(stats.code_block_count, 1);
    }

    #[test]
    fn test_reading_time() {
        let text = "word ".repeat(500); // 500 words
        let stats = DocumentStats::from_text(&text);

        assert_eq!(stats.reading_time_minutes, 2); // 500/250 = 2 minutes
    }
}
