#![allow(dead_code)]
// File system operations
use std::fs::{self, File, Metadata, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct FileSystem;

impl FileSystem {
    /// Read entire file contents as a string
    pub fn read_to_string(path: &str) -> Result<String, String> {
        fs::read_to_string(path).map_err(|e| format!("Failed to read file '{}': {}", path, e))
    }

    /// Read file contents as bytes
    pub fn read_bytes(path: &str) -> Result<Vec<u8>, String> {
        fs::read(path).map_err(|e| format!("Failed to read file '{}': {}", path, e))
    }

    /// Read file line by line
    pub fn read_lines(path: &str) -> Result<Vec<String>, String> {
        let file =
            File::open(path).map_err(|e| format!("Failed to open file '{}': {}", path, e))?;

        let reader = BufReader::new(file);
        let lines: Result<Vec<String>, _> = reader.lines().collect();

        lines.map_err(|e| format!("Failed to read lines from '{}': {}", path, e))
    }

    /// Write string to file (overwrites existing content)
    pub fn write_string(path: &str, content: &str) -> Result<(), String> {
        fs::write(path, content).map_err(|e| format!("Failed to write to file '{}': {}", path, e))
    }

    /// Write bytes to file (overwrites existing content)
    pub fn write_bytes(path: &str, content: &[u8]) -> Result<(), String> {
        fs::write(path, content).map_err(|e| format!("Failed to write to file '{}': {}", path, e))
    }

    /// Append string to file
    pub fn append_string(path: &str, content: &str) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| format!("Failed to open file '{}' for appending: {}", path, e))?;

        file.write_all(content.as_bytes())
            .map_err(|e| format!("Failed to append to file '{}': {}", path, e))
    }

    /// Check if file exists
    pub fn exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    /// Check if path is a file
    pub fn is_file(path: &str) -> bool {
        Path::new(path).is_file()
    }

    /// Check if path is a directory
    pub fn is_dir(path: &str) -> bool {
        Path::new(path).is_dir()
    }

    /// Create a directory (and all parent directories)
    pub fn create_dir(path: &str) -> Result<(), String> {
        fs::create_dir_all(path)
            .map_err(|e| format!("Failed to create directory '{}': {}", path, e))
    }

    /// Delete a file
    pub fn delete_file(path: &str) -> Result<(), String> {
        fs::remove_file(path).map_err(|e| format!("Failed to delete file '{}': {}", path, e))
    }

    /// Delete a directory (and all contents)
    pub fn delete_dir(path: &str) -> Result<(), String> {
        fs::remove_dir_all(path)
            .map_err(|e| format!("Failed to delete directory '{}': {}", path, e))
    }

    /// Copy a file
    pub fn copy_file(from: &str, to: &str) -> Result<(), String> {
        fs::copy(from, to)
            .map_err(|e| format!("Failed to copy file from '{}' to '{}': {}", from, to, e))?;
        Ok(())
    }

    /// Move/rename a file or directory
    pub fn move_path(from: &str, to: &str) -> Result<(), String> {
        fs::rename(from, to).map_err(|e| format!("Failed to move '{}' to '{}': {}", from, to, e))
    }

    /// List directory contents
    pub fn list_dir(path: &str) -> Result<Vec<String>, String> {
        let entries = fs::read_dir(path)
            .map_err(|e| format!("Failed to read directory '{}': {}", path, e))?;

        let mut result = Vec::new();
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                result.push(name.to_string());
            }
        }

        Ok(result)
    }

    /// Get file metadata
    pub fn metadata(path: &str) -> Result<FileMetadata, String> {
        let metadata = fs::metadata(path)
            .map_err(|e| format!("Failed to get metadata for '{}': {}", path, e))?;

        Ok(FileMetadata::from_metadata(path, metadata))
    }

    /// Get absolute path
    pub fn absolute_path(path: &str) -> Result<String, String> {
        let path_buf = fs::canonicalize(path)
            .map_err(|e| format!("Failed to get absolute path for '{}': {}", path, e))?;

        Ok(path_buf.to_string_lossy().to_string())
    }

    /// Join path components
    pub fn join_path(base: &str, component: &str) -> String {
        let mut path = PathBuf::from(base);
        path.push(component);
        path.to_string_lossy().to_string()
    }

    /// Get file extension
    pub fn extension(path: &str) -> Option<String> {
        Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
    }

    /// Get file name (without directory)
    pub fn filename(path: &str) -> Option<String> {
        Path::new(path)
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
    }

    /// Get parent directory
    pub fn parent_dir(path: &str) -> Option<String> {
        Path::new(path)
            .parent()
            .and_then(|p| p.to_str())
            .map(|s| s.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: String,
    pub size: u64,
    pub is_file: bool,
    pub is_dir: bool,
    pub modified: Option<SystemTime>,
    pub created: Option<SystemTime>,
    pub readonly: bool,
}

impl FileMetadata {
    fn from_metadata(path: &str, metadata: Metadata) -> Self {
        FileMetadata {
            path: path.to_string(),
            size: metadata.len(),
            is_file: metadata.is_file(),
            is_dir: metadata.is_dir(),
            modified: metadata.modified().ok(),
            created: metadata.created().ok(),
            readonly: metadata.permissions().readonly(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_write_and_read_string() {
        let path = "/tmp/test_file.txt";
        let content = "Hello, World!";

        FileSystem::write_string(path, content).unwrap();
        let read_content = FileSystem::read_to_string(path).unwrap();

        assert_eq!(content, read_content);
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_write_and_read_bytes() {
        let path = "/tmp/test_bytes.bin";
        let content = vec![1, 2, 3, 4, 5];

        FileSystem::write_bytes(path, &content).unwrap();
        let read_content = FileSystem::read_bytes(path).unwrap();

        assert_eq!(content, read_content);
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_append_string() {
        let path = "/tmp/test_append.txt";

        FileSystem::write_string(path, "Line 1\n").unwrap();
        FileSystem::append_string(path, "Line 2\n").unwrap();

        let content = FileSystem::read_to_string(path).unwrap();
        assert_eq!(content, "Line 1\nLine 2\n");

        fs::remove_file(path).ok();
    }

    #[test]
    fn test_file_exists() {
        let path = "/tmp/test_exists.txt";

        assert!(!FileSystem::exists(path));

        FileSystem::write_string(path, "test").unwrap();
        assert!(FileSystem::exists(path));

        fs::remove_file(path).ok();
    }

    #[test]
    fn test_create_and_delete_dir() {
        let path = "/tmp/test_dir";

        FileSystem::create_dir(path).unwrap();
        assert!(FileSystem::is_dir(path));

        FileSystem::delete_dir(path).unwrap();
        assert!(!FileSystem::exists(path));
    }

    #[test]
    fn test_copy_file() {
        let src = "/tmp/test_copy_src.txt";
        let dst = "/tmp/test_copy_dst.txt";

        FileSystem::write_string(src, "copy test").unwrap();
        FileSystem::copy_file(src, dst).unwrap();

        let content = FileSystem::read_to_string(dst).unwrap();
        assert_eq!(content, "copy test");

        fs::remove_file(src).ok();
        fs::remove_file(dst).ok();
    }

    #[test]
    fn test_path_operations() {
        assert_eq!(FileSystem::join_path("/home", "user"), "/home/user");
        assert_eq!(FileSystem::extension("file.txt"), Some("txt".to_string()));
        assert_eq!(
            FileSystem::filename("/path/to/file.txt"),
            Some("file.txt".to_string())
        );
        assert_eq!(
            FileSystem::parent_dir("/path/to/file.txt"),
            Some("/path/to".to_string())
        );
    }

    #[test]
    fn test_read_lines() {
        let path = "/tmp/test_lines.txt";
        FileSystem::write_string(path, "line1\nline2\nline3").unwrap();

        let lines = FileSystem::read_lines(path).unwrap();
        assert_eq!(lines, vec!["line1", "line2", "line3"]);

        fs::remove_file(path).ok();
    }
}
