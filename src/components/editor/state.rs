//! Editor state management
//!
//! Centralized state for the editor component.

use serde::{Deserialize, Serialize};

use super::{
    cursor::{Cursor, CursorPosition, CursorSet},
    history::History,
};

/// Editor configuration options.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct EditorConfig {
    /// Tab size in spaces
    pub tab_size: usize,
    /// Whether to insert spaces instead of tabs
    pub insert_spaces: bool,
    /// Whether word wrap is enabled
    pub word_wrap: bool,
    /// Whether to show line numbers
    pub show_line_numbers: bool,
    /// Whether to highlight the current line
    pub highlight_current_line: bool,
    /// Whether to show whitespace characters
    pub show_whitespace: bool,
    /// Whether bracket matching is enabled
    pub match_brackets: bool,
    /// Whether auto-indent is enabled
    pub auto_indent: bool,
    /// Whether auto-close brackets is enabled
    pub auto_close_brackets: bool,
    /// Font size in pixels
    pub font_size: f32,
    /// Line height multiplier (1.0 = same as font size)
    pub line_height: f32,
    /// Maximum line width (0 = no limit)
    pub max_line_width: usize,
    /// Whether the editor is read-only
    pub read_only: bool,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            tab_size: 4,
            insert_spaces: true,
            word_wrap: true,
            show_line_numbers: true,
            highlight_current_line: true,
            show_whitespace: false,
            match_brackets: true,
            auto_indent: true,
            auto_close_brackets: true,
            font_size: 14.0,
            line_height: 1.5,
            max_line_width: 0,
            read_only: false,
        }
    }
}

/// The complete state of an editor instance.
#[derive(Debug, Clone)]
pub struct EditorState {
    /// The document content
    pub content: String,
    /// Cursor positions (supports multi-cursor)
    pub cursors: CursorSet,
    /// Edit history for undo/redo
    pub history: History,
    /// Configuration
    pub config: EditorConfig,
    /// Content version (incremented on each change)
    pub version: u64,
    /// Whether the content has been modified since last save
    pub is_modified: bool,
    /// Current scroll position (line number)
    pub scroll_line: usize,
    /// Current scroll offset (pixels)
    pub scroll_offset: f32,
    /// Detected or explicitly set language
    pub language: Option<String>,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            content: String::new(),
            cursors: CursorSet::new(Cursor::zero()),
            history: History::new(),
            config: EditorConfig::default(),
            version: 0,
            is_modified: false,
            scroll_line: 0,
            scroll_offset: 0.0,
            language: None,
        }
    }
}

impl EditorState {
    /// Create a new editor state with the given content.
    #[must_use]
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            ..Default::default()
        }
    }

    /// Create with custom configuration.
    #[must_use]
    pub fn with_config(content: impl Into<String>, config: EditorConfig) -> Self {
        Self {
            content: content.into(),
            config,
            ..Default::default()
        }
    }

    /// Get the current content.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Set new content.
    pub fn set_content(&mut self, content: impl Into<String>) {
        let new_content = content.into();
        if new_content != self.content {
            // Save to history before modifying
            self.history
                .push(self.content.clone(), self.cursors.clone());
            self.content = new_content;
            self.version += 1;
            self.is_modified = true;
        }
    }

    /// Replace content without adding to history (for external updates).
    pub fn replace_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
        self.version += 1;
    }

    /// Get the primary cursor position.
    #[must_use]
    pub fn cursor_position(&self) -> CursorPosition {
        self.cursors.primary().head
    }

    /// Set the primary cursor position.
    pub fn set_cursor(&mut self, position: CursorPosition) {
        self.cursors.primary_mut().move_to(position, false);
    }

    /// Set the cursor with selection.
    pub fn set_cursor_with_selection(&mut self, head: CursorPosition, anchor: CursorPosition) {
        let cursor = self.cursors.primary_mut();
        cursor.head = head;
        cursor.anchor = anchor;
    }

    /// Get the line count.
    #[must_use]
    pub fn line_count(&self) -> usize {
        if self.content.is_empty() {
            1
        } else {
            self.content.chars().filter(|&c| c == '\n').count() + 1
        }
    }

    /// Get a specific line (0-indexed).
    #[must_use]
    pub fn get_line(&self, index: usize) -> Option<&str> {
        self.content.lines().nth(index)
    }

    /// Insert text at the current cursor position.
    pub fn insert(&mut self, text: &str) {
        if self.config.read_only {
            return;
        }

        let position = self.cursor_position();
        if let Some(offset) = self.position_to_offset(position) {
            self.history
                .push(self.content.clone(), self.cursors.clone());

            // Handle selection - delete selected text first
            let cursor = self.cursors.primary();
            if cursor.has_selection() {
                let (start, end) = (
                    self.position_to_offset(cursor.selection_start()),
                    self.position_to_offset(cursor.selection_end()),
                );
                if let (Some(start), Some(end)) = (start, end) {
                    self.content =
                        format!("{}{}{}", &self.content[..start], text, &self.content[end..]);
                    // Move cursor to end of inserted text
                    let new_offset = start + text.len();
                    if let Some(new_pos) = self.offset_to_position(new_offset) {
                        self.set_cursor(new_pos);
                    }
                }
            } else {
                // No selection - just insert
                self.content.insert_str(offset, text);
                let new_offset = offset + text.len();
                if let Some(new_pos) = self.offset_to_position(new_offset) {
                    self.set_cursor(new_pos);
                }
            }

            self.version += 1;
            self.is_modified = true;
        }
    }

    /// Delete the character before the cursor (backspace).
    pub fn delete_backward(&mut self) {
        if self.config.read_only {
            return;
        }

        let cursor = self.cursors.primary();
        if cursor.has_selection() {
            self.delete_selection();
            return;
        }

        let position = cursor.head;
        if let Some(offset) = self.position_to_offset(position) {
            if offset == 0 {
                return;
            }

            self.history
                .push(self.content.clone(), self.cursors.clone());

            // Find the previous character boundary
            let prev_offset = self.content[..offset]
                .char_indices()
                .last()
                .map_or(0, |(i, _)| i);

            self.content = format!(
                "{}{}",
                &self.content[..prev_offset],
                &self.content[offset..]
            );

            if let Some(new_pos) = self.offset_to_position(prev_offset) {
                self.set_cursor(new_pos);
            }

            self.version += 1;
            self.is_modified = true;
        }
    }

    /// Delete the character after the cursor (delete).
    pub fn delete_forward(&mut self) {
        if self.config.read_only {
            return;
        }

        let cursor = self.cursors.primary();
        if cursor.has_selection() {
            self.delete_selection();
            return;
        }

        let position = cursor.head;
        if let Some(offset) = self.position_to_offset(position) {
            if offset >= self.content.len() {
                return;
            }

            self.history
                .push(self.content.clone(), self.cursors.clone());

            // Find the next character boundary
            let next_offset = self.content[offset..]
                .char_indices()
                .nth(1)
                .map_or(self.content.len(), |(i, _)| offset + i);

            self.content = format!(
                "{}{}",
                &self.content[..offset],
                &self.content[next_offset..]
            );

            self.version += 1;
            self.is_modified = true;
        }
    }

    /// Delete the current selection.
    fn delete_selection(&mut self) {
        let cursor = self.cursors.primary();
        if !cursor.has_selection() {
            return;
        }

        let start_pos = cursor.selection_start();
        let end_pos = cursor.selection_end();

        if let (Some(start), Some(end)) = (
            self.position_to_offset(start_pos),
            self.position_to_offset(end_pos),
        ) {
            self.history
                .push(self.content.clone(), self.cursors.clone());

            self.content = format!("{}{}", &self.content[..start], &self.content[end..]);
            self.set_cursor(start_pos);

            self.version += 1;
            self.is_modified = true;
        }
    }

    /// Undo the last change.
    pub fn undo(&mut self) -> bool {
        if let Some(entry) = self.history.undo(&self.content, &self.cursors) {
            self.content = entry.content;
            self.cursors = entry.cursors;
            self.version += 1;
            true
        } else {
            false
        }
    }

    /// Redo the last undone change.
    pub fn redo(&mut self) -> bool {
        if let Some(entry) = self.history.redo(&self.content, &self.cursors) {
            self.content = entry.content;
            self.cursors = entry.cursors;
            self.version += 1;
            true
        } else {
            false
        }
    }

    /// Check if undo is available.
    #[must_use]
    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    /// Check if redo is available.
    #[must_use]
    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    /// Mark the content as saved (clears modified flag).
    pub fn mark_saved(&mut self) {
        self.is_modified = false;
    }

    /// Convert a cursor position to a byte offset.
    #[must_use]
    pub fn position_to_offset(&self, position: CursorPosition) -> Option<usize> {
        let mut current_line = 0;
        let mut offset = 0;

        for (i, ch) in self.content.char_indices() {
            if current_line == position.line {
                let line_start = i;
                let mut col = 0;
                for (j, c) in self.content[line_start..].char_indices() {
                    if col == position.column {
                        return Some(line_start + j);
                    }
                    if c == '\n' {
                        break;
                    }
                    col += 1;
                }
                // Position at end of line
                if col == position.column {
                    return Some(
                        line_start
                            + self.content[line_start..]
                                .find('\n')
                                .unwrap_or(self.content.len() - line_start),
                    );
                }
                return None;
            }
            if ch == '\n' {
                current_line += 1;
            }
            offset = i + ch.len_utf8();
        }

        // Handle position at end of last line
        if current_line == position.line && position.column == 0 {
            return Some(offset);
        }

        None
    }

    /// Convert a byte offset to a cursor position.
    #[must_use]
    pub fn offset_to_position(&self, offset: usize) -> Option<CursorPosition> {
        if offset > self.content.len() {
            return None;
        }

        let mut line = 0;
        let mut col = 0;

        for (i, ch) in self.content.char_indices() {
            if i >= offset {
                return Some(CursorPosition::new(line, col));
            }
            if ch == '\n' {
                line += 1;
                col = 0;
            } else {
                col += 1;
            }
        }

        // Position at end of content
        Some(CursorPosition::new(line, col))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_state_new() {
        let state = EditorState::new("Hello, World!");
        assert_eq!(state.content(), "Hello, World!");
        assert_eq!(state.line_count(), 1);
        assert!(!state.is_modified);
    }

    #[test]
    fn test_insert() {
        let mut state = EditorState::new("");
        state.insert("Hello");
        assert_eq!(state.content(), "Hello");
        assert!(state.is_modified);
    }

    #[test]
    fn test_undo_redo() {
        let mut state = EditorState::new("initial");
        state.set_content("modified");

        assert!(state.undo());
        assert_eq!(state.content(), "initial");

        assert!(state.redo());
        assert_eq!(state.content(), "modified");
    }

    #[test]
    fn test_position_offset_conversion() {
        let state = EditorState::new("hello\nworld\nfoo");

        assert_eq!(state.position_to_offset(CursorPosition::new(0, 0)), Some(0));
        assert_eq!(state.position_to_offset(CursorPosition::new(1, 0)), Some(6));
        assert_eq!(
            state.position_to_offset(CursorPosition::new(2, 0)),
            Some(12)
        );

        assert_eq!(state.offset_to_position(0), Some(CursorPosition::new(0, 0)));
        assert_eq!(state.offset_to_position(6), Some(CursorPosition::new(1, 0)));
    }
}
