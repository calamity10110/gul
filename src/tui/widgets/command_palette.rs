// GUL TUI Command Palette Widget
// Quick command search and execution

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Widget},
};

use crate::tui::theme::GulTheme;

/// Command
#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub shortcut: Option<String>,
    pub category: String,
}

/// Command palette state
#[derive(Default)]
pub struct CommandPaletteState {
    pub query: String,
    pub selected: usize,
    pub visible: bool,
}

/// Command palette widget
pub struct CommandPaletteWidget<'a> {
    /// Commands
    commands: &'a [Command],
    /// State
    state: &'a CommandPaletteState,
    /// Theme
    theme: &'a GulTheme,
}

impl<'a> CommandPaletteWidget<'a> {
    pub fn new(
        commands: &'a [Command],
        state: &'a CommandPaletteState,
        theme: &'a GulTheme,
    ) -> Self {
        CommandPaletteWidget {
            commands,
            state,
            theme,
        }
    }

    fn filter_commands(&self) -> Vec<&Command> {
        if self.state.query.is_empty() {
            return self.commands.iter().collect();
        }

        let query_lower = self.state.query.to_lowercase();
        self.commands
            .iter()
            .filter(|cmd| {
                cmd.name.to_lowercase().contains(&query_lower)
                    || cmd.description.to_lowercase().contains(&query_lower)
                    || cmd.category.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
}

impl<'a> Widget for CommandPaletteWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.state.visible {
            return;
        }

        // Calculate palette dimensions
        let width = 60.min(area.width - 4);
        let height = 15.min(area.height - 4);
        let x = (area.width - width) / 2 + area.x;
        let y = (area.height - height) / 3 + area.y;

        let palette_area = Rect::new(x, y, width, height);

        // Clear background
        Clear.render(palette_area, buf);

        // Draw border
        let block = Block::default()
            .title(" Command Palette ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border_focused));

        let inner = block.inner(palette_area);
        block.render(palette_area, buf);

        // Search input
        let search_text = format!("> {}", self.state.query);
        buf.set_string(inner.x, inner.y, &search_text, self.theme.title);

        // Separator
        let separator = "─".repeat(inner.width as usize);
        buf.set_string(
            inner.x,
            inner.y + 1,
            &separator,
            Style::default().fg(self.theme.border),
        );

        // Commands list
        let filtered = self.filter_commands();
        let visible = (inner.height - 2) as usize;

        for (i, cmd) in filtered.iter().take(visible).enumerate() {
            let y = inner.y + 2 + i as u16;
            let is_selected = i == self.state.selected;

            let style = if is_selected {
                self.theme.menu_selected
            } else {
                self.theme.menu
            };

            // Command name
            let name = format!("  {}", cmd.name);
            buf.set_string(inner.x, y, &name, style.add_modifier(Modifier::BOLD));

            // Shortcut (if any)
            if let Some(ref shortcut) = cmd.shortcut {
                let shortcut_x = inner.x + inner.width - shortcut.len() as u16 - 2;
                buf.set_string(shortcut_x, y, shortcut, style);
            }
        }
    }
}

/// Default commands for GUL IDE
pub fn default_commands() -> Vec<Command> {
    vec![
        Command {
            name: "Open File".to_string(),
            description: "Open a file for editing".to_string(),
            shortcut: Some("Ctrl+O".to_string()),
            category: "File".to_string(),
        },
        Command {
            name: "Save File".to_string(),
            description: "Save the current file".to_string(),
            shortcut: Some("Ctrl+S".to_string()),
            category: "File".to_string(),
        },
        Command {
            name: "Build Project".to_string(),
            description: "Build the current GUL project".to_string(),
            shortcut: Some("Ctrl+B".to_string()),
            category: "Build".to_string(),
        },
        Command {
            name: "Run Project".to_string(),
            description: "Run the current GUL project".to_string(),
            shortcut: Some("Ctrl+R".to_string()),
            category: "Build".to_string(),
        },
        Command {
            name: "Format Code".to_string(),
            description: "Format the current file".to_string(),
            shortcut: Some("Ctrl+Shift+F".to_string()),
            category: "Edit".to_string(),
        },
        Command {
            name: "Toggle Terminal".to_string(),
            description: "Show/hide the integrated terminal".to_string(),
            shortcut: Some("Ctrl+`".to_string()),
            category: "View".to_string(),
        },
        Command {
            name: "Toggle File Browser".to_string(),
            description: "Show/hide the file browser".to_string(),
            shortcut: Some("Ctrl+E".to_string()),
            category: "View".to_string(),
        },
        Command {
            name: "New File".to_string(),
            description: "Create a new file".to_string(),
            shortcut: Some("Ctrl+N".to_string()),
            category: "File".to_string(),
        },
        Command {
            name: "Close Tab".to_string(),
            description: "Close the current tab".to_string(),
            shortcut: Some("Ctrl+W".to_string()),
            category: "File".to_string(),
        },
        Command {
            name: "Quit".to_string(),
            description: "Exit the IDE".to_string(),
            shortcut: Some("Ctrl+Q".to_string()),
            category: "Application".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_commands() {
        let commands = default_commands();
        assert!(!commands.is_empty());
        assert!(commands.iter().any(|c| c.name == "Build Project"));
    }

    #[test]
    fn test_command_filtering() {
        let theme = GulTheme::default();
        let commands = default_commands();
        let state = CommandPaletteState {
            query: "build".to_string(),
            selected: 0,
            visible: true,
        };

        let widget = CommandPaletteWidget::new(&commands, &state, &theme);
        let filtered = widget.filter_commands();

        assert!(filtered.iter().any(|c| c.name == "Build Project"));
    }
}
