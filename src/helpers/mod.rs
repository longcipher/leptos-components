//! Internal helper utilities
//!
//! This module contains internal helper functions and utilities
//! used across the component library.
//!
//! Some utilities are also exported publicly for user convenience.

mod dom;
mod text;

// Internal re-exports (crate-visible)
// Note: These are available for use via dom:: prefix
#[allow(unused_imports)]
pub(crate) use dom::{get_document, is_browser, on_browser};
// Public re-exports (for users who need these utilities)
pub use text::{count_lines, line_range, offset_to_position, position_to_offset, text_stats};
