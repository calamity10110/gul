// GUL TUI Image Widget
// Terminal image rendering with multiple protocol support
// Inspired by ratatui-image

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Widget},
};
use std::path::PathBuf;

use crate::tui::theme::GulTheme;

/// Image rendering protocol
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageProtocol {
    /// Auto-detect best protocol
    Auto,
    /// Sixel graphics (DEC terminals, mlterm, foot)
    Sixel,
    /// Kitty graphics protocol
    Kitty,
    /// iTerm2 inline images
    ITerm2,
    /// Unicode halfblocks (fallback, works everywhere)
    Halfblocks,
    /// Braille dots (ultra-low resolution)
    Braille,
}

impl Default for ImageProtocol {
    fn default() -> Self {
        ImageProtocol::Auto
    }
}

/// Image fit mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImageFit {
    /// Scale to fit within bounds, maintain aspect ratio
    Contain,
    /// Scale to cover bounds, may crop
    Cover,
    /// Stretch to fill bounds exactly
    Fill,
    /// No scaling, original size
    None,
}

impl Default for ImageFit {
    fn default() -> Self {
        ImageFit::Contain
    }
}

/// Image state for stateful rendering
#[derive(Default)]
pub struct ImageState {
    /// Cached pixel data
    pub cached: bool,
    /// Last rendered size
    pub last_size: (u16, u16),
}

/// Image widget
pub struct ImageWidget<'a> {
    /// Image source path
    source: Option<PathBuf>,
    /// Image data (raw pixels or encoded)
    data: Option<&'a [u8]>,
    /// Rendering protocol
    protocol: ImageProtocol,
    /// Fit mode
    fit: ImageFit,
    /// Alt text for accessibility
    alt: Option<&'a str>,
    /// Theme
    theme: &'a GulTheme,
    /// Border title
    title: Option<&'a str>,
}

impl<'a> ImageWidget<'a> {
    pub fn new(theme: &'a GulTheme) -> Self {
        ImageWidget {
            source: None,
            data: None,
            protocol: ImageProtocol::default(),
            fit: ImageFit::default(),
            alt: None,
            theme,
            title: None,
        }
    }

    pub fn source(mut self, path: PathBuf) -> Self {
        self.source = Some(path);
        self
    }

    pub fn data(mut self, data: &'a [u8]) -> Self {
        self.data = Some(data);
        self
    }

    pub fn protocol(mut self, protocol: ImageProtocol) -> Self {
        self.protocol = protocol;
        self
    }

    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    pub fn alt(mut self, alt: &'a str) -> Self {
        self.alt = Some(alt);
        self
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    /// Detect best protocol for current terminal
    #[allow(dead_code)]
    fn detect_protocol() -> ImageProtocol {
        // Check environment variables for terminal type
        if let Ok(term) = std::env::var("TERM") {
            if term.contains("kitty") {
                return ImageProtocol::Kitty;
            }
            if term.contains("mlterm") || term.contains("foot") {
                return ImageProtocol::Sixel;
            }
        }

        if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
            if term_program == "iTerm.app" {
                return ImageProtocol::ITerm2;
            }
            if term_program == "WezTerm" {
                return ImageProtocol::Kitty;
            }
        }

        // Fallback to halfblocks (works everywhere)
        ImageProtocol::Halfblocks
    }

    /// Render image using halfblocks (fallback)
    fn render_halfblocks(&self, area: Rect, buf: &mut Buffer) {
        // Placeholder rendering using braille/block characters
        let placeholder = if let Some(alt) = self.alt {
            format!("🖼️ {}", alt)
        } else if let Some(ref path) = self.source {
            format!(
                "🖼️ {}",
                path.file_name().unwrap_or_default().to_string_lossy()
            )
        } else {
            "🖼️ [Image]".to_string()
        };

        // Center the placeholder text
        let x = area.x + (area.width.saturating_sub(placeholder.len() as u16)) / 2;
        let y = area.y + area.height / 2;

        if y < area.y + area.height && x < area.x + area.width {
            buf.set_string(x, y, &placeholder, Style::default());
        }
    }
}

impl<'a> Widget for ImageWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Draw border if title is set
        if let Some(title) = self.title {
            let block = Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.border));

            let inner = block.inner(area);
            block.render(area, buf);
            self.render_halfblocks(inner, buf);
        } else {
            self.render_halfblocks(area, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_widget_creation() {
        let theme = GulTheme::default();
        let widget = ImageWidget::new(&theme)
            .source(PathBuf::from("test.png"))
            .alt("Test image");

        assert_eq!(widget.alt, Some("Test image"));
    }

    #[test]
    fn test_protocol_detection() {
        // Just verify it doesn't panic
        let _ = ImageWidget::detect_protocol();
    }
}
