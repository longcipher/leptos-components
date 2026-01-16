//! Find and Replace functionality
//!
//! Provides search and replace capabilities for the editor.

#[cfg(feature = "find-replace")]
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Options for find operations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FindOptions {
    /// Whether the search is case-sensitive
    pub case_sensitive: bool,
    /// Whether to match whole words only
    pub whole_word: bool,
    /// Whether to use regex matching
    pub use_regex: bool,
    /// Whether to wrap around at document boundaries
    pub wrap_around: bool,
}

/// A single find result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FindResult {
    /// Start byte offset in the document
    pub start: usize,
    /// End byte offset in the document
    pub end: usize,
}

impl FindResult {
    /// Create a new find result.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Get the length of the match.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    /// Check if the match is empty.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

/// State for find/replace operations.
#[derive(Debug, Clone, Default)]
pub struct FindState {
    /// Current search query
    pub query: String,
    /// Replacement text
    pub replacement: String,
    /// Find options
    pub options: FindOptions,
    /// All matches in the current document
    pub matches: Vec<FindResult>,
    /// Index of the currently selected match
    pub current_index: usize,
    /// Whether the find panel is visible
    pub is_visible: bool,
    /// Whether replace mode is active
    pub is_replace_mode: bool,
}

impl FindState {
    /// Create a new find state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the search query and find all matches.
    pub fn search(&mut self, text: &str) {
        self.matches.clear();
        self.current_index = 0;

        if self.query.is_empty() {
            return;
        }

        if self.options.use_regex {
            self.search_regex(text);
        } else {
            self.search_literal(text);
        }
    }

    /// Search using literal string matching.
    fn search_literal(&mut self, text: &str) {
        let search_text = if self.options.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };

        let query = if self.options.case_sensitive {
            self.query.clone()
        } else {
            self.query.to_lowercase()
        };

        let mut start = 0;
        while let Some(pos) = search_text[start..].find(&query) {
            let match_start = start + pos;
            let match_end = match_start + self.query.len();

            // Check whole word boundary
            if self.options.whole_word {
                let is_start_boundary = match_start == 0
                    || !text[..match_start]
                        .chars()
                        .last()
                        .map(|c| c.is_alphanumeric() || c == '_')
                        .unwrap_or(false);

                let is_end_boundary = match_end >= text.len()
                    || !text[match_end..]
                        .chars()
                        .next()
                        .map(|c| c.is_alphanumeric() || c == '_')
                        .unwrap_or(false);

                if !is_start_boundary || !is_end_boundary {
                    start = match_start + 1;
                    continue;
                }
            }

            self.matches.push(FindResult::new(match_start, match_end));
            start = match_end;
        }
    }

    /// Search using regex.
    #[cfg(feature = "find-replace")]
    fn search_regex(&mut self, text: &str) {
        let pattern = if self.options.case_sensitive {
            self.query.clone()
        } else {
            format!("(?i){}", self.query)
        };

        let pattern = if self.options.whole_word {
            format!(r"\b{}\b", pattern)
        } else {
            pattern
        };

        if let Ok(re) = Regex::new(&pattern) {
            for m in re.find_iter(text) {
                self.matches.push(FindResult::new(m.start(), m.end()));
            }
        }
    }

    /// Navigate to the next match.
    ///
    /// Returns the new current match if any.
    pub fn next(&mut self) -> Option<FindResult> {
        if self.matches.is_empty() {
            return None;
        }

        self.current_index = (self.current_index + 1) % self.matches.len();
        self.current_match()
    }

    /// Navigate to the previous match.
    ///
    /// Returns the new current match if any.
    pub fn prev(&mut self) -> Option<FindResult> {
        if self.matches.is_empty() {
            return None;
        }

        self.current_index = if self.current_index == 0 {
            self.matches.len() - 1
        } else {
            self.current_index - 1
        };

        self.current_match()
    }

    /// Get the current match.
    #[must_use]
    pub fn current_match(&self) -> Option<FindResult> {
        self.matches.get(self.current_index).copied()
    }

    /// Get the match count.
    #[must_use]
    pub fn match_count(&self) -> usize {
        self.matches.len()
    }

    /// Check if there are any matches.
    #[must_use]
    pub fn has_matches(&self) -> bool {
        !self.matches.is_empty()
    }

    /// Replace the current match.
    ///
    /// Returns the new text if replacement was made.
    pub fn replace_current(&self, text: &str) -> Option<String> {
        let current = self.current_match()?;

        let mut result = String::with_capacity(text.len());
        result.push_str(&text[..current.start]);
        result.push_str(&self.replacement);
        result.push_str(&text[current.end..]);

        Some(result)
    }

    /// Replace all matches.
    ///
    /// Returns the new text with all replacements made.
    pub fn replace_all(&self, text: &str) -> String {
        if self.matches.is_empty() {
            return text.to_string();
        }

        let mut result = String::with_capacity(text.len());
        let mut last_end = 0;

        for m in &self.matches {
            result.push_str(&text[last_end..m.start]);
            result.push_str(&self.replacement);
            last_end = m.end;
        }

        result.push_str(&text[last_end..]);
        result
    }

    /// Show the find panel.
    pub fn show(&mut self) {
        self.is_visible = true;
        self.is_replace_mode = false;
    }

    /// Show the find and replace panel.
    pub fn show_replace(&mut self) {
        self.is_visible = true;
        self.is_replace_mode = true;
    }

    /// Hide the panel.
    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    /// Clear the search state.
    pub fn clear(&mut self) {
        self.query.clear();
        self.matches.clear();
        self.current_index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_literal() {
        let mut state = FindState::new();
        state.query = "hello".to_string();
        state.search("hello world hello");

        assert_eq!(state.match_count(), 2);
        assert_eq!(state.matches[0], FindResult::new(0, 5));
        assert_eq!(state.matches[1], FindResult::new(12, 17));
    }

    #[test]
    fn test_find_case_insensitive() {
        let mut state = FindState::new();
        state.query = "Hello".to_string();
        state.options.case_sensitive = false;
        state.search("hello HELLO Hello");

        assert_eq!(state.match_count(), 3);
    }

    #[test]
    fn test_find_whole_word() {
        let mut state = FindState::new();
        state.query = "test".to_string();
        state.options.whole_word = true;
        state.search("test testing tested test");

        assert_eq!(state.match_count(), 2);
    }

    #[test]
    fn test_replace_all() {
        let mut state = FindState::new();
        state.query = "old".to_string();
        state.replacement = "new".to_string();
        state.search("old and old");

        let result = state.replace_all("old and old");
        assert_eq!(result, "new and new");
    }
}
