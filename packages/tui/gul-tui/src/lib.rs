use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

pub mod widgets {
    use super::*;

    pub struct Button<'a> {
        label: &'a str,
        selected: bool,
    }

    impl<'a> Button<'a> {
        pub fn new(label: &'a str) -> Self {
            Self {
                label,
                selected: false,
            }
        }

        pub fn selected(mut self, selected: bool) -> Self {
            self.selected = selected;
            self
        }

        pub fn render(&self) -> Paragraph<'a> {
            let style = if self.selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            Paragraph::new(self.label)
                .block(Block::default().borders(Borders::ALL))
                .style(style)
        }
    }

    pub struct InputField<'a> {
        value: &'a str,
        placeholder: &'a str,
    }

    impl<'a> InputField<'a> {
        pub fn new(value: &'a str) -> Self {
            Self {
                value,
                placeholder: "Enter text...",
            }
        }

        pub fn placeholder(mut self, p: &'a str) -> Self {
            self.placeholder = p;
            self
        }

        pub fn render(&self) -> Paragraph<'a> {
            let text = if self.value.is_empty() {
                self.placeholder
            } else {
                self.value
            };
            Paragraph::new(text).block(Block::default().borders(Borders::BOTTOM))
        }
    }
}
