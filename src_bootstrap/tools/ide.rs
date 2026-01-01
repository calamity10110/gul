#![allow(dead_code)]
// IDE integration module
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub root_path: PathBuf,
    pub files: Vec<PathBuf>,
    pub dependencies: Vec<String>,
    pub build_config: BuildConfig,
}

#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub target: BuildTarget,
    pub optimization_level: OptimizationLevel,
    pub debug_symbols: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuildTarget {
    Native,
    Wasm,
    Esp32,
    Rp2040,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OptimizationLevel {
    None,
    Basic,
    Aggressive,
}

pub struct IdeState {
    pub current_project: Option<Project>,
    pub open_files: HashMap<PathBuf, String>,
    pub active_file: Option<PathBuf>,
    pub cursor_position: (usize, usize), // (line, column)
}

impl Default for IdeState {
    fn default() -> Self {
        Self::new()
    }
}

impl IdeState {
    pub fn new() -> Self {
        IdeState {
            current_project: None,
            open_files: HashMap::new(),
            active_file: None,
            cursor_position: (0, 0),
        }
    }

    pub fn open_project(&mut self, project: Project) {
        self.current_project = Some(project);
    }

    pub fn close_project(&mut self) {
        self.current_project = None;
        self.open_files.clear();
        self.active_file = None;
    }

    pub fn open_file(&mut self, path: PathBuf, content: String) {
        self.open_files.insert(path.clone(), content);
        self.active_file = Some(path);
    }

    pub fn close_file(&mut self, path: &PathBuf) {
        self.open_files.remove(path);
        if self.active_file.as_ref() == Some(path) {
            self.active_file = None;
        }
    }

    pub fn get_file_content(&self, path: &PathBuf) -> Option<&String> {
        self.open_files.get(path)
    }
    #[allow(clippy::ptr_arg)]
    pub fn update_file_content(&mut self, path: &PathBuf, content: String) {
        self.open_files.insert(path.clone(), content);
    }

    pub fn set_cursor_position(&mut self, line: usize, column: usize) {
        self.cursor_position = (line, column);
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        self.cursor_position
    }
}

pub struct CommandPalette {
    commands: HashMap<String, Box<dyn Fn() + Send + Sync>>,
}

impl CommandPalette {
    pub fn new() -> Self {
        CommandPalette {
            commands: HashMap::new(),
        }
    }

    pub fn register_command<F>(&mut self, name: String, _handler: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        // Simplified command registration
        // In a real implementation, this would store the handler
        self.commands.insert(name, Box::new(|| {}));
    }

    pub fn execute_command(&self, name: &str) -> Result<(), String> {
        if self.commands.contains_key(name) {
            Ok(())
        } else {
            Err(format!("Command '{}' not found", name))
        }
    }

    pub fn list_commands(&self) -> Vec<&String> {
        self.commands.keys().collect()
    }
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FileExplorer {
    root_path: PathBuf,
    expanded_dirs: Vec<PathBuf>,
}

impl FileExplorer {
    pub fn new(root_path: PathBuf) -> Self {
        FileExplorer {
            root_path,
            expanded_dirs: Vec::new(),
        }
    }

    pub fn expand_directory(&mut self, path: PathBuf) {
        if !self.expanded_dirs.contains(&path) {
            self.expanded_dirs.push(path);
        }
    }

    pub fn collapse_directory(&mut self, path: &PathBuf) {
        self.expanded_dirs.retain(|p| p != path);
    }

    pub fn is_expanded(&self, path: &PathBuf) -> bool {
        self.expanded_dirs.contains(path)
    }

    pub fn get_root_path(&self) -> &PathBuf {
        &self.root_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ide_state_creation() {
        let state = IdeState::new();
        assert!(state.current_project.is_none());
        assert_eq!(state.open_files.len(), 0);
    }

    #[test]
    fn test_open_close_file() {
        let mut state = IdeState::new();
        let path = PathBuf::from("test.mn");

        state.open_file(path.clone(), "content".to_string());
        assert_eq!(state.open_files.len(), 1);
        assert_eq!(state.active_file, Some(path.clone()));

        state.close_file(&path);
        assert_eq!(state.open_files.len(), 0);
        assert_eq!(state.active_file, None);
    }

    #[test]
    fn test_cursor_position() {
        let mut state = IdeState::new();

        state.set_cursor_position(10, 5);
        assert_eq!(state.get_cursor_position(), (10, 5));
    }

    #[test]
    fn test_command_palette() {
        let mut palette = CommandPalette::new();

        palette.register_command("test".to_string(), || {});
        assert!(palette.execute_command("test").is_ok());
        assert!(palette.execute_command("nonexistent").is_err());
    }

    #[test]
    fn test_file_explorer() {
        let root = PathBuf::from("/project");
        let mut explorer = FileExplorer::new(root.clone());

        let dir = PathBuf::from("/project/src");
        explorer.expand_directory(dir.clone());
        assert!(explorer.is_expanded(&dir));

        explorer.collapse_directory(&dir);
        assert!(!explorer.is_expanded(&dir));
    }

    #[test]
    fn test_project_creation() {
        let project = Project {
            name: "test_project".to_string(),
            root_path: PathBuf::from("/test"),
            files: vec![],
            dependencies: vec![],
            build_config: BuildConfig {
                target: BuildTarget::Native,
                optimization_level: OptimizationLevel::Basic,
                debug_symbols: true,
            },
        };

        assert_eq!(project.name, "test_project");
        assert_eq!(project.build_config.target, BuildTarget::Native);
    }
}
