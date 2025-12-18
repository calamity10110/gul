// GUL TUI File Tree Widget
// Project file browser with v2.1 file type support

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, StatefulWidget, Widget},
};
use std::path::PathBuf;

use crate::tui::theme::GulTheme;

/// File tree state
#[derive(Default, Clone)]
pub struct FileTreeState {
    /// Selected index
    pub selected: usize,
    /// Scroll offset
    pub scroll: usize,
    /// Expanded directories
    pub expanded: Vec<PathBuf>,
}

impl FileTreeState {
    pub fn is_expanded(&self, path: &PathBuf) -> bool {
        self.expanded.contains(path)
    }

    pub fn toggle_expanded(&mut self, path: PathBuf) {
        if let Some(pos) = self.expanded.iter().position(|p| p == &path) {
            self.expanded.remove(pos);
        } else {
            self.expanded.push(path);
        }
    }
}

/// File tree entry
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub is_dir: bool,
    pub depth: usize,
}

/// File tree widget
pub struct FileTreeWidget<'a> {
    /// Entries
    entries: &'a [FileEntry],
    /// Theme
    theme: &'a GulTheme,
    /// Title
    title: &'a str,
}

impl<'a> FileTreeWidget<'a> {
    pub fn new(entries: &'a [FileEntry], theme: &'a GulTheme) -> Self {
        FileTreeWidget {
            entries,
            theme,
            title: "Files",
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    /// Get icon for file type
    fn get_icon(&self, entry: &FileEntry, is_expanded: bool) -> &'static str {
        if entry.is_dir {
            return if is_expanded { "📂" } else { "📁" };
        }

        match entry.path.extension().and_then(|s| s.to_str()) {
            Some("mn") => "📄",              // Main file
            Some("def") => "📋",             // Definition
            Some("fnc") => "⚙️",             // Function
            Some("cs") => "🔗",              // Cross-script
            Some("sct") => "🔐",             // Secret credential
            Some("gul") => "📜",             // Legacy
            Some("md") => "📝",              // Markdown
            Some("toml") => "⚙️",            // Config
            Some("json") => "📦",            // JSON
            Some("rs") => "🦀",              // Rust
            Some("py") => "🐍",              // Python
            Some("js") | Some("ts") => "🟨", // JavaScript/TypeScript
            _ => "📄",
        }
    }

    /// Get style for file type
    fn get_style(&self, entry: &FileEntry) -> Style {
        if entry.is_dir {
            return Style::default().add_modifier(Modifier::BOLD);
        }

        match entry.path.extension().and_then(|s| s.to_str()) {
            Some("mn") => self.theme.file_type_main,
            Some("def") => self.theme.file_type_def,
            Some("fnc") => self.theme.file_type_fnc,
            Some("cs") => self.theme.file_type_cs,
            Some("sct") => self.theme.file_type_sct,
            _ => Style::default(),
        }
    }
}

impl<'a> StatefulWidget for FileTreeWidget<'a> {
    type State = FileTreeState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let block = Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border));

        let inner = block.inner(area);
        block.render(area, buf);

        let mut visible_entries = Vec::new();
        let mut skip_depth: Option<usize> = None;

        for entry in self.entries.iter() {
            if let Some(depth) = skip_depth {
                if entry.depth > depth {
                    continue;
                }
                skip_depth = None;
            }

            visible_entries.push(entry);

            if entry.is_dir && !state.is_expanded(&entry.path) {
                skip_depth = Some(entry.depth);
            }
        }

        let selected_path = self.entries.get(state.selected).map(|e| &e.path);
        let start = state.scroll;
        let visible_count = inner.height as usize;

        for (i, entry) in visible_entries
            .iter()
            .skip(start)
            .take(visible_count)
            .enumerate()
        {
            let y = inner.y + i as u16;
            let is_expanded = state.is_expanded(&entry.path);

            // Indentation
            let indent = "  ".repeat(entry.depth);
            let icon = self.get_icon(entry, is_expanded);
            let name = entry
                .path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("");

            let style = if Some(&entry.path) == selected_path {
                self.theme.menu_selected
            } else {
                self.get_style(entry)
            };

            let line = format!("{}{} {}", indent, icon, name);
            buf.set_string(inner.x, y, &line, style);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_tree_icons() {
        let theme = GulTheme::default();
        let entries = vec![];
        let widget = FileTreeWidget::new(&entries, &theme);

        let mn_entry = FileEntry {
            path: PathBuf::from("main.mn"),
            is_dir: false,
            depth: 0,
        };
        assert_eq!(widget.get_icon(&mn_entry, false), "📄");

        let sct_entry = FileEntry {
            path: PathBuf::from("secrets.sct"),
            is_dir: false,
            depth: 0,
        };
        assert_eq!(widget.get_icon(&sct_entry, false), "🔐");

        let fnc_entry = FileEntry {
            path: PathBuf::from("my_function.fnc"),
            is_dir: false,
            depth: 0,
        };
        assert_eq!(widget.get_icon(&fnc_entry, false), "⚙️");

        let cs_entry = FileEntry {
            path: PathBuf::from("cross_script.cs"),
            is_dir: false,
            depth: 0,
        };
        assert_eq!(widget.get_icon(&cs_entry, false), "🔗");
    }
}
