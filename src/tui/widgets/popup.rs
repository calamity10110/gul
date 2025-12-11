// GUL TUI Popup Widget
// Modal dialogs and popups
// Inspired by tui-popup

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

use crate::tui::theme::GulTheme;

/// Popup button
#[derive(Debug, Clone)]
pub struct PopupButton {
    pub label: String,
    pub action: String,
    pub primary: bool,
    pub danger: bool,
}

impl PopupButton {
    pub fn new(label: impl Into<String>, action: impl Into<String>) -> Self {
        PopupButton {
            label: label.into(),
            action: action.into(),
            primary: false,
            danger: false,
        }
    }

    pub fn primary(mut self) -> Self {
        self.primary = true;
        self
    }

    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }
}

/// Popup type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PopupType {
    Info,
    Success,
    Warning,
    Error,
    Confirm,
    Input,
}

impl PopupType {
    fn icon(&self) -> &'static str {
        match self {
            PopupType::Info => "ℹ️",
            PopupType::Success => "✅",
            PopupType::Warning => "⚠️",
            PopupType::Error => "❌",
            PopupType::Confirm => "❓",
            PopupType::Input => "✏️",
        }
    }
}

/// Popup state
#[derive(Default, Clone)]
pub struct PopupState {
    /// Is visible
    pub visible: bool,
    /// Selected button index
    pub selected_button: usize,
    /// Input value (for input popups)
    pub input_value: String,
}

impl PopupState {
    pub fn show(&mut self) {
        self.visible = true;
        self.selected_button = 0;
        self.input_value.clear();
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn next_button(&mut self, button_count: usize) {
        if button_count > 0 {
            self.selected_button = (self.selected_button + 1) % button_count;
        }
    }

    pub fn prev_button(&mut self, button_count: usize) {
        if button_count > 0 {
            self.selected_button = self
                .selected_button
                .checked_sub(1)
                .unwrap_or(button_count - 1);
        }
    }
}

/// Popup widget
pub struct PopupWidget<'a> {
    /// Title
    title: &'a str,
    /// Content
    content: &'a str,
    /// Popup type
    popup_type: PopupType,
    /// Buttons
    buttons: Vec<PopupButton>,
    /// Width percentage (0-100)
    width_percent: u16,
    /// Height percentage (0-100)
    height_percent: u16,
    /// Theme
    theme: &'a GulTheme,
    /// State
    state: &'a PopupState,
}

impl<'a> PopupWidget<'a> {
    pub fn new(
        title: &'a str,
        content: &'a str,
        theme: &'a GulTheme,
        state: &'a PopupState,
    ) -> Self {
        PopupWidget {
            title,
            content,
            popup_type: PopupType::Info,
            buttons: vec![PopupButton::new("OK", "ok").primary()],
            width_percent: 50,
            height_percent: 40,
            theme,
            state,
        }
    }

    pub fn popup_type(mut self, popup_type: PopupType) -> Self {
        self.popup_type = popup_type;
        self
    }

    pub fn buttons(mut self, buttons: Vec<PopupButton>) -> Self {
        self.buttons = buttons;
        self
    }

    pub fn width_percent(mut self, percent: u16) -> Self {
        self.width_percent = percent.min(100);
        self
    }

    pub fn height_percent(mut self, percent: u16) -> Self {
        self.height_percent = percent.min(100);
        self
    }

    /// Helper to create a centered rect
    fn centered_rect(&self, area: Rect) -> Rect {
        let width = (area.width as u32 * self.width_percent as u32 / 100) as u16;
        let height = (area.height as u32 * self.height_percent as u32 / 100) as u16;

        let x = area.x + (area.width.saturating_sub(width)) / 2;
        let y = area.y + (area.height.saturating_sub(height)) / 3; // Slightly above center

        Rect::new(x, y, width, height)
    }
}

impl<'a> Widget for PopupWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.state.visible {
            return;
        }

        let popup_area = self.centered_rect(area);

        // Clear background
        Clear.render(popup_area, buf);

        // Draw border
        let title_with_icon = format!(" {} {} ", self.popup_type.icon(), self.title);
        let block = Block::default()
            .title(title_with_icon)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border_focused));

        let inner = block.inner(popup_area);
        block.render(popup_area, buf);

        // Content area (leave room for buttons)
        let content_height = inner.height.saturating_sub(3);
        let content_area = Rect::new(inner.x, inner.y, inner.width, content_height);

        // Render content
        let paragraph = Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);
        paragraph.render(content_area, buf);

        // Render buttons
        if !self.buttons.is_empty() {
            let button_y = inner.y + inner.height - 1;
            let total_button_width: usize = self.buttons.iter().map(|b| b.label.len() + 4).sum();

            let mut x = inner.x + (inner.width.saturating_sub(total_button_width as u16)) / 2;

            for (i, button) in self.buttons.iter().enumerate() {
                let is_selected = i == self.state.selected_button;

                let style = if is_selected {
                    if button.danger {
                        Style::default()
                            .fg(ratatui::style::Color::White)
                            .bg(ratatui::style::Color::Red)
                            .add_modifier(Modifier::BOLD)
                    } else if button.primary {
                        self.theme.menu_selected.add_modifier(Modifier::BOLD)
                    } else {
                        self.theme.menu_selected
                    }
                } else {
                    self.theme.menu
                };

                let text = format!(" {} ", button.label);
                buf.set_string(x, button_y, &text, style);
                x += text.len() as u16 + 1;
            }
        }
    }
}

/// Alert popup (convenience wrapper)
pub struct AlertWidget<'a> {
    title: &'a str,
    message: &'a str,
    alert_type: PopupType,
    theme: &'a GulTheme,
}

impl<'a> AlertWidget<'a> {
    pub fn info(message: &'a str, theme: &'a GulTheme) -> Self {
        AlertWidget {
            title: "Info",
            message,
            alert_type: PopupType::Info,
            theme,
        }
    }

    pub fn success(message: &'a str, theme: &'a GulTheme) -> Self {
        AlertWidget {
            title: "Success",
            message,
            alert_type: PopupType::Success,
            theme,
        }
    }

    pub fn warning(message: &'a str, theme: &'a GulTheme) -> Self {
        AlertWidget {
            title: "Warning",
            message,
            alert_type: PopupType::Warning,
            theme,
        }
    }

    pub fn error(message: &'a str, theme: &'a GulTheme) -> Self {
        AlertWidget {
            title: "Error",
            message,
            alert_type: PopupType::Error,
            theme,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }
}

impl<'a> Widget for AlertWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Calculate size based on content
        let width = 40.min(area.width - 4);
        let height = 7.min(area.height - 4);

        let x = (area.width - width) / 2 + area.x;
        let y = (area.height - height) / 3 + area.y;

        let alert_area = Rect::new(x, y, width, height);

        Clear.render(alert_area, buf);

        let title = format!(" {} {} ", self.alert_type.icon(), self.title);
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border_focused));

        let inner = block.inner(alert_area);
        block.render(alert_area, buf);

        // Center message
        let msg_x = inner.x + (inner.width.saturating_sub(self.message.len() as u16)) / 2;
        let msg_y = inner.y + inner.height / 2;
        buf.set_string(msg_x, msg_y, self.message, Style::default());
    }
}

/// Confirm dialog
pub struct ConfirmWidget<'a> {
    title: &'a str,
    message: &'a str,
    confirm_label: &'a str,
    cancel_label: &'a str,
    selected: usize,
    theme: &'a GulTheme,
}

impl<'a> ConfirmWidget<'a> {
    pub fn new(title: &'a str, message: &'a str, theme: &'a GulTheme) -> Self {
        ConfirmWidget {
            title,
            message,
            confirm_label: "OK",
            cancel_label: "Cancel",
            selected: 1, // Default to cancel
            theme,
        }
    }

    pub fn confirm_label(mut self, label: &'a str) -> Self {
        self.confirm_label = label;
        self
    }

    pub fn cancel_label(mut self, label: &'a str) -> Self {
        self.cancel_label = label;
        self
    }

    pub fn selected(mut self, selected: usize) -> Self {
        self.selected = selected;
        self
    }
}

impl<'a> Widget for ConfirmWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = 50.min(area.width - 4);
        let height = 8.min(area.height - 4);

        let x = (area.width - width) / 2 + area.x;
        let y = (area.height - height) / 3 + area.y;

        let dialog_area = Rect::new(x, y, width, height);

        Clear.render(dialog_area, buf);

        let title = format!(" ❓ {} ", self.title);
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border_focused));

        let inner = block.inner(dialog_area);
        block.render(dialog_area, buf);

        // Message
        let msg_y = inner.y + 1;
        let paragraph = Paragraph::new(self.message)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        let msg_area = Rect::new(inner.x, msg_y, inner.width, 2);
        paragraph.render(msg_area, buf);

        // Buttons
        let button_y = inner.y + inner.height - 1;
        let confirm_btn = format!(" {} ", self.confirm_label);
        let cancel_btn = format!(" {} ", self.cancel_label);

        let total_width = confirm_btn.len() + cancel_btn.len() + 2;
        let start_x = inner.x + (inner.width.saturating_sub(total_width as u16)) / 2;

        // Confirm button
        let confirm_style = if self.selected == 0 {
            self.theme.menu_selected.add_modifier(Modifier::BOLD)
        } else {
            self.theme.menu
        };
        buf.set_string(start_x, button_y, &confirm_btn, confirm_style);

        // Cancel button
        let cancel_style = if self.selected == 1 {
            self.theme.menu_selected
        } else {
            self.theme.menu
        };
        buf.set_string(
            start_x + confirm_btn.len() as u16 + 2,
            button_y,
            &cancel_btn,
            cancel_style,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popup_button() {
        let btn = PopupButton::new("Save", "save").primary();
        assert!(btn.primary);
        assert!(!btn.danger);
    }

    #[test]
    fn test_popup_state() {
        let mut state = PopupState::default();
        assert!(!state.visible);

        state.show();
        assert!(state.visible);

        state.next_button(3);
        assert_eq!(state.selected_button, 1);
    }

    #[test]
    fn test_popup_type_icon() {
        assert_eq!(PopupType::Success.icon(), "✅");
        assert_eq!(PopupType::Error.icon(), "❌");
    }
}
