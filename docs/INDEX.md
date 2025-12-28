# 📚 GUL Documentation Index

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

Complete documentation for the GUL programming language with MCP integration.

---

## 🚀 Quick Start

- **[Quick Reference](QUICK_REFERENCE.md)** - v3.2 syntax cheat sheet
- **[First Program](guides/first-program.md)** - Your first GUL program
- **[Quick Start Guide](guides/quickstart.md)** - Get started in 5 minutes
- **[MCP Quick Start](guides/MCP_QUICKSTART.md)** - AI assistant setup

---

## 📖 Guides

### Getting Started

- [Introduction](guides/introduction.md) - Language overview
- [Installation](guides/installation.md) - Setup and install
- [First Program](guides/first-program.md) - Hello World and beyond
- [Quick Start](guides/quickstart.md) - 5-minute guide

### Development

- [Web Development](guides/web-development.md) - Build web applications
- [Web Server](guides/web-server.md) - Create HTTP servers
- [Data Analysis](guides/data-analysis.md) - Data science with GUL
- [Data Engineering](guides/data-engineering.md) - SaaS data pipelines 🆕
- [Microservices](guides/microservices-guide.md) - Polyglot services 🆕
- [Scientific Computing](guides/scientific-computing.md) - Math and science
- [IoT/Embedded](guides/iot-embedded.md) - Hardware programming

### Tools & IDEs

- [TUI IDE](guides/tui.md) - Terminal IDE
- [WebUI](guides/webui.md) - Browser-based IDE
- [MCP Integration](guides/MCP_QUICKSTART.md) - AI assistant integration

### Advanced Topics

- [Data-Flow](guides/dataflow.md) - Node-based programming
- [Compiler](guides/compiler.md) - Compilation details
- [Integration](guides/integration.md) - Integrate with other languages
- [Creating Packages](guides/creating-packages.md) - Build packages
- [Secrets Management](guides/secrets.md) - Handle secrets

### Learning

- [GUL Course](guides/course.md) - Complete learning path

---

## 📚 Reference

### Language

- [Syntax](reference/syntax.md) - Complete v3.2 syntax
- [Types](reference/types.md) - Type system with @ prefix
- [Ownership](reference/ownership.md) - Memory model
- [Specification](reference/specification.md) - Language specification
- [Knowledge Base](reference/knowledgebase.md) - Comprehensive guide
- [Structure](reference/structure.md) - Program structure

### Packages

- [Package Catalog](reference/package-catalog.md) - **180 packages** across 22 categories 🆕
  - Authentication & Authorization (8)
  - Developer Tools (12)
  - DevOps & Infrastructure (14)
  - API & Integration (10)
  - Caching & Performance (8)
  - Database Extensions (12)
  - Security & Compliance (10)
  - Testing & QA (10)
  - Multi-Tenancy & SaaS (8)
  - Mobile & Desktop (8)
  - Data Engineering (12)
  - Microservices (10)
  - 3D Modeling (8)
  - Science & Engineering (14)
  - Plus 48 more packages

---

## 🔌 API Documentation

### Standard Library

- [Standard Library](api/standard-library.md) - Core modules overview
- [Math & Science](api/math-science.md) - Mathematical functions
- [HTTP](api/http.md) - Web requests and servers
- [Database](api/database.md) - SQL and NoSQL operations
- [Filesystem](api/filesystem.md) - File operations
- [UI Components](api/ui-components.md) - User interface

### MCP Server

- [MCP Complete](api/MCP_COMPLETE.md) - MCP server API
- [MCP Quickstart](guides/MCP_QUICKSTART.md) - Getting started
- [MCP Advanced](../MCP_ADVANCED.md) - Advanced features

---

## 🎯 Platform Support

- [Supported Platforms](targets/platforms.md) - OS and architecture support

---

## 📊 Project Information

- [Project Status](PROJECT_STATUS.md) - Current state
- [Development History](devhistory.md) - Evolution from v1.0 to v3.2
- [v3.2 Update](V32_UPDATE_COMPLETE.md) - Latest changes
- [Database Integration Tests](../examples/database-integration-tests.md) - Public DB tests 🆕

---

## 🤖 MCP Features

### AI-Powered Development

- **Code Generation**: Natural language → Working code
- **Auto-Maintenance**: Automated linting, formatting, testing
- **Workflows**: Multi-step automation
- **Scheduling**: Triggered tasks

### Interfaces

- **CLI**: `gul-mcp` command-line tool
- **TUI**: Terminal dashboard
- **WebUI**: Browser interface
- **Protocol**: MCP for AI assistants

### Documentation

- [MCP Quick Start](guides/MCP_QUICKSTART.md)
- [MCP Complete Guide](api/MCP_COMPLETE.md)
- [MCP Advanced Features](../MCP_ADVANCED.md)

---

## 📂 Directory Structure

```
docs/
├── README.md                  # This file
├── QUICK_REFERENCE.md         # Syntax cheat sheet
├── PROJECT_STATUS.md          # Current status
├── devhistory.md             # Development history
├── V32_UPDATE_COMPLETE.md    # v3.2 changes
│
├── guides/                   # Learning guides
│   ├── introduction.md
│   ├── quickstart.md
│   ├── first-program.md
│   ├── MCP_QUICKSTART.md
│   ├── web-development.md
│   ├── data-analysis.md
│   ├── iot-embedded.md
│   ├── tui.md
│   └── ...
│
├── reference/                # Language reference
│   ├── syntax.md
│   ├── types.md
│   ├── ownership.md
│   ├── specification.md
│   ├── knowledgebase.md
│   └── package-catalog.md
│
├── api/                      # API documentation
│   ├── standard-library.md
│   ├── math-science.md
│   ├── http.md
│   ├── database.md
│   ├── MCP_COMPLETE.md
│   └── ...
│
└── targets/                  # Platform docs
    └── platforms.md
```

---

## 🔍 Search by Topic

### Web Development

- [Web Development Guide](guides/web-development.md)
- [HTTP API](api/http.md)
- [Web Server Tutorial](guides/web-server.md)

### Data Science & Engineering

- [Data Analysis](guides/data-analysis.md)
- [Data Engineering](guides/data-engineering.md) - SaaS pipelines 🆕
- [Scientific Computing](guides/scientific-computing.md)
- [Math & Science API](api/math-science.md)
- [Database Tests](../examples/database-integration-tests.md) 🆕

### Hardware

- [IoT/Embedded Guide](guides/iot-embedded.md)
- [Platforms](targets/platforms.md)

### AI & Automation

- [MCP Quick Start](guides/MCP_QUICKSTART.md)
- [MCP Advanced](../MCP_ADVANCED.md)
- [Integration Guide](guides/integration.md)

---

## 💡 Quick Examples

### Hello World

```gul
@imp std.io

mn:
    print(@str("Hello, GUL v3.2!"))
```

### Web Server

```gul
@imp std.http

fn @dict handler(req):
    return @dict{status: "ok", message: "Hello!"}

mn:
    http.listen(8080, handler)
```

### Data Analysis

```gul
@imp python{pandas, numpy}

@python {
    df = pd.read_csv("data.csv")
    mean = df['age'].mean()
}
```

### MCP Usage

```bash
# Generate code
gul-mcp generate "sentiment analysis API"

# Auto-maintenance
gul-mcp auto all

# Run workflow
gul-mcp workflow run ci_workflow
```

---

## 📱 Contact & Community

- **GitHub**: https://github.com/calamity10110/gul
- **Issues**: https://github.com/calamity10110/gul/issues
- **Discussions**: https://github.com/calamity10110/gul/discussions

---

## 📝 Contributing

Documentation contributions welcome! See [CONTRIBUTING.md](../CONTRIBUTING.md)

---

**Last Updated**: 2025-12-28  
**Version**: 0.13.0  
**Syntax**: v3.2  
**Status**: Production Ready ✅
