# GUL - GUL Universal Language 🚀

**_A modern, multi-paradigm programming language designed for everyone_**

Version: **0.13.0** | Syntax: **v3.2** | Status: **Production Ready**

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tests](https://img.shields.io/badge/tests-521%20passing-green.svg)](#testing)

---

## 🌟 What is GUL?

GUL is a modern, multi-paradigm programming language that combines the best features of popular languages:

- 🐍 **Python's** simplicity and readability  
- 🦀 **Rust's** safety and performance  
- ⚡ **JavaScript's** async capabilities  
- 🔬 **Scientific notation** for math and physics  
- 🌐 **Multi-language integration** (Python, Rust, JS, SQL)

**Universal Deployment**: Desktop, Web (WASM), Embedded (ESP32, RP2040), Cloud

---

## ✨ Key Features

### Language Features

- ✅ **v3.2 Syntax**: Modern `let`/`var` keywords with `@` type constructors
- ✅ **Gradual Typing**: Optional type annotations with inference
- ✅ **Ownership System**: `borrow`, `ref`, `move`, `kept` modes
- ✅ **Async/Await**: Built-in cooperative multitasking
- ✅ **Pattern Matching**: Rust-style match expressions
- ✅ **Foreign Code**: Embed Python, Rust, JavaScript, SQL directly

### Ecosystem

- 📦 **180+ Packages** across 22 categories
- 📚 **13 Standard Library Modules** (networking, HTTP, database, crypto, etc.)
- 🔧 **3 Runtime Integrations**: Python, JavaScript, Rust
- 🤖 **MCP Server**: AI-powered development with Model Context Protocol
- 🛠️ **Complete Toolchain**: Compiler, REPL, TUI/Web IDE, Package Manager

### Production Ready

- ✅ **521 Tests Passing**
- ✅ **Comprehensive Benchmarks**
- ✅ **Full Documentation**
- ✅ **Example Gallery** (22 examples)
- ✅ **Embedded Support** (ESP32-S3, RP2040)

---

## 🚀 Quick Start

### Installation

```bash
# Clone repository
git clone https://github.com/calamity10110/gul.git
cd gul

# Build from source
cargo build --release

# Or use install script
./update.sh
```

**See**: [Installation Guide](docs/guides/installation.md)

### Hello World

```gul
mn:
    print("Hello, GUL v3.2!")
```

Run it:

```bash
gul run hello.mn
```

### Your First Program

```gul
@imp std.io

fn greet(name: str) -> str:
    return "Hello, " + name

mn:
    let message = greet("World")
    print(message)
```

**Learn More**: [First Program Guide](docs/guides/first-program.md)

---

## 📖 Language Overview

### Variables & Types

```gul
# Immutable variables (default)
let name = "Alice"
let age = 25

# Mutable variables
var count = 0
count = count + 1

# With type constructors
let user = @dict{name: "Bob", age: 30}
let numbers = @list[1, 2, 3, 4, 5]
let tuple = (1, 2)
let msg = f"User: {user['name']}"
```

### Functions

```gul
# Regular function
fn add(a: int, b: int) -> int:
    return a + b

# Async function
async fetch_data(url: str) -> dict:
    let response = await http.get(url)
    return response.json()

# Arrow function
let double = (x) => x * 2
```

### Structs

```gul
struct User:
    name: str
    email: str
    age: int

    fn greet(self) -> str:
        return "Hello, " + self.name

# Create instance
let user = User{
    name: "Alice",
    email: "alice@example.com",
    age: 25
}

print(user.greet())
```

### Control Flow

```gul
# If-else
if age >= 18:
    print("Adult")
elif age >= 13:
    print("Teenager")
else:
    print("Child")

# For loops
for i in 0..10:
    print(i)

# Match expressions
# Match expressions
match status_code:
    200 => print("OK")
    404 => print("Not Found")
    code => print("Error: " + code)
```

### Imports

```gul
# Single module
@imp std.io

# Multiple modules
@imp std{io, http, json}

# Python packages
@imp python{pandas, numpy}

# Block style
@imp:
    std.io,
    std.http,
    python{pandas}
```

### Foreign Code Integration

**Python**:

```gul
@python {
    import pandas as pd
    df = pd.read_csv("data.csv")
    result = df['age'].mean()
}
```

**Rust**:

```gul
@rust {
    fn fast_compute(n: i64) -> i64 {
        (0..n).sum()
    }
}
```

**SQL**:

```gul
@sql {
    SELECT * FROM users WHERE age > 18
}
```

---

## 📚 Documentation

### 📖 Learning Resources

| Resource | Description | Link |
| ---------- | ------------- | ------ |
| **The GUL Book** | Complete language guide | [docs/book/](docs/book/) |
| **Quick Reference** | Syntax v3.2 cheat sheet | [QUICK_REFERENCE.md](docs/QUICK_REFERENCE.md) |
| **Knowledgebase** | Comprehensive reference | [knowledgebase.md](docs/reference/knowledgebase.md) |
| **First Program** | Step-by-step tutorial | [first-program.md](docs/guides/first-program.md) |
| **Quickstart** | Get started in 5 minutes | [quickstart.md](docs/guides/quickstart.md) |

### 📝 Language Reference

| Topic | Link |
| ------- | ------ |
| **Specification** | [specification.md](docs/reference/specification.md) |
| **Syntax v3.2** | [syntax.md](docs/reference/syntax.md) |
| **Types** | [types.md](docs/reference/types.md) |
| **Ownership** | [ownership.md](docs/reference/ownership.md) |
| **Structure** | [structure.md](docs/reference/structure.md) |

### 🔌 API Documentation

| Category | Link |
| ---------- | ------ |
| **Standard Library** | [standard-library.md](docs/api/standard-library.md) |
| **HTTP & Networking** | [http.md](docs/api/http.md) |
| **Filesystem** | [filesystem.md](docs/api/filesystem.md) |
| **Math & Science** | [math-science.md](docs/api/math-science.md) |
| **Scientific Computing** | [scientific-computing.md](docs/api/scientific-computing.md) |
| **Foreign Integration** | [integration.md](docs/api/integration.md) |
| **UI Components** | [ui-components.md](docs/api/ui-components.md) |
| **Compiler** | [compiler.md](docs/api/compiler.md) |

### 🎓 Comprehensive Guides

| Guide | Description | Link |
| ------- | ------------- | ------ |
| **Web Development** | Build web apps & APIs | [web-development.md](docs/guides/web-development.md) |
| **Web Server** | HTTP server guide | [web-server.md](docs/guides/web-server.md) |
| **Web UI** | Frontend development | [webui.md](docs/guides/webui.md) |
| **Microservices** | Microservices architecture | [microservices-guide.md](docs/guides/microservices-guide.md) |
| **Data Engineering** | Data pipelines & ETL | [data-engineering.md](docs/guides/data-engineering.md) |
| **Data Science** | ML & analytics | [data-science.md](docs/guides/data-science.md) |
| **Data Analysis** | Data analysis workflows | [data-analysis.md](docs/guides/data-analysis.md) |
| **Database** | Database integration | [database.md](docs/guides/database.md) |
| **Dataflow** | Reactive programming | [dataflow.md](docs/guides/dataflow.md) |
| **TUI Development** | Terminal UI apps | [tui.md](docs/guides/tui.md) |
| **IoT & Embedded** | ESP32, RP2040 development | [iot-embedded.md](docs/guides/iot-embedded.md) |
| **Secrets Management** | Secure credentials | [secrets.md](docs/guides/secrets.md) |
| **Creating Packages** | Package development | [creating-packages.md](docs/guides/creating-packages.md) |
| **Course** | Complete learning path | [course.md](docs/guides/course.md) |

### 🤖 MCP (AI Agent Integration)

| Resource | Link |
| ---------- | ------ |
| **MCP Quickstart** | [MCP_QUICKSTART.md](docs/guides/MCP_QUICKSTART.md) |
| **MCP Server** | [MCP_SERVER.md](docs/api/MCP_SERVER.md) |
| **MCP Advanced** | [MCP_ADVANCED.md](docs/api/MCP_ADVANCED.md) |
| **MCP README** | [MCP-README.md](docs/MCP-README.md) |

### 🎯 Development & Deployment

| Resource | Link |
| ---------- | ------ |
| **Package Catalog** | [package-catalog.md](docs/reference/package-catalog.md) |
| **Packages Implemented** | [PACKAGES_IMPLEMENTED.md](docs/PACKAGES_IMPLEMENTED.md) |
| **Production Deployment** | [PRODUCTION_DEPLOYMENT.md](docs/PRODUCTION_DEPLOYMENT.md) |
| **Embedded Build & Flash** | [build_and_flash_embedded.md](docs/build_and_flash_embedded.md) |

---

## 💻 Examples

Explore **22 examples** in the [`examples/`](examples/) directory:

### Getting Started

- [hello_world.mn](examples/hello_world.mn) - Your first GUL program
- [beginner_tutorial.mn](examples/beginner_tutorial.mn) - Learn the basics
- [revised_syntax_demo.mn](examples/revised_syntax_demo.mn) - v3.2 syntax tour

### Web & API

- [web_api_v32.mn](examples/web_api_v32.mn) - REST API with v3.2 syntax
- [web_fetch.mn](examples/web_fetch.mn) - HTTP client example

### Data Processing

- [data_processing_v32.mn](examples/data_processing_v32.mn) - Modern data pipelines
- [dataflow_calculator.mn](examples/dataflow_calculator.mn) - Reactive calculations
- [sql_query.mn](examples/sql_query.mn) - Database queries

### Foreign Code

- [python_inline.cs](examples/python_inline.cs) - Python integration
- [rust_accelerated.cs](examples/rust_accelerated.cs) - Rust performance
- [js_inline.cs](examples/js_inline.cs) - JavaScript interop
- [ts_inline.cs](examples/ts_inline.cs) - TypeScript support
- [c_inline.mn](examples/c_inline.mn) - C integration

### UI & TUI

- [ui_slider.mn](examples/ui_slider.mn) - UI components
- [tui_demo.rs](examples/tui_demo.rs) - Terminal UI demo
- [new_tui_demo.rs](examples/new_tui_demo.rs) - Modern TUI

### Embedded

- [embedded_blink.mn](examples/embedded_blink.mn) - LED blink for ESP32/RP2040

### Showcase

- [v32_showcase.mn](examples/v32_showcase.mn) - All v3.2 features
- [v32_verify.mn](examples/v32_verify.mn) - Syntax verification

### Testing

- [database-integration-tests.md](examples/database-integration-tests.md) - Database tests

---

## 🛠️ CLI Commands

### Build & Run

```bash
# Build project
gul build
gul build --release          # Optimized build

# Run code
gul run main.mn
gul run --watch main.mn      # Auto-reload

# Check code
gul check                    # Type check
gul fmt                      # Format code
gul lint                     # Lint code
```

### Package Management

```bash
# List packages (180 total)
gul package list
gul package list --language rust

# Search & info
gul package search auth
gul package info gul-auth

# Install & update
gul package install gul-auth
gul package update gul-auth
gul package remove gul-auth

# Maintenance
gul package audit            # Security audit
gul package outdated         # Check updates
```

### IDE & Development Tools

```bash
# IDEs
gul ide                      # TUI IDE
gul ide --mode web           # Web IDE
gul repl                     # Interactive REPL

# MCP Server
gul-mcp serve --port 3000    # Start MCP server
gul-mcp tui                  # MCP dashboard
gul-mcp webui --port 8080    # Web UI

# Code generation
gul-mcp generate "REST API"
gul-mcp create my-app --type web

# Auto-maintenance
gul-mcp auto fmt             # Format
gul-mcp auto lint            # Lint
gul-mcp auto check           # Type check
gul-mcp auto all             # All checks
```

### Runtime Operations

```bash
# Execute foreign code
gul runtime python "print('Hello from Python')"
gul runtime js "console.log('Hello from JS')"

# Load Rust libraries
gul runtime load-lib path/to/lib.so
```

### GUL Testing

```bash
# Run tests
gul test
gul test main.mn             # Specific file
cargo test --lib             # Rust tests

# Benchmarks
cargo bench
```

### Documentation

```bash
# Generate docs
gul doc
cargo doc --open             # Rust docs
```

---

## 📦 Package Ecosystem

**180+ packages** across **22 categories**:

### Core Categories

- 🔐 **Authentication & Authorization**: OAuth, JWT, RBAC
- 🛠️ **Developer Tools**: Logging, Debugging, Profiling
- ⚙️ **DevOps**: CI/CD, Monitoring, Deployment
- 🔌 **API & Integration**: REST, GraphQL, gRPC, WebSockets
- 💾 **Caching**: Redis, Memcached, In-memory
- 🗄️ **Database Extensions**: ORMs, Migrations, Pooling
- 🔒 **Security**: Encryption, Hashing, Rate Limiting
- ✅ **Testing**: Unit, Integration, E2E, Mocking
- 🏢 **Multi-Tenancy**: Tenant isolation, management
- 📱 **Mobile & Desktop**: Cross-platform UI
- 📊 **Data Engineering**: ETL, Pipelines, Processing
- 🚀 **Microservices**: Service mesh, Discovery
- 🎨 **3D Modeling**: Graphics, Rendering
- 🔬 **Scientific Computing**: NumPy-like arrays, ML

### Standard Library (13 Modules)

- `std.io` - File I/O and streams
- `std.http` - HTTP client/server
- `std.json` - JSON parsing
- `std.math` - Mathematical functions
- `std.collections` - Data structures (Map, Set, Queue)
- `std.crypto` - Cryptography
- `std.db` - Database abstraction
- `std.time` - Date/time utilities
- `std.net` - Networking (TCP, UDP, WebSocket)
- `std.async` - Async runtime
- `std.fs` - Filesystem operations
- `std.sync` - Synchronization primitives
- `std.compress` - Compression (gzip, zip)

**See**: [Package Catalog](docs/reference/package-catalog.md) | [Packages Implemented](docs/PACKAGES_IMPLEMENTED.md)

---

## 🧪 Testing & Quality

### Test Suite

```bash
# Run all tests (521 passing)
cargo test --lib

# Specific test suites
cargo test --lib --test integration
cargo test --lib --test parser
cargo test --lib --test interpreter

# With output
cargo test --lib -- --nocapture
```

### Benchmarks

```bash
# Run benchmarks
cargo bench

# Specific benchmark
cargo bench --bench parser_bench
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint (no warnings)
cargo clippy

# Type check
cargo check
```

**Status**: ✅ 521 tests passing | ✅ Zero warnings | ✅ Production ready

---

## 🎯 Platform Support

### Desktop

- ✅ Linux (x64, ARM)
- ✅ macOS (Intel, Apple Silicon)
- ✅ Windows (x64)

### Web

- ✅ WebAssembly (WASM)
- ✅ Browser targets
- ✅ Node.js integration

### GUL Embedded

- ✅ ESP32-S3 (Xtensa)
- ✅ RP2040 (ARM Cortex-M0+)
- ✅ RISC-V support (planned)

**See**: [Platform Targets](docs/targets/platforms.md) | [IoT Guide](docs/guides/iot-embedded.md) | [Build & Flash](docs/build_and_flash_embedded.md)

---

## 🌟 Use Cases

### Web Development

Build modern web applications and APIs:

```gul
@imp std.http

async handle_request(req):
    return @dict{
        status: "ok",
        data: "Hello, API!"
    }

mn:
    http.listen(8080, handle_request)
```

**Guide**: [Web Development](docs/guides/web-development.md)

### Data Science

Leverage Python's ecosystem:

```gul
@imp python{pandas, numpy}

@python {
    df = pd.read_csv("data.csv")
    mean_age = df['age'].mean()
}

mn:
    print("Average age:", python.mean_age)
```

**Guide**: [Data Science](docs/guides/data-science.md)

### Microservices

Build scalable microservices:

```gul
@imp std.http
@imp std.db

async get_users():
    return await db.query("SELECT * FROM users")

mn:
    http.get("/users", get_users)
    http.listen(3000)
```

**Guide**: [Microservices](docs/guides/microservices-guide.md)

### Embedded Systems

Control hardware directly:

```gul
@imp embedded.gpio

mn:
    let led = gpio.pin(2, OUTPUT)
    
    loop:
        led.toggle()
        sleep(1000)
```

**Guide**: [IoT & Embedded](docs/guides/iot-embedded.md)

---

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. **Fork** the repository
2. **Clone** your fork: `git clone https://github.com/YOUR_USERNAME/gul.git`
3. **Create** a branch: `git checkout -b feature/amazing-feature`
4. **Make** your changes
5. **Test**: `cargo test --lib`
6. **Commit**: `git commit -m "Add amazing feature"`
7. **Push**: `git push origin feature/amazing-feature`
8. **Open** a Pull Request

### Development Setup

```bash
# Install dependencies
cargo build

# Run tests
cargo test --lib

# Format code
cargo fmt

# Run linter
cargo clippy
```

See: [Contributing Guide](CONTRIBUTING.md) (coming soon)

---

## 📜 License

MIT License - See [LICENSE](LICENSE) for details.

---

## 🔗 Quick Links

### GUL Documentation

- 📖 [The GUL Book](docs/book/)
- 📝 [Quick Reference](docs/QUICK_REFERENCE.md)
- 📚 [Knowledgebase](docs/reference/knowledgebase.md)  
- 🔍 [Language Specification](docs/reference/specification.md)

### Guides

- 🚀 [Quickstart](docs/guides/quickstart.md)
- 👶 [First Program](docs/guides/first-program.md)
- 🌐 [Web Development](docs/guides/web-development.md)
- 📊 [Data Engineering](docs/guides/data-engineering.md)

### References

- 📦 [Package Catalog](docs/reference/package-catalog.md)
- 🔌 [Standard Library](docs/api/standard-library.md)
- 🤖 [MCP Quickstart](docs/guides/MCP_QUICKSTART.md)

### Resources

- 🐙 [GitHub](https://github.com/calamity10110/gul)
- 💬 [Discussions](https://github.com/calamity10110/gul/discussions)
- 🐛 [Issues](https://github.com/calamity10110/gul/issues)

---

## 🎓 Learn More

### Beginner Track

1. [Installation](docs/guides/installation.md)
2. [First Program](docs/guides/first-program.md)
3. [Quickstart](docs/guides/quickstart.md)
4. [The GUL Book - Basics](docs/book/01_basics.md)

### Intermediate Track

1. [Functions & Ownership](docs/book/02_functions.md)
2. [Modules](docs/book/03_modules.md)
3. [Web Development](docs/guides/web-development.md)
4. [Database](docs/guides/database.md)

### Advanced Track

1. [Advanced Concepts](docs/book/04_advanced.md)
2. [Microservices](docs/guides/microservices-guide.md)
3. [Data Engineering](docs/guides/data-engineering.md)
4. [Creating Packages](docs/guides/creating-packages.md)

**Start building with GUL v3.2 today!** 🚀

[Get Started](docs/guides/quickstart.md) • [Documentation](docs/book/) • [Examples](examples/) • [GitHub](https://github.com/calamity10110/gul)

---
