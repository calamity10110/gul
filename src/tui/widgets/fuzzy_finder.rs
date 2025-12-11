// GUL TUI Fuzzy Finder Widget
// Television-inspired fuzzy search
// Inspired by television and fzf

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, StatefulWidget, Widget},
};

use crate::tui::theme::GulTheme;

/// Fuzzy match result
#[derive(Debug, Clone)]
pub struct FuzzyMatch {
    /// Original text
    pub text: String,
    /// Score (higher = better match)
    pub score: i64,
    /// Matched character indices
    pub matches: Vec<usize>,
    /// Optional preview content
    pub preview: Option<String>,
    /// Optional icon
    pub icon: Option<String>,
    /// Optional metadata
    pub metadata: Option<String>,
}

impl FuzzyMatch {
    pub fn new(text: impl Into<String>) -> Self {
        FuzzyMatch {
            text: text.into(),
            score: 0,
            matches: Vec::new(),
            preview: None,
            icon: None,
            metadata: None,
        }
    }

    pub fn with_score(mut self, score: i64) -> Self {
        self.score = score;
        self
    }

    pub fn with_matches(mut self, matches: Vec<usize>) -> Self {
        self.matches = matches;
        self
    }

    pub fn with_preview(mut self, preview: impl Into<String>) -> Self {
        self.preview = Some(preview.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }
}

/// Simple fuzzy matching algorithm (nucleo-like)
pub fn fuzzy_match(pattern: &str, text: &str) -> Option<FuzzyMatch> {
    if pattern.is_empty() {
        return Some(FuzzyMatch::new(text));
    }

    let pattern_chars: Vec<char> = pattern.to_lowercase().chars().collect();
    let text_chars: Vec<char> = text.to_lowercase().chars().collect();

    let mut pattern_idx = 0;
    let mut matches = Vec::new();
    let mut score = 0i64;
    let mut consecutive = 0;

    for (i, &tc) in text_chars.iter().enumerate() {
        if pattern_idx < pattern_chars.len() && tc == pattern_chars[pattern_idx] {
            matches.push(i);
            pattern_idx += 1;

            // Bonus for consecutive matches
            if !matches.is_empty() && matches.len() > 1 {
                let last = matches[matches.len() - 2];
                if i == last + 1 {
                    consecutive += 1;
                    score += consecutive * 10;
                } else {
                    consecutive = 0;
                }
            }

            // Bonus for start of word
            if i == 0 || !text_chars[i - 1].is_alphanumeric() {
                score += 15;
            }

            // Base match score
            score += 10;
        }
    }

    if pattern_idx == pattern_chars.len() {
        // Penalty for length difference
        score -= (text.len() as i64 - pattern.len() as i64).abs();

        Some(
            FuzzyMatch::new(text)
                .with_score(score)
                .with_matches(matches),
        )
    } else {
        None
    }
}

/// Fuzzy finder state
#[derive(Default, Clone)]
pub struct FuzzyFinderState {
    /// Search query
    pub query: String,
    /// Selected index
    pub selected: usize,
    /// Scroll offset
    pub scroll: usize,
    /// Is visible
    pub visible: bool,
    /// Cursor position in query
    pub cursor: usize,
}

impl FuzzyFinderState {
    pub fn new() -> Self {
        FuzzyFinderState::default()
    }

    pub fn open(&mut self) {
        self.visible = true;
        self.query.clear();
        self.selected = 0;
        self.scroll = 0;
        self.cursor = 0;
    }

    pub fn close(&mut self) {
        self.visible = false;
    }

    pub fn toggle(&mut self) {
        if self.visible {
            self.close();
        } else {
            self.open();
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.query.insert(self.cursor, c);
        self.cursor += 1;
        self.selected = 0;
        self.scroll = 0;
    }

    pub fn delete_char(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.query.remove(self.cursor);
            self.selected = 0;
        }
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.cursor = 0;
        self.selected = 0;
    }

    pub fn move_cursor_left(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    pub fn move_cursor_right(&mut self) {
        self.cursor = (self.cursor + 1).min(self.query.len());
    }

    pub fn select_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
        if self.selected < self.scroll {
            self.scroll = self.selected;
        }
    }

    pub fn select_down(&mut self, max: usize) {
        if max > 0 {
            self.selected = (self.selected + 1).min(max - 1);
        }
    }

    pub fn page_up(&mut self, page_size: usize) {
        self.selected = self.selected.saturating_sub(page_size);
    }

    pub fn page_down(&mut self, max: usize, page_size: usize) {
        if max > 0 {
            self.selected = (self.selected + page_size).min(max - 1);
        }
    }
}

/// Fuzzy finder widget
pub struct FuzzyFinderWidget<'a> {
    /// Items to search
    items: &'a [FuzzyMatch],
    /// Theme
    theme: &'a GulTheme,
    /// Title/prompt
    prompt: &'a str,
    /// Show preview panel
    show_preview: bool,
    /// Preview panel width percentage
    preview_width: u16,
}

impl<'a> FuzzyFinderWidget<'a> {
    pub fn new(items: &'a [FuzzyMatch], theme: &'a GulTheme) -> Self {
        FuzzyFinderWidget {
            items,
            theme,
            prompt: "> ",
            show_preview: true,
            preview_width: 50,
        }
    }

    pub fn prompt(mut self, prompt: &'a str) -> Self {
        self.prompt = prompt;
        self
    }

    pub fn show_preview(mut self, show: bool) -> Self {
        self.show_preview = show;
        self
    }

    pub fn preview_width(mut self, width: u16) -> Self {
        self.preview_width = width.min(80);
        self
    }
}

impl<'a> StatefulWidget for FuzzyFinderWidget<'a> {
    type State = FuzzyFinderState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if !state.visible {
            return;
        }

        // Clear background
        Clear.render(area, buf);

        // Calculate layout
        let results_width = if self.show_preview && area.width > 60 {
            (area.width * (100 - self.preview_width)) / 100
        } else {
            area.width
        };

        let results_area = Rect::new(area.x, area.y, results_width, area.height);

        // Draw main panel
        let block = Block::default()
            .title(" 🔍 Fuzzy Finder ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border_focused));

        let inner = block.inner(results_area);
        block.render(results_area, buf);

        if inner.height < 3 {
            return;
        }

        // Draw query input
        let query_y = inner.y;
        let prompt_style = Style::default().fg(Color::Cyan);
        buf.set_string(inner.x, query_y, self.prompt, prompt_style);

        let query_x = inner.x + self.prompt.len() as u16;
        buf.set_string(query_x, query_y, &state.query, Style::default());

        // Draw cursor
        let cursor_x = query_x + state.cursor as u16;
        if cursor_x < inner.x + inner.width {
            buf.set_string(cursor_x, query_y, "▌", Style::default().fg(Color::Yellow));
        }

        // Draw separator
        let sep_y = inner.y + 1;
        let sep = "─".repeat(inner.width as usize);
        buf.set_string(inner.x, sep_y, &sep, Style::default().fg(self.theme.border));

        // Draw results
        let results_start = inner.y + 2;
        let results_height = inner.height.saturating_sub(2) as usize;

        // Update scroll
        if state.selected >= state.scroll + results_height {
            state.scroll = state.selected - results_height + 1;
        } else if state.selected < state.scroll {
            state.scroll = state.selected;
        }

        // Draw result count
        let count_text = format!("{}/{}", self.items.len(), self.items.len());
        let count_x = inner.x + inner.width - count_text.len() as u16;
        buf.set_string(
            count_x,
            query_y,
            &count_text,
            Style::default().fg(Color::DarkGray),
        );

        for (i, item) in self
            .items
            .iter()
            .skip(state.scroll)
            .take(results_height)
            .enumerate()
        {
            let y = results_start + i as u16;
            let is_selected = state.scroll + i == state.selected;

            let base_style = if is_selected {
                self.theme.menu_selected
            } else {
                Style::default()
            };

            let mut x = inner.x;

            // Draw icon
            if let Some(ref icon) = item.icon {
                buf.set_string(x, y, icon, base_style);
                x += 2;
            }

            // Draw text with highlighted matches
            for (char_idx, c) in item.text.chars().enumerate() {
                if x >= inner.x + inner.width {
                    break;
                }

                let style = if item.matches.contains(&char_idx) {
                    base_style.fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    base_style
                };

                buf.set_string(x, y, &c.to_string(), style);
                x += 1;
            }

            // Draw metadata
            if let Some(ref metadata) = item.metadata {
                let meta_style = base_style.fg(Color::DarkGray);
                let meta_x = inner.x + inner.width - metadata.len() as u16 - 1;
                if meta_x > x + 2 {
                    buf.set_string(meta_x, y, metadata, meta_style);
                }
            }
        }

        // Draw preview panel
        if self.show_preview && results_width < area.width {
            let preview_area = Rect::new(
                area.x + results_width,
                area.y,
                area.width - results_width,
                area.height,
            );

            let preview_block = Block::default()
                .title(" Preview ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.border));

            let preview_inner = preview_block.inner(preview_area);
            preview_block.render(preview_area, buf);

            // Show preview of selected item
            if let Some(item) = self.items.get(state.selected) {
                if let Some(ref preview) = item.preview {
                    for (i, line) in preview.lines().enumerate() {
                        if i as u16 >= preview_inner.height {
                            break;
                        }
                        let truncated: String =
                            line.chars().take(preview_inner.width as usize).collect();
                        buf.set_string(
                            preview_inner.x,
                            preview_inner.y + i as u16,
                            &truncated,
                            Style::default(),
                        );
                    }
                }
            }
        }
    }
}

/// File finder with common patterns
pub struct FileFinder;

impl FileFinder {
    /// Create file matches from paths
    pub fn from_paths(paths: &[String], pattern: &str) -> Vec<FuzzyMatch> {
        paths
            .iter()
            .filter_map(|p| fuzzy_match(pattern, p))
            .map(|mut m| {
                // Add file icon based on extension
                let icon = Self::get_file_icon(&m.text);
                m.icon = Some(icon.to_string());
                m
            })
            .collect()
    }

    fn get_file_icon(path: &str) -> &'static str {
        if let Some(ext) = std::path::Path::new(path).extension() {
            match ext.to_string_lossy().as_ref() {
                "mn" => "📄",
                "def" => "📋",
                "fnc" => "⚙️",
                "cs" => "🔗",
                "sct" => "🔐",
                "rs" => "🦀",
                "py" => "🐍",
                "js" | "ts" => "🟨",
                "md" => "📝",
                "json" => "📦",
                _ => "📄",
            }
        } else {
            "📁"
        }
    }
}

/// Command finder for command palette integration
pub struct CommandFinder;

impl CommandFinder {
    pub fn from_commands(commands: &[(String, String, String)], pattern: &str) -> Vec<FuzzyMatch> {
        commands
            .iter()
            .filter_map(|(name, shortcut, description)| {
                let search_text = format!("{} {}", name, description);
                fuzzy_match(pattern, &search_text).map(|mut m| {
                    m.text = name.clone();
                    m.metadata = Some(shortcut.clone());
                    m.preview = Some(description.clone());
                    m
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match() {
        let result = fuzzy_match("mn", "main.mn");
        assert!(result.is_some());

        let m = result.unwrap();
        assert!(m.score > 0);
        assert!(!m.matches.is_empty());
    }

    #[test]
    fn test_fuzzy_match_no_match() {
        let result = fuzzy_match("xyz", "abc");
        assert!(result.is_none());
    }

    #[test]
    fn test_fuzzy_state() {
        let mut state = FuzzyFinderState::new();

        state.open();
        assert!(state.visible);

        state.insert_char('a');
        state.insert_char('b');
        assert_eq!(state.query, "ab");
        assert_eq!(state.cursor, 2);

        state.delete_char();
        assert_eq!(state.query, "a");
    }

    #[test]
    fn test_file_finder() {
        let paths = vec![
            "src/main.mn".to_string(),
            "src/lib.rs".to_string(),
            "test/test.mn".to_string(),
        ];

        let results = FileFinder::from_paths(&paths, "mn");
        assert_eq!(results.len(), 2);
    }
}
