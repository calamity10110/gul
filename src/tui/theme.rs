// GUL TUI Theme - Color schemes and styling
// Supports v2.1 syntax highlighting

use ratatui::style::{Color, Modifier, Style};

/// GUL v2.1 Color Theme
#[derive(Clone, Debug)]
pub struct GulTheme {
    // Editor colors
    pub background: Color,
    pub foreground: Color,
    pub selection: Color,
    pub cursor: Color,
    pub line_number: Color,
    pub line_number_active: Color,
    pub cursor_style: Style,
    pub selection_style: Style,

    // Syntax colors
    pub keyword: Style,
    pub function: Style,
    pub string: Style,
    pub number: Style,
    pub comment: Style,
    pub operator: Style,
    pub identifier: Style,

    // v2.1 specific
    pub ui_marker: Style,      // ^&^ highlighting
    pub ui_component: Style,   // button, table, etc.
    pub bracket: Style,        // All bracket types
    pub bracket_match: Style,  // Matching bracket pair
    pub file_type_main: Style, // .mn
    pub file_type_def: Style,  // .def
    pub file_type_fnc: Style,  // .fnc
    pub file_type_cs: Style,   // .cs
    pub file_type_sct: Style,  // .sct (secret)

    // UI elements
    pub border: Color,
    pub border_focused: Color,
    pub title: Style,
    pub status_bar: Style,
    pub menu: Style,
    pub menu_selected: Style,
}

impl Default for GulTheme {
    fn default() -> Self {
        Self::dark()
    }
}

impl GulTheme {
    /// Dark theme (default)
    pub fn dark() -> Self {
        GulTheme {
            // Editor colors
            background: Color::Rgb(30, 30, 46),
            foreground: Color::Rgb(205, 214, 244),
            selection: Color::Rgb(88, 91, 112),
            cursor: Color::Rgb(245, 224, 220),
            line_number: Color::Rgb(88, 91, 112),
            line_number_active: Color::Rgb(205, 214, 244),
            cursor_style: Style::default().bg(Color::Rgb(245, 224, 220)).fg(Color::Rgb(30, 30, 46)),
            selection_style: Style::default().bg(Color::Rgb(88, 91, 112)),

            // Syntax colors (Catppuccin Mocha inspired)
            keyword: Style::default()
                .fg(Color::Rgb(203, 166, 247)) // Mauve
                .add_modifier(Modifier::BOLD),
            function: Style::default().fg(Color::Rgb(137, 180, 250)), // Blue
            string: Style::default().fg(Color::Rgb(166, 227, 161)),   // Green
            number: Style::default().fg(Color::Rgb(250, 179, 135)),   // Peach
            comment: Style::default()
                .fg(Color::Rgb(108, 112, 134)) // Overlay0
                .add_modifier(Modifier::ITALIC),
            operator: Style::default().fg(Color::Rgb(148, 226, 213)), // Teal
            identifier: Style::default().fg(Color::Rgb(205, 214, 244)), // Text

            // v2.1 specific
            ui_marker: Style::default()
                .fg(Color::Rgb(249, 226, 175)) // Yellow
                .add_modifier(Modifier::BOLD),
            ui_component: Style::default().fg(Color::Rgb(148, 226, 213)), // Teal
            bracket: Style::default().fg(Color::Rgb(243, 139, 168)),      // Pink
            bracket_match: Style::default()
                .bg(Color::Rgb(69, 71, 90)) // Surface1
                .add_modifier(Modifier::BOLD),
            file_type_main: Style::default().fg(Color::Rgb(137, 180, 250)), // Blue
            file_type_def: Style::default().fg(Color::Rgb(166, 227, 161)),  // Green
            file_type_fnc: Style::default().fg(Color::Rgb(250, 179, 135)),  // Peach
            file_type_cs: Style::default().fg(Color::Rgb(203, 166, 247)),   // Mauve
            file_type_sct: Style::default()
                .fg(Color::Rgb(243, 139, 168)) // Pink
                .add_modifier(Modifier::BOLD),

            // UI elements
            border: Color::Rgb(88, 91, 112),
            border_focused: Color::Rgb(137, 180, 250),
            title: Style::default()
                .fg(Color::Rgb(205, 214, 244))
                .add_modifier(Modifier::BOLD),
            status_bar: Style::default()
                .bg(Color::Rgb(49, 50, 68))
                .fg(Color::Rgb(205, 214, 244)),
            menu: Style::default()
                .bg(Color::Rgb(49, 50, 68))
                .fg(Color::Rgb(205, 214, 244)),
            menu_selected: Style::default()
                .bg(Color::Rgb(137, 180, 250))
                .fg(Color::Rgb(30, 30, 46)),
        }
    }

    /// Light theme
    pub fn light() -> Self {
        GulTheme {
            // Editor colors
            background: Color::Rgb(239, 241, 245),
            foreground: Color::Rgb(76, 79, 105),
            selection: Color::Rgb(172, 176, 190),
            cursor: Color::Rgb(220, 138, 120),
            line_number: Color::Rgb(156, 160, 176),
            line_number_active: Color::Rgb(76, 79, 105),
            cursor_style: Style::default().bg(Color::Rgb(220, 138, 120)).fg(Color::Rgb(239, 241, 245)),
            selection_style: Style::default().bg(Color::Rgb(172, 176, 190)),

            // Syntax colors (Catppuccin Latte inspired)
            keyword: Style::default()
                .fg(Color::Rgb(136, 57, 239)) // Mauve
                .add_modifier(Modifier::BOLD),
            function: Style::default().fg(Color::Rgb(30, 102, 245)), // Blue
            string: Style::default().fg(Color::Rgb(64, 160, 43)),    // Green
            number: Style::default().fg(Color::Rgb(254, 100, 11)),   // Peach
            comment: Style::default()
                .fg(Color::Rgb(156, 160, 176)) // Overlay0
                .add_modifier(Modifier::ITALIC),
            operator: Style::default().fg(Color::Rgb(23, 146, 153)), // Teal
            identifier: Style::default().fg(Color::Rgb(76, 79, 105)), // Text

            // v2.1 specific
            ui_marker: Style::default()
                .fg(Color::Rgb(223, 142, 29)) // Yellow
                .add_modifier(Modifier::BOLD),
            ui_component: Style::default().fg(Color::Rgb(23, 146, 153)), // Teal
            bracket: Style::default().fg(Color::Rgb(234, 118, 203)),     // Pink
            bracket_match: Style::default()
                .bg(Color::Rgb(204, 208, 218)) // Surface1
                .add_modifier(Modifier::BOLD),
            file_type_main: Style::default().fg(Color::Rgb(30, 102, 245)), // Blue
            file_type_def: Style::default().fg(Color::Rgb(64, 160, 43)),   // Green
            file_type_fnc: Style::default().fg(Color::Rgb(254, 100, 11)),  // Peach
            file_type_cs: Style::default().fg(Color::Rgb(136, 57, 239)),   // Mauve
            file_type_sct: Style::default()
                .fg(Color::Rgb(234, 118, 203)) // Pink
                .add_modifier(Modifier::BOLD),

            // UI elements
            border: Color::Rgb(156, 160, 176),
            border_focused: Color::Rgb(30, 102, 245),
            title: Style::default()
                .fg(Color::Rgb(76, 79, 105))
                .add_modifier(Modifier::BOLD),
            status_bar: Style::default()
                .bg(Color::Rgb(230, 233, 239))
                .fg(Color::Rgb(76, 79, 105)),
            menu: Style::default()
                .bg(Color::Rgb(230, 233, 239))
                .fg(Color::Rgb(76, 79, 105)),
            menu_selected: Style::default()
                .bg(Color::Rgb(30, 102, 245))
                .fg(Color::Rgb(239, 241, 245)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dark_theme() {
        let theme = GulTheme::dark();
        assert_eq!(theme.background, Color::Rgb(30, 30, 46));
    }

    #[test]
    fn test_light_theme() {
        let theme = GulTheme::light();
        assert_eq!(theme.background, Color::Rgb(239, 241, 245));
    }
}
