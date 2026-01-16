//! Undo/Redo history management
//!
//! Provides efficient history tracking with coalescing of related edits.

use std::time::Instant;

use serde::{Deserialize, Serialize};

use super::cursor::CursorSet;

/// A single history entry representing an edit operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// The content before this edit
    pub content: String,
    /// Cursor state before this edit
    pub cursors: CursorSet,
    /// Timestamp when this entry was created (for coalescing)
    #[serde(skip)]
    pub timestamp: Option<Instant>,
}

impl HistoryEntry {
    /// Create a new history entry.
    #[must_use]
    pub fn new(content: String, cursors: CursorSet) -> Self {
        Self {
            content,
            cursors,
            timestamp: Some(Instant::now()),
        }
    }
}

/// Configuration for history behavior.
#[derive(Debug, Clone)]
pub struct HistoryConfig {
    /// Maximum number of undo entries to keep
    pub max_entries: usize,
    /// Time window for coalescing edits (milliseconds)
    pub coalesce_window_ms: u64,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            max_entries: 1000,
            coalesce_window_ms: 500,
        }
    }
}

/// Manages undo/redo history for the editor.
#[derive(Debug, Clone, Default)]
pub struct History {
    /// Undo stack (most recent at end)
    undo_stack: Vec<HistoryEntry>,
    /// Redo stack (most recent at end)
    redo_stack: Vec<HistoryEntry>,
    /// Configuration
    config: HistoryConfig,
    /// Whether we're currently in the middle of an undo/redo operation
    is_undoing: bool,
}

impl History {
    /// Create a new history manager.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with custom configuration.
    #[must_use]
    pub fn with_config(config: HistoryConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    /// Record a new state in history.
    ///
    /// This will clear the redo stack and potentially coalesce with the
    /// previous entry if the edit happened within the coalesce window.
    pub fn push(&mut self, content: String, cursors: CursorSet) {
        if self.is_undoing {
            return;
        }

        let entry = HistoryEntry::new(content, cursors);

        // Check if we should coalesce with the previous entry
        if let Some(last) = self.undo_stack.last()
            && let (Some(last_ts), Some(entry_ts)) = (last.timestamp, entry.timestamp)
        {
            let elapsed =
                u64::try_from(entry_ts.duration_since(last_ts).as_millis()).unwrap_or(u64::MAX);
            if elapsed < self.config.coalesce_window_ms {
                // Coalesce by not adding a new entry, just update the timestamp
                // The previous state is preserved
                return;
            }
        }

        self.undo_stack.push(entry);
        self.redo_stack.clear();

        // Trim history if needed
        if self.undo_stack.len() > self.config.max_entries {
            self.undo_stack.remove(0);
        }
    }

    /// Record a state without coalescing (for explicit save points).
    pub fn push_checkpoint(&mut self, content: String, cursors: CursorSet) {
        if self.is_undoing {
            return;
        }

        let mut entry = HistoryEntry::new(content, cursors);
        // Set timestamp to None to prevent coalescing with the next edit
        entry.timestamp = None;

        self.undo_stack.push(entry);
        self.redo_stack.clear();

        if self.undo_stack.len() > self.config.max_entries {
            self.undo_stack.remove(0);
        }
    }

    /// Undo the last change.
    ///
    /// Returns the previous state if available.
    pub fn undo(
        &mut self,
        current_content: &str,
        current_cursors: &CursorSet,
    ) -> Option<HistoryEntry> {
        let entry = self.undo_stack.pop()?;

        // Save current state to redo stack
        self.redo_stack.push(HistoryEntry::new(
            current_content.to_string(),
            current_cursors.clone(),
        ));

        Some(entry)
    }

    /// Redo the last undone change.
    ///
    /// Returns the next state if available.
    pub fn redo(
        &mut self,
        current_content: &str,
        current_cursors: &CursorSet,
    ) -> Option<HistoryEntry> {
        let entry = self.redo_stack.pop()?;

        // Save current state to undo stack
        self.undo_stack.push(HistoryEntry::new(
            current_content.to_string(),
            current_cursors.clone(),
        ));

        Some(entry)
    }

    /// Check if undo is available.
    #[must_use]
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Check if redo is available.
    #[must_use]
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clear all history.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }

    /// Get the number of undo entries.
    #[must_use]
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Get the number of redo entries.
    #[must_use]
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Mark that we're in the middle of an undo/redo operation.
    pub fn begin_undo(&mut self) {
        self.is_undoing = true;
    }

    /// Mark that the undo/redo operation is complete.
    pub fn end_undo(&mut self) {
        self.is_undoing = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::editor::cursor::{Cursor, CursorPosition};

    fn test_cursors() -> CursorSet {
        CursorSet::new(Cursor::new(CursorPosition::zero()))
    }

    #[test]
    fn test_undo_redo() {
        let mut history = History::new();

        history.push("state1".to_string(), test_cursors());
        std::thread::sleep(std::time::Duration::from_millis(600));
        history.push("state2".to_string(), test_cursors());

        let entry = history.undo("state3", &test_cursors());
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().content, "state2");

        let entry = history.redo("state2", &test_cursors());
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().content, "state3");
    }

    #[test]
    fn test_redo_cleared_on_new_edit() {
        let mut history = History::new();

        history.push("state1".to_string(), test_cursors());
        std::thread::sleep(std::time::Duration::from_millis(600));
        history.push("state2".to_string(), test_cursors());

        history.undo("state3", &test_cursors());
        assert!(history.can_redo());

        std::thread::sleep(std::time::Duration::from_millis(600));
        history.push("state4".to_string(), test_cursors());
        assert!(!history.can_redo());
    }
}
