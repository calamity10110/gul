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
  - Keywords (`let`, `var`, `fn`, `async`, `import`)
  - Annotations (`@int`, `@list`, `@dict`, etc.)
  - Operators (`+`, `-`, `*`, `/`, `^`, etc.)
  - F-Strings and String literals
  - Identifiers and Types

### STAGE 2 — Parser

- Consumes token stream
- Produces Abstract Syntax Tree (AST)
- Validates syntax structure
- Handles:
  - Import declarations (`@imp`)
  - Variable definitions (`let`/`var`)
  - Function definitions (`@fn`, `@async`)
  - Foreign language blocks (`@python`, `@rust`, `@sql`)
  - Main entry points (`mn:`)
  - Control flow (`if`, `match`, `for`)
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
- Ownership validation (`borrow`, `move`, `ref`, `kept`)
- Mutability validation (`var` vs `let`)
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
gul build main.mn

# Watch mode
gul run --watch main.mn

# Check without building
gul check main.mn

# Format code
gul fmt main.mn

# Run linter
gul lint main.mn

# Target-specific builds
gul build --target wasm32-unknown-unknown
gul build --target xtensa-esp32s3-none-elf
gul build --target x86_64-unknown-linux-gnu
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
