//! # GUL - GUL Universal Language
//!
//! **Version**: 0.14.0-dev | **Syntax**: v3.2 | **Status**: Production Ready
//!
//! GUL is a modern, multi-paradigm programming language that combines:
//! - 🐍 Python's simplicity and readability
//! - 🦀 Rust's safety and performance
//! - ⚡ JavaScript's async capabilities
//! - 🔬 Scientific notation for math and physics
//! - 🌐 Multi-language integration (Python, Rust, JS, SQL)
//!
//! ## Quick Example
//!
//! ```gul
//! @imp std.io
//!
//! fn greet(name: str) -> str:
//!     return "Hello, " + name
//!
//! mn:
//!     let message = greet("World")
//!     print(message)
//! ```
//!
//! ## Features
//!
//! - **v3.2 Syntax**: Modern `let`/`var` keywords with `@` type annotations
//! - **180+ Packages**: Comprehensive ecosystem across 22 categories
//! - **13 Standard Library Modules**: Including networking, HTTP, database
//! - **3 Runtime Integrations**: Python, JavaScript, Rust interop
//! - **MCP Server**: AI-powered development with Model Context Protocol
//! - **Production Ready**: 521 tests passing
//!
//! ## Module Overview
//!
//! - [`lexer`] - Tokenization and lexical analysis
//! - [`parser`] - Parse tokens into Abstract Syntax Tree (AST)
//! - [`ast`] - Abstract Syntax Tree definitions
//! - [`semantic`] - Semantic analysis and type checking
//! - [`compiler`] - Code generation and compilation
//! - [`interpreter`] - Runtime interpretation and execution
//! - [`stdlib`] - Standard library implementation
//! - [`ownership`] - Ownership and borrow checking
//! - [`runtime`] - Runtime operations and foreign code execution
//! - [`mcp`] - Model Context Protocol (AI agent integration)
//! - [`tui`] - Terminal User Interface IDE
//! - [`tools`] - Development tools and utilities
//!
//! ## Getting Started
//!
//! See the [GUL Book](https://github.com/calamity10110/gul/tree/main/docs/book)
//! for comprehensive documentation.
//!
//! ## Links
//!
//! - [GitHub Repository](https://github.com/calamity10110/gul)
//! - [Documentation](https://github.com/calamity10110/gul/tree/main/docs)
//! - [Quick Reference](https://github.com/calamity10110/gul/blob/main/docs/QUICK_REFERENCE.md)
//! - [Examples](https://github.com/calamity10110/gul/tree/main/examples)

// === Pipeline groups ===
/// Frontend: source code → AST pipeline
///
/// Contains lexical analysis, parsing, and semantic analysis stages.
/// - [`ast`] - Abstract Syntax Tree definitions
/// - [`lexer`] - Tokenization and lexical analysis
/// - [`parser`] - Parse tokens into AST
/// - [`semantic`] - Semantic analysis and type checking
pub mod frontend;

/// Backend: AST → execution pipeline
///
/// Contains compilation, code generation, and runtime execution stages.
/// - [`interpreter`] - Runtime interpretation and execution
/// - [`codegen`] - Code generation for various targets
/// - [`vm`] - Virtual machine execution
pub mod backend;

// === Runtime & support ===
/// Runtime operations and foreign code execution
///
/// Executes Python, JavaScript, and Rust code at runtime.
/// Manages dynamic library loading.
pub mod runtime;

/// Standard library implementation
///
/// Provides core modules:
/// - `std.io` - Input/Output
/// - `std.http` - HTTP client/server
/// - `std.json` - JSON parsing
/// - `std.math` - Mathematical functions
/// - `std.collections` - Data structures
/// - `std.crypto` - Cryptography
/// - `std.db` - Database operations
/// - `std.time` - Date/time utilities
/// - `std.net` - Networking (TCP, UDP, WebSocket)
/// - `std.async` - Async runtime
/// - `std.fs` - Filesystem operations
/// - `std.sync` - Synchronization primitives
/// - `std.compress` - Compression (gzip, zip)
pub mod stdlib;

// === Domains ===
/// Domain-specific language features
///
/// Contains specialized functionality for different problem domains:
/// - [`advanced`] - Advanced language features
/// - [`embedded`] - Embedded systems support
/// - [`ai`] - AI integration
/// - [`dataflow`] - Reactive dataflow programming
pub mod domains;

// === Tools & infrastructure ===
/// Development tools and CLI utilities
pub mod tools;

/// Platform-specific code and targets
pub mod platform;

/// Foreign language interoperability
///
/// Enables embedding and calling:
/// - Python code
/// - Rust code
/// - JavaScript/TypeScript
/// - SQL queries
pub mod interop;

/// Model Context Protocol (MCP) server
///
/// AI agent integration for:
/// - Code generation
/// - Auto-maintenance (fmt, lint, check)
/// - Workflow automation
/// - Package management
pub mod mcp;

/// Terminal User Interface (TUI) IDE
///
/// Interactive development environment with:
/// - Code editor
/// - File browser
/// - REPL
/// - Debugger
/// - Package manager UI
pub mod tui;

/// Interactive TUI REPL
///
/// Split-pane REPL with output history, input editing,
/// variables/functions sidebar, and project banner.
pub mod repl;

/// Autonomous code organization and refactoring
pub mod autonomous;

/// Memory management utilities
pub mod memory;

/// Performance benchmarks and profiling
pub mod benchmarks;
