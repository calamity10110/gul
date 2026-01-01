// GUL TUI Throbber Widget
// Loading spinners and progress indicators
// Inspired by throbber-widgets-tui

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};

use crate::tui::theme::GulTheme;

/// Throbber animation set
#[derive(Debug, Clone)]
pub struct ThrobberSet {
    pub frames: Vec<&'static str>,
    pub interval_ms: u64,
}

impl ThrobberSet {
    /// Braille spinner (smooth)
    pub fn braille() -> Self {
        ThrobberSet {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            interval_ms: 80,
        }
    }

    /// ASCII spinner (compatible)
    pub fn ascii() -> Self {
        ThrobberSet {
            frames: vec!["-", "\\", "|", "/"],
            interval_ms: 100,
        }
    }

    /// Clock emoji spinner
    pub fn clock() -> Self {
        ThrobberSet {
            frames: vec![
                "🕐", "🕑", "🕒", "🕓", "🕔", "🕕", "🕖", "🕗", "🕘", "🕙", "🕚", "🕛",
            ],
            interval_ms: 100,
        }
    }

    /// Line height animation
    pub fn line() -> Self {
        ThrobberSet {
            frames: vec![
                "▁", "▂", "▃", "▄", "▅", "▆", "▇", "█", "▇", "▆", "▅", "▄", "▃", "▂",
            ],
            interval_ms: 80,
        }
    }

    /// Block fill animation
    pub fn block() -> Self {
        ThrobberSet {
            frames: vec!["░", "▒", "▓", "█", "▓", "▒"],
            interval_ms: 120,
        }
    }

    /// Dots animation
    pub fn dots() -> Self {
        ThrobberSet {
            frames: vec!["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"],
            interval_ms: 80,
        }
    }

    /// Arrow spinner
    pub fn arrows() -> Self {
        ThrobberSet {
            frames: vec!["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
            interval_ms: 100,
        }
    }

    /// Circle quarters
    pub fn circle() -> Self {
        ThrobberSet {
            frames: vec!["◐", "◓", "◑", "◒"],
            interval_ms: 100,
        }
    }

    /// Growing bar
    pub fn growing() -> Self {
        ThrobberSet {
            frames: vec!["▏", "▎", "▍", "▌", "▋", "▊", "▉", "█"],
            interval_ms: 100,
        }
    }

    /// Bouncing ball
    pub fn bounce() -> Self {
        ThrobberSet {
            frames: vec!["⠁", "⠂", "⠄", "⠂"],
            interval_ms: 120,
        }
    }
}

/// Throbber state for animation
#[derive(Default, Clone)]
pub struct ThrobberState {
    /// Current frame index
    pub index: usize,
    /// Is animation active
    pub active: bool,
}

impl ThrobberState {
    pub fn new() -> Self {
        ThrobberState {
            index: 0,
            active: true,
        }
    }

    /// Calculate next frame
    pub fn calc_next(&mut self, frame_count: usize) {
        if self.active && frame_count > 0 {
            self.index = (self.index + 1) % frame_count;
        }
    }

    /// Get current frame from a set
    pub fn current_frame<'a>(&self, set: &'a ThrobberSet) -> &'a str {
        if set.frames.is_empty() {
            return " ";
        }
        set.frames[self.index % set.frames.len()]
    }
}

/// Throbber widget
pub struct ThrobberWidget<'a> {
    /// Animation set
    set: ThrobberSet,
    /// Label text
    label: Option<&'a str>,
    /// Throbber style
    throbber_style: Style,
    /// Label style
    label_style: Style,
    /// Theme
    #[allow(dead_code)]
    theme: &'a GulTheme,
}

impl<'a> ThrobberWidget<'a> {
    pub fn new(theme: &'a GulTheme) -> Self {
        ThrobberWidget {
            set: ThrobberSet::braille(),
            label: None,
            throbber_style: Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
            label_style: Style::default(),
            theme,
        }
    }

    pub fn set(mut self, set: ThrobberSet) -> Self {
        self.set = set;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn throbber_style(mut self, style: Style) -> Self {
        self.throbber_style = style;
        self
    }

    pub fn label_style(mut self, style: Style) -> Self {
        self.label_style = style;
        self
    }

    /// Render with state
    pub fn render_with_state(self, area: Rect, buf: &mut Buffer, state: &ThrobberState) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let frame = state.current_frame(&self.set);

        // Render throbber
        buf.set_string(area.x, area.y, frame, self.throbber_style);

        // Render label
        if let Some(label) = self.label {
            let label_x = area.x + 2;
            if label_x < area.x + area.width {
                buf.set_string(label_x, area.y, label, self.label_style);
            }
        }
    }
}

impl<'a> Widget for ThrobberWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Static render (first frame)
        let state = ThrobberState::new();
        self.render_with_state(area, buf, &state);
    }
}

/// Simple spinner widget (simpler API)
pub struct SpinnerWidget<'a> {
    /// Style preset
    style: SpinnerStyle,
    /// Label
    label: Option<&'a str>,
    /// Color
    color: Color,
}

/// Spinner style presets
#[derive(Debug, Clone, Copy)]
pub enum SpinnerStyle {
    Dots,
    Braille,
    Ascii,
    Clock,
    Line,
    Block,
    Arrows,
    Circle,
}

impl<'a> SpinnerWidget<'a> {
    pub fn new(style: SpinnerStyle) -> Self {
        SpinnerWidget {
            style,
            label: None,
            color: Color::Cyan,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    fn get_set(&self) -> ThrobberSet {
        match self.style {
            SpinnerStyle::Dots => ThrobberSet::dots(),
            SpinnerStyle::Braille => ThrobberSet::braille(),
            SpinnerStyle::Ascii => ThrobberSet::ascii(),
            SpinnerStyle::Clock => ThrobberSet::clock(),
            SpinnerStyle::Line => ThrobberSet::line(),
            SpinnerStyle::Block => ThrobberSet::block(),
            SpinnerStyle::Arrows => ThrobberSet::arrows(),
            SpinnerStyle::Circle => ThrobberSet::circle(),
        }
    }
}

impl<'a> Widget for SpinnerWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width == 0 || area.height == 0 {
            return;
        }

        let set = self.get_set();
        let frame = set.frames.first().unwrap_or(&" ");
        let style = Style::default().fg(self.color).add_modifier(Modifier::BOLD);

        buf.set_string(area.x, area.y, *frame, style);

        if let Some(label) = self.label {
            let label_x = area.x + 2;
            if label_x < area.x + area.width {
                buf.set_string(label_x, area.y, label, Style::default());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_throbber_sets() {
        let braille = ThrobberSet::braille();
        assert_eq!(braille.frames.len(), 10);

        let clock = ThrobberSet::clock();
        assert_eq!(clock.frames.len(), 12);
    }

    #[test]
    fn test_throbber_state() {
        let mut state = ThrobberState::new();
        let set = ThrobberSet::braille();

        assert_eq!(state.current_frame(&set), "⠋");

        state.calc_next(set.frames.len());
        assert_eq!(state.current_frame(&set), "⠙");
    }

    #[test]
    fn test_spinner_widget() {
        let spinner = SpinnerWidget::new(SpinnerStyle::Dots)
            .label("Loading...")
            .color(Color::Green);

        assert_eq!(spinner.label, Some("Loading..."));
    }
}
