//! Selection handling for the editor
//!
//! Manages text selection, selection ranges, and selection operations.

use serde::{Deserialize, Serialize};

use super::cursor::CursorPosition;

/// A text selection range in the document.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Selection {
    /// Start position of the selection
    pub start: CursorPosition,
    /// End position of the selection
    pub end: CursorPosition,
}

impl Selection {
    /// Create a new selection range.
    #[must_use]
    pub const fn new(start: CursorPosition, end: CursorPosition) -> Self {
        Self { start, end }
    }

    /// Create an empty selection at a position.
    #[must_use]
    pub const fn empty(position: CursorPosition) -> Self {
        Self {
            start: position,
            end: position,
        }
    }

    /// Check if the selection is empty (no text selected).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Check if a position is within this selection.
    #[must_use]
    pub fn contains(&self, position: CursorPosition) -> bool {
        let (min, max) = self.normalized();
        position >= min && position < max
    }

    /// Get normalized start and end (start is always before end).
    #[must_use]
    pub fn normalized(&self) -> (CursorPosition, CursorPosition) {
        if self.start.is_before(&self.end) {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        }
    }

    /// Check if this selection overlaps with another.
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        let (self_start, self_end) = self.normalized();
        let (other_start, other_end) = other.normalized();

        !(self_end <= other_start || other_end <= self_start)
    }

    /// Merge this selection with another (if they overlap or are adjacent).
    #[must_use]
    pub fn merge(&self, other: &Self) -> Option<Self> {
        let (self_start, self_end) = self.normalized();
        let (other_start, other_end) = other.normalized();

        // Check if they overlap or are adjacent
        if self_end >= other_start && other_end >= self_start {
            Some(Self {
                start: self_start.min(other_start),
                end: self_end.max(other_end),
            })
        } else {
            None
        }
    }
}

/// The type of selection being made.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SelectionMode {
    /// Normal character-by-character selection
    #[default]
    Character,
    /// Select whole words
    Word,
    /// Select whole lines
    Line,
    /// Block/column selection
    Block,
}

/// Selection direction for keyboard navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SelectionDirection {
    /// Selection is moving forward (right/down)
    Forward,
    /// Selection is moving backward (left/up)
    Backward,
}

/// Get the word boundaries around a position in text.
#[must_use]
#[allow(dead_code)]
pub fn word_at_position(text: &str, line: usize, column: usize) -> Option<(usize, usize)> {
    let lines: Vec<&str> = text.lines().collect();
    let line_text = lines.get(line)?;

    if column > line_text.len() {
        return None;
    }

    // Find word start
    let mut start = column;
    for (i, c) in line_text[..column].char_indices().rev() {
        if !is_word_char(c) {
            start = i + c.len_utf8();
            break;
        }
        if i == 0 {
            start = 0;
        }
    }

    // Find word end
    let mut end = column;
    for (i, c) in line_text[column..].char_indices() {
        if !is_word_char(c) {
            end = column + i;
            break;
        }
        end = column + i + c.len_utf8();
    }

    if start == end {
        None
    } else {
        Some((start, end))
    }
}

/// Check if a character is part of a word.
#[allow(dead_code)]
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_normalized() {
        let sel = Selection::new(CursorPosition::new(1, 5), CursorPosition::new(0, 3));

        let (start, end) = sel.normalized();
        assert_eq!(start, CursorPosition::new(0, 3));
        assert_eq!(end, CursorPosition::new(1, 5));
    }

    #[test]
    fn test_selection_overlaps() {
        let a = Selection::new(CursorPosition::new(0, 0), CursorPosition::new(0, 5));
        let b = Selection::new(CursorPosition::new(0, 3), CursorPosition::new(0, 10));
        let c = Selection::new(CursorPosition::new(0, 6), CursorPosition::new(0, 10));

        assert!(a.overlaps(&b));
        assert!(!a.overlaps(&c));
    }

    #[test]
    fn test_word_at_position() {
        let text = "hello world foo_bar";

        assert_eq!(word_at_position(text, 0, 2), Some((0, 5)));
        assert_eq!(word_at_position(text, 0, 8), Some((6, 11)));
        assert_eq!(word_at_position(text, 0, 15), Some((12, 19)));
    }
}
