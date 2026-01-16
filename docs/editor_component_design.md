# Editor Component Design Document

The `Editor` component is a production-ready, full-featured text and code editor for Leptos.

## Overview

The Editor is built on top of a standard `<textarea>` but enhanced with sophisticated state management to support multi-cursor editing, undo/redo history, and syntax highlighting.

---

## Props Reference

```rust
pub fn Editor(
    // Required
    value: Signal<String>,              // Editor content

    // Callbacks
    on_change: Option<Callback<String>>,
    on_focus: Option<Callback<()>>,
    on_blur: Option<Callback<()>>,
    on_cursor_change: Option<Callback<(usize, usize)>>,
    on_selection_change: Option<Callback<Option<String>>>,

    // Appearance
    class: Option<String>,              // Custom CSS classes
    placeholder: Option<String>,        // Placeholder text
    language: Option<String>,           // Syntax language
    font_size: f32,                     // Font size (px)
    min_height: Option<String>,         // Min height (CSS)
    max_height: Option<String>,         // Max height (CSS)

    // Behavior
    read_only: bool,                    // Read-only mode
    show_line_numbers: bool,            // Show gutter
    word_wrap: bool,                    // Enable wrapping
    tab_size: usize,                    // Tab width
    match_brackets: bool,               // Bracket matching
    highlight_current_line: bool,       // Highlight active line
    autofocus: bool,                    // Focus on mount

    // Identity
    id: Option<String>,                 // DOM id attribute
) -> impl IntoView
```

---

## State Management

The editor uses a centralized `EditorState` to track all information about the current editing session.

```rust
/// Complete editor state
pub struct EditorState {
    pub content: String,
    pub cursors: CursorSet,
    pub history: History,
    pub config: EditorConfig,
    pub version: u64,
    pub is_modified: bool,
    pub scroll_line: usize,
    pub scroll_offset: f32,
    pub language: Option<String>,
}
```

### Cursor System

The cursor system supports multiple cursors, each with its own selection range. Cursors are automatically merged when they overlap.

### History Management

Undo and redo are handled via a history stack that stores diffs or snapshots of the editor state. Operations within a short "coalesce window" are merged into a single history entry.

---

## Styling System

### CSS Custom Properties

The editor is highly themeable via CSS variables:

```css
.leptos-editor {
    /* Colors */
    --editor-bg: #1e1e1e;
    --editor-fg: #d4d4d4;
    --editor-line-number-fg: #858585;
    --editor-line-number-active-fg: #c6c6c6;
    --editor-selection-bg: #264f78;
    --editor-cursor: #aeafad;
    --editor-gutter-bg: #1e1e1e;
    --editor-border: #3c3c3c;
    --editor-current-line-bg: rgba(255, 255, 255, 0.04);

    /* Typography */
    --editor-font-size: 14px;
    --editor-line-height: 1.5;
    --editor-tab-size: 4;
}
```

### Theme Variants

Light and dark themes are supported by toggling classes:

```css
/* Light theme example */
.leptos-editor.light {
    --editor-bg: #ffffff;
    --editor-fg: #1e293b;
    --editor-border: #e2e8f0;
}
```

---

## Implementation Details

### Accessibility

The editor uses a hidden textarea for input to leverage native browser accessibility features while rendering the visual editor in a separate layer.

```rust
view! {
    <textarea
        aria-label="Code editor"
        aria-multiline="true"
        aria-readonly=read_only
        // ...
    />
}
```

### Performance

- **Deferred Syntax Highlighting**: Heavy colorization is performed asynchronously or delayed while the user is typing rapidly.
- **Bundle Size Targets**:
  - `editor` only: < 50 KB (gzipped)
  - `editor-full`: < 150 KB (gzipped)

---

## API Reference

### Core Types

```rust
// Cursor position (0-indexed)
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

// A cursor with optional selection
pub struct Cursor {
    pub head: CursorPosition,
    pub anchor: CursorPosition,
}

// Editor configuration
pub struct EditorConfig {
    pub tab_size: usize,
    pub insert_spaces: bool,
    pub word_wrap: bool,
    pub show_line_numbers: bool,
    // ...
}
```

---

## Changelog

### v0.1.0 (Initial Release)

- Core Editor component
- Cursor management with multi-cursor support
- Undo/redo with coalescing
- Optional features: find-replace, folding, statistics
- Full SSR/CSR support
- CSS custom property theming
