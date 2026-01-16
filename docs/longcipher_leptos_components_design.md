# Leptos Components Library Design Document

A production-ready UI component library for Leptos applications.

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Design Principles](#design-principles)
4. [Component Standards](#component-standards)
5. [Feature System](#feature-system)
6. [Accessibility Standards](#accessibility-standards)
7. [Performance Strategies](#performance-strategies)
8. [Testing Standards](#testing-standards)

---

## Overview

**longcipher-leptos-components** is a modular, tree-shakeable UI component library built specifically for Leptos web applications. It provides production-grade components with:

- **Type-safe props** using Leptos's component system
- **SSR/CSR compatibility** for full-stack applications
- **Feature flags** for minimal bundle sizes
- **Accessible by default** following WCAG 2.1 guidelines
- **Themeable** via CSS custom properties

### Target Users

- Leptos application developers building production web apps
- Teams requiring consistent, reusable UI components
- Projects needing SSR-compatible components

### Design Philosophy

> Build components that are **simple to use, hard to misuse**, and **production-ready out of the box**.

---

## Architecture

### Crate Structure

```text
longcipher-leptos-components/
├── Cargo.toml              # Workspace manifest with feature definitions
├── src/
│   ├── lib.rs              # Crate root with module exports
│   ├── components/         # Component implementations
│   │   ├── mod.rs
│   │   └── editor/         # Editor component family
│   │       ├── mod.rs
│   │       ├── core.rs     # Main Editor component
│   │       ├── cursor.rs   # Cursor management
│   │       ├── history.rs  # Undo/redo
│   │       ├── selection.rs
│   │       ├── state.rs    # Editor state
│   │       ├── find_replace.rs  # (feature: find-replace)
│   │       ├── folding.rs       # (feature: folding)
│   │       ├── statistics.rs    # (feature: statistics)
│   │       └── minimap.rs       # (feature: minimap)
│   └── helpers/            # Internal utilities
│       ├── mod.rs
│       ├── dom.rs          # DOM helpers
│       └── text.rs         # Text utilities
├── examples/               # Example applications
└── tests/                  # Integration tests
```

### Dependency Graph

```text
longcipher-leptos-components
├── leptos (core)
├── web-sys (feature: editor)
├── wasm-bindgen (feature: editor)
├── syntect (feature: syntax-highlighting)
├── comrak (feature: markdown)
├── regex (feature: find-replace)
├── serde (serialization)
└── thiserror (error handling)
```

---

## Design Principles

### 1. Props with `#[prop(into)]`

All component props that accept strings, signals, or callbacks MUST use `#[prop(into)]` to enable flexible input types:

```rust
#[component]
pub fn Button(
    #[prop(into)] label: String,           // Accepts &str, String, Cow<str>
    #[prop(into, optional)] class: Option<String>,
    #[prop(into, optional)] on_click: Option<Callback<()>>,
) -> impl IntoView { /* ... */ }
```

### 2. Optional Props with Defaults

Optional props should be clearly marked and have sensible defaults. Use `Option<T>` for props that can be omitted.

### 3. Custom Styling via `class`

Every component MUST accept an optional `class` prop to allow users to add custom CSS classes.

### 4. Controlled Components

Components should prefer being "controlled" where possible, using signals passed from the parent to manage state.

---

## Component Standards

### Implementation Checklist

- [ ] Uses `#[component]` macro
- [ ] Returns `impl IntoView`
- [ ] All public props documented
- [ ] Supports SSR (no direct window/document access without checks)
- [ ] Follows BEM-like CSS class naming
- [ ] Includes unit tests in the same file
- [ ] Includes doc tests for main component

### File Naming

- Components: `snake_case.rs` (match the component name in lowercase)
- Modules: `mod.rs` for exporting sub-components

---

## Feature System

The library uses Cargo features to keep bundle sizes small.

### Naming Conventions

- Component-level: `editor`, `modal`, `tabs`
- Feature-level: `syntax-highlighting`, `find-replace`
- Bundle: `editor-full` (includes all sub-features)

### Feature Gating Code

```rust
#[cfg(feature = "editor")]
pub mod editor;
```

---

## Accessibility Standards

### Requirements

All components MUST meet WCAG 2.1 AA standards:

1. **Color contrast**: 4.5:1 for normal text, 3:1 for large text
2. **Keyboard navigation**: All interactive elements focusable
3. **Focus indicators**: Visible focus states
4. **ARIA labels**: Proper labeling for screen readers
5. **Reduced motion**: Respect `prefers-reduced-motion`

---

## Performance Strategies

1. **Feature-based tree shaking**: Only include used features
2. **Lazy initialization**: Defer heavy operations
3. **Memoization**: Use `Memo` for computed values
4. **Virtual scrolling**: For large lists/documents
5. **Reduced DOM depth**: Keep the DOM tree as shallow as possible

---

## Testing Standards

### Unit Tests

Every module MUST include unit tests in a `tests` submodule:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    // ... tests ...
}
```

### Integration Tests

Complex interactions and cross-component behavior should be tested in the `/tests` directory.

### WASM Tests

Use `wasm-bindgen-test` for components that require browser APIs.

---

## License

MIT OR Apache-2.0
