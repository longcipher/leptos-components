//! DOM helper utilities
//!
//! Provides cross-platform DOM manipulation helpers that work
//! in both SSR and CSR contexts.

#[cfg(feature = "editor")]
use web_sys::Document;

/// Get the current document, if available.
///
/// Returns `None` in SSR context or if document is not available.
#[cfg(feature = "editor")]
#[allow(dead_code)]
pub fn get_document() -> Option<Document> {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window().and_then(|w| w.document())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}

/// Check if we're running in a browser context.
#[must_use]
#[allow(dead_code)]
pub fn is_browser() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window().is_some()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

/// Safely execute code only in browser context.
///
/// This is useful for operations that should only run on the client,
/// such as accessing localStorage, clipboard, or other browser APIs.
#[allow(dead_code)]
pub fn on_browser<F, T>(f: F) -> Option<T>
where
    F: FnOnce() -> T,
{
    if is_browser() { Some(f()) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_browser_in_tests() {
        // In test context (not wasm), this should be false
        #[cfg(not(target_arch = "wasm32"))]
        assert!(!is_browser());
    }
}
