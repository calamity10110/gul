# Agent Guidelines for GUL Language Repository

## Build/Test Commands

- **Build**: `cargo build --verbose`
- **Test all**: `cargo test --all-features --verbose`
- **Test single**: `cargo test <test_name>` (e.g., `cargo test test_sql_bridge_creation`)
- **Doc tests**: `cargo test --doc --verbose`
- **Lint**: `cargo clippy --all-targets --all-features -- -D warnings`
- **Format check**: `cargo fmt -- --check`
- **Format**: `cargo fmt`
- **Security audit**: `cargo audit`

## GUL Language Syntax (v2.0)

### Keywords

- **Variables**: `const` (immutable), `mut` (mutable)
- **Functions**: `fn` (sync), `async` (async)
- **Imports**: `import` (with optional grouping `{...}`)
- **Multi-language**: `extern language {...}`
- **Entry point**: `mn:`
- **Ownership**: `own`, `ref`, `copy` (unchanged)

### File Types

- **.mn**: General purpose files
- **.def**: Definition files (types, constants, imports)
- **.fnc**: Function files (pure logic)
- **.mn**: Main files (entry point and orchestration)

### Examples

```gul
import std.io
import python{numpy, pandas}

let PI = 3.14159
var counter = 0

fn greet(name: str) -> str:
    return "Hello, " + name

extern python {
    fn analyze(data: list) -> dict {
        import numpy as np
        return {"mean": np.mean(data)}
    }
}

mn:
    print("Hello, GUL v2.0!")
    result = analyze([1, 2, 3, 4, 5])
    print("Analysis:", result)
```

## Code Style Guidelines

- **Language**: Rust 2021 edition
- **Error handling**: Use `Result<T, String>` or `anyhow::Result<T>`; avoid panics
- **Types**: Derive `Debug`, `Clone`, `PartialEq` for data structures
- **Naming**: snake_case for functions/variables, PascalCase for types/structs
- **Imports**: Group by std, external crates, then local modules
- **Async**: Use `tokio` with `async-trait` for async functions
- **Serialization**: Use `serde` with derive macros
- **CLI**: Use `clap` with derive macros for command-line interfaces
- **Testing**: Use `#[cfg(test)]` modules with `#[test]` functions
- **Documentation**: Add doc comments for public APIs
- **Safety**: Prefer safe Rust; use `unsafe` only when necessary with justification
