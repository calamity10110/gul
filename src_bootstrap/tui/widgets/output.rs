// GUL TUI Output Widget
// Compiler output and terminal output panel

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Widget},
};

use crate::tui::theme::GulTheme;

/// Output line type
#[derive(Debug, Clone)]
pub enum OutputType {
    Info,
    Success,
    Warning,
    Error,
    Command,
}

/// Output line
#[derive(Debug, Clone)]
pub struct OutputLine {
    pub text: String,
    pub output_type: OutputType,
}

/// Output widget
pub struct OutputWidget<'a> {
    /// Output lines
    lines: &'a [OutputLine],
    /// Title
    title: &'a str,
    /// Scroll offset
    scroll: usize,
    /// Theme
    theme: &'a GulTheme,
}

impl<'a> OutputWidget<'a> {
    pub fn new(lines: &'a [OutputLine], theme: &'a GulTheme) -> Self {
        OutputWidget {
            lines,
            title: "Output",
            scroll: 0,
            theme,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn scroll(mut self, scroll: usize) -> Self {
        self.scroll = scroll;
        self
    }

    fn get_style(&self, output_type: &OutputType) -> Style {
        match output_type {
            OutputType::Info => Style::default(),
            OutputType::Success => Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
            OutputType::Warning => Style::default().fg(Color::Yellow),
            OutputType::Error => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            OutputType::Command => Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::ITALIC),
        }
    }

    fn get_prefix(&self, output_type: &OutputType) -> &'static str {
        match output_type {
            OutputType::Info => "  ",
            OutputType::Success => "✓ ",
            OutputType::Warning => "⚠ ",
            OutputType::Error => "✗ ",
            OutputType::Command => "$ ",
        }
    }
}

impl<'a> Widget for OutputWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border));

        let inner = block.inner(area);
        block.render(area, buf);

        let visible = inner.height as usize;
        let start = self.scroll;
        let end = (start + visible).min(self.lines.len());

        for (i, idx) in (start..end).enumerate() {
            let y = inner.y + i as u16;
            let line = &self.lines[idx];

            let prefix = self.get_prefix(&line.output_type);
            let style = self.get_style(&line.output_type);

            let text = format!("{}{}", prefix, line.text);
            buf.set_string(inner.x, y, &text, style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_widget() {
        let theme = GulTheme::default();
        let lines = vec![
            OutputLine {
                text: "Building project...".to_string(),
                output_type: OutputType::Info,
            },
            OutputLine {
                text: "Build successful!".to_string(),
                output_type: OutputType::Success,
            },
        ];

        let widget = OutputWidget::new(&lines, &theme);
        assert_eq!(widget.title, "Output");
    }
}
