// GUL TUI Tabs Widget
// Tab bar for multiple open files

use ratatui::{buffer::Buffer, layout::Rect, style::Modifier, widgets::Widget};

use crate::tui::theme::GulTheme;

/// Tab info
#[derive(Debug, Clone)]
pub struct Tab {
    pub name: String,
    pub file_type: String,
    pub modified: bool,
    pub path: String,
}

/// Tabs widget
pub struct TabsWidget<'a> {
    /// Tabs
    tabs: &'a [Tab],
    /// Active tab index
    active: usize,
    /// Theme
    theme: &'a GulTheme,
}

impl<'a> TabsWidget<'a> {
    pub fn new(tabs: &'a [Tab], active: usize, theme: &'a GulTheme) -> Self {
        TabsWidget {
            tabs,
            active,
            theme,
        }
    }

    fn get_icon(&self, file_type: &str) -> &'static str {
        match file_type {
            "Main" => "📄",
            "Definition" => "📋",
            "Function" => "⚙️",
            "Cross-Script" => "🔗",
            "Secret Credential" => "🔐",
            _ => "📄",
        }
    }
}

impl<'a> Widget for TabsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut x = area.x;

        for (i, tab) in self.tabs.iter().enumerate() {
            let is_active = i == self.active;
            let icon = self.get_icon(&tab.file_type);
            let modified = if tab.modified { " ●" } else { "" };
            let text = format!(" {} {}{} ", icon, tab.name, modified);

            let style = if is_active {
                self.theme.menu_selected.add_modifier(Modifier::BOLD)
            } else {
                self.theme.menu
            };

            // Calculate tab width
            let tab_width = text.chars().count() as u16;

            if x + tab_width > area.x + area.width {
                break;
            }

            // Render tab
            buf.set_string(x, area.y, &text, style);

            // Tab separator
            if i < self.tabs.len() - 1 {
                buf.set_string(x + tab_width, area.y, "│", self.theme.menu);
            }

            x += tab_width + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tabs_widget() {
        let theme = GulTheme::default();
        let tabs = vec![
            Tab {
                name: "main.mn".to_string(),
                file_type: "Main".to_string(),
                modified: false,
                path: "/path/main.mn".to_string(),
            },
            Tab {
                name: "lib.def".to_string(),
                file_type: "Definition".to_string(),
                modified: true,
                path: "/path/lib.def".to_string(),
            },
        ];

        let widget = TabsWidget::new(&tabs, 0, &theme);
        assert_eq!(widget.active, 0);
    }
}
