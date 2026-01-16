//! Syntax highlighting support
//!
//! Provides code syntax highlighting using syntect.

#[cfg(feature = "syntax-highlighting")]
use syntect::highlighting::ThemeSet;
#[cfg(feature = "syntax-highlighting")]
use syntect::parsing::SyntaxSet;

/// Supported languages for syntax highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// Rust
    Rust,
    /// JavaScript
    JavaScript,
    /// TypeScript
    TypeScript,
    /// Python
    Python,
    /// HTML
    Html,
    /// CSS
    Css,
    /// JSON
    Json,
    /// YAML
    Yaml,
    /// TOML
    Toml,
    /// Markdown
    Markdown,
    /// SQL
    Sql,
    /// Shell/Bash
    Shell,
    /// Go
    Go,
    /// C
    C,
    /// C++
    Cpp,
    /// Java
    Java,
    /// Plain text (no highlighting)
    PlainText,
}

impl Language {
    /// Detect language from file extension.
    #[must_use]
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Self::Rust,
            "js" | "mjs" | "cjs" => Self::JavaScript,
            "ts" | "mts" | "cts" | "tsx" => Self::TypeScript,
            "py" | "pyi" => Self::Python,
            "html" | "htm" => Self::Html,
            "css" | "scss" | "sass" | "less" => Self::Css,
            "json" => Self::Json,
            "yaml" | "yml" => Self::Yaml,
            "toml" => Self::Toml,
            "md" | "markdown" => Self::Markdown,
            "sql" => Self::Sql,
            "sh" | "bash" | "zsh" | "fish" => Self::Shell,
            "go" => Self::Go,
            "c" | "h" => Self::C,
            "cpp" | "cxx" | "cc" | "hpp" | "hxx" => Self::Cpp,
            "java" => Self::Java,
            _ => Self::PlainText,
        }
    }

    /// Get the syntect syntax name.
    #[must_use]
    pub fn syntax_name(&self) -> &'static str {
        match self {
            Self::Rust => "Rust",
            Self::JavaScript => "JavaScript",
            Self::TypeScript => "TypeScript",
            Self::Python => "Python",
            Self::Html => "HTML",
            Self::Css => "CSS",
            Self::Json => "JSON",
            Self::Yaml => "YAML",
            Self::Toml => "TOML",
            Self::Markdown => "Markdown",
            Self::Sql => "SQL",
            Self::Shell => "Bourne Again Shell (bash)",
            Self::Go => "Go",
            Self::C => "C",
            Self::Cpp => "C++",
            Self::Java => "Java",
            Self::PlainText => "Plain Text",
        }
    }
}

/// Configuration for syntax highlighting.
#[derive(Debug, Clone)]
pub struct SyntaxConfig {
    /// The language to use
    pub language: Language,
    /// Whether to use dark theme
    pub is_dark: bool,
    /// Whether highlighting is enabled
    pub enabled: bool,
}

impl Default for SyntaxConfig {
    fn default() -> Self {
        Self {
            language: Language::PlainText,
            is_dark: true,
            enabled: true,
        }
    }
}

/// A highlighted line with styled spans.
#[derive(Debug, Clone)]
pub struct HighlightedLine {
    /// Spans of text with their styles
    pub spans: Vec<HighlightedSpan>,
}

/// A span of highlighted text.
#[derive(Debug, Clone)]
pub struct HighlightedSpan {
    /// The text content
    pub text: String,
    /// Foreground color (CSS format)
    pub color: String,
    /// Font weight (normal, bold)
    pub font_weight: String,
    /// Font style (normal, italic)
    pub font_style: String,
}

impl HighlightedSpan {
    /// Create a plain (unstyled) span.
    #[must_use]
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: "inherit".to_string(),
            font_weight: "normal".to_string(),
            font_style: "normal".to_string(),
        }
    }

    /// Generate CSS style string for this span.
    #[must_use]
    pub fn style(&self) -> String {
        format!(
            "color: {}; font-weight: {}; font-style: {}",
            self.color, self.font_weight, self.font_style
        )
    }
}

/// Syntax highlighter.
#[cfg(feature = "syntax-highlighting")]
pub struct Highlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

#[cfg(feature = "syntax-highlighting")]
impl Highlighter {
    /// Create a new highlighter with default syntax and theme sets.
    #[must_use]
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    /// Highlight a line of code.
    pub fn highlight_line(&self, line: &str, language: Language, is_dark: bool) -> HighlightedLine {
        use syntect::easy::HighlightLines;

        let theme_name = if is_dark {
            "base16-ocean.dark"
        } else {
            "base16-ocean.light"
        };

        let syntax = self
            .syntax_set
            .find_syntax_by_name(language.syntax_name())
            .or_else(|| Some(self.syntax_set.find_syntax_plain_text()));

        let theme = self.theme_set.themes.get(theme_name).unwrap_or_else(|| {
            self.theme_set
                .themes
                .values()
                .next()
                .expect("No themes available")
        });

        let spans = if let Some(syntax) = syntax {
            let mut highlighter = HighlightLines::new(syntax, theme);

            match highlighter.highlight_line(line, &self.syntax_set) {
                Ok(ranges) => ranges
                    .iter()
                    .map(|(style, text)| HighlightedSpan {
                        text: text.to_string(),
                        color: format!(
                            "rgb({}, {}, {})",
                            style.foreground.r, style.foreground.g, style.foreground.b
                        ),
                        font_weight: if style
                            .font_style
                            .contains(syntect::highlighting::FontStyle::BOLD)
                        {
                            "bold".to_string()
                        } else {
                            "normal".to_string()
                        },
                        font_style: if style
                            .font_style
                            .contains(syntect::highlighting::FontStyle::ITALIC)
                        {
                            "italic".to_string()
                        } else {
                            "normal".to_string()
                        },
                    })
                    .collect(),
                Err(_) => vec![HighlightedSpan::plain(line)],
            }
        } else {
            vec![HighlightedSpan::plain(line)]
        };

        HighlightedLine { spans }
    }
}

#[cfg(feature = "syntax-highlighting")]
impl Default for Highlighter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_extension() {
        assert_eq!(Language::from_extension("rs"), Language::Rust);
        assert_eq!(Language::from_extension("js"), Language::JavaScript);
        assert_eq!(Language::from_extension("py"), Language::Python);
        assert_eq!(Language::from_extension("unknown"), Language::PlainText);
    }

    #[test]
    fn test_highlighted_span_style() {
        let span = HighlightedSpan {
            text: "let".to_string(),
            color: "rgb(255, 0, 0)".to_string(),
            font_weight: "bold".to_string(),
            font_style: "normal".to_string(),
        };

        let style = span.style();
        assert!(style.contains("color: rgb(255, 0, 0)"));
        assert!(style.contains("font-weight: bold"));
    }
}
