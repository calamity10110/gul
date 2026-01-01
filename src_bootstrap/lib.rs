//! # GUL - GUL Universal Language
//!
//! **Version**: 0.13.0 | **Syntax**: v3.2 | **Status**: Production Ready
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

/// Advanced language features and constructs
pub mod advanced;

/// AI integration and autonomous features
pub mod ai;

/// Abstract Syntax Tree (AST) definitions
///
/// Core data structures representing parsed GUL code.
/// Includes expressions, statements, and type annotations.
pub mod ast;

/// Autonomous code organization and refactoring
pub mod autonomous;

/// Performance benchmarks and profiling
pub mod benchmarks;

/// Code generation and compilation to various targets
///
/// Supports compilation to:
/// - Native machine code
/// - WebAssembly
/// - Interpreted bytecode
pub mod compiler;

/// Reactive dataflow programming support
pub mod dataflow;

/// Embedded systems support (ESP32, RP2040)
pub mod embedded;

/// Foreign language interoperability
///
/// Enables embedding and calling:
/// - Python code
/// - Rust code
/// - JavaScript/TypeScript
/// - SQL queries
pub mod interop;

/// Runtime interpreter and execution engine
///
/// Executes GUL programs with support for:
/// - Variables and scoping
/// - Function calls
/// - Async/await
/// - Foreign code execution
pub mod interpreter;

/// Lexical analysis and tokenization
///
/// Converts source code into tokens for parsing.
/// Supports v3.2 syntax including `let`, `var`, `@imp`, etc.
pub mod lexer;

/// Model Context Protocol (MCP) server
///
/// AI agent integration for:
/// - Code generation
/// - Auto-maintenance (fmt, lint, check)
/// - Workflow automation
/// - Package management
pub mod mcp;

/// Memory management utilities
pub mod memory;

/// Ownership and borrow checking
///
/// Implements Rust-like ownership with:
/// - `borrow` - Read-only reference
/// - `ref` - Mutable reference
/// - `move` - Transfer ownership
/// - `kept` - Make a copy
pub mod ownership;

/// Parser - converts tokens to AST
///
/// Recursive descent parser supporting:
/// - v3.2 syntax
/// - Type annotations
/// - Pattern matching
/// - Async functions
/// - Foreign code blocks
pub mod parser;

/// Platform-specific code and targets
pub mod platform;

/// Runtime operations and foreign code execution
///
/// Executes Python, JavaScript, and Rust code at runtime.
/// Manages dynamic library loading.
pub mod runtime;

/// Semantic analysis and type checking
///
/// Validates program semantics:
/// - Type checking
/// - Name resolution
/// - Scope analysis
/// - Ownership verification
pub mod semantic;

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

/// Development tools and CLI utilities
pub mod tools;

/// Trait definitions and implementations
pub mod traits;

/// Terminal User Interface (TUI) IDE
///
/// Interactive development environment with:
/// - Code editor
/// - File browser
/// - REPL
/// - Debugger
/// - Package manager UI
pub mod tui;
