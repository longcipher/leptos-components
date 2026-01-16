//! Basic Editor Example
//!
//! Demonstrates the fundamental usage of the Editor component.

use leptos::prelude::*;
use longcipher_leptos_components::editor::Editor;

/// A simple editor demo application.
#[component]
pub fn App() -> impl IntoView {
    // Create a signal to hold the editor content
    let (content, set_content) = signal(String::from(
        r#"# Welcome to Leptos Components

This is a **production-ready** text editor built with Leptos.

## Features

- Syntax highlighting
- Line numbers
- Find & Replace
- Undo/Redo
- Code folding

## Example Code

```rust
fn main() {
    println!("Hello, Leptos!");
}
```

Try editing this text!
"#,
    ));

    // Track cursor position for status display
    let (cursor_pos, set_cursor_pos) = signal((1usize, 1usize));

    // Track selection for status display
    let (selection, set_selection) = signal(Option::<String>::None);

    view! {
      <div class="app-container">
        <header class="app-header">
          <h1>"Leptos Editor Demo"</h1>
          <p>"A production-ready text editor component"</p>
        </header>

        <main class="app-main">
          <Editor
            value=content
            on_change=move |v| set_content.set(v)
            on_cursor_change=move |(line, col)| set_cursor_pos.set((line, col))
            on_selection_change=move |sel| set_selection.set(sel)
            language="markdown"
            show_line_numbers=true
            word_wrap=true
            tab_size=4
            font_size=14.0
            placeholder="Start typing..."
            highlight_current_line=true
            match_brackets=true
            min_height="400px"
            class="demo-editor"
          />

          <div class="status-bar">
            <span class="cursor-info">
              "Ln " {move || cursor_pos.get().0} ", Col " {move || cursor_pos.get().1}
            </span>
            <span class="selection-info">
              {move || {
                selection
                  .get()
                  .map(|s| {
                    let chars = s.chars().count();
                    format!("{} chars selected", chars)
                  })
              }}
            </span>
            <span class="word-count">
              {move || {
                let text = content.get();
                let words = text.split_whitespace().count();
                format!("{} words", words)
              }}
            </span>
          </div>
        </main>

        <footer class="app-footer">
          <p>
            "Built with " <a href="https://leptos.dev" target="_blank">
              "Leptos"
            </a> " and "
            <a href="https://github.com/longcipher/longcipher-leptos-components" target="_blank">
              "longcipher-leptos-components"
            </a>
          </p>
        </footer>
      </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
