# Compiler

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# Compiler Architecture

## Compilation Pipeline

The compiler follows an 8-stage pipeline:

### STAGE 1 — Tokenizer (Lexer)

- Reads `.mn` source files
- Produces token stream
- Handles:
  - Keywords (`def`, `fn`, `asy`, `imp`, `mn`)
  - Annotations (`@int`, `@asy`, `@ref`, etc.)
  - Mutability token (`?`)
  - Operators (`+`, `-`, `*`, `/`, `^`, etc.)
  - Scientific units (`m/s`, `m/s^2`, etc.)
  - UI syntax (`^÷^[...]`)
  - String literals, numbers, identifiers

### STAGE 2 — Parser

- Consumes token stream
- Produces Abstract Syntax Tree (AST)
- Validates syntax structure
- Handles:
  - Flexible import declarations (brackets, braces, parens)
  - Definition blocks with mutability (`?`)
  - Annotated function definitions (`@asy`, `@fn`)
  - Foreign language blocks (`@cs`)
  - Main entry points
  - UI inline syntax
  - Annotation parsing

### STAGE 3 — AST Builder

- Enriches AST with semantic information
- Resolves symbols and annotations
- Type inference with annotation hints
- Ownership analysis
- Builds symbol tables with mutability tracking

### STAGE 4 — Block Organizer

- Splits AST into package blocks:
  - Imports → `imports.imp`
  - Definitions → `definitions.def`
  - Async functions → `async.asy`
  - Sync functions → `functions.fnc`
  - Foreign blocks → `custom.cs`
  - Main logic → `main.mn`
- Generates `scrt.def` from secret annotations

### STAGE 5 — Semantic Analyzer

- Type checking (validates `@int`, `@str`, etc.)
- Ownership validation (`@own`, `@ref`, `@copy`)
- Mutability validation (checks `?` usage)
- Async/await validation
- Secret leakage detection
- Dead code detection
- Unused import detection

### STAGE 6 — FFI Resolver

- Resolves foreign function interfaces
- Validates Rust/C/Python/JS/TS/SQL blocks
- Generates FFI bindings
- Handles zero-copy optimizations

### STAGE 7 — Code Generation

- Generates intermediate representation (IR)
- Optimizations:
  - Dead code elimination
  - Constant folding
  - Inlining
  - Vectorization
  - Database query optimization
- Async runtime integration
- UI runtime integration

### STAGE 8 — Target Emission

Emits code for multiple targets:

- **Native binaries** (Linux, macOS, Windows)
- **WebAssembly** (WASM)
- **Embedded firmware** (ESP32, RP2040, etc.)
- **Mobile** (Android, iOS via WASM)

## Runtime Components

### Async Executor

- Tokio-inspired async runtime
- Supports:
  - `async`/`await` syntax
  - Futures and promises
  - Concurrent task execution
  - Async I/O

### UI Kernel

- Renders inline UI elements
- Supports:
  - Trees
  - Sliders
  - Buttons
  - Images
  - Sprites
  - Custom widgets
- Backends:
  - Web (HTML5 Canvas)
  - TUI (Terminal)
  - Embedded (framebuffer)

### Secret Engine

- Manages secrets securely
- Features:
  - Encrypted storage
  - Runtime decryption
  - Leakage prevention
  - Auto-redaction on publish

### Database Engine

- SQL integration
- Query optimization
- Token-aware execution
- Vectorized operations

### FFI Bridge

- Zero-copy where possible
- Supports:
  - Rust (native)
  - C (via FFI)
  - Python (via PyO3)
  - JavaScript (via V8/QuickJS)
  - TypeScript (transpile + V8/QuickJS)
  - SQL (via embedded engine)

## Optimization Strategies

### Token-Oriented Runtime

- Recognizes common patterns
- Optimized execution paths for:
  - Math operations
  - Physics calculations
  - Database queries
  - UI rendering

### Vectorization

- SIMD instructions where available
- Batch processing
- Parallel execution

### Database Acceleration

- Query plan optimization
- Index-aware execution
- Lazy evaluation

## Error Handling

- Comprehensive error messages
- Source location tracking
- Suggestions for fixes
- Auto-fix capabilities via linter

## Compiler CLI

```bash
# Compile a project
ulc build main.mn

# Watch mode
ulc watch main.mn

# Organize into blocks
ulc organize main.mn

# Check without building
ulc check main.mn

# Format code
ulc fmt main.mn

# Run linter
ulc lint main.mn

# Target-specific builds
ulc build --target wasm main.mn
ulc build --target esp32 main.mn
ulc build --target linux main.mn
```

## Compiler Configuration

`ulc.toml`:

```toml
[compiler]
target = "native"
optimize = true
debug = false

[runtime]
async = true
ui = true
secrets = true

[output]
format = "binary"
path = "./build"

[linter]
enabled = true
auto_fix = true
strict = false
```
