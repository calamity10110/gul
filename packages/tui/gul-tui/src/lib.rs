// GUL TUI - Terminal User Interface Framework
// Based on ratatui for building terminal UIs

use std::io;

pub mod layout;
pub mod style;
pub mod widgets;

pub use layout::*;
pub use style::*;
pub use widgets::*;

/// Terminal backend
pub struct Terminal {
    width: u16,
    height: u16,
}

impl Terminal {
    pub fn new() -> Result<Self, io::Error> {
        Ok(Self {
            width: 80,
            height: 24,
        })
    }

    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn draw<F>(&mut self, f: F) -> Result<(), io::Error>
    where
        F: FnOnce(&mut Frame),
    {
        let mut frame = Frame::new(self.width, self.height);
        f(&mut frame);
        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

/// Frame for drawing
pub struct Frame {
    width: u16,
    height: u16,
}

impl Frame {
    fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn render_widget<W: Widget>(&mut self, widget: W, area: Rect) {
        widget.render(area, self);
    }
}

/// Rectangle area
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Widget trait
pub trait Widget {
    fn render(&self, area: Rect, frame: &mut Frame);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminal_creation() {
        let terminal = Terminal::new().unwrap();
        assert_eq!(terminal.size(), (80, 24));
    }

    #[test]
    fn test_rect() {
        let rect = Rect::new(0, 0, 80, 24);
        assert_eq!(rect.width, 80);
        assert_eq!(rect.height, 24);
    }
}
