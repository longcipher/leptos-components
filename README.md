# Leptos Components

A production-ready UI component library for [Leptos](https://leptos.dev) applications.

[![Crates.io](https://img.shields.io/crates/v/longcipher-leptos-components.svg)](https://crates.io/crates/longcipher-leptos-components)
[![Documentation](https://docs.rs/longcipher-leptos-components/badge.svg)](https://docs.rs/longcipher-leptos-components)
[![License](https://img.shields.io/crates/l/longcipher-leptos-components.svg)](LICENSE)

## Features

- 🚀 **Production-ready** components with sensible defaults
- 🎯 **Type-safe** props using Leptos's component system
- 📦 **Tree-shakeable** via Cargo feature flags
- 🌐 **SSR/CSR compatible** for full-stack applications
- ♿ **Accessible** following WCAG 2.1 guidelines
- 🎨 **Themeable** via CSS custom properties

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
longcipher-leptos-components = { version = "0.1", features = ["editor"] }
```

## Quick Start

```rust
use leptos::prelude::*;
use longcipher_leptos_components::editor::Editor;

#[component]
fn App() -> impl IntoView {
    let (content, set_content) = signal(String::new());

    view! {
        <Editor
            value=content
            on_change=move |v| set_content.set(v)
            language="rust"
            show_line_numbers=true
            placeholder="Start typing..."
        />
    }
}
```

## Available Components

### Editor

A full-featured text editor with:

- Syntax highlighting (feature: `syntax-highlighting`)
- Line numbers
- Find & Replace (feature: `find-replace`)
- Undo/Redo
- Code folding (feature: `folding`)
- Document statistics (feature: `statistics`)

```rust
<Editor
    value=content
    on_change=move |v| set_content.set(v)
    language="markdown"
    show_line_numbers=true
    word_wrap=true
    tab_size=4
    font_size=14.0
    highlight_current_line=true
    match_brackets=true
/>
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `editor` | Core text editor (default) |
| `syntax-highlighting` | Code syntax coloring |
| `markdown` | Markdown parsing and preview |
| `find-replace` | Search and replace |
| `folding` | Code folding |
| `statistics` | Word/character counts |
| `line-numbers` | Line number gutter |
| `minimap` | VS Code-style navigation |
| `editor-full` | All editor features |
| `ssr` | Server-side rendering |
| `hydrate` | Hydration support |
| `csr` | Client-side only |

### Usage Examples

```toml
# Minimal editor
longcipher-leptos-components = { version = "0.1", features = ["editor"] }

# Full-featured editor
longcipher-leptos-components = { version = "0.1", features = ["editor-full"] }

# With SSR support
longcipher-leptos-components = { version = "0.1", features = ["editor", "ssr"] }
```

## Styling

Components use CSS custom properties for easy theming:

```css
.leptos-editor {
    --editor-bg: #1e1e1e;
    --editor-fg: #d4d4d4;
    --editor-line-number-fg: #858585;
    --editor-selection-bg: #264f78;
    --editor-cursor: #aeafad;
    --editor-border: #3c3c3c;
}

/* Light theme */
.leptos-editor.light {
    --editor-bg: #ffffff;
    --editor-fg: #1e293b;
    --editor-border: #e2e8f0;
}
```

## Component Design Principles

All components follow these standards:

1. **`#[prop(into)]`** - Flexible input types
2. **Optional props with defaults** - Works with minimal config
3. **`class` prop** - Custom styling support
4. **`impl IntoView`** - Standard Leptos return type
5. **Controlled components** - External state management
6. **Accessible by default** - ARIA labels and keyboard support

## Documentation

- [Library Design](docs/longcipher_leptos_components_design.md) - Architecture and standards
- [Editor Design](docs/editor_component_design.md) - Editor component details
- [API Reference](https://docs.rs/longcipher-leptos-components) - Full API docs
- [Examples](examples/) - Code examples

## Contributing

Contributions are welcome! Please read our contributing guidelines.

### Development

```bash
# Run tests
cargo test --all-features

# Check lints
cargo clippy --all-features

# Format code
cargo fmt

# Build docs
cargo doc --all-features --open
```

## License

Licensed under MIT License.

## Acknowledgments

- Built with [Leptos](https://leptos.dev)
- Design system by [ui-ux-pro-max](https://github.com/example/ui-ux-pro-max)
