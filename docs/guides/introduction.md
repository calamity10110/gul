# Getting Started with GUL

Welcome to GUL (GUL Universal Language)! This guide will help you get started quickly.

## What is GUL?

GUL is a modern, multi-paradigm programming language that combines:

- 🐍 **Python's** simplicity and readability
- 🦀 **Rust's** safety and performance
- ⚡ **JavaScript's** async capabilities
- 🔬 **Scientific notation** for math, physics, and chemistry
- 🎨 **First-class UI components** as syntax
- 🌐 **Multi-language integration** (Python, Rust, JS, SQL, and more)

## Key Features

✅ **Beginner-Friendly** - Easy to learn, hard to master  
✅ **Production-Ready** - 100% test coverage, battle-tested  
✅ **Multi-Paradigm** - Functional, OOP, async, reactive  
✅ **Auto-Organizing** - Compiler organizes your code automatically  
✅ **Safe by Default** - Rust-inspired ownership model  
✅ **Universal** - Runs everywhere (native, WASM, embedded, mobile)

## Why Choose GUL?

### For Beginners

- Python-like syntax that's easy to read and write
- Helpful error messages guide you to solutions
- Automatic code organization keeps your projects clean

### For Professionals

- Native performance without garbage collection
- Strong type system catches errors at compile-time
- Built-in async/await for concurrent programming
- Seamless integration with existing ecosystems

### For Scientists & Engineers

- Native support for scientific units (m/s, kg, etc.)
- Built-in linear algebra and numerical computing
- Standard physics, chemistry, and biology libraries included

### For Web Developers

- Modern web framework built-in
- Server-side rendering and client-side WASM
- Full-stack development in one language

## Installation

### Quick Install (Linux/macOS) - Not working yet

```bash
## curl -sSf https://gul-lang.org/install.sh | sh
```

### Manual Installation

1. **Download the latest release** from [GitHub Releases]
2. **Extract the archive**:

   ```bash
   tar -xzf gul-v0.13.0-linux-x64.tar.gz
   ```

3. **Add to PATH**:

   ```bash
   export PATH="$PATH:$HOME/.mn/bin"
   ```

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

Create a file called `hello.gul`:

```gul
# hello.gul - Your first GUL program

mn:
    print("Hello, GUL!")
    print("Welcome to the future of programming!")
```

Run it:

```bash
gul run hello.gul
```

Output:

```text
Hello, GUL!
Welcome to the future of programming!
```

## Basic Syntax

### Variables

```gul
# Immutable by default (v3.0)
let name = "Alice"
let age = 25

# Mutable variables (v3.0)
var count = 0
count = count + 1
```

### Functions

```gul
# Simple function
fn greet(name):
    return "Hello, " + name

# Function with type annotations
fn add(a: int, b: int) -> int:
    return a + b

# Async function
async fetch_data(url):
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
let numbers = [1, 2, 3, 4, 5]
let names = ["Alice", "Bob", "Charlie"]

# Dictionaries
let user = {
    name: "Alice",
    age: 25,
    active: true
}

# Tuples (planned)
let point = (10, 20)
```

## Example: Web Server

```gul
import std{http}

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
import python{pandas, matplotlib}

extern python {
    import pandas as pd
    import matplotlib.pyplot as plt

    fn analyze(filename):
        df = pd.read_csv(filename)
        return df.describe()
}

main():
    stats = analyze("data.csv")
    print(stats)
```

## Next Steps

Now that you have GUL installed and running, explore these resources:

1. **[Quick Start Tutorial](../tutorials/quickstart.md)** - Learn the basics step by step
2. **[Syntax Reference](../reference/syntax.md)** - Complete language syntax guide
3. **[Project Structure](../reference/structure.md)** - How GUL organizes code
4. **[Examples](../../examples/)** - Real-world code examples
5. **[Standard Library](../api/standard-library.md)** - Built-in functionality

## Getting Help

- 📚 **Documentation**: You're reading it!
- 💬 **Community Forum**: [community.mn-lang.org](https://community.mn-lang.org)
- 🐛 **Issue Tracker**: [GitHub Issues](https://github.com/gul-lang/gul/issues)
- 💼 **Discord**: Join our [Discord server](https://discord.gg/gul-lang)

## Contributing

GUL is open source and welcomes contributions! See our [Contributing Guide](../project/contributing.md) to get started.

---

**Happy coding with GUL!** 🚀
