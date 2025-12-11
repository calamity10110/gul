// GUL TUI Editor Widget
// Code editor with syntax highlighting and v2.1 bracket support

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Span,
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::tui::theme::GulTheme;

/// Editor state
#[derive(Default, Clone)]
pub struct EditorState {
    /// Current cursor position (line, column)
    pub cursor: (usize, usize),
    /// Scroll offset (line, column)
    pub scroll: (usize, usize),
    /// Selection start (if any)
    pub selection_start: Option<(usize, usize)>,
    /// Selection end (if any)
    pub selection_end: Option<(usize, usize)>,
}

/// Editor widget
pub struct EditorWidget<'a> {
    /// Content lines
    content: Vec<&'a str>,
    /// Theme
    theme: &'a GulTheme,
    /// File name
    file_name: Option<&'a str>,
    /// Show line numbers
    show_line_numbers: bool,
    /// Highlight current line
    #[allow(dead_code)]
    highlight_current_line: bool,
}

impl<'a> EditorWidget<'a> {
    pub fn new(content: Vec<&'a str>, theme: &'a GulTheme) -> Self {
        EditorWidget {
            content,
            theme,
            file_name: None,
            show_line_numbers: true,
            highlight_current_line: true,
        }
    }

    pub fn file_name(mut self, name: &'a str) -> Self {
        self.file_name = Some(name);
        self
    }

    pub fn show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Tokenize a line for syntax highlighting
    fn tokenize_line(&self, line: &str) -> Vec<Span<'a>> {
        let mut spans = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];

            // Comment
            if ch == '#' {
                let comment: String = chars[i..].iter().collect();
                spans.push(Span::styled(comment, self.theme.comment));
                break;
            }

            // UI Marker ^&^
            if ch == '^' && i + 2 < chars.len() && chars[i + 1] == '&' && chars[i + 2] == '^' {
                spans.push(Span::styled("^&^".to_string(), self.theme.ui_marker));
                i += 3;
                continue;
            }

            // Brackets (v2.1: all equivalent)
            if "([{".contains(ch) || ")]}".contains(ch) {
                spans.push(Span::styled(ch.to_string(), self.theme.bracket));
                i += 1;
                continue;
            }

            // String
            if ch == '"' {
                let mut string_val = String::new();
                string_val.push(ch);
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        string_val.push(chars[i]);
                        i += 1;
                    }
                    string_val.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() {
                    string_val.push(chars[i]);
                    i += 1;
                }
                spans.push(Span::styled(string_val, self.theme.string));
                continue;
            }

            // Number
            if ch.is_ascii_digit() {
                let mut num = String::new();
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    num.push(chars[i]);
                    i += 1;
                }
                spans.push(Span::styled(num, self.theme.number));
                continue;
            }

            // Identifier or keyword
            if ch.is_alphabetic() || ch == '_' {
                let mut ident = String::new();
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    ident.push(chars[i]);
                    i += 1;
                }

                let style = match ident.as_str() {
                    "main" | "fn" | "async" | "struct" | "import" | "const" | "mut" | "if"
                    | "elif" | "else" | "for" | "while" | "loop" | "break" | "continue"
                    | "return" | "await" | "try" | "catch" | "match" | "extern" | "pub"
                    | "true" | "false" => self.theme.keyword,

                    // UI components
                    "button" | "input" | "textarea" | "checkbox" | "radio" | "select"
                    | "slider" | "toggle" | "label" | "text" | "bigtext" | "paragraph"
                    | "sparkline" | "gauge" | "barchart" | "canvas" | "container" | "block"
                    | "row" | "column" | "grid" | "stack" | "split" | "tabs" | "scrollview"
                    | "popup" | "table" | "list" | "tree" | "calendar" | "chart" | "spinner"
                    | "throbber" | "progress" | "toast" | "alert" | "badge" | "menu"
                    | "menubar" | "contextmenu" | "breadcrumb" | "pagination" | "image"
                    | "video" | "audio" | "markdown" | "code" | "prompt" => self.theme.ui_component,

                    _ => self.theme.identifier,
                };

                spans.push(Span::styled(ident, style));
                continue;
            }

            // Operators
            if "+-*/%=<>!&|:.".contains(ch) {
                spans.push(Span::styled(ch.to_string(), self.theme.operator));
                i += 1;
                continue;
            }

            // Default
            spans.push(Span::raw(ch.to_string()));
            i += 1;
        }

        spans
    }
}

impl<'a> StatefulWidget for EditorWidget<'a> {
    type State = EditorState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let title = self.file_name.unwrap_or("Untitled");
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border_focused));

        let inner = block.inner(area);
        block.render(area, buf);

        // Calculate line number width
        let line_num_width = if self.show_line_numbers {
            (self.content.len().max(1).ilog10() + 1) as u16 + 2
        } else {
            0
        };

        // Render visible lines
        let visible_lines = inner.height as usize;
        let start_line = state.scroll.0;
        let end_line = (start_line + visible_lines).min(self.content.len());

        for (i, line_idx) in (start_line..end_line).enumerate() {
            let y = inner.y + i as u16;

            // Line numbers
            if self.show_line_numbers {
                let line_num = format!(
                    "{:>width$} ",
                    line_idx + 1,
                    width = (line_num_width - 2) as usize
                );
                let style = if line_idx == state.cursor.0 {
                    Style::default().fg(self.theme.line_number_active)
                } else {
                    Style::default().fg(self.theme.line_number)
                };
                buf.set_string(inner.x, y, &line_num, style);
            }

            // Line content
            if line_idx < self.content.len() {
                let spans = self.tokenize_line(self.content[line_idx]);
                let mut x = inner.x + line_num_width;

                for span in spans {
                    let text = span.content.to_string();
                    buf.set_string(x, y, &text, span.style);
                    x += text.len() as u16;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_widget_creation() {
        let theme = GulTheme::dark();
        let content = vec!["# Hello", "main:", "    print(\"Hello\")"];
        let editor = EditorWidget::new(content, &theme);

        assert!(editor.show_line_numbers);
    }
}
