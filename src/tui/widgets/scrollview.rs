// GUL TUI ScrollView Widget
// Scrollable container for content
// Inspired by tui-scrollview

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{
        Block, Borders, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget,
    },
};

use crate::tui::theme::GulTheme;

/// Scroll direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScrollDirection {
    Vertical,
    Horizontal,
    Both,
}

/// ScrollView state
#[derive(Default, Clone)]
pub struct ScrollViewState {
    /// Vertical scroll offset
    pub offset_y: usize,
    /// Horizontal scroll offset
    pub offset_x: usize,
    /// Content height (for scroll calculation)
    pub content_height: usize,
    /// Content width (for scroll calculation)
    pub content_width: usize,
    /// Viewport height
    pub viewport_height: usize,
    /// Viewport width
    pub viewport_width: usize,
}

impl ScrollViewState {
    pub fn new() -> Self {
        ScrollViewState::default()
    }

    /// Scroll up by n lines
    pub fn scroll_up(&mut self, n: usize) {
        self.offset_y = self.offset_y.saturating_sub(n);
    }

    /// Scroll down by n lines
    pub fn scroll_down(&mut self, n: usize) {
        let max_offset = self.content_height.saturating_sub(self.viewport_height);
        self.offset_y = (self.offset_y + n).min(max_offset);
    }

    /// Scroll left by n columns
    pub fn scroll_left(&mut self, n: usize) {
        self.offset_x = self.offset_x.saturating_sub(n);
    }

    /// Scroll right by n columns
    pub fn scroll_right(&mut self, n: usize) {
        let max_offset = self.content_width.saturating_sub(self.viewport_width);
        self.offset_x = (self.offset_x + n).min(max_offset);
    }

    /// Page up
    pub fn page_up(&mut self) {
        self.scroll_up(self.viewport_height.saturating_sub(2));
    }

    /// Page down
    pub fn page_down(&mut self) {
        self.scroll_down(self.viewport_height.saturating_sub(2));
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.offset_y = 0;
        self.offset_x = 0;
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.offset_y = self.content_height.saturating_sub(self.viewport_height);
    }

    /// Ensure cursor is visible
    pub fn ensure_visible(&mut self, line: usize) {
        if line < self.offset_y {
            self.offset_y = line;
        } else if line >= self.offset_y + self.viewport_height {
            self.offset_y = line.saturating_sub(self.viewport_height) + 1;
        }
    }

    /// Get vertical scroll percentage (0.0 - 1.0)
    pub fn scroll_percentage_y(&self) -> f64 {
        if self.content_height <= self.viewport_height {
            return 0.0;
        }
        let max_offset = self.content_height - self.viewport_height;
        self.offset_y as f64 / max_offset as f64
    }

    /// Get horizontal scroll percentage
    pub fn scroll_percentage_x(&self) -> f64 {
        if self.content_width <= self.viewport_width {
            return 0.0;
        }
        let max_offset = self.content_width - self.viewport_width;
        self.offset_x as f64 / max_offset as f64
    }

    /// Check if can scroll up
    pub fn can_scroll_up(&self) -> bool {
        self.offset_y > 0
    }

    /// Check if can scroll down
    pub fn can_scroll_down(&self) -> bool {
        self.offset_y + self.viewport_height < self.content_height
    }
}

/// ScrollView widget
pub struct ScrollViewWidget<'a> {
    /// Content lines
    content: &'a [String],
    /// Theme
    theme: &'a GulTheme,
    /// Title
    title: Option<&'a str>,
    /// Show vertical scrollbar
    show_scrollbar_v: bool,
    /// Show horizontal scrollbar
    show_scrollbar_h: bool,
    /// Scroll direction
    direction: ScrollDirection,
}

impl<'a> ScrollViewWidget<'a> {
    pub fn new(content: &'a [String], theme: &'a GulTheme) -> Self {
        ScrollViewWidget {
            content,
            theme,
            title: None,
            show_scrollbar_v: true,
            show_scrollbar_h: false,
            direction: ScrollDirection::Vertical,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn show_scrollbar_v(mut self, show: bool) -> Self {
        self.show_scrollbar_v = show;
        self
    }

    pub fn show_scrollbar_h(mut self, show: bool) -> Self {
        self.show_scrollbar_h = show;
        self
    }

    pub fn direction(mut self, direction: ScrollDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<'a> StatefulWidget for ScrollViewWidget<'a> {
    type State = ScrollViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // Draw border
        let block = if let Some(title) = self.title {
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.border))
        } else {
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.border))
        };

        let inner = block.inner(area);
        block.render(area, buf);

        // Update state with viewport size
        state.content_height = self.content.len();
        state.viewport_height = inner.height as usize;
        state.viewport_width = inner.width as usize;

        // Calculate max content width
        state.content_width = self.content.iter().map(|s| s.len()).max().unwrap_or(0);

        // Render visible content
        let start_line = state.offset_y;
        let end_line = (start_line + state.viewport_height).min(self.content.len());

        for (i, line_idx) in (start_line..end_line).enumerate() {
            let y = inner.y + i as u16;
            let line = &self.content[line_idx];

            // Apply horizontal scroll
            let visible_line = if state.offset_x < line.len() {
                let end = (state.offset_x + state.viewport_width).min(line.len());
                &line[state.offset_x..end]
            } else {
                ""
            };

            buf.set_string(inner.x, y, visible_line, Style::default());
        }

        // Render vertical scrollbar if needed
        if self.show_scrollbar_v && self.content.len() > state.viewport_height {
            let scrollbar_area = Rect::new(area.x + area.width - 1, inner.y, 1, inner.height);

            let mut scrollbar_state =
                ScrollbarState::new(self.content.len()).position(state.offset_y);

            let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("▲"))
                .end_symbol(Some("▼"));

            scrollbar.render(scrollbar_area, buf, &mut scrollbar_state);
        }
    }
}

/// Text content wrapper for easier usage
pub struct ScrollableText<'a> {
    text: &'a str,
    #[allow(dead_code)]
    theme: &'a GulTheme,
    title: Option<&'a str>,
}

impl<'a> ScrollableText<'a> {
    pub fn new(text: &'a str, theme: &'a GulTheme) -> Self {
        ScrollableText {
            text,
            theme,
            title: None,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Convert to lines for ScrollViewWidget
    pub fn to_lines(&self) -> Vec<String> {
        self.text.lines().map(String::from).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrollview_state() {
        let mut state = ScrollViewState::new();
        state.content_height = 100;
        state.viewport_height = 20;

        state.scroll_down(10);
        assert_eq!(state.offset_y, 10);

        state.scroll_up(5);
        assert_eq!(state.offset_y, 5);

        state.page_down();
        assert_eq!(state.offset_y, 23); // 5 + 18

        state.scroll_to_top();
        assert_eq!(state.offset_y, 0);
    }

    #[test]
    fn test_ensure_visible() {
        let mut state = ScrollViewState::new();
        state.content_height = 100;
        state.viewport_height = 20;
        state.offset_y = 0;

        state.ensure_visible(25);
        assert_eq!(state.offset_y, 6); // 25 - 20 + 1

        state.ensure_visible(0);
        assert_eq!(state.offset_y, 0);
    }

    #[test]
    fn test_scroll_percentage() {
        let mut state = ScrollViewState::new();
        state.content_height = 100;
        state.viewport_height = 20;
        state.offset_y = 40;

        let percentage = state.scroll_percentage_y();
        assert!((percentage - 0.5).abs() < 0.01);
    }
}
