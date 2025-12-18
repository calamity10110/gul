// GUL TUI IDE - Terminal User Interface IDE for GUL v2.1
// Built with Ratatui (formerly tui-rs)
// Supports v2.1 bracket equivalence and .mn file type

use std::io;
use std::path::{Path, PathBuf};

/// v2.1 Syntax highlighting token types
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // Core tokens
    Keyword,
    Identifier,
    Function,
    String,
    Number,
    Comment,
    Operator,

    // v2.1 Brackets (all equivalent semantically)
    BracketOpen,  // ( [ {
    BracketClose, // ) ] }

    // v2.1 UI Syntax
    UiMarker,    // ^&^
    UiComponent, // button, table, etc.
    UiProperty,  // text:, data:, etc.

    // v2.1 File Types
    FileTypeMain, // .mn
    FileTypeDef,  // .def
    FileTypeFnc,  // .fnc
    FileTypeCs,   // .cs
    FileTypeSct,  // .sct (secret credential)

    // Special
    Whitespace,
    Unknown,
}

/// v2.1 Bracket type for matching validation
#[derive(Clone, Debug, PartialEq)]
pub enum BracketType {
    Paren,  // ()
    Square, // []
    Curly,  // {}
}

/// Bracket pair for highlighting
#[derive(Clone, Debug)]
pub struct BracketPair {
    pub open_pos: usize,
    pub close_pos: usize,
    pub bracket_type: BracketType,
    pub matched: bool, // v2.1: true even if types differ but match logically
}

/// Syntax highlighting theme for GUL v2.1
#[derive(Clone, Debug)]
pub struct GulTheme {
    pub keyword_color: (u8, u8, u8),
    pub function_color: (u8, u8, u8),
    pub string_color: (u8, u8, u8),
    pub number_color: (u8, u8, u8),
    pub comment_color: (u8, u8, u8),
    pub ui_marker_color: (u8, u8, u8),  // ^&^ highlighting
    pub bracket_match_bg: (u8, u8, u8), // v2.1 bracket pairs
    pub file_type_color: (u8, u8, u8),  // .mn, .def, .fnc, .cs
}

impl Default for GulTheme {
    fn default() -> Self {
        GulTheme {
            keyword_color: (198, 120, 221),   // Purple
            function_color: (97, 175, 239),   // Blue
            string_color: (152, 195, 121),    // Green
            number_color: (229, 192, 123),    // Yellow
            comment_color: (92, 99, 112),     // Gray
            ui_marker_color: (86, 182, 194),  // Cyan
            bracket_match_bg: (62, 68, 81),   // Dark gray
            file_type_color: (152, 195, 121), // Light green
        }
    }
}

/// GUL v2.1 Syntax Highlighter
pub struct SyntaxHighlighter {
    #[allow(dead_code)]
    theme: GulTheme,
    keywords: Vec<&'static str>,
    ui_components: Vec<&'static str>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        SyntaxHighlighter {
            theme: GulTheme::default(),
            keywords: vec![
                "main", "fn", "async", "struct", "import", "const", "mut", "if", "elif", "else",
                "for", "while", "loop", "break", "continue", "return", "await", "try", "catch",
                "match", "extern", "pub",
            ],
            ui_components: vec![
                // Basic Input Widgets
                "button",
                "input",
                "textarea",
                "checkbox",
                "radio",
                "select",
                "slider",
                "toggle",
                // Display Widgets
                "label",
                "text",
                "bigtext",
                "paragraph",
                "sparkline",
                "gauge",
                "barchart",
                "canvas",
                // Container Widgets
                "container",
                "block",
                "row",
                "column",
                "grid",
                "stack",
                "split",
                "tabs",
                "scrollview",
                "popup",
                // Data Widgets
                "table",
                "list",
                "tree",
                "calendar",
                "chart",
                // Feedback Widgets
                "spinner",
                "throbber",
                "progress",
                "toast",
                "alert",
                "badge",
                // Navigation Widgets
                "menu",
                "menubar",
                "contextmenu",
                "breadcrumb",
                "pagination",
                // Media Widgets
                "image",
                "video",
                "audio",
                "markdown",
                "code",
                // Prompt Widgets
                "prompt",
            ],
        }
    }

    /// Tokenize a line of GUL code
    pub fn tokenize_line(&self, line: &str) -> Vec<(TokenType, String)> {
        let mut tokens = Vec::new();
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let ch = chars[i];

            // Comment
            if ch == '#' {
                let comment: String = chars[i..].iter().collect();
                tokens.push((TokenType::Comment, comment));
                break;
            }

            // UI Marker ^&^
            if ch == '^' && i + 2 < chars.len() && chars[i + 1] == '&' && chars[i + 2] == '^' {
                tokens.push((TokenType::UiMarker, "^&^".to_string()));
                i += 3;
                continue;
            }

            // v2.1: All brackets are equivalent
            if ch == '(' || ch == '[' || ch == '{' {
                tokens.push((TokenType::BracketOpen, ch.to_string()));
                i += 1;
                continue;
            }

            if ch == ')' || ch == ']' || ch == '}' {
                tokens.push((TokenType::BracketClose, ch.to_string()));
                i += 1;
                continue;
            }

            // String
            if ch == '"' {
                let mut string_val = String::new();
                string_val.push(ch);
                i += 1;
                while i < chars.len() && chars[i] != '"' {
                    if chars[i] == '\\' && i + 1 < chars.len() {
                        string_val.push(chars[i]);
                        i += 1;
                    }
                    string_val.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() {
                    string_val.push(chars[i]);
                    i += 1;
                }
                tokens.push((TokenType::String, string_val));
                continue;
            }

            // Number
            if ch.is_ascii_digit() {
                let mut num = String::new();
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    num.push(chars[i]);
                    i += 1;
                }
                tokens.push((TokenType::Number, num));
                continue;
            }

            // Identifier or keyword
            if ch.is_alphabetic() || ch == '_' {
                let mut ident = String::new();
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    ident.push(chars[i]);
                    i += 1;
                }

                if self.keywords.contains(&ident.as_str()) {
                    tokens.push((TokenType::Keyword, ident));
                } else if self.ui_components.contains(&ident.as_str()) {
                    tokens.push((TokenType::UiComponent, ident));
                } else {
                    // Check if followed by bracket (function call)
                    if i < chars.len() && (chars[i] == '(' || chars[i] == '[' || chars[i] == '{') {
                        tokens.push((TokenType::Function, ident));
                    } else {
                        tokens.push((TokenType::Identifier, ident));
                    }
                }
                continue;
            }

            // File type extensions
            if ch == '.' {
                let _ext_start = i;
                i += 1;
                let mut ext = String::from(".");
                while i < chars.len() && chars[i].is_alphanumeric() {
                    ext.push(chars[i]);
                    i += 1;
                }

                match ext.as_str() {
                    ".mn" => tokens.push((TokenType::FileTypeMain, ext)),
                    ".def" => tokens.push((TokenType::FileTypeDef, ext)),
                    ".fnc" => tokens.push((TokenType::FileTypeFnc, ext)),
                    ".cs" => tokens.push((TokenType::FileTypeCs, ext)),
                    ".sct" => tokens.push((TokenType::FileTypeSct, ext)),
                    _ => tokens.push((TokenType::Identifier, ext)),
                }
                continue;
            }

            // Operators
            if "+-*/%=<>!&|:.".contains(ch) {
                tokens.push((TokenType::Operator, ch.to_string()));
                i += 1;
                continue;
            }

            // Whitespace
            if ch.is_whitespace() {
                let mut ws = String::new();
                while i < chars.len() && chars[i].is_whitespace() {
                    ws.push(chars[i]);
                    i += 1;
                }
                tokens.push((TokenType::Whitespace, ws));
                continue;
            }

            // Unknown
            tokens.push((TokenType::Unknown, ch.to_string()));
            i += 1;
        }

        tokens
    }

    /// Find matching bracket pairs (v2.1: all bracket types can match)
    pub fn find_bracket_pairs(&self, content: &str) -> Vec<BracketPair> {
        let mut pairs = Vec::new();
        let mut stack: Vec<(usize, char)> = Vec::new();

        for (pos, ch) in content.chars().enumerate() {
            if ch == '(' || ch == '[' || ch == '{' {
                stack.push((pos, ch));
            } else if ch == ')' || ch == ']' || ch == '}' {
                if let Some((open_pos, open_ch)) = stack.pop() {
                    // v2.1: Brackets match if they're the same type
                    let matched = matches!((open_ch, ch), ('(', ')') | ('[', ']') | ('{', '}'));

                    let bracket_type = match open_ch {
                        '(' => BracketType::Paren,
                        '[' => BracketType::Square,
                        '{' => BracketType::Curly,
                        _ => BracketType::Paren,
                    };

                    pairs.push(BracketPair {
                        open_pos,
                        close_pos: pos,
                        bracket_type,
                        matched,
                    });
                }
            }
        }

        pairs
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

/// Main TUI IDE application state
pub struct GulTuiIde {
    /// Current file being edited
    current_file: Option<PathBuf>,
    /// Editor buffer content
    buffer: String,
    /// Cursor position (line, column)
    cursor: (usize, usize),
    /// File browser state
    file_browser: FileBrowser,
    /// Command palette state
    command_palette: CommandPalette,
    /// Status bar message
    status_message: String,
    /// Whether the IDE is running
    running: bool,
    /// Syntax highlighter for v2.1
    syntax_highlighter: SyntaxHighlighter,
    /// Bracket pairs in current buffer
    bracket_pairs: Vec<BracketPair>,
}

/// File browser component
pub struct FileBrowser {
    /// Current directory
    current_dir: PathBuf,
    /// List of files in current directory
    files: Vec<PathBuf>,
    /// Selected file index
    selected: usize,
    /// Whether the browser is visible
    visible: bool,
}

/// Command palette for quick actions
pub struct CommandPalette {
    /// Search query
    query: String,
    /// Available commands
    commands: Vec<Command>,
    /// Selected command index
    selected: usize,
    /// Whether the palette is visible
    visible: bool,
}

/// IDE command
#[derive(Clone, Debug)]
pub struct Command {
    /// Command name
    pub name: String,
    /// Command description
    pub description: String,
    /// Command action
    pub action: CommandAction,
}

/// Command actions
#[derive(Clone, Debug)]
pub enum CommandAction {
    /// Open file
    OpenFile,
    /// Save file
    SaveFile,
    /// Build project
    Build,
    /// Run project
    Run,
    /// Format code
    Format,
    /// Show Git status
    GitStatus,
    /// Quit IDE
    Quit,
}

impl GulTuiIde {
    /// Create a new TUI IDE instance
    pub fn new() -> Self {
        GulTuiIde {
            current_file: None,
            buffer: String::new(),
            cursor: (0, 0),
            file_browser: FileBrowser::new(),
            command_palette: CommandPalette::new(),
            status_message: "Welcome to GUL TUI IDE v2.1".to_string(),
            running: true,
            syntax_highlighter: SyntaxHighlighter::new(),
            bracket_pairs: Vec::new(),
        }
    }

    /// Run the IDE main loop
    pub fn run(&mut self) -> io::Result<()> {
        // Initialize terminal
        self.status_message = "TUI IDE initialized. Press Ctrl+P for commands.".to_string();

        // Main event loop would go here
        // For now, this is a placeholder

        Ok(())
    }

    /// Open a file
    pub fn open_file(&mut self, path: PathBuf) -> io::Result<()> {
        self.buffer = std::fs::read_to_string(&path)?;
        self.current_file = Some(path);
        self.cursor = (0, 0);
        self.update_bracket_pairs();

        // Check file type
        let file_type = self.get_file_type();
        self.status_message = format!("Opened: {:?} ({})", self.current_file, file_type);
        Ok(())
    }

    /// Get file type description for v2.1
    fn get_file_type(&self) -> &'static str {
        if let Some(ref path) = self.current_file {
            match path.extension().and_then(|s| s.to_str()) {
                Some("mn") => "Main",
                Some("def") => "Definition",
                Some("fnc") => "Function",
                Some("cs") => "Cross-Script",
                Some("sct") => "Secret Credential",
                Some("gul") => "Legacy GUL",
                _ => "Unknown",
            }
        } else {
            "No file"
        }
    }

    /// Update bracket pairs after buffer change
    fn update_bracket_pairs(&mut self) {
        self.bracket_pairs = self.syntax_highlighter.find_bracket_pairs(&self.buffer);
    }

    /// Get syntax-highlighted tokens for a line
    pub fn get_highlighted_line(&self, line_num: usize) -> Vec<(TokenType, String)> {
        if let Some(line) = self.buffer.lines().nth(line_num) {
            self.syntax_highlighter.tokenize_line(line)
        } else {
            Vec::new()
        }
    }

    /// Find bracket pair at cursor position
    pub fn get_bracket_pair_at_cursor(&self) -> Option<&BracketPair> {
        let cursor_offset = self.get_cursor_offset();

        self.bracket_pairs
            .iter()
            .find(|pair| pair.open_pos == cursor_offset || pair.close_pos == cursor_offset)
    }

    /// Get cursor offset in buffer
    fn get_cursor_offset(&self) -> usize {
        let (line, col) = self.cursor;
        let mut offset = 0;

        for (i, l) in self.buffer.lines().enumerate() {
            if i == line {
                return offset + col.min(l.len());
            }
            offset += l.len() + 1; // +1 for newline
        }

        offset
    }

    /// Save current file
    pub fn save_file(&mut self) -> io::Result<()> {
        if let Some(ref path) = self.current_file {
            std::fs::write(path, &self.buffer)?;
            self.status_message = format!("Saved: {:?}", path);
        }
        Ok(())
    }

    /// Build the current project
    pub fn build(&mut self) -> io::Result<()> {
        self.status_message = "Building project...".to_string();
        // Integration with compiler would go here
        Ok(())
    }

    /// Run the current project
    pub fn run_project(&mut self) -> io::Result<()> {
        self.status_message = "Running project...".to_string();
        // Integration with runtime would go here
        Ok(())
    }

    /// Format current file
    pub fn format(&mut self) -> io::Result<()> {
        self.status_message = "Formatting code...".to_string();
        // Integration with formatter would go here
        Ok(())
    }

    /// Toggle command palette
    pub fn toggle_command_palette(&mut self) {
        self.command_palette.visible = !self.command_palette.visible;
    }

    /// Toggle file browser
    pub fn toggle_file_browser(&mut self) {
        self.file_browser.visible = !self.file_browser.visible;
    }

    /// Quit the IDE
    pub fn quit(&mut self) {
        self.running = false;
    }
}

impl FileBrowser {
    /// Create a new file browser
    pub fn new() -> Self {
        FileBrowser {
            current_dir: std::env::current_dir().unwrap_or_default(),
            files: Vec::new(),
            selected: 0,
            visible: false,
        }
    }

    /// Refresh file list
    pub fn refresh(&mut self) -> io::Result<()> {
        self.files.clear();
        for entry in std::fs::read_dir(&self.current_dir)? {
            let entry = entry?;
            self.files.push(entry.path());
        }
        // Sort: directories first, then by name
        self.files.sort_by(|a, b| match (a.is_dir(), b.is_dir()) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        });
        Ok(())
    }

    /// Get file type icon for v2.1 file types
    pub fn get_file_icon(&self, path: &Path) -> &'static str {
        if path.is_dir() {
            return "📁";
        }

        match path.extension().and_then(|s| s.to_str()) {
            Some("mn") => "📄",   // Main file
            Some("def") => "📋",  // Definition
            Some("fnc") => "⚙️",  // Function
            Some("cs") => "🔗",   // Cross-script
            Some("sct") => "🔐",  // Secret credential
            Some("gul") => "📜",  // Legacy
            Some("md") => "📝",   // Markdown
            Some("toml") => "⚙️", // Config
            Some("json") => "📦", // JSON
            _ => "📄",
        }
    }

    /// Navigate up one directory
    pub fn navigate_up(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            let _ = self.refresh();
        }
    }

    /// Navigate into selected directory
    pub fn navigate_into(&mut self) {
        if let Some(path) = self.files.get(self.selected) {
            if path.is_dir() {
                self.current_dir = path.clone();
                let _ = self.refresh();
            }
        }
    }

    /// Get selected file
    pub fn get_selected(&self) -> Option<&PathBuf> {
        self.files.get(self.selected)
    }
}

impl CommandPalette {
    /// Create a new command palette
    pub fn new() -> Self {
        let commands = vec![
            Command {
                name: "Open File".to_string(),
                description: "Open a file for editing".to_string(),
                action: CommandAction::OpenFile,
            },
            Command {
                name: "Save File".to_string(),
                description: "Save the current file".to_string(),
                action: CommandAction::SaveFile,
            },
            Command {
                name: "Build".to_string(),
                description: "Build the current project".to_string(),
                action: CommandAction::Build,
            },
            Command {
                name: "Run".to_string(),
                description: "Run the current project".to_string(),
                action: CommandAction::Run,
            },
            Command {
                name: "Format".to_string(),
                description: "Format the current file".to_string(),
                action: CommandAction::Format,
            },
            Command {
                name: "Git Status".to_string(),
                description: "Show Git status".to_string(),
                action: CommandAction::GitStatus,
            },
            Command {
                name: "Quit".to_string(),
                description: "Exit the IDE".to_string(),
                action: CommandAction::Quit,
            },
        ];

        CommandPalette {
            query: String::new(),
            commands,
            selected: 0,
            visible: false,
        }
    }

    /// Filter commands by query
    pub fn filter_commands(&self) -> Vec<&Command> {
        if self.query.is_empty() {
            self.commands.iter().collect()
        } else {
            self.commands
                .iter()
                .filter(|cmd| {
                    cmd.name.to_lowercase().contains(&self.query.to_lowercase())
                        || cmd
                            .description
                            .to_lowercase()
                            .contains(&self.query.to_lowercase())
                })
                .collect()
        }
    }

    /// Get selected command
    pub fn get_selected(&self) -> Option<&Command> {
        let filtered = self.filter_commands();
        filtered.get(self.selected).copied()
    }
}

impl Default for GulTuiIde {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FileBrowser {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ide_creation() {
        let ide = GulTuiIde::new();
        assert!(ide.running);
        assert!(ide.current_file.is_none());
    }

    #[test]
    fn test_command_palette() {
        let palette = CommandPalette::new();
        assert!(!palette.visible);
        assert_eq!(palette.commands.len(), 7);
    }

    #[test]
    fn test_file_browser() {
        let browser = FileBrowser::new();
        assert!(!browser.visible);
    }

    #[test]
    fn test_syntax_highlighter_keywords() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.tokenize_line("main:");

        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].0, TokenType::Keyword);
        assert_eq!(tokens[0].1, "main");
    }

    #[test]
    fn test_syntax_highlighter_brackets() {
        let highlighter = SyntaxHighlighter::new();

        // v2.1: All bracket types are tokenized as BracketOpen/BracketClose
        let tokens = highlighter.tokenize_line("print(x) print[x] print{x}");

        let open_brackets: Vec<_> = tokens
            .iter()
            .filter(|(t, _)| *t == TokenType::BracketOpen)
            .collect();
        let close_brackets: Vec<_> = tokens
            .iter()
            .filter(|(t, _)| *t == TokenType::BracketClose)
            .collect();

        assert_eq!(open_brackets.len(), 3);
        assert_eq!(close_brackets.len(), 3);
    }

    #[test]
    fn test_syntax_highlighter_ui_marker() {
        let highlighter = SyntaxHighlighter::new();
        let tokens = highlighter.tokenize_line("^&^[button{text: \"Click\"}]");

        assert_eq!(tokens[0].0, TokenType::UiMarker);
        assert_eq!(tokens[0].1, "^&^");
    }

    #[test]
    fn test_bracket_pair_matching() {
        let highlighter = SyntaxHighlighter::new();

        // v2.1: Matching pairs
        let pairs = highlighter.find_bracket_pairs("(a + b)");
        assert_eq!(pairs.len(), 1);
        assert!(pairs[0].matched);

        // v2.1: Mismatched brackets
        let pairs = highlighter.find_bracket_pairs("(a + b]");
        assert_eq!(pairs.len(), 1);
        assert!(!pairs[0].matched);
    }

    #[test]
    fn test_file_type_icons() {
        let browser = FileBrowser::new();

        assert_eq!(browser.get_file_icon(&PathBuf::from("main.mn")), "📄");
        assert_eq!(browser.get_file_icon(&PathBuf::from("types.def")), "📋");
        assert_eq!(browser.get_file_icon(&PathBuf::from("utils.fnc")), "⚙️");
        assert_eq!(browser.get_file_icon(&PathBuf::from("extern.cs")), "🔗");
        assert_eq!(browser.get_file_icon(&PathBuf::from("secrets.sct")), "🔐");
    }
}
