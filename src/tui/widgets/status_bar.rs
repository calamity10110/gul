// GUL TUI Status Bar Widget
// Bottom status bar with file info, cursor position, git branch

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::tui::theme::GulTheme;

/// Status bar widget
pub struct StatusBarWidget<'a> {
    /// Current file name
    file_name: Option<&'a str>,
    /// File type
    file_type: Option<&'a str>,
    /// Cursor position (line, column)
    cursor: (usize, usize),
    /// Encoding
    encoding: &'a str,
    /// Git branch
    git_branch: Option<&'a str>,
    /// Is file modified
    modified: bool,
    /// Theme
    theme: &'a GulTheme,
}

impl<'a> StatusBarWidget<'a> {
    pub fn new(theme: &'a GulTheme) -> Self {
        StatusBarWidget {
            file_name: None,
            file_type: None,
            cursor: (1, 1),
            encoding: "UTF-8",
            git_branch: None,
            modified: false,
            theme,
        }
    }

    pub fn file(mut self, name: &'a str, file_type: &'a str) -> Self {
        self.file_name = Some(name);
        self.file_type = Some(file_type);
        self
    }

    pub fn cursor(mut self, line: usize, col: usize) -> Self {
        self.cursor = (line, col);
        self
    }

    pub fn git_branch(mut self, branch: &'a str) -> Self {
        self.git_branch = Some(branch);
        self
    }

    pub fn modified(mut self, modified: bool) -> Self {
        self.modified = modified;
        self
    }
}

impl<'a> Widget for StatusBarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Background
        for x in area.x..area.x + area.width {
            if let Some(cell) = buf.cell_mut((x, area.y)) {
                cell.set_style(self.theme.status_bar);
            }
        }

        // Left side: file info
        let mut left = String::new();

        if let Some(name) = self.file_name {
            left.push_str(name);
            if self.modified {
                left.push_str(" ●");
            }
        } else {
            left.push_str("No file");
        }

        buf.set_string(area.x + 1, area.y, &left, self.theme.status_bar);

        // Right side: cursor, encoding, branch
        let mut right_parts = Vec::new();

        // Cursor position
        right_parts.push(format!("Ln {}, Col {}", self.cursor.0, self.cursor.1));

        // File type
        if let Some(ft) = self.file_type {
            right_parts.push(ft.to_string());
        }

        // Encoding
        right_parts.push(self.encoding.to_string());

        // Git branch
        if let Some(branch) = self.git_branch {
            right_parts.push(format!("🔀 {}", branch));
        }

        let right = right_parts.join(" │ ");
        let right_x = area.x + area.width - right.len() as u16 - 1;
        buf.set_string(right_x, area.y, &right, self.theme.status_bar);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_creation() {
        let theme = GulTheme::default();
        let status = StatusBarWidget::new(&theme)
            .file("main.mn", "Main")
            .cursor(10, 5)
            .git_branch("main");

        assert_eq!(status.cursor, (10, 5));
    }
}
