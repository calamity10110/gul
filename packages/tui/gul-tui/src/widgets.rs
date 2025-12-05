// TUI Widgets

use crate::{Frame, Rect, Widget};

/// Text widget
pub struct Text {
    content: String,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl Widget for Text {
    fn render(&self, _area: Rect, _frame: &mut Frame) {
        // Render text
    }
}

/// Block widget (border)
pub struct Block {
    title: Option<String>,
}

impl Block {
    pub fn new() -> Self {
        Self { title: None }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for Block {
    fn render(&self, _area: Rect, _frame: &mut Frame) {
        // Render block
    }
}

/// List widget
pub struct List {
    items: Vec<String>,
}

impl List {
    pub fn new(items: Vec<String>) -> Self {
        Self { items }
    }
}

impl Widget for List {
    fn render(&self, _area: Rect, _frame: &mut Frame) {
        // Render list
    }
}
