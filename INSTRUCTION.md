# GUL (GUL Universal Language) - Getting Started Guide

Welcome to **GUL** - the GUL Universal Language! This guide will teach you everything you need to start coding in GUL.

## 📋 Table of Contents

1. [Installation](#installation)
2. [Your First GUL Program](#your-first-gul-program)
3. [Core Syntax](#core-syntax)
4. [Project Structure](#project-structure)
5. [Using the TUI IDE](#using-the-tui-ide)
6. [Using the Web IDE](#using-the-web-ide)
7. [Multi-Language Integration](#multi-language-integration)
8. [Advanced Features](#advanced-features)
9. [LLM Training Prompt](#llm-training-prompt)

---

## 🚀 Installation

### Prerequisites

- Rust 1.70+ (for building from source)
- Python 3.8+ (optional, for Python FFI)
- Node.js 16+ (optional, for JavaScript FFI)

### Install GUL Compiler

```bash
# Clone the repository
git clone https://github.com/gul-lang/gul
cd gul

# Build and install
cargo build --release
cargo install --path .

# Verify installation
gul --version
```

---

## 👋 Your First GUL Program

Create a file called `hello.mn`:

```gul
# hello.mn - Your first GUL program
mn main():
    print("Hello, GUL!")
```

Run it:

```bash
gul run hello.mn
```

Output:

```
Hello, GUL!
```

---

## 📖 Core Syntax

### 1. Variables

**Immutable (default):**

```gul
def name = "Alice"
def age = 30
def pi = 3.14159
```

**Mutable (with `?` prefix):**

```gul
?count = 0
?total = 100

?count = ?count + 1  # Allowed
```

### 2. Type Annotations

```gul
@int age = 25
@str name = "Bob"
@float price = 19.99
@bool is_active = true
@lst numbers = [1, 2, 3, 4, 5]
@map person = {name: "Charlie", age: 35}
```

### 3. Functions

**Synchronous Function:**

```gul
@fn add(a, b):
    return a + b

result = add(5, 3)  # 8
```

**Asynchronous Function:**

```gul
@asy fetch_data(url):
    response = await http.get(url)
    return response.json()

mn main():
    data = await fetch_data("https://api.example.com/data")
    print(data)
```

### 4. Control Flow

**If/Elif/Else:**

```gul
@if score >= 90:
    print("A")
@elif score >= 80:
    print("B")
@else:
    print("C")
```

**Loops:**

```gul
# While loop
?i = 0
@while ?i < 10:
    print(?i)
    ?i = ?i + 1

# For loop
@for item in [1, 2, 3, 4, 5]:
    print(item)
```

### 5. Flexible Imports

All these formats are equivalent:

```gul
# Format 1: Individual imports
imp python: numpy
imp python: matplotlib
imp rust: tokio

# Format 2: Grouped with brackets
imp [
    python: (numpy, matplotlib),
    rust: tokio
]

# Format 3: Grouped with braces
imp python: {numpy, matplotlib}
imp rust: tokio

# Format 4: Mixed
imp {python: [numpy, matplotlib], rust: tokio}
```

---

## 📁 Project Structure

GUL automatically organizes your code into blocks:

```
my_project/
├── main.mn          # Your source code
├── package.toml     # Project metadata
└── build/           # Generated blocks
    ├── imports.imp  # Import statements
    ├── definitions.def  # Variable definitions
    ├── functions.fnc    # Synchronous functions
    ├── async.asy        # Async functions
    ├── custom.cs        # Foreign language code
    └── main.mn          # Main entry point
```

**You write everything in `main.mn`, GUL organizes it automatically!**

---

## 🖥️ Using the TUI IDE

### Starting the TUI IDE

```bash
cd my_project
cargo run --example programming_deck
```

### TUI IDE Features

- **File Explorer**: Navigate your project files
- **Code Editor**: Edit GUL code with syntax highlighting
- **System Monitor**: View CPU, memory, disk usage
- **Integrated Terminal**: Run commands without leaving the IDE

### Keyboard Shortcuts

- `Ctrl+N`: New file
- `Ctrl+O`: Open file
- `Ctrl+S`: Save file
- `Ctrl+R`: Run current file
- `Ctrl+Q`: Quit

---

## 🌐 Using the Web IDE

### Starting the Web IDE

```bash
cd web
dx serve
```

Then open your browser to `http://localhost:8080`

### Web IDE Features

- **Browser-based**: No installation required
- **Real-time collaboration**: Share your workspace
- **Syntax highlighting**: Full GUL language support
- **Integrated console**: See output instantly

### Deploying Your Web IDE

```bash
# Build for production
cd web
dx build --release

# Serve the built files
cd dist
python3 -m http.server 8080
```

---

## 🔗 Multi-Language Integration

### Python Integration

```gul
@cs python:
    import numpy as np

    def analyze_data(data):
        arr = np.array(data)
        return {
            'mean': float(np.mean(arr)),
            'std': float(np.std(arr))
        }

mn main():
    data = [1, 2, 3, 4, 5]
    result = python.analyze_data(data)
    print(result)
```

### JavaScript Integration

```gul
@cs js:
    export function formatDate(timestamp) {
        return new Date(timestamp).toLocaleDateString();
    }

mn main():
    date = js.formatDate(Date.now())
    print(date)
```

### Rust Integration

```gul
@cs rust:
    fn fast_sum(numbers: &[i32]) -> i32 {
        numbers.iter().sum()
    }

mn main():
    numbers = [1, 2, 3, 4, 5]
    total = rust.fast_sum(numbers)
    print(total)  # 15
```

### SQL Integration

```gul
@cs sql:
    CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        email TEXT UNIQUE
    );

    INSERT INTO users (name, email) VALUES
        ('Alice', 'alice@example.com'),
        ('Bob', 'bob@example.com');

mn main():
    db.execute(sql.create_table)
    db.execute(sql.insert_users)

    results = db.query("SELECT * FROM users")
    @for row in results:
        print(row.name, row.email)
```

---

## 🎯 Advanced Features

### 1. Ownership Annotations

```gul
@fn process_data(@own data):
    # Takes ownership of data
    return data.transform()

@fn analyze_data(@ref data):
    # Borrows data (read-only)
    return data.stats()

@fn clone_data(@copy data):
    # Creates a copy of data
    return data.duplicate()
```

### 2. Global and Static Variables

```gul
@global ?counter = 0  # Managed by async functions
@static cache = {}    # Shared across all functions

@asy increment():
    @global ?counter = @global ?counter + 1

@fn get_cached(key):
    return @static cache.get(key)
```

### 3. Statistical Functions

```gul
data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

mean_val = @mean(data)
median_val = @median(data)
std_dev = @stddev(data)
correlation = @correlation(data1, data2)
```

### 4. UI Components (TUI)

```gul
# Create a progress bar
ui.print(^÷^[progress{value=75, max=100, label="Loading"}])

# Create a table
ui.print(^÷^[table{
    headers: ["ID", "Name", "Status"],
    rows: [
        ["1", "Alice", "Active"],
        ["2", "Bob", "Inactive"]
    ]
}])

# Create a button
ui.print(^÷^[button{text="Click Me!"}])
```

---

## 🤖 LLM Training Prompt

Use this prompt to teach an LLM how to code in GUL:

````
You are an expert GUL (GUL Universal Language) programmer. GUL is a universal programming language with these key features:

1. SYNTAX:
   - Variables: `def name = value` (immutable), `?name = value` (mutable)
   - Functions: `@fn name(params):` (sync), `@asy name(params):` (async)
   - Main entry: `mn main():`
   - Annotations: `@int`, `@str`, `@float`, `@bool`, `@lst`, `@map`, `@ref`, `@own`, `@copy`
   - Control flow: `@if`, `@elif`, `@else`, `@while`, `@for`, `@break`, `@continue`, `@return`

2. IMPORTS (all equivalent):
   - `imp python: numpy`
   - `imp [python: (numpy, pandas)]`
   - `imp {python: [numpy, pandas]}`

3. MULTI-LANGUAGE INTEGRATION:
   - Python: `@cs python: <code>`
   - JavaScript: `@cs js: <code>`
   - Rust: `@cs rust: <code>`
   - SQL: `@cs sql: <query>`

4. EXAMPLE PROGRAM:
```gul
imp [python: numpy, rust: tokio]

@cs python:
    import numpy as np
    def analyze(data):
        return np.mean(data)

@asy fetch_data(url):
    response = await http.get(url)
    return response.json()

mn main():
    data = [1, 2, 3, 4, 5]
    mean = python.analyze(data)
    print(f"Mean: {mean}")
````

When writing GUL code:

- Use `@` prefix for annotations
- Use `?` prefix for mutable variables
- Use `mn main():` for entry point
- Leverage multi-language integration for specialized tasks
- Follow automatic code organization (GUL handles file structure)

```

---

## 📚 Next Steps

1. **Read the full documentation**: Check out [SYNTAX.md](SYNTAX.md) and [STRUCTURE.md](STRUCTURE.md)
2. **Try the examples**: Explore the `examples/` directory
3. **Join the community**: Visit our [GitHub Discussions](https://github.com/gul-lang/gul/discussions)
4. **Take the course**: Complete the beginner and intermediate courses at `http://localhost:8080/course`

---

## 🆘 Getting Help

- **Documentation**: [https://gul-lang.org/docs](https://gul-lang.org/docs)
- **GitHub Issues**: [https://github.com/gul-lang/gul/issues](https://github.com/gul-lang/gul/issues)
- **Discord**: [https://discord.gg/gul-lang](https://discord.gg/gul-lang)
- **Stack Overflow**: Tag your questions with `gul-lang`

---

**Happy coding with GUL!** 🚀
```
