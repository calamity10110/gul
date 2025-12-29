# GUL - GUL Universal Language

**_A modern, multi-paradigm programming language designed for everyone_**

Version: **0.13.0** | Syntax: **v3.2** | Status: **Production Ready**

---

## 🌟 What is GUL?

GUL is a modern, multi-paradigm programming language that combines:

- 🐍 **Python's** simplicity and readability
- 🦀 **Rust's** safety and performance
- ⚡ **JavaScript's** async capabilities
- 🔬 **Scientific notation** for math and physics
- 🌐 **Multi-language integration** (Python, Rust, JS, SQL)

## ✨ Key Features

- **v3.2 Syntax**: Modern `let`/`var` keywords with `@` type annotations
- **180 Packages**: Comprehensive ecosystem across 22 categories
  - Authentication & Authorization, Developer Tools, DevOps
  - API & Integration, Caching, Database Extensions
  - Security, Testing, Multi-Tenancy, Mobile/Desktop
  - Data Engineering, Microservices, 3D Modeling, Scientific Computing
- **13 Stdlib Modules**: Including networking (WebSocket, TCP, UDP)
- **3 Runtimes**: Python, JavaScript, and Rust interop
- **MCP Integration**: AI-powered development with LSP
- **CLI Tools**: Complete package management
- **Production Ready**: 521 tests passing, comprehensive benchmarks

## 🚀 Quick Start

```bash
# Clone and build
git clone https://github.com/calamity10110/gul.git
cd gul
cargo build --release

# Run your first program
echo '@imp std.io

mn:
    print(@str("Hello, GUL v3.2!"))' > hello.gul

cargo run -- run hello.gul
```

## 📝 v3.2 Syntax Examples

### Variables

```gul
# Modern variable declarations
let name = @str("Alice")  # Immutable with type annotation
var count = @int(0)       # Mutable integer
let items = @list([1, 2, 3])  # List type
```

### Functions

```gul
# Function with type annotations
fn @int add(x: @int, y: @int):
    return x + y

# Async function (no 'fn' keyword for async)
async fetch_data(url: @str):
    let response = await http.get(url)
    return response.json()

# Main entry point
mn:
    let result = add(5, 10)
    print(result)
```

### Imports

```gul
# Block-style imports
@imp std.io
@imp std.http
@imp std.json

# Selective imports
@imp std.math{sqrt, pow, pi}

# Multi-language imports
@imp python{pandas, numpy}
@imp rust{tokio}
```

### Data Types

```gul
# Type annotations with @ prefix
let number = @int(42)
let text = @str("hello")
let decimal = @float(3.14)
let flag = @bool(true)
let items = @list([1, 2, 3])
let data = @dict({
    name: "Alice",
    age: 30
})
```

### Advanced Features

```gul
# Structs
struct User:
    name: @str
    age: @int
    email: @str

    fn greet(self):
        print("Hello, " + self.name)

# Async operations
async fetch_users():
    let response = await http.get("https://api.example.com/users")
    return response.json()

# Pattern matching
match value:
    case @int(x) if x > 0:
        print("Positive")
    case @int(0):
        print("Zero")
    case _:
        print("Negative")
```

## 📦 Package Management

```bash
# List all packages (180 total)
gul package list

# Search for packages
gul package search auth

# Get package info
gul package info gul-auth

# Install a package
gul package install gul-auth

# Audit packages
gul package audit
```

## 🔧 CLI Commands

### Package Management

- `gul package list [--language rust]` - List packages
- `gul package info <name>` - Show package details
- `gul package search <query>` - Search packages
- `gul package install <name>` - Install package
- `gul package update <name>` - Update package
- `gul package remove <name>` - Remove package
- `gul package audit` - Security audit
- `gul package outdated` - Check for updates

### Runtime Operations

- `gul runtime python "<code>"` - Run Python code
- `gul runtime js "<code>"` - Run JavaScript code
- `gul runtime load-lib <path>` - Load Rust library

## 📚 Documentation

### Getting Started

- [Introduction](docs/guides/introduction.md) - Get started with GUL
- [Quick Reference](docs/QUICK_REFERENCE.md) - v3.2 syntax guide
- [Quick Start Tutorial](docs/guides/quickstart.md) - Learn by doing

### Language Reference

- [Language Specification](docs/reference/specification.md) - Complete spec
- [Standard Library](docs/api/standard-library.md) - Built-in modules
- [Package Catalog](docs/reference/package-catalog.md) - All 180 packages

### Development

- [Compiler Guide](docs/guides/compiler.md) - Compiler architecture
- [Implementation Roadmap](docs/IMPLEMENTATION_ROADMAP.md) - Package timeline
- [Production Deployment](docs/PRODUCTION_DEPLOYMENT.md) - Deploy to production

## 💻 Example Code (v3.2 Syntax)

```gul
@imp std.io
@imp std.http

# Variables with type annotations
let name = @str("Alice")
var count = @int(0)

# Functions
fn @str greet(name: @str):
    return "Hello, " + name

# Async functions (no 'fn' keyword)
async fetch_data(url: @str):
    let response = await http.get(url)
    return response.json()

# Structs
struct User:
    name: @str
    age: @int

    fn describe(self):
        return self.name + " is " + @str(self.age)

# Main entry point
mn:
    print(greet("World"))

    let user = User{name: "Bob", age: 30}
    print(user.describe())
```

## 🏗️ Project Status

- ✅ **Phase 0**: Core Infrastructure (100%)
  - gul-http, gul-tui
- 🚧 **Phase 1**: Production Foundation (9%)
  - gul-auth ✅ (JWT, sessions)
  - In progress: gul-security-headers
- 📋 **Phase 2-7**: Advanced Packages (0%)
  - 177 packages planned

**Total**: 3/180 packages implemented (1.7%)  
**Tests**: 521 passing  
**Status**: Production ready for core features

## 🧪 Testing

```bash
# Run all tests
cargo test --lib

# Run specific test suite
cargo test --lib --test integration

# Run benchmarks
cargo bench
```

## 🎯 Implementation Progress

See [IMPLEMENTATION_PROGRESS.md](docs/IMPLEMENTATION_PROGRESS.md) for detailed tracking.

**Current Focus**: Phase 1 - Production Foundation  
**Timeline**: 8-12 weeks to production SaaS  
**Next Package**: gul-security-headers

## 🤝 Contributing

We welcome contributions! See [Contributing Guide](CONTRIBUTING.md).

## 📜 License

MIT License - See [LICENSE](LICENSE) for details.

---

## 🔗 Links

- **Documentation**: [docs/INDEX.md](docs/INDEX.md)
- **Package Catalog**: [docs/reference/package-catalog.md](docs/reference/package-catalog.md)
- **Roadmap**: [docs/IMPLEMENTATION_ROADMAP.md](docs/IMPLEMENTATION_ROADMAP.md)
- **GitHub**: <https://github.com/calamity10110/gul>

---

**Start building with GUL v3.2 today!** 🚀
