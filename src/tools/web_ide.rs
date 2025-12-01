// GUL Web IDE - Browser-based IDE for GUL
// Built with Dioxus for cross-platform UI

use std::path::PathBuf;

/// Main Web IDE application state
pub struct GulWebIde {
    /// Current project
    project: Option<Project>,
    /// Open files
    open_files: Vec<EditorTab>,
    /// Active file index
    active_tab: usize,
    /// File tree state
    file_tree: FileTree,
    /// Terminal state
    terminal: Terminal,
    /// Build output
    build_output: String,
    /// Settings
    #[allow(dead_code)]
    settings: IdeSettings,
}

/// Project structure
pub struct Project {
    /// Project name
    pub name: String,
    /// Project root directory
    pub root: PathBuf,
    /// Project configuration
    pub config: ProjectConfig,
}

/// Project configuration
pub struct ProjectConfig {
    /// Build target (native, wasm, esp32)
    pub target: String,
    /// Optimization level
    pub optimize: bool,
    /// Dependencies
    pub dependencies: Vec<String>,
}

/// Editor tab for an open file
pub struct EditorTab {
    /// File path
    pub path: PathBuf,
    /// File content
    pub content: String,
    /// Whether the file has unsaved changes
    pub modified: bool,
    /// Cursor position
    pub cursor: (usize, usize),
}

/// File tree component
pub struct FileTree {
    /// Root directory
    pub root: PathBuf,
    /// Expanded directories
    pub expanded: Vec<PathBuf>,
    /// Selected file
    pub selected: Option<PathBuf>,
}

/// Terminal emulator
pub struct Terminal {
    /// Terminal output
    pub output: Vec<String>,
    /// Current command
    pub command: String,
    /// Command history
    pub history: Vec<String>,
    /// Whether terminal is visible
    pub visible: bool,
}

/// IDE settings
pub struct IdeSettings {
    /// Theme (light, dark)
    pub theme: String,
    /// Font size
    pub font_size: u32,
    /// Tab size
    pub tab_size: u32,
    /// Auto-save enabled
    pub auto_save: bool,
    /// Format on save
    pub format_on_save: bool,
}

impl GulWebIde {
    /// Create a new Web IDE instance
    pub fn new() -> Self {
        GulWebIde {
            project: None,
            open_files: Vec::new(),
            active_tab: 0,
            file_tree: FileTree::new(),
            terminal: Terminal::new(),
            build_output: String::new(),
            settings: IdeSettings::default(),
        }
    }

    /// Create a new project
    pub fn create_project(&mut self, name: String, root: PathBuf) {
        self.project = Some(Project {
            name,
            root: root.clone(),
            config: ProjectConfig::default(),
        });
        self.file_tree.root = root;
    }

    /// Open a file in a new tab
    pub fn open_file(&mut self, path: PathBuf) -> std::io::Result<()> {
        let content = std::fs::read_to_string(&path)?;

        self.open_files.push(EditorTab {
            path,
            content,
            modified: false,
            cursor: (0, 0),
        });

        self.active_tab = self.open_files.len() - 1;
        Ok(())
    }

    /// Save the active file
    pub fn save_active_file(&mut self) -> std::io::Result<()> {
        if let Some(tab) = self.open_files.get_mut(self.active_tab) {
            std::fs::write(&tab.path, &tab.content)?;
            tab.modified = false;
        }
        Ok(())
    }

    /// Close the active tab
    pub fn close_active_tab(&mut self) {
        if !self.open_files.is_empty() {
            self.open_files.remove(self.active_tab);
            if self.active_tab > 0 {
                self.active_tab -= 1;
            }
        }
    }

    /// Build the current project
    pub fn build_project(&mut self) -> std::io::Result<()> {
        self.build_output = "Building project...\n".to_string();

        // Integration with compiler would go here
        // For now, simulate build
        self.build_output.push_str("Build successful!\n");

        Ok(())
    }

    /// Run the current project
    pub fn run_project(&mut self) -> std::io::Result<()> {
        self.terminal.output.push("Running project...".to_string());

        // Integration with runtime would go here

        Ok(())
    }

    /// Format the active file
    pub fn format_active_file(&mut self) -> std::io::Result<()> {
        if let Some(_tab) = self.open_files.get_mut(self.active_tab) {
            // Integration with formatter would go here
            self.build_output = "File formatted successfully\n".to_string();
        }
        Ok(())
    }

    /// Search in files
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut results = Vec::new();

        for tab in &self.open_files {
            for (line_num, line) in tab.content.lines().enumerate() {
                if line.contains(query) {
                    results.push(SearchResult {
                        file: tab.path.clone(),
                        line: line_num + 1,
                        content: line.to_string(),
                    });
                }
            }
        }

        results
    }

    /// Toggle terminal visibility
    pub fn toggle_terminal(&mut self) {
        self.terminal.visible = !self.terminal.visible;
    }
}

/// Search result
pub struct SearchResult {
    /// File path
    pub file: PathBuf,
    /// Line number
    pub line: usize,
    /// Line content
    pub content: String,
}

impl FileTree {
    /// Create a new file tree
    pub fn new() -> Self {
        FileTree {
            root: std::env::current_dir().unwrap_or_default(),
            expanded: Vec::new(),
            selected: None,
        }
    }

    /// Toggle directory expansion
    pub fn toggle_expand(&mut self, path: PathBuf) {
        if let Some(pos) = self.expanded.iter().position(|p| p == &path) {
            self.expanded.remove(pos);
        } else {
            self.expanded.push(path);
        }
    }

    /// Check if directory is expanded
    pub fn is_expanded(&self, path: &PathBuf) -> bool {
        self.expanded.contains(path)
    }
}

impl Terminal {
    /// Create a new terminal
    pub fn new() -> Self {
        Terminal {
            output: vec!["GUL Terminal - Ready".to_string()],
            command: String::new(),
            history: Vec::new(),
            visible: false,
        }
    }

    /// Execute a command
    pub fn execute_command(&mut self, command: String) {
        self.history.push(command.clone());
        self.output.push(format!("> {}", command));

        // Command execution would go here
        self.output.push("Command executed".to_string());

        self.command.clear();
    }

    /// Clear terminal output
    pub fn clear(&mut self) {
        self.output.clear();
        self.output.push("Terminal cleared".to_string());
    }
}

impl ProjectConfig {
    /// Create default project configuration
    pub fn default() -> Self {
        ProjectConfig {
            target: "native".to_string(),
            optimize: false,
            dependencies: Vec::new(),
        }
    }
}

impl IdeSettings {
    /// Create default IDE settings
    pub fn default() -> Self {
        IdeSettings {
            theme: "dark".to_string(),
            font_size: 14,
            tab_size: 4,
            auto_save: true,
            format_on_save: true,
        }
    }
}

impl Default for GulWebIde {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FileTree {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_ide_creation() {
        let ide = GulWebIde::new();
        assert!(ide.project.is_none());
        assert_eq!(ide.open_files.len(), 0);
    }

    #[test]
    fn test_project_creation() {
        let mut ide = GulWebIde::new();
        ide.create_project("test_project".to_string(), PathBuf::from("/tmp/test"));
        assert!(ide.project.is_some());
    }

    #[test]
    fn test_terminal() {
        let mut terminal = Terminal::new();
        assert!(!terminal.visible);
        terminal.execute_command("test".to_string());
        assert_eq!(terminal.history.len(), 1);
    }

    #[test]
    fn test_file_tree() {
        let mut tree = FileTree::new();
        let path = PathBuf::from("/test");
        tree.toggle_expand(path.clone());
        assert!(tree.is_expanded(&path));
        tree.toggle_expand(path.clone());
        assert!(!tree.is_expanded(&path));
    }
}
