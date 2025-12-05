// TUI Layout

use crate::Rect;

/// Layout direction
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}

/// Layout constraint
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Constraint {
    Percentage(u16),
    Length(u16),
    Min(u16),
    Max(u16),
}

/// Layout builder
pub struct Layout {
    direction: Direction,
    constraints: Vec<Constraint>,
}

impl Layout {
    pub fn default() -> Self {
        Self {
            direction: Direction::Vertical,
            constraints: Vec::new(),
        }
    }

    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    pub fn constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.constraints = constraints;
        self
    }

    pub fn split(&self, area: Rect) -> Vec<Rect> {
        // Simple split implementation
        vec![area]
    }
}
