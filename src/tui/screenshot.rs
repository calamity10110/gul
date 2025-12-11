// GUL TUI Screenshot Module
// Freeze-inspired code screenshots
// Inspired by charmbracelet/freeze

use ratatui::style::{Color, Modifier, Style};
use std::collections::HashMap;

/// Screenshot configuration
#[derive(Debug, Clone)]
pub struct ScreenshotConfig {
    /// Output format
    pub format: ScreenshotFormat,
    /// Window decorations
    pub decorations: WindowDecoration,
    /// Theme
    pub theme: ScreenshotTheme,
    /// Padding
    pub padding: u16,
    /// Line numbers
    pub show_line_numbers: bool,
    /// Window title
    pub title: Option<String>,
    /// Language for syntax highlighting
    pub language: Option<String>,
    /// Shadow
    pub shadow: bool,
    /// Border radius (for image output)
    pub border_radius: u16,
}

impl Default for ScreenshotConfig {
    fn default() -> Self {
        ScreenshotConfig {
            format: ScreenshotFormat::Ansi,
            decorations: WindowDecoration::MacOS,
            theme: ScreenshotTheme::catppuccin_mocha(),
            padding: 2,
            show_line_numbers: true,
            title: None,
            language: None,
            shadow: true,
            border_radius: 8,
        }
    }
}

/// Screenshot output format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScreenshotFormat {
    /// ANSI escape codes (terminal)
    Ansi,
    /// SVG vector graphics
    Svg,
    /// PNG image
    Png,
    /// HTML
    Html,
}

/// Window decoration style
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowDecoration {
    /// macOS-style traffic lights
    MacOS,
    /// Windows-style buttons
    Windows,
    /// Linux/GNOME style
    Gnome,
    /// No decorations
    None,
}

/// Screenshot theme
#[derive(Debug, Clone)]
pub struct ScreenshotTheme {
    pub name: String,
    pub background: Color,
    pub foreground: Color,
    pub selection: Color,
    pub cursor: Color,
    pub line_numbers: Color,
    pub gutter: Color,
    pub syntax: HashMap<String, Style>,
}

impl ScreenshotTheme {
    /// Catppuccin Mocha theme
    pub fn catppuccin_mocha() -> Self {
        let mut syntax = HashMap::new();
        syntax.insert(
            "keyword".to_string(),
            Style::default()
                .fg(Color::Rgb(203, 166, 247))
                .add_modifier(Modifier::BOLD),
        );
        syntax.insert(
            "function".to_string(),
            Style::default().fg(Color::Rgb(137, 180, 250)),
        );
        syntax.insert(
            "string".to_string(),
            Style::default().fg(Color::Rgb(166, 227, 161)),
        );
        syntax.insert(
            "number".to_string(),
            Style::default().fg(Color::Rgb(250, 179, 135)),
        );
        syntax.insert(
            "comment".to_string(),
            Style::default()
                .fg(Color::Rgb(108, 112, 134))
                .add_modifier(Modifier::ITALIC),
        );
        syntax.insert(
            "operator".to_string(),
            Style::default().fg(Color::Rgb(148, 226, 213)),
        );
        syntax.insert(
            "type".to_string(),
            Style::default().fg(Color::Rgb(249, 226, 175)),
        );

        ScreenshotTheme {
            name: "Catppuccin Mocha".to_string(),
            background: Color::Rgb(30, 30, 46),
            foreground: Color::Rgb(205, 214, 244),
            selection: Color::Rgb(88, 91, 112),
            cursor: Color::Rgb(245, 224, 220),
            line_numbers: Color::Rgb(88, 91, 112),
            gutter: Color::Rgb(49, 50, 68),
            syntax,
        }
    }

    /// Dracula theme
    pub fn dracula() -> Self {
        let mut syntax = HashMap::new();
        syntax.insert(
            "keyword".to_string(),
            Style::default()
                .fg(Color::Rgb(255, 121, 198))
                .add_modifier(Modifier::BOLD),
        );
        syntax.insert(
            "function".to_string(),
            Style::default().fg(Color::Rgb(80, 250, 123)),
        );
        syntax.insert(
            "string".to_string(),
            Style::default().fg(Color::Rgb(241, 250, 140)),
        );
        syntax.insert(
            "number".to_string(),
            Style::default().fg(Color::Rgb(189, 147, 249)),
        );
        syntax.insert(
            "comment".to_string(),
            Style::default()
                .fg(Color::Rgb(98, 114, 164))
                .add_modifier(Modifier::ITALIC),
        );
        syntax.insert(
            "operator".to_string(),
            Style::default().fg(Color::Rgb(255, 184, 108)),
        );
        syntax.insert(
            "type".to_string(),
            Style::default().fg(Color::Rgb(139, 233, 253)),
        );

        ScreenshotTheme {
            name: "Dracula".to_string(),
            background: Color::Rgb(40, 42, 54),
            foreground: Color::Rgb(248, 248, 242),
            selection: Color::Rgb(68, 71, 90),
            cursor: Color::Rgb(248, 248, 242),
            line_numbers: Color::Rgb(98, 114, 164),
            gutter: Color::Rgb(68, 71, 90),
            syntax,
        }
    }

    /// Nord theme
    pub fn nord() -> Self {
        let mut syntax = HashMap::new();
        syntax.insert(
            "keyword".to_string(),
            Style::default()
                .fg(Color::Rgb(129, 161, 193))
                .add_modifier(Modifier::BOLD),
        );
        syntax.insert(
            "function".to_string(),
            Style::default().fg(Color::Rgb(136, 192, 208)),
        );
        syntax.insert(
            "string".to_string(),
            Style::default().fg(Color::Rgb(163, 190, 140)),
        );
        syntax.insert(
            "number".to_string(),
            Style::default().fg(Color::Rgb(180, 142, 173)),
        );
        syntax.insert(
            "comment".to_string(),
            Style::default()
                .fg(Color::Rgb(76, 86, 106))
                .add_modifier(Modifier::ITALIC),
        );
        syntax.insert(
            "operator".to_string(),
            Style::default().fg(Color::Rgb(129, 161, 193)),
        );
        syntax.insert(
            "type".to_string(),
            Style::default().fg(Color::Rgb(235, 203, 139)),
        );

        ScreenshotTheme {
            name: "Nord".to_string(),
            background: Color::Rgb(46, 52, 64),
            foreground: Color::Rgb(216, 222, 233),
            selection: Color::Rgb(67, 76, 94),
            cursor: Color::Rgb(216, 222, 233),
            line_numbers: Color::Rgb(76, 86, 106),
            gutter: Color::Rgb(59, 66, 82),
            syntax,
        }
    }

    /// One Dark theme
    pub fn one_dark() -> Self {
        let mut syntax = HashMap::new();
        syntax.insert(
            "keyword".to_string(),
            Style::default()
                .fg(Color::Rgb(198, 120, 221))
                .add_modifier(Modifier::BOLD),
        );
        syntax.insert(
            "function".to_string(),
            Style::default().fg(Color::Rgb(97, 175, 239)),
        );
        syntax.insert(
            "string".to_string(),
            Style::default().fg(Color::Rgb(152, 195, 121)),
        );
        syntax.insert(
            "number".to_string(),
            Style::default().fg(Color::Rgb(209, 154, 102)),
        );
        syntax.insert(
            "comment".to_string(),
            Style::default()
                .fg(Color::Rgb(92, 99, 112))
                .add_modifier(Modifier::ITALIC),
        );
        syntax.insert(
            "operator".to_string(),
            Style::default().fg(Color::Rgb(86, 182, 194)),
        );
        syntax.insert(
            "type".to_string(),
            Style::default().fg(Color::Rgb(229, 192, 123)),
        );

        ScreenshotTheme {
            name: "One Dark".to_string(),
            background: Color::Rgb(40, 44, 52),
            foreground: Color::Rgb(171, 178, 191),
            selection: Color::Rgb(62, 68, 81),
            cursor: Color::Rgb(171, 178, 191),
            line_numbers: Color::Rgb(92, 99, 112),
            gutter: Color::Rgb(50, 55, 65),
            syntax,
        }
    }
}

/// Screenshot builder
pub struct ScreenshotBuilder {
    config: ScreenshotConfig,
    content: String,
}

impl ScreenshotBuilder {
    pub fn new(content: impl Into<String>) -> Self {
        ScreenshotBuilder {
            config: ScreenshotConfig::default(),
            content: content.into(),
        }
    }

    pub fn format(mut self, format: ScreenshotFormat) -> Self {
        self.config.format = format;
        self
    }

    pub fn decorations(mut self, decorations: WindowDecoration) -> Self {
        self.config.decorations = decorations;
        self
    }

    pub fn theme(mut self, theme: ScreenshotTheme) -> Self {
        self.config.theme = theme;
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.config.title = Some(title.into());
        self
    }

    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.config.language = Some(language.into());
        self
    }

    pub fn line_numbers(mut self, show: bool) -> Self {
        self.config.show_line_numbers = show;
        self
    }

    pub fn padding(mut self, padding: u16) -> Self {
        self.config.padding = padding;
        self
    }

    pub fn shadow(mut self, shadow: bool) -> Self {
        self.config.shadow = shadow;
        self
    }

    /// Generate ANSI output
    pub fn to_ansi(&self) -> String {
        let mut output = String::new();
        let lines: Vec<&str> = self.content.lines().collect();
        let line_num_width = lines.len().to_string().len();

        // Window decorations
        if self.config.decorations != WindowDecoration::None {
            output.push_str(&self.render_decorations());
        }

        // Content
        for (i, line) in lines.iter().enumerate() {
            // Line number
            if self.config.show_line_numbers {
                output.push_str(&format!(
                    "\x1b[38;2;{};{};{}m{:>width$} │ \x1b[0m",
                    88,
                    91,
                    112, // line number color
                    i + 1,
                    width = line_num_width
                ));
            }

            // Content
            output.push_str(line);
            output.push('\n');
        }

        output
    }

    /// Render window decorations
    fn render_decorations(&self) -> String {
        match self.config.decorations {
            WindowDecoration::MacOS => {
                let title = self.config.title.as_deref().unwrap_or("Untitled");
                format!(
                    "┌{}┐\n│ \x1b[31m●\x1b[0m \x1b[33m●\x1b[0m \x1b[32m●\x1b[0m  {} │\n├{}┤\n",
                    "─".repeat(title.len() + 12),
                    title,
                    "─".repeat(title.len() + 12)
                )
            }
            WindowDecoration::Windows => {
                let title = self.config.title.as_deref().unwrap_or("Untitled");
                format!(
                    "┌{}┐\n│ {} {} ─ □ × │\n├{}┤\n",
                    "─".repeat(title.len() + 12),
                    title,
                    " ".repeat(6),
                    "─".repeat(title.len() + 12)
                )
            }
            WindowDecoration::Gnome => {
                let title = self.config.title.as_deref().unwrap_or("Untitled");
                format!(
                    "┌{}┐\n│ {} {} × │\n├{}┤\n",
                    "─".repeat(title.len() + 10),
                    title,
                    " ".repeat(8),
                    "─".repeat(title.len() + 10)
                )
            }
            WindowDecoration::None => String::new(),
        }
    }

    /// Generate SVG output
    pub fn to_svg(&self) -> String {
        let lines: Vec<&str> = self.content.lines().collect();
        let max_line_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);
        let char_width = 8;
        let line_height = 20;
        let padding = self.config.padding as usize * 10;

        let width = (max_line_len + 6) * char_width + padding * 2;
        let height = (lines.len() + 2) * line_height + padding * 2;

        let bg_color = match self.config.theme.background {
            Color::Rgb(r, g, b) => format!("rgb({},{},{})", r, g, b),
            _ => "rgb(30,30,46)".to_string(),
        };

        let fg_color = match self.config.theme.foreground {
            Color::Rgb(r, g, b) => format!("rgb({},{},{})", r, g, b),
            _ => "rgb(205,214,244)".to_string(),
        };

        let mut svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
  <defs>
    <filter id="shadow" x="-20%" y="-20%" width="140%" height="140%">
      <feDropShadow dx="0" dy="4" stdDeviation="8" flood-color="rgba(0,0,0,0.3)"/>
    </filter>
  </defs>
  <rect x="{}" y="{}" width="{}" height="{}" rx="{}" fill="{}" {}/>
"#,
            width,
            height,
            width,
            height,
            if self.config.shadow { 10 } else { 0 },
            if self.config.shadow { 10 } else { 0 },
            width - if self.config.shadow { 20 } else { 0 },
            height - if self.config.shadow { 20 } else { 0 },
            self.config.border_radius,
            bg_color,
            if self.config.shadow {
                r#"filter="url(#shadow)""#
            } else {
                ""
            }
        );

        // Window decorations
        if self.config.decorations == WindowDecoration::MacOS {
            let y = padding / 2 + 15;
            svg.push_str(&format!(
                r##"  <circle cx="{}" cy="{}" r="6" fill="#ff5f56"/>
  <circle cx="{}" cy="{}" r="6" fill="#ffbd2e"/>
  <circle cx="{}" cy="{}" r="6" fill="#27c93f"/>
"##,
                padding + 10,
                y,
                padding + 30,
                y,
                padding + 50,
                y
            ));
        }

        // Title
        if let Some(ref title) = self.config.title {
            let y = padding / 2 + 20;
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" font-family="monospace" font-size="14" fill="{}">{}</text>
"#,
                width / 2,
                y,
                fg_color,
                title
            ));
        }

        // Content
        let start_y = padding + 40;
        let line_num_width = lines.len().to_string().len();

        for (i, line) in lines.iter().enumerate() {
            let y = start_y + (i * line_height);

            // Line number
            if self.config.show_line_numbers {
                svg.push_str(&format!(
                    r#"  <text x="{}" y="{}" font-family="monospace" font-size="14" fill="rgb(88,91,112)">{:>width$}</text>
"#,
                    padding, y, i + 1, width = line_num_width
                ));
            }

            // Code content
            let x = padding + (line_num_width + 3) * char_width;
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" font-family="monospace" font-size="14" fill="{}">{}</text>
"#,
                x,
                y,
                fg_color,
                html_escape(line)
            ));
        }

        svg.push_str("</svg>");
        svg
    }

    /// Generate HTML output
    pub fn to_html(&self) -> String {
        let bg_color = match self.config.theme.background {
            Color::Rgb(r, g, b) => format!("rgb({},{},{})", r, g, b),
            _ => "rgb(30,30,46)".to_string(),
        };

        let fg_color = match self.config.theme.foreground {
            Color::Rgb(r, g, b) => format!("rgb({},{},{})", r, g, b),
            _ => "rgb(205,214,244)".to_string(),
        };

        let lines: Vec<&str> = self.content.lines().collect();
        let line_num_width = lines.len().to_string().len();

        let mut html = format!(
            r##"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>{}</title>
    <style>
        .code-window {{
            background: {};
            color: {};
            font-family: 'SF Mono', 'Fira Code', 'Monaco', monospace;
            font-size: 14px;
            border-radius: {}px;
            padding: {}px;
            {}
        }}
        .title-bar {{
            display: flex;
            align-items: center;
            padding: 10px;
            border-bottom: 1px solid rgba(255,255,255,0.1);
        }}
        .traffic-lights {{
            display: flex;
            gap: 8px;
        }}
        .traffic-light {{
            width: 12px;
            height: 12px;
            border-radius: 50%;
        }}
        .close {{ background: #ff5f56; }}
        .minimize {{ background: #ffbd2e; }}
        .maximize {{ background: #27c93f; }}
        .title {{
            flex: 1;
            text-align: center;
            color: rgba(255,255,255,0.6);
        }}
        .code {{
            padding: 15px;
            overflow-x: auto;
        }}
        .line {{
            display: flex;
            line-height: 1.6;
        }}
        .line-number {{
            color: rgba(255,255,255,0.3);
            padding-right: 15px;
            text-align: right;
            min-width: {}ch;
            user-select: none;
        }}
    </style>
</head>
<body style="margin: 40px; background: #1a1a2e;">
    <div class="code-window">
"##,
            self.config.title.as_deref().unwrap_or("Code"),
            bg_color,
            fg_color,
            self.config.border_radius,
            self.config.padding * 5,
            if self.config.shadow {
                "box-shadow: 0 10px 40px rgba(0,0,0,0.4);"
            } else {
                ""
            },
            line_num_width
        );

        // Title bar
        if self.config.decorations == WindowDecoration::MacOS {
            html.push_str(&format!(
                r#"        <div class="title-bar">
            <div class="traffic-lights">
                <div class="traffic-light close"></div>
                <div class="traffic-light minimize"></div>
                <div class="traffic-light maximize"></div>
            </div>
            <div class="title">{}</div>
        </div>
"#,
                self.config.title.as_deref().unwrap_or("")
            ));
        }

        // Code content
        html.push_str("        <div class=\"code\">\n");

        for (i, line) in lines.iter().enumerate() {
            html.push_str("            <div class=\"line\">\n");

            if self.config.show_line_numbers {
                html.push_str(&format!(
                    "                <span class=\"line-number\">{}</span>\n",
                    i + 1
                ));
            }

            html.push_str(&format!(
                "                <span class=\"code-content\">{}</span>\n",
                html_escape(line)
            ));

            html.push_str("            </div>\n");
        }

        html.push_str("        </div>\n");
        html.push_str("    </div>\n</body>\n</html>");

        html
    }
}

/// Escape HTML special characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace(' ', "&nbsp;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screenshot_config() {
        let config = ScreenshotConfig::default();
        assert!(config.show_line_numbers);
        assert!(config.shadow);
    }

    #[test]
    fn test_theme_catppuccin() {
        let theme = ScreenshotTheme::catppuccin_mocha();
        assert_eq!(theme.name, "Catppuccin Mocha");
    }

    #[test]
    fn test_screenshot_builder() {
        let screenshot = ScreenshotBuilder::new("fn main() {}")
            .title("main.rs")
            .language("rust")
            .theme(ScreenshotTheme::dracula());

        let ansi = screenshot.to_ansi();
        assert!(ansi.contains("fn main()"));
    }

    #[test]
    fn test_svg_output() {
        let screenshot = ScreenshotBuilder::new("let x = 42;").title("example.gul");

        let svg = screenshot.to_svg();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_html_output() {
        let screenshot = ScreenshotBuilder::new("print(\"Hello\")").title("hello.mn");

        let html = screenshot.to_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Hello"));
    }
}
