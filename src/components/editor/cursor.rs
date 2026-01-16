//! Cursor management for the editor
//!
//! Handles cursor positioning, movement, and multi-cursor support.

use serde::{Deserialize, Serialize};

/// A position in the document (line and column, both 0-indexed).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorPosition {
    /// Line number (0-indexed)
    pub line: usize,
    /// Column number (0-indexed, in characters)
    pub column: usize,
}

impl CursorPosition {
    /// Create a new cursor position.
    #[must_use]
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// Create a position at the start of the document.
    #[must_use]
    pub const fn zero() -> Self {
        Self { line: 0, column: 0 }
    }

    /// Check if this position is before another position.
    #[must_use]
    pub fn is_before(&self, other: &Self) -> bool {
        self.line < other.line || (self.line == other.line && self.column < other.column)
    }

    /// Get the minimum (earlier) of two positions.
    #[must_use]
    pub fn min(&self, other: &Self) -> Self {
        if self.is_before(other) { *self } else { *other }
    }

    /// Get the maximum (later) of two positions.
    #[must_use]
    pub fn max(&self, other: &Self) -> Self {
        if self.is_before(other) { *other } else { *self }
    }
}

impl PartialOrd for CursorPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CursorPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.line.cmp(&other.line) {
            std::cmp::Ordering::Equal => self.column.cmp(&other.column),
            ord => ord,
        }
    }
}

/// A cursor in the editor, with head and anchor for selection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cursor {
    /// The head (active end) of the cursor/selection
    pub head: CursorPosition,
    /// The anchor (fixed end) of the cursor/selection
    pub anchor: CursorPosition,
    /// Preferred column for vertical movement (remembers column when moving through short lines)
    pub preferred_column: Option<usize>,
}

impl Cursor {
    /// Create a new cursor at a position (no selection).
    #[must_use]
    pub const fn new(position: CursorPosition) -> Self {
        Self {
            head: position,
            anchor: position,
            preferred_column: None,
        }
    }

    /// Create a cursor at the start of the document.
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(CursorPosition::zero())
    }

    /// Create a cursor with a selection range.
    #[must_use]
    pub const fn with_selection(head: CursorPosition, anchor: CursorPosition) -> Self {
        Self {
            head,
            anchor,
            preferred_column: None,
        }
    }

    /// Check if the cursor has an active selection.
    #[must_use]
    pub fn has_selection(&self) -> bool {
        self.head != self.anchor
    }

    /// Get the selection start (minimum position).
    #[must_use]
    pub fn selection_start(&self) -> CursorPosition {
        self.head.min(self.anchor)
    }

    /// Get the selection end (maximum position).
    #[must_use]
    pub fn selection_end(&self) -> CursorPosition {
        self.head.max(self.anchor)
    }

    /// Collapse selection by moving anchor to head.
    pub fn collapse(&mut self) {
        self.anchor = self.head;
    }

    /// Move the cursor to a new position, optionally extending selection.
    pub fn move_to(&mut self, position: CursorPosition, extend_selection: bool) {
        self.head = position;
        if !extend_selection {
            self.anchor = position;
        }
    }

    /// Set the preferred column for vertical movement.
    pub fn set_preferred_column(&mut self, column: usize) {
        self.preferred_column = Some(column);
    }

    /// Clear the preferred column.
    pub fn clear_preferred_column(&mut self) {
        self.preferred_column = None;
    }
}

/// A set of cursors for multi-cursor support.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CursorSet {
    /// All active cursors (primary cursor is first)
    cursors: Vec<Cursor>,
}

impl CursorSet {
    /// Create a new cursor set with a single cursor.
    #[must_use]
    pub fn new(cursor: Cursor) -> Self {
        Self {
            cursors: vec![cursor],
        }
    }

    /// Get the primary (first) cursor.
    ///
    /// # Panics
    ///
    /// Panics if the cursor set is empty.
    #[must_use]
    pub fn primary(&self) -> &Cursor {
        self.cursors
            .first()
            .expect("CursorSet must have at least one cursor")
    }

    /// Get mutable reference to the primary cursor.
    ///
    /// # Panics
    ///
    /// Panics if the cursor set is empty.
    pub fn primary_mut(&mut self) -> &mut Cursor {
        self.cursors
            .first_mut()
            .expect("CursorSet must have at least one cursor")
    }

    /// Get all cursors.
    #[must_use]
    pub fn all(&self) -> &[Cursor] {
        &self.cursors
    }

    /// Check if there are multiple cursors.
    #[must_use]
    pub fn is_multi(&self) -> bool {
        self.cursors.len() > 1
    }

    /// Add a new cursor.
    pub fn add(&mut self, cursor: Cursor) {
        self.cursors.push(cursor);
        self.merge_overlapping();
    }

    /// Remove all cursors except the primary.
    pub fn collapse_to_primary(&mut self) {
        if self.cursors.len() > 1 {
            let primary = self.cursors[0];
            self.cursors.clear();
            self.cursors.push(primary);
        }
    }

    /// Merge overlapping cursors/selections.
    fn merge_overlapping(&mut self) {
        if self.cursors.len() <= 1 {
            return;
        }

        // Sort by selection start
        self.cursors.sort_by(|a, b| {
            let a_start = a.selection_start();
            let b_start = b.selection_start();
            a_start.cmp(&b_start)
        });

        let mut merged = Vec::with_capacity(self.cursors.len());
        merged.push(self.cursors[0]);

        for cursor in &self.cursors[1..] {
            let last = merged.last_mut().expect("merged is not empty");
            let last_end = last.selection_end();
            let cursor_start = cursor.selection_start();

            if cursor_start <= last_end {
                // Overlapping - merge by extending the last cursor
                let cursor_end = cursor.selection_end();
                if cursor_end > last_end {
                    if cursor.head > cursor.anchor {
                        last.head = cursor.head;
                    } else {
                        last.anchor = cursor_end;
                    }
                }
            } else {
                // Not overlapping - add as new cursor
                merged.push(*cursor);
            }
        }

        self.cursors = merged;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor_position_ordering() {
        let a = CursorPosition::new(0, 5);
        let b = CursorPosition::new(1, 0);
        let c = CursorPosition::new(1, 3);

        assert!(a.is_before(&b));
        assert!(b.is_before(&c));
        assert!(!c.is_before(&a));
    }

    #[test]
    fn test_cursor_selection() {
        let cursor = Cursor::with_selection(CursorPosition::new(1, 5), CursorPosition::new(0, 3));

        assert!(cursor.has_selection());
        assert_eq!(cursor.selection_start(), CursorPosition::new(0, 3));
        assert_eq!(cursor.selection_end(), CursorPosition::new(1, 5));
    }

    #[test]
    fn test_cursor_set_merge() {
        let mut set = CursorSet::new(Cursor::with_selection(
            CursorPosition::new(0, 0),
            CursorPosition::new(0, 2),
        ));
        set.add(Cursor::new(CursorPosition::new(0, 1)));
        set.add(Cursor::new(CursorPosition::new(2, 0)));

        // First two should merge since they overlap
        assert_eq!(set.all().len(), 2);
    }
}
