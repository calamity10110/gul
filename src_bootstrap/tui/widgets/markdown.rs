// GUL TUI Markdown Preview Widget
// Glow-inspired markdown rendering
// Inspired by charmbracelet/glow

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, StatefulWidget, Widget},
};

use crate::tui::theme::GulTheme;

/// Markdown element type
#[derive(Debug, Clone, PartialEq)]
pub enum MarkdownElement {
    Heading(u8, String),
    Paragraph(String),
    CodeBlock(String, String), // (language, content)
    InlineCode(String),
    Bold(String),
    Italic(String),
    Link(String, String), // (text, url)
    ListItem(u8, String), // (depth, text)
    Quote(String),
    HorizontalRule,
    Image(String, String), // (alt, url)
}

/// Parse markdown text into elements
pub fn parse_markdown(text: &str) -> Vec<MarkdownElement> {
    let mut elements = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];

        // Heading
        if let Some(heading) = parse_heading(line) {
            elements.push(heading);
            i += 1;
            continue;
        }

        // Code block
        if line.starts_with("```") {
            let lang = line.trim_start_matches('`').to_string();
            let mut code = String::new();
            i += 1;
            while i < lines.len() && !lines[i].starts_with("```") {
                code.push_str(lines[i]);
                code.push('\n');
                i += 1;
            }
            elements.push(MarkdownElement::CodeBlock(
                lang,
                code.trim_end().to_string(),
            ));
            i += 1;
            continue;
        }

        // Horizontal rule
        if line.starts_with("---") || line.starts_with("***") || line.starts_with("___") {
            elements.push(MarkdownElement::HorizontalRule);
            i += 1;
            continue;
        }

        // Quote
        if line.starts_with('>') {
            let content = line.trim_start_matches('>').trim().to_string();
            elements.push(MarkdownElement::Quote(content));
            i += 1;
            continue;
        }

        // List item
        if let Some(list_item) = parse_list_item(line) {
            elements.push(list_item);
            i += 1;
            continue;
        }

        // Paragraph
        if !line.is_empty() {
            elements.push(MarkdownElement::Paragraph(line.to_string()));
        }

        i += 1;
    }

    elements
}

fn parse_heading(line: &str) -> Option<MarkdownElement> {
    if line.starts_with('#') {
        let level = line.chars().take_while(|&c| c == '#').count() as u8;
        let text = line.trim_start_matches('#').trim().to_string();
        Some(MarkdownElement::Heading(level.min(6), text))
    } else {
        None
    }
}

fn parse_list_item(line: &str) -> Option<MarkdownElement> {
    let trimmed = line.trim_start();
    let depth = (line.len() - trimmed.len()) / 2;

    if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
        let text = trimmed[2..].to_string();
        Some(MarkdownElement::ListItem(depth as u8, text))
    } else if let Some((num, rest)) = trimmed.split_once(". ") {
        if num.chars().all(|c| c.is_ascii_digit()) {
            Some(MarkdownElement::ListItem(depth as u8, rest.to_string()))
        } else {
            None
        }
    } else {
        None
    }
}

/// Rendered markdown line
#[derive(Debug, Clone)]
pub struct RenderedLine {
    pub content: String,
    pub style: Style,
    pub prefix: String,
}

/// Markdown preview state
#[derive(Default, Clone)]
pub struct MarkdownState {
    pub scroll: usize,
    pub content_height: usize,
}

impl MarkdownState {
    pub fn scroll_up(&mut self, n: usize) {
        self.scroll = self.scroll.saturating_sub(n);
    }

    pub fn scroll_down(&mut self, n: usize, viewport_height: usize) {
        let max = self.content_height.saturating_sub(viewport_height);
        self.scroll = (self.scroll + n).min(max);
    }
}

/// Markdown preview widget
pub struct MarkdownWidget<'a> {
    content: &'a str,
    theme: &'a GulTheme,
    title: Option<&'a str>,
}

impl<'a> MarkdownWidget<'a> {
    pub fn new(content: &'a str, theme: &'a GulTheme) -> Self {
        MarkdownWidget {
            content,
            theme,
            title: None,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    fn render_element(&self, element: &MarkdownElement, width: u16) -> Vec<RenderedLine> {
        let mut lines = Vec::new();

        match element {
            MarkdownElement::Heading(level, text) => {
                let prefix = match level {
                    1 => "# ",
                    2 => "## ",
                    3 => "### ",
                    4 => "#### ",
                    _ => "##### ",
                };
                let style = self.theme.keyword.add_modifier(Modifier::BOLD);
                lines.push(RenderedLine {
                    content: text.clone(),
                    style,
                    prefix: prefix.to_string(),
                });
                lines.push(RenderedLine {
                    content: String::new(),
                    style: Style::default(),
                    prefix: String::new(),
                });
            }

            MarkdownElement::Paragraph(text) => {
                // Word wrap
                for wrapped in Self::word_wrap(text, width as usize - 2) {
                    lines.push(RenderedLine {
                        content: wrapped,
                        style: Style::default().fg(self.theme.foreground),
                        prefix: String::new(),
                    });
                }
                lines.push(RenderedLine {
                    content: String::new(),
                    style: Style::default(),
                    prefix: String::new(),
                });
            }

            MarkdownElement::CodeBlock(lang, code) => {
                let header = format!("┌─ {} ", if lang.is_empty() { "code" } else { lang });
                lines.push(RenderedLine {
                    content: header,
                    style: self.theme.comment,
                    prefix: String::new(),
                });

                for line in code.lines() {
                    lines.push(RenderedLine {
                        content: line.to_string(),
                        style: self.theme.string.bg(Color::Rgb(30, 30, 40)),
                        prefix: "│ ".to_string(),
                    });
                }

                lines.push(RenderedLine {
                    content: "└─".to_string(),
                    style: self.theme.comment,
                    prefix: String::new(),
                });
            }

            MarkdownElement::Quote(text) => {
                lines.push(RenderedLine {
                    content: text.clone(),
                    style: self.theme.comment.add_modifier(Modifier::ITALIC),
                    prefix: "│ ".to_string(),
                });
            }

            MarkdownElement::ListItem(depth, text) => {
                let indent = "  ".repeat(*depth as usize);
                lines.push(RenderedLine {
                    content: text.clone(),
                    style: Style::default().fg(self.theme.foreground),
                    prefix: format!("{}• ", indent),
                });
            }

            MarkdownElement::HorizontalRule => {
                lines.push(RenderedLine {
                    content: "─".repeat(width as usize - 2),
                    style: Style::default().fg(self.theme.border),
                    prefix: String::new(),
                });
            }

            _ => {}
        }

        lines
    }

    fn word_wrap(text: &str, max_width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + 1 + word.len() <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        if lines.is_empty() {
            lines.push(String::new());
        }

        lines
    }
}

impl<'a> StatefulWidget for MarkdownWidget<'a> {
    type State = MarkdownState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = if let Some(title) = self.title {
            Block::default()
                .title(format!(" 📝 {} ", title))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.border))
        } else {
            Block::default()
                .title(" 📝 Markdown Preview ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.border))
        };

        let inner = block.inner(area);
        block.render(area, buf);

        // Parse and render markdown
        let elements = parse_markdown(self.content);
        let mut rendered_lines: Vec<RenderedLine> = Vec::new();

        for element in &elements {
            rendered_lines.extend(self.render_element(element, inner.width));
        }

        state.content_height = rendered_lines.len();

        // Render visible lines
        let visible_lines = inner.height as usize;
        for (i, line) in rendered_lines
            .iter()
            .skip(state.scroll)
            .take(visible_lines)
            .enumerate()
        {
            let y = inner.y + i as u16;

            // Draw prefix
            buf.set_string(inner.x, y, &line.prefix, line.style);

            // Draw content
            let content_x = inner.x + line.prefix.len() as u16;
            buf.set_string(content_x, y, &line.content, line.style);
        }

        // Draw scroll indicator
        if rendered_lines.len() > visible_lines {
            let indicator = format!(
                " {}/{} ",
                state.scroll + 1,
                rendered_lines.len().saturating_sub(visible_lines) + 1
            );
            let x = area.x + area.width - indicator.len() as u16 - 1;
            buf.set_string(x, area.y, &indicator, Style::default().fg(Color::DarkGray));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heading() {
        let elements = parse_markdown("# Hello World");
        assert_eq!(elements.len(), 1);

        if let MarkdownElement::Heading(level, text) = &elements[0] {
            assert_eq!(*level, 1);
            assert_eq!(text, "Hello World");
        } else {
            panic!("Expected heading");
        }
    }

    #[test]
    fn test_parse_list() {
        let elements = parse_markdown("- Item 1\n- Item 2");
        assert_eq!(elements.len(), 2);
    }

    #[test]
    fn test_parse_code_block() {
        let md = "```rust\nfn main() {}\n```";
        let elements = parse_markdown(md);

        assert_eq!(elements.len(), 1);
        if let MarkdownElement::CodeBlock(lang, code) = &elements[0] {
            assert_eq!(lang, "rust");
            assert!(code.contains("fn main"));
        }
    }

    #[test]
    fn test_word_wrap() {
        let wrapped = MarkdownWidget::word_wrap("hello world foo bar", 10);
        // "hello", "world foo", "bar" = 3 lines
        assert_eq!(wrapped.len(), 3);
    }
}
