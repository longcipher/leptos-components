//! Editor Component Module
//!
//! This module provides a rich text editor component with features including:
//!
//! - **Basic Editing** - Text input, cursor movement, selection
//! - **Undo/Redo** - Full history management
//! - **Line Numbers** - Optional line number gutter
//! - **Syntax Highlighting** - Code syntax coloring (with `syntax-highlighting` feature)
//! - **Find & Replace** - Search and replace functionality (with `find-replace` feature)
//! - **Code Folding** - Collapse/expand regions (with `folding` feature)
//! - **Statistics** - Word count, character count, etc. (with `statistics` feature)
//! - **Minimap** - VS Code-style navigation (with `minimap` feature)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use longcipher_leptos_components::editor::Editor;
//!
//! #[component]
//! fn MyEditor() -> impl IntoView {
//!     let (content, set_content) = signal(String::new());
//!
//!     view! {
//!         <Editor
//!             value=content
//!             on_change=move |v| set_content.set(v)
//!             language="rust"
//!             show_line_numbers=true
//!             class="my-custom-editor"
//!         />
//!     }
//! }
//! ```
//!
//! ## Styling
//!
//! The editor uses CSS custom properties (variables) for theming:
//!
//! ```css
//! .leptos-editor {
//!     --editor-bg: #1e1e1e;
//!     --editor-fg: #d4d4d4;
//!     --editor-line-number-fg: #858585;
//!     --editor-line-number-active-fg: #c6c6c6;
//!     --editor-selection-bg: #264f78;
//!     --editor-cursor: #aeafad;
//!     --editor-gutter-bg: #1e1e1e;
//!     --editor-border: #3c3c3c;
//! }
//! ```

// Core modules (always available with editor feature)
mod core;
mod cursor;
mod history;
mod selection;
mod state;

// Feature-gated modules
#[cfg(feature = "find-replace")]
#[cfg_attr(docsrs, doc(cfg(feature = "find-replace")))]
mod find_replace;

#[cfg(feature = "folding")]
#[cfg_attr(docsrs, doc(cfg(feature = "folding")))]
mod folding;

#[cfg(feature = "line-numbers")]
#[cfg_attr(docsrs, doc(cfg(feature = "line-numbers")))]
mod line_numbers;

#[cfg(feature = "minimap")]
#[cfg_attr(docsrs, doc(cfg(feature = "minimap")))]
mod minimap;

#[cfg(feature = "statistics")]
#[cfg_attr(docsrs, doc(cfg(feature = "statistics")))]
mod statistics;

#[cfg(feature = "syntax-highlighting")]
#[cfg_attr(docsrs, doc(cfg(feature = "syntax-highlighting")))]
mod syntax;

// ============================================================================
// Public re-exports
// ============================================================================

// Core types (always available)
pub use core::{DEFAULT_STYLES, Editor, EditorProps};

pub use cursor::{Cursor, CursorPosition, CursorSet};
// Feature-gated re-exports
#[cfg(feature = "find-replace")]
#[cfg_attr(docsrs, doc(cfg(feature = "find-replace")))]
pub use find_replace::{FindOptions, FindResult, FindState};
#[cfg(feature = "folding")]
#[cfg_attr(docsrs, doc(cfg(feature = "folding")))]
pub use folding::{FoldKind, FoldRegion, FoldState, detect_markdown_folds};
pub use history::{History, HistoryConfig, HistoryEntry};
#[cfg(feature = "line-numbers")]
#[cfg_attr(docsrs, doc(cfg(feature = "line-numbers")))]
pub use line_numbers::{count_lines, gutter_width};
#[cfg(feature = "minimap")]
#[cfg_attr(docsrs, doc(cfg(feature = "minimap")))]
pub use minimap::{MINIMAP_STYLES, Minimap, MinimapOutput};
pub use selection::{Selection, SelectionMode};
pub use state::{EditorConfig, EditorState};
#[cfg(feature = "statistics")]
#[cfg_attr(docsrs, doc(cfg(feature = "statistics")))]
pub use statistics::{DocumentStats, TextStats};
#[cfg(feature = "syntax-highlighting")]
#[cfg_attr(docsrs, doc(cfg(feature = "syntax-highlighting")))]
pub use syntax::{HighlightedLine, HighlightedSpan, Highlighter, Language, SyntaxConfig};
