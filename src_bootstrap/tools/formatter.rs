// Code formatter - formats GUL source code
#![allow(dead_code)]

use std::path::Path;

pub struct Formatter {
    indent_size: usize,
    max_line_length: usize,
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter {
    pub fn new() -> Self {
        Formatter {
            indent_size: 4,
            max_line_length: 100,
        }
    }

    pub fn with_indent_size(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }

    pub fn with_max_line_length(mut self, length: usize) -> Self {
        self.max_line_length = length;
        self
    }

    pub fn format_file(&self, content: &str) -> String {
        // Simplified formatting - just ensure consistent spacing
        content
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn format_spacing(&self, code: &str) -> String {
        // Add spacing around operators
        code.replace("=", " = ")
            .replace("+", " + ")
            .replace("-", " - ")
            .replace("*", " * ")
            .replace("/", " / ")
            .replace("  ", " ") // Remove double spaces
    }

    pub fn format_comments(&self, code: &str) -> String {
        // Ensure comments have space after #
        code.lines()
            .map(|line| {
                if line.trim_start().starts_with('#') && !line.trim_start().starts_with("# ") {
                    line.replacen("#", "# ", 1)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// Public API function for CLI
pub fn format_file(file: &Path) -> Result<(), String> {
    let source =
        std::fs::read_to_string(file).map_err(|e| format!("Failed to read file: {}", e))?;

    let formatter = Formatter::new();
    let formatted = formatter.format_file(&source);

    std::fs::write(file, formatted).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatter_creation() {
        let formatter = Formatter::new();
        assert_eq!(formatter.indent_size, 4);
        assert_eq!(formatter.max_line_length, 100);
    }

    #[test]
    fn test_format_simple_code() {
        let formatter = Formatter::new();
        let code = "def x = 10\ndef y = 20";
        let formatted = formatter.format_file(code);
        assert!(formatted.contains("def x = 10"));
        assert!(formatted.contains("def y = 20"));
    }

    #[test]
    fn test_spacing() {
        let formatter = Formatter::new();
        let code = "x=10+20";
        let formatted = formatter.format_spacing(code);
        assert!(formatted.contains(" = "));
        assert!(formatted.contains(" + "));
    }

    #[test]
    fn test_comment_formatting() {
        let formatter = Formatter::new();
        let code = "#comment\n# good comment";
        let formatted = formatter.format_comments(code);
        assert!(formatted.contains("# comment"));
        assert!(formatted.contains("# good comment"));
    }
}
