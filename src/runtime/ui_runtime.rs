#![allow(dead_code)]
// UI runtime - renders inline UI components (TUI)
use crossterm::{
    execute,
    style::{
        Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
    terminal::{Clear, ClearType},
};
use std::io::{stdout, Write};
use unicode_width::UnicodeWidthStr;

/// UI Component types for TUI rendering
#[derive(Debug, Clone)]
pub enum UiComponent {
    Tree {
        nodes: Vec<TreeNode>,
    },
    Slider {
        min: i32,
        max: i32,
        value: i32,
        label: Option<String>,
    },
    Button {
        text: String,
    },
    Text {
        content: String,
        fg: Option<String>,
        bg: Option<String>,
        bold: bool,
    },
    Progress {
        value: i32,
        max: i32,
        label: Option<String>,
    },
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    Canvas {
        width: usize,
        height: usize,
        pixels: Vec<Vec<char>>,
    },
    Input {
        prompt: String,
    },
    Menu {
        title: String,
        options: Vec<String>,
        selected: usize,
    },
    VBox {
        children: Vec<UiComponent>,
    },
    HBox {
        children: Vec<UiComponent>,
    },
}

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub children: Vec<TreeNode>,
}

pub struct UiRuntime {
    components: Vec<UiComponent>,
}

impl Default for UiRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl UiRuntime {
    pub fn new() -> Self {
        UiRuntime {
            components: Vec::new(),
        }
    }

    pub fn print(&mut self, component: UiComponent) {
        self.render(&component);
        self.components.push(component);
    }

    pub fn clear(&self) {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All)).ok();
    }

    fn render(&self, component: &UiComponent) {
        match component {
            UiComponent::Tree { nodes } => self.render_tree(nodes, 0, ""),
            UiComponent::Slider {
                min,
                max,
                value,
                label,
            } => self.render_slider(*min, *max, *value, label),
            UiComponent::Button { text } => self.render_button(text),
            UiComponent::Text {
                content,
                fg,
                bg,
                bold,
            } => self.render_text(content, fg, bg, *bold),
            UiComponent::Progress { value, max, label } => {
                self.render_progress(*value, *max, label)
            }
            UiComponent::Table { headers, rows } => self.render_table(headers, rows),
            UiComponent::Canvas {
                width,
                height,
                pixels,
            } => self.render_canvas(*width, *height, pixels),
            UiComponent::Input { prompt } => self.render_input(prompt),
            UiComponent::Menu {
                title,
                options,
                selected,
            } => self.render_menu(title, options, *selected),
            UiComponent::VBox { children } => self.render_vbox(children),
            UiComponent::HBox { children } => self.render_hbox(children),
        }
    }
    #[allow(clippy::only_used_in_recursion)]
    fn render_tree(&self, nodes: &[TreeNode], depth: usize, prefix: &str) {
        for (i, node) in nodes.iter().enumerate() {
            let is_last = i == nodes.len() - 1;
            let connector = if is_last { "└── " } else { "├── " };
            println!("{}{}{}", prefix, connector, node.name);

            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            self.render_tree(&node.children, depth + 1, &new_prefix);
        }
    }

    fn render_slider(&self, min: i32, max: i32, value: i32, label: &Option<String>) {
        let range = max - min;
        let normalized = ((value - min) as f32 / range as f32 * 40.0) as usize;

        let filled = "=".repeat(normalized);
        let empty = " ".repeat(40 - normalized);

        if let Some(lbl) = label {
            println!("{}: [{}|{}] {}/{}", lbl, filled, empty, value, max);
        } else {
            println!("[{}|{}] {}/{}", filled, empty, value, max);
        }
    }

    fn render_button(&self, text: &str) {
        let width = text.width() + 2;
        let top = format!("┌{}┐", "─".repeat(width));
        let middle = format!("│ {} │", text);
        let bottom = format!("└{}┘", "─".repeat(width));

        println!("{}", top);
        println!("{}", middle);
        println!("{}", bottom);
    }

    fn render_text(&self, content: &str, fg: &Option<String>, bg: &Option<String>, bold: bool) {
        let mut stdout = stdout();

        if let Some(fg_color) = fg {
            let color = self.parse_color(fg_color);
            execute!(stdout, SetForegroundColor(color)).ok();
        }

        if let Some(bg_color) = bg {
            let color = self.parse_color(bg_color);
            execute!(stdout, SetBackgroundColor(color)).ok();
        }

        if bold {
            execute!(stdout, SetAttribute(Attribute::Bold)).ok();
        }

        execute!(stdout, Print(content), Print("\n"), ResetColor).ok();
    }

    fn render_progress(&self, value: i32, max: i32, label: &Option<String>) {
        let percentage = (value as f32 / max as f32 * 100.0) as usize;
        let filled = (value as f32 / max as f32 * 60.0) as usize;
        let empty = 60 - filled;

        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));

        if let Some(lbl) = label {
            println!("{}: [{}] {}%", lbl, bar, percentage);
        } else {
            println!("Progress: [{}] {}%", bar, percentage);
        }
    }

    fn render_table(&self, headers: &[String], rows: &[Vec<String>]) {
        // Calculate column widths
        let mut col_widths: Vec<usize> = headers.iter().map(|h| h.width()).collect();

        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    col_widths[i] = col_widths[i].max(cell.width());
                }
            }
        }

        // Top border
        print!("┌");
        for (i, width) in col_widths.iter().enumerate() {
            print!("{}", "─".repeat(width + 2));
            if i < col_widths.len() - 1 {
                print!("┬");
            }
        }
        println!("┐");

        // Headers
        print!("│");
        for (i, header) in headers.iter().enumerate() {
            print!(" {:<width$} ", header, width = col_widths[i]);
            print!("│");
        }
        println!();

        // Header separator
        print!("├");
        for (i, width) in col_widths.iter().enumerate() {
            print!("{}", "─".repeat(width + 2));
            if i < col_widths.len() - 1 {
                print!("┼");
            }
        }
        println!("┤");

        // Rows
        for row in rows {
            print!("│");
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    print!(" {:<width$} ", cell, width = col_widths[i]);
                    print!("│");
                }
            }
            println!();
        }

        // Bottom border
        print!("└");
        for (i, width) in col_widths.iter().enumerate() {
            print!("{}", "─".repeat(width + 2));
            if i < col_widths.len() - 1 {
                print!("┴");
            }
        }
        println!("┘");
    }

    fn render_canvas(&self, width: usize, height: usize, pixels: &[Vec<char>]) {
        for y in 0..height {
            for x in 0..width {
                if y < pixels.len() && x < pixels[y].len() {
                    print!("{}", pixels[y][x]);
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn render_input(&self, prompt: &str) {
        print!("{}", prompt);
        stdout().flush().ok();
    }

    fn render_menu(&self, title: &str, options: &[String], selected: usize) {
        println!("{}", title);
        for (i, option) in options.iter().enumerate() {
            if i == selected {
                println!("  > {}", option);
            } else {
                println!("    {}", option);
            }
        }
    }

    fn render_vbox(&self, children: &[UiComponent]) {
        for child in children {
            self.render(child);
        }
    }

    fn render_hbox(&self, children: &[UiComponent]) {
        // Simple horizontal layout - render side by side
        // For simplicity, we'll just render them on the same line
        for child in children {
            self.render(child);
        }
    }

    fn parse_color(&self, color_name: &str) -> Color {
        match color_name.to_lowercase().as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "grey" | "gray" => Color::Grey,
            _ => Color::White,
        }
    }

    // Interactive input
    pub fn input(&self, prompt: &str) -> String {
        use std::io::{stdin, BufRead};

        print!("{}", prompt);
        stdout().flush().ok();

        let stdin = stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line).ok();
        line.trim().to_string()
    }

    // Menu selection
    pub fn select(&self, title: &str, options: &[String]) -> usize {
        println!("{}", title);
        for (i, option) in options.iter().enumerate() {
            println!("  {}. {}", i + 1, option);
        }

        print!("Select (1-{}): ", options.len());
        stdout().flush().ok();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();

        input
            .trim()
            .parse::<usize>()
            .unwrap_or(1)
            .saturating_sub(1)
            .min(options.len() - 1)
    }
}

// Canvas helper for drawing
impl UiComponent {
    pub fn new_canvas(width: usize, height: usize) -> Self {
        UiComponent::Canvas {
            width,
            height,
            pixels: vec![vec![' '; width]; height],
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        if let UiComponent::Canvas {
            width,
            height,
            pixels,
        } = self
        {
            // Bresenham's line algorithm
            let dx = (x1 - x0).abs();
            let dy = (y1 - y0).abs();
            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };
            let mut err = dx - dy;
            let mut x = x0;
            let mut y = y0;

            loop {
                if x >= 0 && x < *width as i32 && y >= 0 && y < *height as i32 {
                    pixels[y as usize][x as usize] = '*';
                }

                if x == x1 && y == y1 {
                    break;
                }

                let e2 = 2 * err;
                if e2 > -dy {
                    err -= dy;
                    x += sx;
                }
                if e2 < dx {
                    err += dx;
                    y += sy;
                }
            }
        }
    }

    pub fn draw_rect(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        if let UiComponent::Canvas {
            width,
            height,
            pixels,
        } = self
        {
            // Draw rectangle borders
            for x in x0..=x1 {
                if x >= 0 && x < *width as i32 {
                    if y0 >= 0 && y0 < *height as i32 {
                        pixels[y0 as usize][x as usize] = if x == x0 {
                            '┌'
                        } else if x == x1 {
                            '┐'
                        } else {
                            '─'
                        };
                    }
                    if y1 >= 0 && y1 < *height as i32 {
                        pixels[y1 as usize][x as usize] = if x == x0 {
                            '└'
                        } else if x == x1 {
                            '┘'
                        } else {
                            '─'
                        };
                    }
                }
            }

            for y in (y0 + 1)..y1 {
                if y >= 0 && y < *height as i32 {
                    if x0 >= 0 && x0 < *width as i32 {
                        pixels[y as usize][x0 as usize] = '│';
                    }
                    if x1 >= 0 && x1 < *width as i32 {
                        pixels[y as usize][x1 as usize] = '│';
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_runtime_creation() {
        let runtime = UiRuntime::new();
        assert_eq!(runtime.components.len(), 0);
    }

    #[test]
    fn test_slider_component() {
        let mut runtime = UiRuntime::new();
        let slider = UiComponent::Slider {
            min: 0,
            max: 100,
            value: 50,
            label: Some("Volume".to_string()),
        };
        runtime.print(slider);
        assert_eq!(runtime.components.len(), 1);
    }

    #[test]
    fn test_button_component() {
        let mut runtime = UiRuntime::new();
        let button = UiComponent::Button {
            text: "Click Me".to_string(),
        };
        runtime.print(button);
        assert_eq!(runtime.components.len(), 1);
    }

    #[test]
    fn test_table_component() {
        let mut runtime = UiRuntime::new();
        let table = UiComponent::Table {
            headers: vec!["Name".to_string(), "Age".to_string()],
            rows: vec![
                vec!["Alice".to_string(), "30".to_string()],
                vec!["Bob".to_string(), "25".to_string()],
            ],
        };
        runtime.print(table);
        assert_eq!(runtime.components.len(), 1);
    }

    #[test]
    fn test_canvas_drawing() {
        let mut canvas = UiComponent::new_canvas(40, 10);
        canvas.draw_line(0, 0, 39, 9);
        canvas.draw_rect(10, 2, 30, 8);

        if let UiComponent::Canvas { pixels, .. } = canvas {
            assert_eq!(pixels[0][0], '*');
        }
    }

    #[test]
    fn test_progress_bar() {
        let mut runtime = UiRuntime::new();
        let progress = UiComponent::Progress {
            value: 75,
            max: 100,
            label: Some("Loading".to_string()),
        };
        runtime.print(progress);
        assert_eq!(runtime.components.len(), 1);
    }
}
