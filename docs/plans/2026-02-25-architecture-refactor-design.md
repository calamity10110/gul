# Architecture Refactor: Pipeline-Centric Reorganization

**Date**: 2026-02-25
**Status**: Approved
**Scope**: Internal reorganization of `src_bootstrap/`

---

## Problem

The `src_bootstrap/` directory contains 127 .rs files in a flat structure with 20+ top-level
modules. The parser is 2,387 lines, the lexer 1,228 lines, and the interpreter 971 lines.
A new developer cannot quickly understand what lives where or how pieces relate.

## Goal

A new developer can understand the codebase structure in 10 minutes by reading the top-level
directory names.

## Constraints

- Zero behavior changes
- No new crates or dependencies
- Same public API surface
- All 490+ tests must pass
- Packages (125+) are untouched

---

## New Directory Structure

```
src_bootstrap/
├── lib.rs / main.rs          # Entry points
├── frontend/                 # Source -> AST
│   ├── mod.rs
│   ├── ast.rs               # AST node definitions
│   ├── lexer/
│   │   ├── mod.rs           # Lexer struct, public API
│   │   ├── scanner.rs       # Character scanning & tokenization
│   │   ├── tokens.rs        # Token enum & keyword tables
│   │   └── errors.rs        # Lexer error types
│   ├── parser/
│   │   ├── mod.rs           # Parser struct, public API
│   │   ├── expressions.rs   # Expression parsing
│   │   ├── statements.rs    # Statement parsing
│   │   ├── types.rs         # Type annotation parsing
│   │   ├── patterns.rs      # Pattern matching parsing
│   │   └── tests.rs         # Parser tests (moved from inline)
│   └── semantic/
│       ├── mod.rs
│       ├── analyzer.rs      # Semantic analysis
│       ├── ownership.rs     # Ownership/borrow checking
│       └── traits.rs        # Trait system
│
├── backend/                  # AST -> Execution
│   ├── mod.rs
│   ├── interpreter/
│   │   ├── mod.rs           # Interpreter struct, public API
│   │   ├── eval.rs          # Expression evaluation
│   │   ├── exec.rs          # Statement execution
│   │   ├── values.rs        # Value enum & operations
│   │   ├── builtins.rs      # Built-in functions
│   │   └── tests.rs         # Interpreter tests (moved from inline)
│   ├── codegen/
│   │   ├── mod.rs
│   │   ├── blocks.rs        # Package blocks
│   │   ├── builder.rs       # Build system integration
│   │   └── emit.rs          # Code emission
│   └── vm/
│       └── mod.rs           # Bytecode VM
│
├── runtime/                  # OS/IO execution support
│   ├── mod.rs
│   ├── filesystem.rs
│   ├── http_client.rs
│   ├── database.rs
│   ├── async_runtime.rs
│   ├── ffi.rs
│   ├── secrets.rs
│   ├── math_science.rs
│   └── ui_runtime.rs
│
├── stdlib.rs                 # Standard library (single file, 559 lines)
│
├── domains/                  # Domain-specific features
│   ├── mod.rs
│   ├── advanced/            # symbolic_math, physics, chemistry
│   ├── embedded/            # ESP32, RP2040, display, HAL
│   ├── ai/                  # AI integration
│   └── dataflow/            # Reactive dataflow graphs
│
├── tools/                    # Development tools
│   ├── mod.rs
│   ├── tui_ide.rs
│   ├── web_ide.rs
│   ├── debugger.rs
│   ├── linter.rs
│   ├── formatter.rs
│   ├── profiler.rs
│   └── course.rs
│
├── platform/                 # Platform targets & packaging
│   ├── mod.rs
│   ├── wasm_backend.rs
│   ├── mobile_platform.rs
│   ├── embedded_targets.rs
│   ├── package_support.rs
│   ├── package_registry.rs
│   └── ...
│
├── interop/                  # Foreign language integration
│   ├── mod.rs
│   └── python.rs, javascript.rs, rust.rs, c.rs, sql.rs, ...
│
├── mcp/                      # MCP server (AI agent integration)
│   ├── mod.rs
│   └── server.rs, cli.rs, tui.rs, webui.rs, ...
│
├── tui/                      # Terminal UI system
│   ├── mod.rs
│   └── app.rs, widgets/, ...
│
├── autonomous/               # AI-powered code tools
│   ├── mod.rs
│   └── ai_codegen.rs, optimizer.rs, refactoring.rs
│
├── memory/                   # Memory management
│   └── pool.rs, arena.rs
│
├── benchmarks/               # Performance benchmarks
│   └── compiler_bench.rs, runtime_bench.rs
│
└── bin/                      # Binary entry points
    ├── gul-mcp.rs
    └── gul_mcp_codegen.rs
```

---

## File Splitting Details

### parser.rs (2,387 lines -> 5 files)

| File | Content | ~Lines |
|------|---------|--------|
| `parser/mod.rs` | Parser struct, `parse()` API, utility helpers | 300 |
| `parser/expressions.rs` | `parse_expression()`, binary/unary/call/literal, operator precedence | 800 |
| `parser/statements.rs` | `parse_statement()`, let/function/if/loop/struct/import | 800 |
| `parser/types.rs` | `parse_type_annotation()`, generics, trait bounds | 300 |
| `parser/patterns.rs` | `parse_pattern()`, match, destructuring | 200 |

### lexer/mod.rs (1,228 lines -> 3 files)

| File | Content | ~Lines |
|------|---------|--------|
| `lexer/mod.rs` | Lexer struct, `tokenize()` API, character scanning | 500 |
| `lexer/tokens.rs` | Token enum, TokenKind, keyword lookup | 400 |
| `lexer/errors.rs` | Lexer error types, error recovery | 100 |

### interpreter.rs (971 lines -> 4 files)

| File | Content | ~Lines |
|------|---------|--------|
| `interpreter/mod.rs` | Interpreter struct, `run()`, `execute_program()` | 200 |
| `interpreter/eval.rs` | Expression evaluation, operator application | 300 |
| `interpreter/exec.rs` | Statement execution, control flow, scope management | 300 |
| `interpreter/values.rs` | Value enum, type conversions, display | 200 |

### stdlib.rs

Stays as a single file (559 lines).

---

## Import Path Strategy

Clean break. All internal `use crate::` paths updated to the new structure.

**Before:**
```rust
use crate::parser::Parser;
use crate::lexer::Lexer;
use crate::interpreter::Interpreter;
```

**After:**
```rust
use crate::frontend::parser::Parser;
use crate::frontend::lexer::Lexer;
use crate::backend::interpreter::Interpreter;
```

No backward-compatible re-exports.

---

## Test Strategy

- Inline `#[cfg(test)]` blocks follow their modules to new locations
- Parser tests move to `frontend/parser/tests.rs`
- Interpreter tests move to `backend/interpreter/tests.rs`
- Integration tests in `tests/` directory unchanged
- All 490+ tests must pass after refactor

---

## What Does NOT Change

- `Cargo.toml` (no new crates or dependencies)
- `packages/` directory (125+ packages untouched)
- `compilers/stable/` and `compilers/nightly/` (separate workspace members)
- `web/` directory (Dioxus frontend)
- `compiler/` directory (self-hosted compiler in GUL)
- `tests/` directory (integration tests)
- `examples/` directory (25 .mn example programs)
- Any runtime behavior
