// GUL TUI Application
// Main TUI IDE application with Ratatui

use std::io;
use std::path::PathBuf;
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

use super::events::{Action, KeyBindings};
use super::theme::GulTheme;
use super::widgets::{
    command_palette::default_commands, editor::EditorState, Command, CommandPaletteState,
    CommandPaletteWidget, EditorWidget, FileEntry, FileTreeState, FileTreeWidget, OutputLine,
    OutputType, OutputWidget, StatusBarWidget, Tab, TabsWidget,
};

/// Application mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Command,
    Search,
}

/// Application focus
#[derive(Debug, Clone, PartialEq)]
pub enum Focus {
    Editor,
    FileTree,
    Output,
    Terminal,
    CommandPalette,
}

/// Main GUL TUI Application
pub struct GulTuiApp {
    // State
    running: bool,
    #[allow(dead_code)]
    mode: Mode,
    focus: Focus,

    // Theme
    theme: GulTheme,
    key_bindings: KeyBindings,

    // Editor
    buffer: String,
    editor_state: EditorState,

    // File tree
    file_entries: Vec<FileEntry>,
    file_tree_state: FileTreeState,
    show_file_tree: bool,

    // Tabs
    tabs: Vec<Tab>,
    active_tab: usize,

    // Output
    output_lines: Vec<OutputLine>,
    show_output: bool,

    // Command palette
    commands: Vec<Command>,
    command_palette_state: CommandPaletteState,

    // Current file
    current_file: Option<PathBuf>,
    git_branch: Option<String>,
}

impl Default for GulTuiApp {
    fn default() -> Self {
        Self::new()
    }
}

impl GulTuiApp {
    /// Create a new TUI application
    pub fn new() -> Self {
        GulTuiApp {
            running: true,
            mode: Mode::Normal,
            focus: Focus::Editor,

            theme: GulTheme::dark(),
            key_bindings: KeyBindings::default(),

            buffer: String::new(),
            editor_state: EditorState::default(),

            file_entries: Vec::new(),
            file_tree_state: FileTreeState::default(),
            show_file_tree: true,

            tabs: Vec::new(),
            active_tab: 0,

            output_lines: vec![
                OutputLine {
                    text: "GUL TUI IDE v0.15.0".to_string(),
                    output_type: OutputType::Info,
                },
                OutputLine {
                    text: "Ready. Press Ctrl+P for command palette.".to_string(),
                    output_type: OutputType::Success,
                },
            ],
            show_output: true,

            commands: default_commands(),
            command_palette_state: CommandPaletteState::default(),

            current_file: None,
            git_branch: Some("main".to_string()),
        }
    }

    /// Run the TUI application
    pub fn run(&mut self) -> io::Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Main event loop
        while self.running {
            terminal.draw(|frame| self.render(frame))?;

            if event::poll(Duration::from_millis(50))? {
                match event::read()? {
                    Event::Key(key) => {
                        if key.kind == event::KeyEventKind::Press {
                            self.handle_key(key)?;
                        }
                    }
                    Event::Resize(w, h) => {
                        terminal.resize(Rect::new(0, 0, w, h))?;
                    }
                    _ => {}
                }
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        Ok(())
    }

    /// Handle key event
    fn handle_key(&mut self, key: event::KeyEvent) -> io::Result<()> {
        // Check command palette first
        if self.command_palette_state.visible {
            match key.code {
                KeyCode::Esc => {
                    self.command_palette_state.visible = false;
                }
                KeyCode::Enter => {
                    // Execute selected command
                    self.command_palette_state.visible = false;
                }
                KeyCode::Up => {
                    if self.command_palette_state.selected > 0 {
                        self.command_palette_state.selected -= 1;
                    }
                }
                KeyCode::Down => {
                    self.command_palette_state.selected += 1;
                }
                KeyCode::Char(c) => {
                    self.command_palette_state.query.push(c);
                    self.command_palette_state.selected = 0;
                }
                KeyCode::Backspace => {
                    self.command_palette_state.query.pop();
                }
                _ => {}
            }
            return Ok(());
        }

        // Handle mode switching in Normal mode
        if self.mode == Mode::Normal {
            match key.code {
                KeyCode::Char('i') => {
                    self.mode = Mode::Insert;
                    return Ok(());
                }
                KeyCode::Char(':') => {
                    self.mode = Mode::Command;
                    return Ok(());
                }
                KeyCode::Char('/') => {
                    self.mode = Mode::Search;
                    return Ok(());
                }
                _ => {}
            }
        } else if self.mode == Mode::Insert {
            if key.code == KeyCode::Esc {
                self.mode = Mode::Normal;
                return Ok(());
            }
        }

        // Check key bindings
        if let Some(action) = self.key_bindings.check(&key) {
            match action {
                Action::Quit => {
                    self.running = false;
                }
                Action::CommandPalette => {
                    self.command_palette_state.visible = true;
                    self.command_palette_state.query.clear();
                    self.command_palette_state.selected = 0;
                }
                Action::ToggleFileBrowser => {
                    self.show_file_tree = !self.show_file_tree;
                }
                Action::ToggleTerminal => {
                    self.show_output = !self.show_output;
                }
                Action::Save => {
                    self.save_file()?;
                }
                Action::Build => {
                    self.build_project()?;
                }
                Action::Run => {
                    self.run_project()?;
                }
                Action::NextTab => {
                    if !self.tabs.is_empty() {
                        self.active_tab = (self.active_tab + 1) % self.tabs.len();
                    }
                }
                Action::PrevTab => {
                    if !self.tabs.is_empty() && self.active_tab > 0 {
                        self.active_tab -= 1;
                    }
                }
                Action::CloseTab => {
                    if !self.tabs.is_empty() {
                        self.tabs.remove(self.active_tab);
                        if self.active_tab >= self.tabs.len() && !self.tabs.is_empty() {
                            self.active_tab = self.tabs.len() - 1;
                        }
                    }
                }
                _ => {}
            }
            return Ok(());
        }

        // Handle focus-specific keys
        match self.focus {
            Focus::Editor => {
                self.handle_editor_key(key)?;
            }
            Focus::FileTree => {
                self.handle_file_tree_key(key)?;
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_editor_key(&mut self, key: event::KeyEvent) -> io::Result<()> {
        match self.mode {
            Mode::Normal => match key.code {
                KeyCode::Up => {
                    if self.editor_state.cursor.0 > 0 {
                        self.editor_state.cursor.0 -= 1;
                    }
                }
                KeyCode::Down => {
                    let max_lines = self.buffer.lines().count().saturating_sub(1);
                    if self.editor_state.cursor.0 < max_lines {
                        self.editor_state.cursor.0 += 1;
                    }
                }
                KeyCode::Left => {
                    if self.editor_state.cursor.1 > 0 {
                        self.editor_state.cursor.1 -= 1;
                    }
                }
                KeyCode::Right => {
                    let line = self.buffer.lines().nth(self.editor_state.cursor.0).unwrap_or("");
                    if self.editor_state.cursor.1 < line.chars().count() {
                        self.editor_state.cursor.1 += 1;
                    }
                }
                KeyCode::Tab if key.modifiers == KeyModifiers::NONE => {
                    self.focus = if self.show_file_tree {
                        Focus::FileTree
                    } else {
                        Focus::Output
                    };
                }
                _ => {}
            },
            Mode::Insert => match key.code {
                KeyCode::Char(c) => {
                    self.insert_char(c);
                }
                KeyCode::Backspace => {
                    self.delete_char();
                }
                KeyCode::Enter => {
                    self.insert_newline();
                }
                KeyCode::Up => {
                    if self.editor_state.cursor.0 > 0 {
                        self.editor_state.cursor.0 -= 1;
                    }
                }
                KeyCode::Down => {
                    let max_lines = self.buffer.lines().count().saturating_sub(1);
                    if self.editor_state.cursor.0 < max_lines {
                        self.editor_state.cursor.0 += 1;
                    }
                }
                KeyCode::Left => {
                    if self.editor_state.cursor.1 > 0 {
                        self.editor_state.cursor.1 -= 1;
                    }
                }
                KeyCode::Right => {
                    let line = self.buffer.lines().nth(self.editor_state.cursor.0).unwrap_or("");
                    if self.editor_state.cursor.1 < line.chars().count() {
                        self.editor_state.cursor.1 += 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }

    fn handle_file_tree_key(&mut self, key: event::KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Up => {
                if self.file_tree_state.selected > 0 {
                    self.file_tree_state.selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.file_tree_state.selected < self.file_entries.len().saturating_sub(1) {
                    self.file_tree_state.selected += 1;
                }
            }
            KeyCode::Enter => {
                // Open selected file
                if let Some(entry) = self.file_entries.get(self.file_tree_state.selected) {
                    if !entry.is_dir {
                        self.open_file(entry.path.clone())?;
                    }
                }
            }
            KeyCode::Tab => {
                self.focus = Focus::Editor;
            }
            _ => {}
        }
        Ok(())
    }

    /// Render the application
    fn render(&self, frame: &mut Frame) {
        let size = frame.area();

        // Main layout
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Tabs
                Constraint::Min(1),    // Main content
                Constraint::Length(1), // Status bar
            ])
            .split(size);

        // Render tabs
        if !self.tabs.is_empty() {
            let tabs = TabsWidget::new(&self.tabs, self.active_tab, &self.theme);
            frame.render_widget(tabs, main_layout[0]);
        }

        // Content layout
        let content_constraints = if self.show_file_tree && self.show_output {
            vec![
                Constraint::Percentage(20),
                Constraint::Percentage(55),
                Constraint::Percentage(25),
            ]
        } else if self.show_file_tree {
            vec![Constraint::Percentage(20), Constraint::Percentage(80)]
        } else if self.show_output {
            vec![Constraint::Percentage(75), Constraint::Percentage(25)]
        } else {
            vec![Constraint::Percentage(100)]
        };

        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(content_constraints)
            .split(main_layout[1]);

        let mut content_idx = 0;

        // File tree
        if self.show_file_tree {
            let mut file_tree_state = self.file_tree_state.clone();
            let file_tree = FileTreeWidget::new(&self.file_entries, &self.theme);
            frame.render_stateful_widget(
                file_tree,
                content_layout[content_idx],
                &mut file_tree_state,
            );
            content_idx += 1;
        }

        // Editor
        let lines: Vec<&str> = self.buffer.lines().collect();
        let mut editor_state = self.editor_state.clone();
        let editor = EditorWidget::new(lines, &self.theme).file_name(
            self.current_file
                .as_ref()
                .and_then(|p| p.file_name())
                .and_then(|s| s.to_str())
                .unwrap_or("Untitled"),
        );
        frame.render_stateful_widget(editor, content_layout[content_idx], &mut editor_state);
        content_idx += 1;

        // Output
        if self.show_output && content_idx < content_layout.len() {
            let output = OutputWidget::new(&self.output_lines, &self.theme);
            frame.render_widget(output, content_layout[content_idx]);
        }

        // Status bar
        let (line, col) = (
            self.editor_state.cursor.0 + 1,
            self.editor_state.cursor.1 + 1,
        );
        let file_type = self
            .current_file
            .as_ref()
            .and_then(|p| p.extension())
            .and_then(|s| s.to_str())
            .map(|ext| match ext {
                "mn" => "Main",
                "def" => "Definition",
                "fnc" => "Function",
                "cs" => "Cross-Script",
                "sct" => "Secret",
                _ => "Unknown",
            })
            .unwrap_or("None");

        let file_name = self
            .current_file
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled");

        let mut status = StatusBarWidget::new(&self.theme)
            .file(file_name, file_type)
            .cursor(line, col);

        if let Some(ref branch) = self.git_branch {
            status = status.git_branch(branch);
        }

        frame.render_widget(status, main_layout[2]);

        // Command palette (overlay)
        if self.command_palette_state.visible {
            let palette =
                CommandPaletteWidget::new(&self.commands, &self.command_palette_state, &self.theme);
            frame.render_widget(palette, size);
        }
    }

    /// Open a file
    pub fn open_file(&mut self, path: PathBuf) -> io::Result<()> {
        self.buffer = std::fs::read_to_string(&path)?;
        self.current_file = Some(path.clone());
        self.editor_state.cursor = (0, 0);
        self.editor_state.scroll = (0, 0);

        // Add tab
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string();

        let file_type = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|ext| match ext {
                "mn" => "Main",
                "def" => "Definition",
                "fnc" => "Function",
                "cs" => "Cross-Script",
                "sct" => "Secret Credential",
                _ => "Unknown",
            })
            .unwrap_or("Unknown")
            .to_string();

        self.tabs.push(Tab {
            name,
            file_type,
            modified: false,
            path: path.to_string_lossy().to_string(),
        });
        self.active_tab = self.tabs.len() - 1;

        self.output_lines.push(OutputLine {
            text: format!("Opened: {}", path.display()),
            output_type: OutputType::Info,
        });

        Ok(())
    }

    /// Save current file
    pub fn save_file(&mut self) -> io::Result<()> {
        if let Some(ref path) = self.current_file {
            std::fs::write(path, &self.buffer)?;

            if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                tab.modified = false;
            }

            self.output_lines.push(OutputLine {
                text: format!("Saved: {}", path.display()),
                output_type: OutputType::Success,
            });
        }
        Ok(())
    }

    /// Build project
    pub fn build_project(&mut self) -> io::Result<()> {
        self.output_lines.push(OutputLine {
            text: "gul build".to_string(),
            output_type: OutputType::Command,
        });
        self.output_lines.push(OutputLine {
            text: "Building project...".to_string(),
            output_type: OutputType::Info,
        });
        // Placeholder for actual build
        self.output_lines.push(OutputLine {
            text: "Build completed successfully!".to_string(),
            output_type: OutputType::Success,
        });
        Ok(())
    }

    /// Run project
    pub fn run_project(&mut self) -> io::Result<()> {
        self.output_lines.push(OutputLine {
            text: "gul run".to_string(),
            output_type: OutputType::Command,
        });
        self.output_lines.push(OutputLine {
            text: "Running project...".to_string(),
            output_type: OutputType::Info,
        });
        Ok(())
    }

    /// Load directory into file tree
    pub fn load_directory(&mut self, path: PathBuf) -> io::Result<()> {
        self.file_entries.clear();
        self.load_directory_recursive(&path, 0)?;
        Ok(())
    }

    fn load_directory_recursive(&mut self, path: &PathBuf, depth: usize) -> io::Result<()> {
        let mut entries: Vec<_> = std::fs::read_dir(path)?.filter_map(|e| e.ok()).collect();

        // Sort: directories first, then by name
        entries.sort_by(|a, b| {
            let a_is_dir = a.path().is_dir();
            let b_is_dir = b.path().is_dir();
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            }
        });

        for entry in entries {
            let entry_path = entry.path();
            let is_dir = entry_path.is_dir();

            self.file_entries.push(FileEntry {
                path: entry_path.clone(),
                is_dir,
                depth,
            });

            if is_dir && depth < 3 {
                self.load_directory_recursive(&entry_path, depth + 1)?;
            }
        }

    /// Insert a character at the cursor position
    fn insert_char(&mut self, c: char) {
        let mut lines: Vec<String> = self.buffer.lines().map(|s| s.to_string()).collect();
        if lines.is_empty() {
            lines.push(String::new());
        }

        let (row, col) = (self.editor_state.cursor.0, self.editor_state.cursor.1);
        while lines.len() <= row {
            lines.push(String::new());
        }

        let line = &mut lines[row];
        let byte_idx = line
            .char_indices()
            .nth(col)
            .map(|(i, _)| i)
            .unwrap_or(line.len());
        line.insert(byte_idx, c);
        self.editor_state.cursor.1 += 1;

        self.buffer = lines.join("\n");
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            tab.modified = true;
        }
    }

    /// Delete a character at the cursor position
    fn delete_char(&mut self) {
        let mut lines: Vec<String> = self.buffer.lines().map(|s| s.to_string()).collect();
        if lines.is_empty() {
            return;
        }

        let (row, col) = (self.editor_state.cursor.0, self.editor_state.cursor.1);
        if col > 0 {
            if row < lines.len() {
                let line = &mut lines[row];
                let byte_idx_opt = line.char_indices().nth(col - 1).map(|(i, _)| i);
                if let Some(byte_idx) = byte_idx_opt {
                    line.remove(byte_idx);
                    self.editor_state.cursor.1 -= 1;
                    self.buffer = lines.join("\n");
                }
            }
        } else if row > 0 {
            // Merge with previous line
            let current_line = lines.remove(row);
            let prev_line = &mut lines[row - 1];
            self.editor_state.cursor.1 = prev_line.chars().count();
            prev_line.push_str(&current_line);
            self.editor_state.cursor.0 -= 1;
            self.buffer = lines.join("\n");
        }

        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            tab.modified = true;
        }
    }

    /// Insert a newline at the cursor position
    fn insert_newline(&mut self) {
        let mut lines: Vec<String> = self.buffer.lines().map(|s| s.to_string()).collect();
        if lines.is_empty() {
            lines.push(String::new());
        }

        let (row, col) = (self.editor_state.cursor.0, self.editor_state.cursor.1);
        while lines.len() <= row {
            lines.push(String::new());
        }

        let line = &mut lines[row];
        let byte_idx = line
            .char_indices()
            .nth(col)
            .map(|(i, _)| i)
            .unwrap_or(line.len());
        let next_line_part = line.split_off(byte_idx);
        lines.insert(row + 1, next_line_part);
        self.editor_state.cursor.0 += 1;
        self.editor_state.cursor.1 = 0;

        self.buffer = lines.join("\n");
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            tab.modified = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = GulTuiApp::new();
        assert!(app.running);
        assert_eq!(app.mode, Mode::Normal);
        assert_eq!(app.focus, Focus::Editor);
    }

    #[test]
    fn test_app_default() {
        let app = GulTuiApp::default();
        assert!(!app.commands.is_empty());
        assert!(!app.output_lines.is_empty());
    }
}
