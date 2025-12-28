# Filesystem

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# File System Module API Reference

The `std.filesystem` module provides comprehensive file and directory operations with a safe, cross-platform API.

## 📁 File Operations

### Reading Files

```gul
import std.filesystem as fs

# Read entire file as string
content = fs.read_text("data.txt")

# Read as bytes
bytes = fs.read_bytes("image.png")

# Read lines
lines = fs.read_lines("data.csv")
for line in lines:
    print(line)

# Read with encoding
content = fs.read_text("data.txt", encoding="utf-8")
```

### Writing Files

```gul
# Write string to file
fs.write_text("output.txt", "Hello, GUL!")

# Write bytes
fs.write_bytes("output.bin", binary_data)

# Append to file
fs.append_text("log.txt", "New log entry\n")

# Write lines
fs.write_lines("output.csv", ["col1,col2", "1,2", "3,4"])
```

### File Metadata

```gul
# Check if file exists
if fs.exists("data.txt"):
    print("File exists")

# Get file size
size = fs.size("data.txt")

# Get modification time
mtime = fs.modified_time("data.txt")

# Check if path is file or directory
is_file = fs.is_file("data.txt")
is_dir = fs.is_directory("folder")
```

## 📂 Directory Operations

### Creating Directories

```gul
# Create directory
fs.create_dir("new_folder")

# Create directory with parents
fs.create_dir_all("path/to/nested/folder")
```

### Listing Directory Contents

```gul
# List all items
items = fs.list_dir("folder")

# List files only
files = fs.list_files("folder")

# List directories only
dirs = fs.list_directories("folder")

# Recursive listing
all_files = fs.walk("folder")
for file_path in all_files:
    print(file_path)
```

### Moving and Copying

```gul
# Copy file
fs.copy_file("source.txt", "destination.txt")

# Copy directory
fs.copy_dir("source_folder", "dest_folder")

# Move/rename file
fs.move("old_name.txt", "new_name.txt")

# Remove file
fs.remove_file("unwanted.txt")

# Remove directory
fs.remove_dir("folder")
fs.remove_dir_all("folder_with_contents")
```

## 🔍 Path Operations

```gul
import std.path

# Join paths
full_path = path.join("folder", "subfolder", "file.txt")

# Get absolute path
abs_path = path.absolute("../relative/path")

# Get directory name
dir = path.dirname("/path/to/file.txt")  # "/path/to"

# Get base name
name = path.basename("/path/to/file.txt")  # "file.txt"

# Get file extension
ext = path.extension("file.txt")  # ".txt"

# Check if path is absolute
is_abs = path.is_absolute("/absolute/path")
```

## 📋 Advanced Features

### File Watching

```gul
import std.filesystem.watch

watcher = watch.FileWatcher("folder")

@watcher.on_create
fn on_create(path):
    print(f"Created: {path}")

@watcher.on_modify
fn on_modify(path):
    print(f"Modified: {path}")

@watcher.on_delete
fn on_delete(path):
    print(f"Deleted: {path}")

watcher.start()
```

### Temporary Files

```gul
# Create temporary file
with fs.temp_file() as temp:
    fs.write_text(temp.path, "temporary data")
    # File automatically deleted when out of scope

# Create temporary directory
with fs.temp_dir() as temp_dir:
    # Use temp_dir.path
    pass
```

### Glob Patterns

```gul
# Find files matching pattern
txt_files = fs.glob("**/*.txt")
py_files = fs.glob("src/**/*.py")

for file in txt_files:
    print(file)
```

## 📚 See Also

- [Standard Library](standard-library.md)
- [First Program Tutorial](../tutorials/first-program.md)

---

**Last Updated**: 2025-12-28  
**Version: 0.13.0  
**License**: MIT
