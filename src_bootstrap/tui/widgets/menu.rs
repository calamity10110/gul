// GUL TUI Menu Widget
// Dropdown and context menus
// Inspired by tui-menu

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Widget},
};

use crate::tui::theme::GulTheme;

/// Menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    /// Display label
    pub label: String,
    /// Keyboard shortcut
    pub shortcut: Option<String>,
    /// Icon (emoji)
    pub icon: Option<String>,
    /// Is separator
    pub separator: bool,
    /// Is disabled
    pub disabled: bool,
    /// Is danger action
    pub danger: bool,
    /// Submenu items
    pub submenu: Vec<MenuItem>,
    /// Action ID
    pub action: Option<String>,
}

impl MenuItem {
    pub fn new(label: impl Into<String>) -> Self {
        MenuItem {
            label: label.into(),
            shortcut: None,
            icon: None,
            separator: false,
            disabled: false,
            danger: false,
            submenu: Vec::new(),
            action: None,
        }
    }

    pub fn separator() -> Self {
        MenuItem {
            label: String::new(),
            shortcut: None,
            icon: None,
            separator: true,
            disabled: false,
            danger: false,
            submenu: Vec::new(),
            action: None,
        }
    }

    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }

    pub fn submenu(mut self, items: Vec<MenuItem>) -> Self {
        self.submenu = items;
        self
    }

    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
}

/// Menu state
#[derive(Default, Clone)]
pub struct MenuState {
    /// Selected index
    pub selected: usize,
    /// Is open
    pub open: bool,
    /// Active submenu index (if any)
    pub submenu_index: Option<usize>,
    /// Submenu selected
    pub submenu_selected: usize,
}

impl MenuState {
    pub fn open(&mut self) {
        self.open = true;
        self.selected = 0;
        self.submenu_index = None;
    }

    pub fn close(&mut self) {
        self.open = false;
        self.submenu_index = None;
    }

    pub fn toggle(&mut self) {
        if self.open {
            self.close();
        } else {
            self.open();
        }
    }

    pub fn move_up(&mut self, items: &[MenuItem]) {
        if items.is_empty() {
            return;
        }

        if self.submenu_index.is_some() {
            if self.submenu_selected > 0 {
                self.submenu_selected -= 1;
            }
        } else if self.selected > 0 {
            self.selected -= 1;
            // Skip separators
            while self.selected > 0
                && items
                    .get(self.selected)
                    .map(|i| i.separator)
                    .unwrap_or(false)
            {
                self.selected -= 1;
            }
        }
    }

    pub fn move_down(&mut self, items: &[MenuItem]) {
        if items.is_empty() {
            return;
        }

        if self.submenu_index.is_some() {
            // Handle submenu navigation
            if let Some(idx) = self.submenu_index {
                if let Some(item) = items.get(idx) {
                    if self.submenu_selected < item.submenu.len().saturating_sub(1) {
                        self.submenu_selected += 1;
                    }
                }
            }
        } else if self.selected < items.len().saturating_sub(1) {
            self.selected += 1;
            // Skip separators
            while self.selected < items.len() - 1
                && items
                    .get(self.selected)
                    .map(|i| i.separator)
                    .unwrap_or(false)
            {
                self.selected += 1;
            }
        }
    }

    pub fn enter_submenu(&mut self, items: &[MenuItem]) {
        if let Some(item) = items.get(self.selected) {
            if !item.submenu.is_empty() {
                self.submenu_index = Some(self.selected);
                self.submenu_selected = 0;
            }
        }
    }

    pub fn exit_submenu(&mut self) {
        self.submenu_index = None;
        self.submenu_selected = 0;
    }
}

/// Menu widget
pub struct MenuWidget<'a> {
    /// Menu items
    items: &'a [MenuItem],
    /// Theme
    theme: &'a GulTheme,
    /// Title
    title: Option<&'a str>,
}

impl<'a> MenuWidget<'a> {
    pub fn new(items: &'a [MenuItem], theme: &'a GulTheme) -> Self {
        MenuWidget {
            items,
            theme,
            title: None,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    fn calculate_width(&self) -> u16 {
        let mut max_width = 0u16;

        for item in self.items {
            if item.separator {
                continue;
            }

            let mut width = item.label.len() as u16;
            if item.icon.is_some() {
                width += 3; // icon + space
            }
            if let Some(ref shortcut) = item.shortcut {
                width += shortcut.len() as u16 + 4; // padding
            }
            if !item.submenu.is_empty() {
                width += 2; // arrow
            }

            max_width = max_width.max(width);
        }

        max_width + 4 // padding
    }
}

impl<'a> Widget for MenuWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.items.is_empty() {
            return;
        }

        let width = self.calculate_width().min(area.width);
        let height = (self.items.len() as u16 + 2).min(area.height);

        let menu_area = Rect::new(area.x, area.y, width, height);

        // Clear background
        Clear.render(menu_area, buf);

        // Draw border
        let block = if let Some(title) = self.title {
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.border_focused))
        } else {
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(self.theme.border_focused))
        };

        let inner = block.inner(menu_area);
        block.render(menu_area, buf);

        // Render items
        for (i, item) in self.items.iter().enumerate() {
            if i >= inner.height as usize {
                break;
            }

            let y = inner.y + i as u16;

            if item.separator {
                let sep = "─".repeat(inner.width as usize);
                buf.set_string(inner.x, y, &sep, Style::default().fg(self.theme.border));
                continue;
            }

            let style = if item.disabled {
                Style::default().fg(self.theme.line_number)
            } else if item.danger {
                Style::default().fg(ratatui::style::Color::Red)
            } else {
                self.theme.menu
            };

            // Icon
            let mut x = inner.x;
            if let Some(ref icon) = item.icon {
                buf.set_string(x, y, icon, style);
                x += 2;
            }

            // Label
            buf.set_string(x, y, &item.label, style);

            // Shortcut (right-aligned)
            if let Some(ref shortcut) = item.shortcut {
                let shortcut_x = inner.x + inner.width - shortcut.len() as u16;
                buf.set_string(shortcut_x, y, shortcut, style.add_modifier(Modifier::DIM));
            }

            // Submenu arrow
            if !item.submenu.is_empty() {
                let arrow_x = inner.x + inner.width - 1;
                buf.set_string(arrow_x, y, "▶", style);
            }
        }
    }
}

/// Context menu widget
pub struct ContextMenuWidget<'a> {
    /// Menu items
    items: &'a [MenuItem],
    /// Position
    x: u16,
    y: u16,
    /// State
    state: &'a MenuState,
    /// Theme
    theme: &'a GulTheme,
}

impl<'a> ContextMenuWidget<'a> {
    pub fn new(
        items: &'a [MenuItem],
        x: u16,
        y: u16,
        state: &'a MenuState,
        theme: &'a GulTheme,
    ) -> Self {
        ContextMenuWidget {
            items,
            x,
            y,
            state,
            theme,
        }
    }
}

impl<'a> Widget for ContextMenuWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.state.open || self.items.is_empty() {
            return;
        }

        // Calculate menu size
        let mut max_width = 20u16;
        for item in self.items {
            let width = item.label.len() as u16 + 6;
            max_width = max_width.max(width);
        }

        let width = max_width.min(area.width - self.x);
        let height = (self.items.len() as u16 + 2).min(area.height - self.y);

        let menu_area = Rect::new(self.x, self.y, width, height);

        // Clear background
        Clear.render(menu_area, buf);

        // Draw border
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border_focused));

        let inner = block.inner(menu_area);
        block.render(menu_area, buf);

        // Render items
        for (i, item) in self.items.iter().enumerate() {
            if i >= inner.height as usize {
                break;
            }

            let y = inner.y + i as u16;
            let is_selected = i == self.state.selected;

            if item.separator {
                let sep = "─".repeat(inner.width as usize);
                buf.set_string(inner.x, y, &sep, Style::default().fg(self.theme.border));
                continue;
            }

            let style = if is_selected {
                self.theme.menu_selected
            } else if item.disabled {
                Style::default().fg(self.theme.line_number)
            } else if item.danger {
                Style::default().fg(ratatui::style::Color::Red)
            } else {
                self.theme.menu
            };

            // Render with padding
            let text = format!(" {} ", item.label);
            buf.set_string(inner.x, y, &text, style);
        }
    }
}

/// Menu bar widget
pub struct MenuBarWidget<'a> {
    /// Menus
    menus: &'a [(&'a str, &'a [MenuItem])],
    /// Active menu index
    active: Option<usize>,
    /// Theme
    theme: &'a GulTheme,
}

impl<'a> MenuBarWidget<'a> {
    pub fn new(menus: &'a [(&'a str, &'a [MenuItem])], theme: &'a GulTheme) -> Self {
        MenuBarWidget {
            menus,
            active: None,
            theme,
        }
    }

    pub fn active(mut self, index: usize) -> Self {
        self.active = Some(index);
        self
    }
}

impl<'a> Widget for MenuBarWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Background
        for x in area.x..area.x + area.width {
            buf.set_string(x, area.y, " ", self.theme.menu);
        }

        let mut x = area.x;

        for (i, (label, _items)) in self.menus.iter().enumerate() {
            let is_active = self.active == Some(i);

            let style = if is_active {
                self.theme.menu_selected
            } else {
                self.theme.menu
            };

            let text = format!(" {} ", label);
            buf.set_string(x, area.y, &text, style);
            x += text.len() as u16;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_item_creation() {
        let item = MenuItem::new("Open")
            .shortcut("Ctrl+O")
            .icon("📂")
            .action("file.open");

        assert_eq!(item.label, "Open");
        assert_eq!(item.shortcut, Some("Ctrl+O".to_string()));
    }

    #[test]
    fn test_menu_state() {
        let mut state = MenuState::default();

        state.open();
        assert!(state.open);

        let items = vec![MenuItem::new("Item 1"), MenuItem::new("Item 2")];

        state.move_down(&items);
        assert_eq!(state.selected, 1);

        state.move_up(&items);
        assert_eq!(state.selected, 0);
    }

    #[test]
    fn test_separator() {
        let sep = MenuItem::separator();
        assert!(sep.separator);
    }
}
