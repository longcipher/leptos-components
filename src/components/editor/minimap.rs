//! Minimap navigation
//!
//! Provides VS Code-style minimap navigation for the editor.

use leptos::prelude::*;

/// Output from minimap interaction.
#[derive(Debug, Clone, Default)]
pub struct MinimapOutput {
    /// Line to scroll to (if user clicked)
    pub scroll_to_line: Option<usize>,
    /// Whether the minimap is being dragged
    pub is_dragging: bool,
}

/// A VS Code-style minimap component.
///
/// Displays a zoomed-out view of the document for quick navigation.
///
/// # Example
///
/// ```rust,ignore
/// <Minimap
///     content=content
///     scroll_line=scroll_line
///     visible_lines=30
///     width=100.0
///     on_navigate=move |line| scroll_to_line(line)
/// />
/// ```
#[component]
pub fn Minimap(
    /// Document content
    #[prop(into)]
    content: Signal<String>,

    /// Current scroll line
    #[prop(into)]
    scroll_line: Signal<usize>,

    /// Number of visible lines in viewport
    #[prop(optional, default = 30)]
    visible_lines: usize,

    /// Width in pixels
    #[prop(optional, default = 80.0)]
    width: f32,

    /// Show search highlights (reserved for future use)
    #[prop(optional, default = false)]
    #[allow(unused_variables)]
    show_highlights: bool,

    /// Navigation callback
    #[prop(into, optional)]
    on_navigate: Option<Callback<usize>>,

    /// Additional CSS classes
    #[prop(into, optional)]
    class: Option<String>,
) -> impl IntoView {
    // Calculate line count
    let line_count = Memo::new(move |_| {
        let text = content.get();
        if text.is_empty() {
            1
        } else {
            text.chars().filter(|&c| c == '\n').count() + 1
        }
    });

    // Handle click on minimap
    let handle_click = move |ev: web_sys::MouseEvent| {
        let target = event_target::<web_sys::HtmlElement>(&ev);
        let rect = target.get_bounding_client_rect();
        let y = ev.client_y() as f64 - rect.top();
        let height = rect.height();

        let total_lines = line_count.get();
        let clicked_line = ((y / height) * total_lines as f64).floor() as usize;
        let target_line = clicked_line.min(total_lines.saturating_sub(1));

        if let Some(callback) = on_navigate.as_ref() {
            callback.run(target_line);
        }
    };

    // Calculate viewport indicator position and height
    let viewport_style = move || {
        let total = line_count.get();
        let scroll = scroll_line.get();
        let visible = visible_lines;

        if total == 0 {
            return "top: 0; height: 100%".to_string();
        }

        let top_percent = (scroll as f32 / total as f32) * 100.0;
        let height_percent = (visible as f32 / total as f32).min(1.0) * 100.0;

        format!(
            "top: {:.1}%; height: {:.1}%",
            top_percent.min(100.0 - height_percent),
            height_percent
        )
    };

    let css_class = move || {
        let mut classes = vec!["leptos-minimap"];
        if let Some(ref custom) = class {
            classes.push(custom);
        }
        classes.join(" ")
    };

    view! {
      <div class=css_class style=format!("width: {}px", width) on:click=handle_click>
        // Document preview (simplified lines)
        <div class="leptos-minimap-content">
          {move || {
            let text = content.get();
            text
              .lines()
              .enumerate()
              .map(|(i, line)| {
                let line_width = (line.len() as f32 * 0.8).min(width - 8.0);
                view! {
                  <div
                    class="leptos-minimap-line"
                    style=format!("width: {}px", line_width)
                    data-line=i
                  />
                }
              })
              .collect::<Vec<_>>()
          }}
        </div>

        // Viewport indicator
        <div class="leptos-minimap-viewport" style=viewport_style />
      </div>
    }
}

/// Default CSS styles for the minimap.
pub const MINIMAP_STYLES: &str = r"
.leptos-minimap {
    position: relative;
    background: rgba(0, 0, 0, 0.2);
    border-left: 1px solid var(--editor-border, #3c3c3c);
    cursor: pointer;
    user-select: none;
    overflow: hidden;
}

.leptos-minimap-content {
    padding: 4px;
}

.leptos-minimap-line {
    height: 2px;
    margin-bottom: 1px;
    background: var(--editor-fg, #d4d4d4);
    opacity: 0.3;
    border-radius: 1px;
}

.leptos-minimap-viewport {
    position: absolute;
    left: 0;
    right: 0;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    pointer-events: none;
}

.leptos-minimap:hover .leptos-minimap-viewport {
    background: rgba(255, 255, 255, 0.15);
}
";
