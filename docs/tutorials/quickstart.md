# Quick Start Tutorial

Get up and running with GUL v0.13.0 in just 5 minutes!

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

Create a file called `hello.gul`:

```gul
# hello.gul - Your first GUL program

mn:
    print("Hello, GUL!")
    print("Welcome to v3.0!")
```

Run it:

```bash
cargo run -- run hello.gul
# or if installed globally:
gul run hello.gul
```

Output:

```
Hello, GUL!
Welcome to v3.0!
```

## 🔢 Variables and Types (v3.0 Syntax)

```gul
mn:
    # Immutable variables (v3.0)
    let name = "Alice"
    let age = 30
    let height = 5.7
    let is_active = true

    # Mutable variables (v3.0)
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
fn greet(name):
    return "Hello, " + name

# Typed function
fn add(a: int, b: int) -> int:
    return a + b

# Async function
async fetch_data(url):
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
    let numbers = [1, 2, 3, 4, 5]
    var items = [1, 2, 3]
    items.push(4)
    print(numbers[0])  # 1

    # Dictionaries
    let person = {
        name: "Bob",
        age: 30,
        city: "NYC"
    }
    print(person.name)  # Bob
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

## 🌐 Imports

```gul
# Import standard library
import std{http, math}

# Import specific module
import std.fs

# Python integration
import python{numpy, pandas}

mn:
    let result = math.sqrt(16)
    print(result)  # 4.0
```

## 📁 File Operations

```gul
import std{fs}

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

## 🚀 Next Steps

1. **Learn More**: Read the [Introduction Guide](../guides/introduction.md)
2. **Syntax Reference**: Check the [v3.0 Syntax Guide](../reference/syntax.md)
3. **Explore Examples**: Browse [`examples/`](../../examples/)
4. **API Reference**: See the [Standard Library](../api/standard-library.md)

## 🎓 Key Concepts

- **mn:** block - Main entry point (v3.0)
- **let/var** - Immutable/mutable variables (v3.0)
- **fn** - Function declaration
- **async** - Async functions (no `fn` keyword needed)
- **import** - Module imports

## 💡 CLI Commands

```bash
# Run a program
gul run file.gul

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

## 📚 Resources

- [Language Specification](../reference/specification.md)
- [Standard Library API](../api/standard-library.md)
- [Quick Reference](../QUICK_REFERENCE.md)
- [Package Catalog](../reference/package-catalog.md)

---

**Congratulations!** 🎉 You've completed the GUL Quick Start with v3.0 syntax!

---

**Last Updated**: 2025-12-18  
**Version**: 0.13.0  
**License**: MIT
