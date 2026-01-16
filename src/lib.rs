//! # Leptos Components
//!
//! A production-ready UI component library for Leptos applications.
//!
//! ## Features
//!
//! This crate uses Cargo features to allow tree-shaking and selective inclusion
//! of components. By default, only the `editor` feature is enabled.
//!
//! ### Available Features
//!
//! - `editor` - Core text editor component with essential functionality
//! - `syntax-highlighting` - Syntax highlighting for code (requires `syntect`)
//! - `markdown` - Markdown parsing and rendering (requires `comrak`)
//! - `find-replace` - Find and replace functionality
//! - `folding` - Code folding support
//! - `statistics` - Document statistics (word count, character count, etc.)
//! - `line-numbers` - Line number gutter display
//! - `minimap` - VS Code-style minimap navigation
//! - `editor-full` - All editor features combined
//! - `ssr` - Server-side rendering support
//! - `hydrate` - Hydration support
//! - `csr` - Client-side rendering only
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use leptos::prelude::*;
//! use longcipher_leptos_components::editor::Editor;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let (content, set_content) = signal(String::from("Hello, World!"));
//!     
//!     view! {
//!         <Editor
//!             value=content
//!             on_change=move |new_value| set_content.set(new_value)
//!             placeholder="Enter text..."
//!         />
//!     }
//! }
//! ```
//!
//! ## Design Principles
//!
//! All components in this library follow these principles:
//!
//! 1. **Props with `#[prop(into)]`** - Allow flexible input types
//! 2. **Optional props with defaults** - Sensible defaults for all optional props
//! 3. **Custom styling via `class`** - Every component accepts a `class` prop
//! 4. **Return `impl IntoView`** - Standard Leptos return type
//! 5. **Accessibility first** - ARIA attributes and keyboard navigation
//! 6. **SSR compatible** - Works with server-side rendering
//!
//! ## Component Categories
//!
//! - **Editor** - Rich text and code editing components
//! - **Display** - Read-only content display components (planned)
//! - **Input** - Form input components (planned)
//! - **Feedback** - Alerts, toasts, and notifications (planned)
//! - **Navigation** - Menus, tabs, and navigation components (planned)

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

// Re-export core dependencies for convenience
pub use leptos;

// ============================================================================
// Modules
// ============================================================================

pub mod components;
pub mod helpers;

// ============================================================================
// Re-exports
// ============================================================================

// Re-export all public components at the crate root for convenience
/// Editor components
#[cfg(feature = "editor")]
#[cfg_attr(docsrs, doc(cfg(feature = "editor")))]
pub mod editor {
    pub use crate::components::editor::{Editor, EditorProps};
}

// ============================================================================
// Prelude
// ============================================================================

/// Prelude module for convenient imports
///
/// ```rust,ignore
/// use longcipher_leptos_components::prelude::*;
/// ```
pub mod prelude {
    #[cfg(feature = "editor")]
    pub use crate::components::editor::*;
}
