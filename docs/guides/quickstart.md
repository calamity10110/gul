# Quickstart

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# Quick Start Tutorial - GUL v3.2

Get up and running with GUL v3.2 in just 5 minutes!

**All examples use v3.2 syntax exclusively.**

## ⚡ Installation

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and build GUL
git clone https://github.com/gul-lang/gul.git
cd gul
cargo build --release

# Optionally install globally
cargo install --path .
```

## 🎯 Your First Program

Create a file called `hello.mn`:

```gul
# hello.mn

mn:
    print("Hello, GUL!")
    print("Welcome to v3.2!")
```

Run it:

```bash
cargo run -- run hello.mn
# or if installed globally:
gul run hello.mn
```

Output:

```text
Hello, GUL!
Welcome to v3.2!
```

## 🔢 Variables (v3.2 Syntax)

```gul
mn:
    # Immutable variables
    let name = "Alice"
    let age = 30
    let height = 5.7
    let is_active = true

    # Mutable variables
    var score = 100
    score = score + 10

    # Type annotations
    let price: float = 19.99
    var count: int = 0

    print(f"{name} is {age} years old")
```

## 🔄 Control Flow

```gul
mn:
    # If statements
    let age = 25
    if age >= 18:
        print("Adult")
    elif age >= 13:
        print("Teenager")
    else:
        print("Child")

    # For loops
    for i in 0..5:
        print(i)

    # While loops
    var count = 0
    while count < 3:
        print(count)
        count = count + 1
```

## 📦 Functions

```gul
# Simple function
@fn greet(name: str) -> str:
    return "Hello, " + name

# Typed function
@fn add(a: int, b: int) -> int:
    return a + b

# Async function
@async fetch_data(url: str) -> dict:
    let response = await http.get(url)
    return response.json()

mn:
    print(greet("Alice"))
    let result = add(5, 3)
    print(f"5 + 3 = {result}")
```

## 📊 Data Structures

```gul
mn:
    # Lists
    let numbers = @list[1, 2, 3, 4, 5]
    var items = @list[1, 2, 3]
    items.push(4)
    print(numbers[0])  # 1

    # Dictionaries
    let person = @dict{
        name: "Bob",
        age: 30,
        city: "NYC"
    }
    print(person.name)  # Bob
```

## 🌐 Imports (v3.2)

```gul
@imp std.http
@imp std.math

# Grouped
@imp python{numpy, pandas}

# Block style
@imp:
    std.fs,
    std.collections

mn:
    let result = math.sqrt(16)
    print(result)  # 4.0
```

## 📁 File Operations

```gul
@imp std.fs

mn:
    # Write to file
    fs.write_file("message.txt", "Hello from GUL!")

    # Read from file
    let content = fs.read_file("message.txt")
    print(content)

    # List directory
    let files = fs.list_dir(".")
    for file in files:
        print(file)
```

## 📦 Package Management

```bash
# List all packages
gul package list

# Search for packages
gul package search web

# Get package info
gul package info actix-web

# Install a package
gul package install actix-web

# Audit packages
gul package audit
```

## 💡 CLI Commands

```bash
# Run a program
gul run file.mn

# Package management
gul package list
gul package search <query>
gul package info <name>

# AI configuration
gul ai status
gul ai set-provider openai

# Runtime operations
gul runtime python "print('hello')"
gul runtime js "console.log('hello')"
```

## 🚀 Next Steps

1. **[Introduction Guide](../guides/introduction.md)** - Complete overview
2. **[Syntax Reference](../reference/syntax.md)** - v3.2 syntax guide
3. **[Standard Library](../api/standard-library.md)** - Built-in modules
4. **[Package Catalog](../reference/package-catalog.md)** - Available packages

## 🎓 Key Concepts

- **mn:** - Main entry point (v3.2)
- **let/var** - Immutable/mutable variables (v3.2)
- **@fn** - Function declaration
- **@async** - Async functions
- **@imp** - Module imports (v3.2)

## 📚 Resources

- [Language Specification](../reference/specification.md)
- [Standard Library API](../api/standard-library.md)
- [Quick Reference](../QUICK_REFERENCE.md)
- [Package Catalog](../reference/package-catalog.md)

---

**Congratulations!** 🎉 You've completed the GUL v3.2 Quick Start!

---

**Last Updated**: 2025-12-28  
**Version: 0.13.0  
**License**: MIT
