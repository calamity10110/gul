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

## GUL Language Syntax (v3.1)

### Keywords

- **Variables**: `let` (immutable), `var` (mutable)
- **Functions**: `fn` (sync), `async` (async)
- **Imports**: `@imp` (with optional grouping `{...}`)
- **Multi-language**: `@python {...}`, `@rust {...}`, `@sql {...}`
- **Entry point**: `mn:`
- **Ownership**: `borrow`, `ref`, `move`, `kept`

### File Types

- **.mn**: Main files (entry point and orchestration, General purpose files)
- **.def**: Definition files (types, constants, imports)
- **.fnc**: Function files (pure logic)
- **.py**, .rs, .go, .js, .ts, .rb, .php, .java, .kt, .swift, .rs: External language files
- **.scrt**: secret files (credentials, API keys, etc.)

### Examples

```gul
@imp std.io
@imp python{numpy, pandas}

let PI = @float(3.14159)
var @int(counter) = 0

fn greet(@str(name)):
    return @str("Hello, " + name)

fn add(@int(a, b)):
    return @int(a + b)

@python {
    fn analyze(data: list) -> dict {
        import numpy as np
        return {"mean": np.mean(data)}
    }
}

mn:
    print(greet("GUL v3.1!"))
    result = add(1, 2)
    print("Result:", result)
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
