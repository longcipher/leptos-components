//! Core Editor Component
//!
//! The main text editor component with full editing capabilities.

use leptos::prelude::*;

use super::state::{EditorConfig, EditorState};

/// A production-ready text editor component.
///
/// The Editor provides a full-featured text editing experience with:
/// - Syntax highlighting (with `syntax-highlighting` feature)
/// - Line numbers
/// - Find and replace (with `find-replace` feature)
/// - Undo/redo
/// - Multiple cursors (planned)
/// - Code folding (with `folding` feature)
///
/// # Example
///
/// ```rust,ignore
/// use leptos::prelude::*;
/// use longcipher_leptos_components::editor::Editor;
///
/// #[component]
/// fn MyEditor() -> impl IntoView {
///     let (content, set_content) = signal(String::new());
///
///     view! {
///         <Editor
///             value=content
///             on_change=move |v| set_content.set(v)
///             language="rust"
///             show_line_numbers=true
///         />
///     }
/// }
/// ```
///
/// # Styling
///
/// The editor uses CSS custom properties for theming. Override these in your CSS:
///
/// ```css
/// .leptos-editor {
///     --editor-bg: #1e1e1e;
///     --editor-fg: #d4d4d4;
///     --editor-line-number-fg: #858585;
///     --editor-selection-bg: #264f78;
///     --editor-cursor: #aeafad;
/// }
/// ```
#[component]
#[allow(
    clippy::too_many_lines,
    clippy::needless_pass_by_value,
    clippy::fn_params_excessive_bools
)]
pub fn Editor(
    /// The current value of the editor (controlled)
    #[prop(into)]
    value: Signal<String>,

    /// Callback when the value changes
    #[prop(into, optional)]
    on_change: Option<Callback<String>>,

    /// Placeholder text shown when editor is empty
    #[prop(into, optional)]
    placeholder: Option<String>,

    /// Programming language for syntax highlighting (e.g., "rust", "javascript")
    #[prop(into, optional)]
    language: Option<String>,

    /// Whether the editor is read-only
    #[prop(optional, default = false)]
    read_only: bool,

    /// Whether to show line numbers
    #[prop(optional, default = true)]
    show_line_numbers: bool,

    /// Whether word wrap is enabled
    #[prop(optional, default = true)]
    word_wrap: bool,

    /// Tab size in spaces
    #[prop(optional, default = 4)]
    tab_size: usize,

    /// Font size in pixels
    #[prop(optional, default = 14.0)]
    font_size: f32,

    /// Additional CSS classes to apply
    #[prop(into, optional)]
    class: Option<String>,

    /// Minimum height (CSS value like "200px" or "10rem")
    #[prop(into, optional)]
    min_height: Option<String>,

    /// Maximum height (CSS value like "500px" or "80vh")
    #[prop(into, optional)]
    max_height: Option<String>,

    /// ID attribute for the editor element
    #[prop(into, optional)]
    id: Option<String>,

    /// Callback when the editor receives focus
    #[prop(into, optional)]
    on_focus: Option<Callback<()>>,

    /// Callback when the editor loses focus
    #[prop(into, optional)]
    on_blur: Option<Callback<()>>,

    /// Callback when cursor position changes (line, column)
    #[prop(into, optional)]
    on_cursor_change: Option<Callback<(usize, usize)>>,

    /// Callback when selection changes (selected text or None)
    #[prop(into, optional)]
    on_selection_change: Option<Callback<Option<String>>>,

    /// Whether to auto-focus on mount
    #[prop(optional, default = false)]
    autofocus: bool,

    /// Whether bracket matching is enabled
    #[prop(optional, default = true)]
    match_brackets: bool,

    /// Whether to highlight the current line
    #[prop(optional, default = true)]
    highlight_current_line: bool,
) -> impl IntoView {
    // Internal state
    let (cursor_line, set_cursor_line) = signal(0usize);
    let (cursor_col, set_cursor_col) = signal(0usize);
    let (is_focused, set_is_focused) = signal(false);

    // Create editor state
    let editor_state = StoredValue::new(EditorState::with_config(
        value.get_untracked(),
        EditorConfig {
            tab_size,
            word_wrap,
            show_line_numbers,
            highlight_current_line,
            match_brackets,
            font_size,
            read_only,
            ..Default::default()
        },
    ));

    // Compute line count for line numbers
    let line_count = Memo::new(move |_| {
        let content = value.get();
        if content.is_empty() {
            1
        } else {
            content.chars().filter(|&c| c == '\n').count() + 1
        }
    });

    // Generate line number elements
    let line_numbers_view = move || {
        if !show_line_numbers {
            return None;
        }

        let count = line_count.get();
        let current_line = cursor_line.get();

        Some(view! {
          <div class="leptos-editor-line-numbers" aria-hidden="true">
            {(1..=count)
              .map(|n| {
                let is_current = n - 1 == current_line;
                view! {
                  <div class="leptos-editor-line-number" class:current=is_current>
                    {n}
                  </div>
                }
              })
              .collect::<Vec<_>>()}
          </div>
        })
    };

    // Build CSS class string
    let css_class = move || {
        let mut classes = vec!["leptos-editor"];

        if is_focused.get() {
            classes.push("focused");
        }
        if read_only {
            classes.push("read-only");
        }
        if word_wrap {
            classes.push("word-wrap");
        }
        if show_line_numbers {
            classes.push("with-line-numbers");
        }

        if let Some(ref custom) = class {
            classes.push(custom);
        }

        classes.join(" ")
    };

    // Build inline styles
    let inline_style = move || {
        let mut styles = vec![
            format!("--editor-font-size: {}px", font_size),
            format!("--editor-tab-size: {}", tab_size),
        ];

        if let Some(ref min_h) = min_height {
            styles.push(format!("min-height: {min_h}"));
        }
        if let Some(ref max_h) = max_height {
            styles.push(format!("max-height: {max_h}"));
        }

        styles.join("; ")
    };

    // Handle input changes
    let handle_input = move |ev: web_sys::Event| {
        if read_only {
            return;
        }

        let target = event_target::<web_sys::HtmlTextAreaElement>(&ev);
        let new_value = target.value();

        if let Some(callback) = on_change.as_ref() {
            callback.run(new_value);
        }
    };

    // Handle focus
    let handle_focus = move |_| {
        set_is_focused.set(true);
        if let Some(callback) = on_focus.as_ref() {
            callback.run(());
        }
    };

    // Handle blur
    let handle_blur = move |_| {
        set_is_focused.set(false);
        if let Some(callback) = on_blur.as_ref() {
            callback.run(());
        }
    };

    // Handle selection change and cursor position
    let handle_select = move |ev: web_sys::Event| {
        let target = event_target::<web_sys::HtmlTextAreaElement>(&ev);

        // Get cursor position
        if let (Ok(start), Ok(end)) = (target.selection_start(), target.selection_end()) {
            let start = start.unwrap_or(0) as usize;
            let end = end.unwrap_or(0) as usize;

            // Calculate line and column from offset
            let content = value.get();
            let (line, col) = offset_to_line_col(&content, start);

            set_cursor_line.set(line);
            set_cursor_col.set(col);

            if let Some(callback) = on_cursor_change.as_ref() {
                callback.run((line + 1, col + 1)); // 1-indexed for display
            }

            // Selection changed
            if let Some(callback) = on_selection_change.as_ref() {
                let selected = if start != end && end <= content.len() {
                    content.get(start..end).map(String::from)
                } else {
                    None
                };
                callback.run(selected);
            }
        }
    };

    // Handle keyboard shortcuts
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        let key = ev.key();
        let ctrl_or_cmd = ev.ctrl_key() || ev.meta_key();
        let shift = ev.shift_key();

        // Tab handling
        if key == "Tab" && !read_only {
            ev.prevent_default();

            // Get current textarea
            let target = event_target::<web_sys::HtmlTextAreaElement>(&ev);
            if let (Ok(Some(start)), Ok(Some(end))) =
                (target.selection_start(), target.selection_end())
            {
                let start = start as usize;
                let end = end as usize;
                let content = value.get();

                let indent = " ".repeat(tab_size);

                if shift {
                    // Shift+Tab: Unindent
                    // TODO: Implement unindent
                } else {
                    // Tab: Indent
                    let new_content = format!("{}{}{}", &content[..start], indent, &content[end..]);

                    if let Some(callback) = on_change.as_ref() {
                        callback.run(new_content);
                    }

                    // Restore cursor position
                    #[allow(clippy::cast_possible_truncation)]
                    let new_pos = (start + tab_size) as u32;
                    let _ = target.set_selection_start(Some(new_pos));
                    let _ = target.set_selection_end(Some(new_pos));
                }
            }
        }

        // Undo: Ctrl+Z
        if ctrl_or_cmd && key == "z" && !shift {
            ev.prevent_default();
            editor_state.update_value(|state| {
                if state.undo()
                    && let Some(callback) = on_change.as_ref()
                {
                    callback.run(state.content.clone());
                }
            });
        }

        // Redo: Ctrl+Shift+Z or Ctrl+Y
        if ctrl_or_cmd && ((key == "z" && shift) || key == "y") {
            ev.prevent_default();
            editor_state.update_value(|state| {
                if state.redo()
                    && let Some(callback) = on_change.as_ref()
                {
                    callback.run(state.content.clone());
                }
            });
        }

        // Select All: Ctrl+A
        if ctrl_or_cmd && key == "a" {
            // Let browser handle this
        }
    };

    view! {
      <div class=css_class style=inline_style>
        // Line numbers gutter
        {line_numbers_view}

        // Main editor area
        <div class="leptos-editor-content">
          <textarea
            id=id
            class="leptos-editor-textarea"
            prop:value=move || value.get()
            placeholder=placeholder.clone().unwrap_or_default()
            readonly=read_only
            spellcheck="false"
            autocomplete="off"
            aria-label="Code editor"
            aria-multiline="true"
            on:input=handle_input
            on:focus=handle_focus
            on:blur=handle_blur
            on:select=handle_select
            on:keydown=handle_keydown
            autofocus=autofocus
          />

          // Placeholder overlay (for styled placeholder)
          {
            let placeholder_for_show = placeholder.clone();
            let placeholder_for_render = placeholder.clone();
            view! {
              <Show when=move || value.get().is_empty() && placeholder_for_show.is_some()>
                <div class="leptos-editor-placeholder" aria-hidden="true">
                  {placeholder_for_render.clone().unwrap_or_default()}
                </div>
              </Show>
            }
          }
        </div>

        // Status bar
        <div class="leptos-editor-status">
          <span class="leptos-editor-status-position">
            "Ln " {move || cursor_line.get() + 1} ", Col " {move || cursor_col.get() + 1}
          </span>
          {
            let language_for_status = language.clone();
            language_for_status
              .as_ref()
              .map(|lang| {
                view! { <span class="leptos-editor-status-language">{lang.clone()}</span> }
              })
          }
        </div>
      </div>
    }
}

/// Convert a byte offset to line and column (0-indexed).
fn offset_to_line_col(text: &str, offset: usize) -> (usize, usize) {
    let mut line = 0;
    let mut col = 0;
    let mut current_offset = 0;

    for ch in text.chars() {
        if current_offset >= offset {
            break;
        }
        current_offset += ch.len_utf8();

        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    (line, col)
}

/// Default CSS styles for the editor component.
///
/// Include this in your application to get the default styling.
pub const DEFAULT_STYLES: &str = r"
.leptos-editor {
    --editor-bg: #1e1e1e;
    --editor-fg: #d4d4d4;
    --editor-line-number-fg: #858585;
    --editor-line-number-active-fg: #c6c6c6;
    --editor-selection-bg: #264f78;
    --editor-cursor: #aeafad;
    --editor-gutter-bg: #1e1e1e;
    --editor-border: #3c3c3c;
    --editor-current-line-bg: rgba(255, 255, 255, 0.04);
    --editor-font-size: 14px;
    --editor-line-height: 1.5;
    --editor-tab-size: 4;

    display: flex;
    flex-direction: column;
    background: var(--editor-bg);
    color: var(--editor-fg);
    border: 1px solid var(--editor-border);
    border-radius: 4px;
    font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', 'Monaco', monospace;
    font-size: var(--editor-font-size);
    line-height: var(--editor-line-height);
    overflow: hidden;
    position: relative;
}

.leptos-editor.focused {
    border-color: #3b82f6;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.2);
}

.leptos-editor.read-only {
    opacity: 0.7;
    cursor: not-allowed;
}

.leptos-editor-content {
    display: flex;
    flex: 1;
    overflow: hidden;
    position: relative;
}

.leptos-editor-line-numbers {
    background: var(--editor-gutter-bg);
    color: var(--editor-line-number-fg);
    padding: 8px 12px 8px 8px;
    text-align: right;
    user-select: none;
    border-right: 1px solid var(--editor-border);
    overflow: hidden;
    flex-shrink: 0;
    min-width: 3em;
}

.leptos-editor-line-number {
    line-height: var(--editor-line-height);
}

.leptos-editor-line-number.current {
    color: var(--editor-line-number-active-fg);
    font-weight: 600;
}

.leptos-editor-textarea {
    flex: 1;
    width: 100%;
    height: 100%;
    min-height: 100px;
    padding: 8px 12px;
    margin: 0;
    border: none;
    outline: none;
    background: transparent;
    color: inherit;
    font: inherit;
    line-height: inherit;
    resize: none;
    tab-size: var(--editor-tab-size);
    -moz-tab-size: var(--editor-tab-size);
    overflow: auto;
}

.leptos-editor-textarea::selection {
    background: var(--editor-selection-bg);
}

.leptos-editor-textarea::-webkit-scrollbar {
    width: 10px;
    height: 10px;
}

.leptos-editor-textarea::-webkit-scrollbar-track {
    background: var(--editor-bg);
}

.leptos-editor-textarea::-webkit-scrollbar-thumb {
    background: #424242;
    border-radius: 5px;
}

.leptos-editor-textarea::-webkit-scrollbar-thumb:hover {
    background: #4f4f4f;
}

.leptos-editor-placeholder {
    position: absolute;
    top: 8px;
    left: 12px;
    color: var(--editor-line-number-fg);
    pointer-events: none;
    font-style: italic;
}

.leptos-editor.with-line-numbers .leptos-editor-placeholder {
    left: calc(3em + 24px);
}

.leptos-editor-status {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 4px 12px;
    background: rgba(0, 0, 0, 0.2);
    border-top: 1px solid var(--editor-border);
    font-size: 0.85em;
    color: var(--editor-line-number-fg);
}

.leptos-editor-status-position {
    font-family: inherit;
}

.leptos-editor-status-language {
    text-transform: capitalize;
}

/* Light theme variant */
.leptos-editor.light {
    --editor-bg: #ffffff;
    --editor-fg: #1e293b;
    --editor-line-number-fg: #94a3b8;
    --editor-line-number-active-fg: #334155;
    --editor-selection-bg: #bfdbfe;
    --editor-cursor: #1e293b;
    --editor-gutter-bg: #f8fafc;
    --editor-border: #e2e8f0;
    --editor-current-line-bg: rgba(0, 0, 0, 0.02);
}

/* Word wrap disabled */
.leptos-editor:not(.word-wrap) .leptos-editor-textarea {
    white-space: pre;
    overflow-x: auto;
}

/* Accessibility: Respect reduced motion preference */
@media (prefers-reduced-motion: reduce) {
    .leptos-editor,
    .leptos-editor * {
        transition: none !important;
    }
}
";
