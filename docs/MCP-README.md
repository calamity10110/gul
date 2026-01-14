# GUL Documentation

**Version**: 0.14.0-dev | **Syntax**: v3.2 | **MCP**: Enabled

---

## Quick Links

- [Quick Reference](QUICK_REFERENCE.md) - v3.2 syntax cheat sheet
- [MCP Quick Start](guides/MCP_QUICKSTART.md) - AI assistant integration
- [Syntax Guide](reference/syntax.md) - Complete v3.2 syntax
- [Standard Library](api/standard-library.md) - 13 modules, 110+ functions
- [MCP Advanced Features](../MCP_ADVANCED.md) - Scheduling, workflows, automation

---

## 🚀 What's New in v3.2

### @ Prefix Type System

```gul
let name = @str("value")
let count = @int(42)
let items = @list[1, 2, 3]
let config = @dict{key: "value"}
```

### Model Context Protocol (MCP) Server

```bash
# AI assistants can now generate entire applications
gul-mcp generate "sentiment analysis API"

# Automated maintenance
gul-mcp auto all

# Workflow execution
gul-mcp workflow run ci_workflow
```

---

## Getting Started

```gul
@imp std.io

mn:
    let message = @str("Hello, GUL v3.2!")
    print(message)
```

**Run**: `gul run hello.mn`

---

## Documentation Structure

### Guides

- [Introduction](guides/introduction.md) - Language overview
- [Quick Start](guides/quickstart.md) - Get started in 5 minutes
- [First Program](guides/first-program.md) - Your first GUL program
- [MCP Quick Start](guides/MCP_QUICKSTART.md) - AI assistant setup
- [Data-Flow](guides/dataflow.md) - Node-based programming
- [Installation](guides/installation.md) - Setup
- [TUI IDE](guides/tui.md) - Terminal IDE
- [Web Development](guides/web-development.md) - Build web apps
- [IoT/Embedded](guides/iot-embedded.md) - Hardware programming
- [Data Analysis](guides/data-analysis.md) - Data science with GUL

### Reference

- [Syntax](reference/syntax.md) - Complete v3.2 syntax
- [Types](reference/types.md) - Type system with @ prefix
- [Ownership](reference/ownership.md) - Memory model
- [Knowledge Base](reference/knowledgebase.md) - Comprehensive guide
- [Package Catalog](reference/package-catalog.md) - 58 packages

### API

- [Standard Library](api/standard-library.md) - Core modules
- [Math & Science](api/math-science.md) - Scientific computing
- [HTTP](api/http.md) - Web requests
- [Database](api/database.md) - SQL and NoSQL
- [UI Components](api/ui-components.md) - User interfaces
- [MCP Complete](api/MCP_COMPLETE.md) - MCP server API

### Advanced

- [MCP Advanced Features](../MCP_ADVANCED.md) - Complete automation guide
- [Compiler Guide](guides/compiler.md) - Compilation details
- [Integration](guides/integration.md) - Integrate with other languages

### History

- [Development History](devhistory.md) - v1.0 → v3.2 evolution
- [v3.2 Update](V32_UPDATE_COMPLETE.md) - Latest changes
- [Project Status](PROJECT_STATUS.md) - Current state

---

## v3.2 Syntax Highlights

### Type Annotations

```gul
# All types use @ prefix
let name = @str("Alice")
let age = @int(30)
let score = @float(95.5)
let active = @bool(true)
```

### Collections

```gul
# Explicit collection types
let numbers = @list[1, 2, 3, 4, 5]
let point = (10, 20)
let tags = @set{"rust", "python", "js"}
let user = @dict{name: "Bob", age: 25}
```

### Functions

```gul
# Return type with @ prefix
@fn add(a, b) -> int:
    return a + b

@fn greet(name) -> str:
    return "Hello, " + name
```

### Structs

```gul
struct User:
    name: str
    age: int

    @fn display(self) -> str:
        return self.name + " (" + str(self.age) + ")"
```

### Foreign Code

```gul
@python {
    import pandas as pd
    df = pd.read_csv("data.csv")
}

@rust {
   @fn fast_calc(x: i64) -> i64 {
        x * x
    }
}
```

---

## MCP Features

### AI-Powered Development

```bash
# Generate code from description
gul-mcp generate "create a REST API for users"

# Create complete projects
gul-mcp create my-app --type web

# Automated maintenance
gul-mcp auto lint    # Run clippy
gul-mcp auto fmt     # Format code
gul-mcp auto audit   # Security audit

# Workflows
gul-mcp workflow run ci_workflow

# Scheduling
gul-mcp schedule list
```

### Interfaces

- **CLI**: Full command-line interface
- **TUI**: Beautiful terminal dashboard
- **WebUI**: Browser-based management
- **MCP Protocol**: AI assistant integration

---

## Quick Examples

### Web API

```gul
@imp std.http

@fn handle_request(req) -> dict:
    return @dict{status: "ok", data: "Hello!"}

mn:
    http.listen(8080, handle_request)
```

### Machine Learning

```gul
@imp python{tensorflow, numpy}

let model = MLModel{input_shape: 784, classes: 10}
model.train(data, labels, epochs=10)
```

### Data Processing

```gul
@imp python{pandas}

@python {
    df = pd.read_csv("data.csv")
    stats = df.describe()
}
```

---

## Key Features

- ✅ **v3.2 Syntax** - @ prefix type system
- ✅ **MCP Server** - AI assistant integration
- ✅ **58 Packages** - Cross-language ecosystem
- ✅ **13 Stdlib Modules** - Comprehensive standard library
- ✅ **3 Runtimes** - Python, Rust, JavaScript interop
- ✅ **Auto-Maintenance** - AI-powered code quality
- ✅ **Workflows** - Multi-step automation
- ✅ **Scheduling** - Automated task execution

---

## Community

- **GitHub**: <https://github.com/calamity10110/gul>
- **Documentation**: <https://github.com/calamity10110/gul/tree/master/docs>
- **Examples**: <https://github.com/calamity10110/gul/tree/master/examples>

---

**Last Updated**: 2026-01-08  
**Version**: 0.14.0-dev  
**Syntax**: v3.2  
**Status**: Production Ready ✅
