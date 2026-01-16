//! Code folding functionality
//!
//! Provides collapsible regions for markdown headings, code blocks, and more.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Type of foldable region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FoldKind {
    /// Markdown heading (H1-H6)
    Heading(u8),
    /// Code block (fenced or indented)
    CodeBlock,
    /// List (ordered or unordered)
    List,
    /// Blockquote
    Blockquote,
    /// Indentation-based fold (for JSON, YAML, etc.)
    Indentation,
    /// Custom region (for explicit fold markers)
    Custom,
}

/// A foldable region in the document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoldRegion {
    /// Unique identifier for this region
    pub id: u64,
    /// Start line (0-indexed)
    pub start_line: usize,
    /// End line (0-indexed, inclusive)
    pub end_line: usize,
    /// Kind of fold region
    pub kind: FoldKind,
    /// Preview text to show when folded
    pub preview: Option<String>,
    /// Whether this region is currently folded
    pub is_folded: bool,
}

impl FoldRegion {
    /// Create a new fold region.
    #[must_use]
    pub fn new(id: u64, start_line: usize, end_line: usize, kind: FoldKind) -> Self {
        Self {
            id,
            start_line,
            end_line,
            kind,
            preview: None,
            is_folded: false,
        }
    }

    /// Create a fold region with preview text.
    #[must_use]
    pub fn with_preview(
        id: u64,
        start_line: usize,
        end_line: usize,
        kind: FoldKind,
        preview: impl Into<String>,
    ) -> Self {
        Self {
            id,
            start_line,
            end_line,
            kind,
            preview: Some(preview.into()),
            is_folded: false,
        }
    }

    /// Get the number of lines in this region.
    #[must_use]
    pub fn line_count(&self) -> usize {
        self.end_line.saturating_sub(self.start_line) + 1
    }

    /// Check if a line is within this region (excluding the start line).
    #[must_use]
    pub fn contains_line(&self, line: usize) -> bool {
        line > self.start_line && line <= self.end_line
    }

    /// Toggle the folded state.
    pub fn toggle(&mut self) {
        self.is_folded = !self.is_folded;
    }
}

/// State for managing fold regions in a document.
#[derive(Debug, Clone, Default)]
pub struct FoldState {
    /// All fold regions, indexed by ID
    regions: HashMap<u64, FoldRegion>,
    /// Next available region ID
    next_id: u64,
    /// Whether the fold state is dirty (needs recalculation)
    is_dirty: bool,
}

impl FoldState {
    /// Create a new fold state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new fold region.
    pub fn add_region(&mut self, start_line: usize, end_line: usize, kind: FoldKind) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let region = FoldRegion::new(id, start_line, end_line, kind);
        self.regions.insert(id, region);
        id
    }

    /// Add a region with preview text.
    pub fn add_region_with_preview(
        &mut self,
        start_line: usize,
        end_line: usize,
        kind: FoldKind,
        preview: impl Into<String>,
    ) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        let region = FoldRegion::with_preview(id, start_line, end_line, kind, preview);
        self.regions.insert(id, region);
        id
    }

    /// Get a region by ID.
    #[must_use]
    pub fn get_region(&self, id: u64) -> Option<&FoldRegion> {
        self.regions.get(&id)
    }

    /// Get a mutable reference to a region.
    pub fn get_region_mut(&mut self, id: u64) -> Option<&mut FoldRegion> {
        self.regions.get_mut(&id)
    }

    /// Get the region that starts at a specific line.
    #[must_use]
    pub fn region_at_line(&self, line: usize) -> Option<&FoldRegion> {
        self.regions.values().find(|r| r.start_line == line)
    }

    /// Toggle fold at a specific line.
    ///
    /// Returns true if a fold was toggled.
    pub fn toggle_at_line(&mut self, line: usize) -> bool {
        if let Some(region) = self.regions.values_mut().find(|r| r.start_line == line) {
            region.toggle();
            true
        } else {
            false
        }
    }

    /// Check if a line is hidden due to folding.
    #[must_use]
    pub fn is_line_hidden(&self, line: usize) -> bool {
        self.regions
            .values()
            .any(|r| r.is_folded && r.contains_line(line))
    }

    /// Get all fold indicator positions (line, is_folded).
    #[must_use]
    pub fn fold_indicators(&self) -> Vec<(usize, bool)> {
        let mut indicators: Vec<_> = self
            .regions
            .values()
            .map(|r| (r.start_line, r.is_folded))
            .collect();
        indicators.sort_by_key(|(line, _)| *line);
        indicators
    }

    /// Fold all regions.
    pub fn fold_all(&mut self) {
        for region in self.regions.values_mut() {
            region.is_folded = true;
        }
    }

    /// Unfold all regions.
    pub fn unfold_all(&mut self) {
        for region in self.regions.values_mut() {
            region.is_folded = false;
        }
    }

    /// Fold all regions of a specific kind.
    pub fn fold_kind(&mut self, kind: FoldKind) {
        for region in self.regions.values_mut() {
            if region.kind == kind {
                region.is_folded = true;
            }
        }
    }

    /// Unfold all regions of a specific kind.
    pub fn unfold_kind(&mut self, kind: FoldKind) {
        for region in self.regions.values_mut() {
            if region.kind == kind {
                region.is_folded = false;
            }
        }
    }

    /// Clear all fold regions.
    pub fn clear(&mut self) {
        self.regions.clear();
    }

    /// Get the next available ID.
    #[must_use]
    pub fn next_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Mark the fold state as clean.
    pub fn mark_clean(&mut self) {
        self.is_dirty = false;
    }

    /// Mark the fold state as dirty (needs recalculation).
    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }

    /// Check if the fold state is dirty.
    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }

    /// Get the number of fold regions.
    #[must_use]
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    /// Iterate over all regions.
    pub fn iter(&self) -> impl Iterator<Item = &FoldRegion> {
        self.regions.values()
    }
}

/// Detect markdown heading level (1-6) from a line.
#[must_use]
pub fn detect_heading_level(line: &str) -> Option<u8> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('#') {
        return None;
    }

    let hash_count = trimmed.chars().take_while(|&c| c == '#').count();
    if hash_count > 6 {
        return None;
    }

    // Must have space after hashes or be just hashes
    let after_hashes = &trimmed[hash_count..];
    if after_hashes.is_empty() || after_hashes.starts_with(' ') {
        return Some(hash_count as u8);
    }

    None
}

/// Detect fold regions in markdown content.
#[must_use]
pub fn detect_markdown_folds(content: &str) -> FoldState {
    let mut state = FoldState::new();
    let lines: Vec<&str> = content.lines().collect();

    if lines.is_empty() {
        return state;
    }

    // Track headings for fold region detection
    let mut headings: Vec<(usize, u8, String)> = Vec::new();

    // First pass: find all headings
    for (line_num, line) in lines.iter().enumerate() {
        if let Some(level) = detect_heading_level(line) {
            let text = line
                .trim_start_matches('#')
                .trim()
                .chars()
                .take(50)
                .collect::<String>();
            headings.push((line_num, level, text));
        }
    }

    // Second pass: create fold regions for headings
    for (i, (start_line, level, preview_text)) in headings.iter().enumerate() {
        // Find the end of this heading's content
        let end_line = if i + 1 < headings.len() {
            // Look for the next heading of same or higher level
            let mut found_end = None;
            for j in (i + 1)..headings.len() {
                let (next_line, next_level, _) = &headings[j];
                if *next_level <= *level {
                    found_end = Some(next_line.saturating_sub(1));
                    break;
                }
            }
            found_end.unwrap_or_else(|| {
                if i + 1 < headings.len() {
                    headings[i + 1].0.saturating_sub(1)
                } else {
                    lines.len().saturating_sub(1)
                }
            })
        } else {
            lines.len().saturating_sub(1)
        };

        // Only create fold if there's content to fold
        if end_line > *start_line {
            // Skip trailing empty lines
            let mut actual_end = end_line;
            while actual_end > *start_line
                && lines
                    .get(actual_end)
                    .map(|l| l.trim().is_empty())
                    .unwrap_or(true)
            {
                actual_end -= 1;
            }

            if actual_end > *start_line {
                state.add_region_with_preview(
                    *start_line,
                    actual_end,
                    FoldKind::Heading(*level),
                    preview_text.clone(),
                );
            }
        }
    }

    // Detect code blocks
    let mut in_code_block = false;
    let mut code_block_start = 0;

    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("```") || trimmed.starts_with("~~~") {
            if in_code_block {
                // End of code block
                if line_num > code_block_start {
                    state.add_region(code_block_start, line_num, FoldKind::CodeBlock);
                }
                in_code_block = false;
            } else {
                // Start of code block
                code_block_start = line_num;
                in_code_block = true;
            }
        }
    }

    state.mark_clean();
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_heading_level() {
        assert_eq!(detect_heading_level("# Heading"), Some(1));
        assert_eq!(detect_heading_level("## Heading"), Some(2));
        assert_eq!(detect_heading_level("### Heading"), Some(3));
        assert_eq!(detect_heading_level("Not a heading"), None);
        assert_eq!(detect_heading_level("#NoSpace"), None);
    }

    #[test]
    fn test_fold_region_contains_line() {
        let region = FoldRegion::new(1, 5, 10, FoldKind::Heading(1));

        assert!(!region.contains_line(5)); // Start line is not "contained"
        assert!(region.contains_line(6));
        assert!(region.contains_line(10));
        assert!(!region.contains_line(11));
    }

    #[test]
    fn test_detect_markdown_folds() {
        let content = "# Title\n\nSome content\n\n## Section\n\nMore content";
        let state = detect_markdown_folds(content);

        assert!(state.region_count() > 0);
    }
}
