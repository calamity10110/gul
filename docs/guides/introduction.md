# Getting Started with GUL v3.2

Welcome to GUL (GUL Universal Language)! This guide uses **v3.2 syntax exclusively**.

**Version**: 0.13.0 | **Status**: Production Ready

---

## What is GUL?

GUL is a modern, multi-paradigm programming language that combines:

- 🐍 **Python's** simplicity and readability
- 🦀 **Rust's** safety and performance
- ⚡ **JavaScript's** async capabilities
- 🔬 **Scientific notation** for math and physics
- 🌐 **Multi-language integration** (Python, Rust, JS, SQL)

## Key Features

✅ **v3.2 Syntax** - Modern `let`/`var` keywords  
✅ **58 Packages** - Cross-language ecosystem  
✅ **13 Stdlib Modules** - Including networking  
✅ **3 Runtimes** - Python, JavaScript, Rust  
✅ **AI Integration** - Multi-provider support  
✅ **Production Ready** - 0 errors, 0 warnings

## Installation

### Build from Source

```bash
# Clone the repository
git clone https://github.com/gul-lang/gul.git
cd gul

# Build with Cargo
cargo build --release

# Install locally
cargo install --path .
```

### Verify Installation

```bash
gul --version
# Should output: gul 0.13.0
```

## Your First GUL Program

Create a file called `hello.mn`:

```gul
# hello.mn

mn:
    print("Hello, GUL!")
    print("Welcome to v3.2!")
```

Run it:

```bash
gul run hello.mn
```

Output:

```text
Hello, GUL!
Welcome to v3.2!
```

## Basic Syntax (v3.2)

### Variables

```gul
# Immutable (v3.2)
let name = "Alice"
let age = 25

# Mutable (v3.2)
var count = 0
count = count + 1
```

### Functions

```gul
# Simple function
fn greet(name: str) -> str:
    return "Hello, " + name

# Typed function
fn add(a: int, b: int) -> int:
    return a + b

# Async function
async fetch_data(url: str) -> dict:
    response = await http.get(url)
    return response.json()
```

### Control Flow

```gul
# If statements
if age >= 18:
    print("Adult")
elif age >= 13:
    print("Teenager")
else:
    print("Child")

# For loops
for i in 0..10:
    print(i)

# While loops
while count < 10:
    count = count + 1
```

### Data Structures

```gul
# Lists
let numbers = @list[1, 2, 3, 4, 5]
let names = @list["Alice", "Bob", "Charlie"]

# Dictionaries
let user = {
    name: "Alice",
    age: 25,
    active: true
}
```

## Imports (v3.2)

```gul
@imp std.http
@imp std.math

# Grouped
@imp python{numpy, pandas}

# Block style
@imp:
    std.fs,
    std.collections
```

## Example: Web Server

```gul
@imp std.http

let PORT = 8080

async handle_request(request):
    return {
        status: 200,
        headers: {"Content-Type": "application/json"},
        body: {message: "Hello from GUL!"}
    }

mn:
    let server = http.Server(PORT)
    server.on("request", handle_request)
    await server.start()
    print(f"Server running on http://localhost:{PORT}")
```

## Example: Data Analysis

```gul
@imp python{numpy, pandas}

@python {
    import pandas as pd
    import numpy as np

    fn analyze(filename):
        df = pd.read_csv(filename)
        return {
            "mean": float(df['value'].mean()),
            "std": float(df['value'].std())
        }
}

mn:
    let stats = analyze("data.csv")
    print("Statistics:", stats)
```

## Package Management

```bash
# List all packages
gul package list

# Search for packages
gul package search web

# Get package info
gul package info actix-web

# Install a package
gul package install actix-web
```

## CLI Commands

### Package Management

```bash
gul package list [--language rust]
gul package info <name>
gul package search <query>
gul package install <name>
gul package update <name>
gul package remove <name>
gul package audit
gul package outdated
```

### AI Configuration

```bash
gul ai status
gul ai set-provider <provider>
gul ai set-model <model>
gul ai set-key <key>
```

### Runtime Operations

```bash
gul runtime python "<code>"
gul runtime js "<code>"
gul runtime load-lib <path>
```

## Next Steps

1. **[Syntax Reference](../reference/syntax.md)** - Complete v3.2 syntax guide
2. **[Quick Start Tutorial](../tutorials/quickstart.md)** - Learn by doing
3. **[Standard Library](../api/standard-library.md)** - Built-in modules
4. **[Package Catalog](../reference/package-catalog.md)** - 58 available packages

## Getting Help

- 📚 **Documentation**: [docs/](../README.md)
- 🐛 **Issue Tracker**: [GitHub Issues](https://github.com/gul-lang/gul/issues)
- � **Community**: [Discord](https://discord.gg/gul-lang)

---

**Happy coding with GUL v3.2!** 🚀

**Last Updated**: 2025-12-28  
**Version**: 0.13.0
