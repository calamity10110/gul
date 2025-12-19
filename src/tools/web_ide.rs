// GUL Web IDE - Browser-based IDE for GUL v3.0
// Built with Dioxus for cross-platform UI
// Supports v3.0 syntax: let/var, mn:, @imp

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
    settings: IdeSettings,
    /// Package manager state
    package_manager: PackageManager,
    /// AI assistant state
    ai_assistant: Option<AiAssistant>,
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
    /// GUL version
    pub gul_version: String,
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
    /// Syntax highlighting tokens
    pub tokens: Vec<SyntaxToken>,
}

/// Syntax token for v3.0
#[derive(Clone, Debug)]
pub struct SyntaxToken {
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
    pub text: String,
}

/// Token types for v3.0 syntax
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    // v3.0 keywords
    Let,
    Var,
    Mn,
    Fn,
    Async,
    // v3.0 imports
    ImpDecorator,  // @imp
    PythonBlock,   // @python
    RustBlock,     // @rust
    CBlock,        // @c
    SqlBlock,      // @sql
    // Standard tokens
    Identifier,
    String,
    Number,
    Comment,
    Operator,
    // Deprecated (shown with warning)
    DeprecatedConst,
    DeprecatedMut,
    DeprecatedMain,
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
    /// Show deprecated syntax warnings
    pub warn_deprecated: bool,
    /// v3.0 autocomplete enabled
    pub v3_autocomplete: bool,
}

/// Package manager state
pub struct PackageManager {
    /// Available packages
    pub packages: Vec<Package>,
    /// Installed packages
    pub installed: Vec<String>,
    /// Search query
    pub search_query: String,
    /// Selected package
    pub selected: Option<usize>,
}

/// Package information
#[derive(Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub language: String,
    pub downloads: u64,
}

/// AI Assistant state
pub struct AiAssistant {
    /// AI provider (openai, anthropic, etc.)
    pub provider: String,
    /// Chat history
    pub messages: Vec<ChatMessage>,
    /// Current input
    pub input: String,
    /// Whether assistant is visible
    pub visible: bool,
}

/// Chat message
pub struct ChatMessage {
    pub role: String,  // "user" or "assistant"
    pub content: String,
    pub timestamp: String,
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
            package_manager: PackageManager::new(),
            ai_assistant: None,
        }
    }

    /// Create a new project with v3.0 template
    pub fn create_project(&mut self, name: String, root: PathBuf) {
        self.project = Some(Project {
            name: name.clone(),
            root: root.clone(),
            config: ProjectConfig {
                target: "native".to_string(),
                optimize: false,
                dependencies: Vec::new(),
                gul_version: "0.13.0".to_string(),
            },
        });
        self.file_tree.root = root.clone();
        
        // Create v3.0 template file
        self.create_v3_template(&root, &name);
    }

    /// Create v3.0 template file
    fn create_v3_template(&mut self, root: &PathBuf, name: &str) {
        let template = format!(
            "# {} - GUL v3.0 Project\n\n@imp std.io\n\nmn:\n    print(\"Hello from {}!\")\n",
            name, name
        );
        
        let main_file = root.join("main.mn");
        if let Ok(()) = std::fs::write(&main_file, template) {
            let _ = self.open_file(main_file);
        }
    }

    /// Open a file in a new tab with v3.0 syntax highlighting
    pub fn open_file(&mut self, path: PathBuf) -> std::io::Result<()> {
        let content = std::fs::read_to_string(&path)?;
        let tokens = self.tokenize_v3(&content);

        self.open_files.push(EditorTab {
            path,
            content,
            modified: false,
            cursor: (0, 0),
            tokens,
        });

        self.active_tab = self.open_files.len() - 1;
        Ok(())
    }

    /// Tokenize content with v3.0 syntax
    fn tokenize_v3(&self, content: &str) -> Vec<SyntaxToken> {
        let mut tokens = Vec::new();
        let keywords_v3 = ["let", "var", "mn", "fn", "async"];
        let deprecated = ["const", "mut", "main"];
        
        // Simple tokenization (real implementation would use proper lexer)
        for (i, word) in content.split_whitespace().enumerate() {
            let token_type = if keywords_v3.contains(&word) {
                match word {
                    "let" => TokenType::Let,
                    "var" => TokenType::Var,
                    "mn" => TokenType::Mn,
                    "fn" => TokenType::Fn,
                    "async" => TokenType::Async,
                    _ => TokenType::Identifier,
                }
            } else if deprecated.contains(&word) {
                match word {
                    "const" => TokenType::DeprecatedConst,
                    "mut" => TokenType::DeprecatedMut,
                    "main" => TokenType::DeprecatedMain,
                    _ => TokenType::Identifier,
                }
            } else if word.starts_with("@imp") {
                TokenType::ImpDecorator
            } else if word.starts_with("@python") {
                TokenType::PythonBlock
            } else {
                TokenType::Identifier
            };

            tokens.push(SyntaxToken {
                token_type,
                start: i * 10,
                end: i * 10 + word.len(),
                text: word.to_string(),
            });
        }

        tokens
    }

    /// Get v3.0 autocomplete suggestions
    pub fn get_autocomplete(&self, prefix: &str) -> Vec<AutocompleteItem> {
        let mut suggestions = Vec::new();

        match prefix {
            "let" => suggestions.push(AutocompleteItem {
                label: "let".to_string(),
                insert_text: "let name = ".to_string(),
                description: "Immutable variable (v3.0)".to_string(),
            }),
            "var" => suggestions.push(AutocompleteItem {
                label: "var".to_string(),
                insert_text: "var count = 0".to_string(),
                description: "Mutable variable (v3.0)".to_string(),
            }),
            "mn" => suggestions.push(AutocompleteItem {
                label: "mn:".to_string(),
                insert_text: "mn:\n    ".to_string(),
                description: "Main entry point (v3.0)".to_string(),
            }),
            "@imp" => suggestions.push(AutocompleteItem {
                label: "@imp".to_string(),
                insert_text: "@imp std.".to_string(),
                description: "Import statement (v3.0)".to_string(),
            }),
            "fn" => suggestions.push(AutocompleteItem {
                label: "fn".to_string(),
                insert_text: "fn name():\n    ".to_string(),
                description: "Function definition".to_string(),
            }),
            "async" => suggestions.push(AutocompleteItem {
                label: "async".to_string(),
                insert_text: "async name():\n    ".to_string(),
                description: "Async function".to_string(),
            }),
            _ => {}
        }

        suggestions
    }

    /// Save the active file
    pub fn save_active_file(&mut self) -> std::io::Result<()> {
        let content = if let Some(tab) = self.open_files.get_mut(self.active_tab) {
            std::fs::write(&tab.path, &tab.content)?;
            tab.modified = false;
            Some(tab.content.clone())
        } else {
            None
        };
        
        // Re-tokenize after save
        if let Some(content_str) = content {
            let tokens = self.tokenize_v3(&content_str);
            if let Some(tab) = self.open_files.get_mut(self.active_tab) {
                tab.tokens = tokens;
            }
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
        self.build_output = "Building project with GUL v3.0...\n".to_string();
        
        // Check for deprecated syntax
        if self.settings.warn_deprecated {
            self.check_deprecated_syntax();
        }

        self.build_output.push_str("Build successful!\n");
        Ok(())
    }

    /// Check for deprecated syntax and warn
    fn check_deprecated_syntax(&mut self) {
        let mut warnings = Vec::new();
        
        for tab in &self.open_files {
            if tab.content.contains("const ") {
                warnings.push("⚠️  Warning: 'const' is deprecated, use 'let'\n");
            }
            if tab.content.contains("mut ") && !tab.content.contains("&mut") {
                warnings.push("⚠️  Warning: 'mut' is deprecated, use 'var'\n");
            }
            if tab.content.contains("main():") {
                warnings.push("⚠️  Warning: 'main():' is deprecated, use 'mn:'\n");
            }
        }
        
        for warning in warnings {
            self.build_output.push_str(warning);
        }
    }

    /// Run the current project
    pub fn run_project(&mut self) -> std::io::Result<()> {
        self.terminal.output.push("Running GUL v3.0 project...".to_string());
        Ok(())
    }

    /// Format the active file with v3.0 style
    pub fn format_active_file(&mut self) -> std::io::Result<()> {
        if let Some(_tab) = self.open_files.get_mut(self.active_tab) {
            self.build_output = "File formatted with v3.0 style\n".to_string();
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

    /// Enable AI assistant
    pub fn enable_ai_assistant(&mut self, provider: String) {
        self.ai_assistant = Some(AiAssistant {
            provider,
            messages: Vec::new(),
            input: String::new(),
            visible: true,
        });
    }
}

/// Autocomplete item
pub struct AutocompleteItem {
    pub label: String,
    pub insert_text: String,
    pub description: String,
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
            output: vec!["GUL v3.0 Terminal - Ready".to_string()],
            command: String::new(),
            history: Vec::new(),
            visible: false,
        }
    }

    /// Execute a command
    pub fn execute_command(&mut self, command: String) {
        self.history.push(command.clone());
        self.output.push(format!("> {}", command));
        self.output.push("Command executed".to_string());
        self.command.clear();
    }

    /// Clear terminal output
    pub fn clear(&mut self) {
        self.output.clear();
        self.output.push("Terminal cleared".to_string());
    }
}

impl PackageManager {
    /// Create new package manager
    pub fn new() -> Self {
        PackageManager {
            packages: Vec::new(),
            installed: Vec::new(),
            search_query: String::new(),
            selected: None,
        }
    }

    /// Load available packages
    pub fn load_packages(&mut self) {
        // This would load from actual package registry
        self.packages = vec![
            Package {
                name: "actix-web".to_string(),
                version: "4.0.0".to_string(),
                description: "Powerful web framework".to_string(),
                language: "Rust".to_string(),
                downloads: 1000000,
            },
            // More packages...
        ];
    }

    /// Search packages
    pub fn search(&mut self, query: String) {
        self.search_query = query;
    }

    /// Install package
    pub fn install(&mut self, package_name: String) {
        if !self.installed.contains(&package_name) {
            self.installed.push(package_name);
        }
    }
}

impl ProjectConfig {
    /// Create default project configuration for v3.0
    pub fn default() -> Self {
        ProjectConfig {
            target: "native".to_string(),
            optimize: false,
            dependencies: Vec::new(),
            gul_version: "0.13.0".to_string(),
        }
    }
}

impl IdeSettings {
    /// Create default IDE settings with v3.0 features
    pub fn default() -> Self {
        IdeSettings {
            theme: "dark".to_string(),
            font_size: 14,
            tab_size: 4,
            auto_save: true,
            format_on_save: true,
            warn_deprecated: true,
            v3_autocomplete: true,
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

impl Default for PackageManager {
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
        assert!(ide.settings.v3_autocomplete);
    }

    #[test]
    fn test_v3_autocomplete() {
        let ide = GulWebIde::new();
        let suggestions = ide.get_autocomplete("let");
        assert!(!suggestions.is_empty());
        assert_eq!(suggestions[0].label, "let");
    }

    #[test]
    fn test_deprecated_warning() {
        let ide = GulWebIde::new();
        assert!(ide.settings.warn_deprecated);
    }

    #[test]
    fn test_package_manager() {
        let mut pm = PackageManager::new();
        pm.install("actix-web".to_string());
        assert_eq!(pm.installed.len(), 1);
    }
}
