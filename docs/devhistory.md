# GUL Development History

**Complete historical record of GUL language evolution**

This document preserves the development history and syntax evolution of GUL for reference purposes. All current documentation uses v3.0 syntax exclusively.

---

## Version History

### v3.0 (Current - 2025-12-18)

- **Status**: Production Ready
- **Syntax**: Modern, clean keywords
- **Breaking Changes**: Deprecated v2.0 keywords (backward compatible)

### v2.1 (2025-12-10)

- **Status**: Stable
- **Features**: Bracket equivalence, UI components
- **Syntax**: Transitional

### v2.0 (2025-12-08)

- **Status**: Deprecated
- **Features**: Type system improvements
- **Syntax**: Legacy keywords

### v1.0 (2025-11-26)

- **Status**: Initial release
- **Features**: Basic language features

---

## Syntax Evolution

### Variable Declarations

#### v1.0 - v2.0 (Legacy)

```gul
# Immutable
const name = "Alice"
const age = 25
const PI = 3.14159

# Mutable
mut counter = 0
mut items = []
```

#### v3.0 (Current)

```gul
# Immutable
let name = "Alice"
let age = 25
let PI = 3.14159

# Mutable
var counter = 0
var items = []
```

**Rationale**: `let`/`var` are more intuitive and align with modern languages.

---

### Main Entry Point

#### v1.0 - v2.0 (Legacy)

```gul
main():
    print("Hello, World!")
```

Alternative v2.0 forms:

```gul
mn main():
    print("Hello")

@main():
    print("Hello")

pub fn main():
    print("Hello")
```

#### v3.0 (Current)

```gul
mn:
    print("Hello, World!")
```

**Rationale**: Shorter, cleaner syntax. `mn:` is unique and instantly recognizable.

---

### Import Statements

#### v1.0 (Legacy)

```gul
imp std.http
imp std.math
```

#### v2.0 (Legacy)

```gul
import std.http
import std.math
```

#### v3.0 (Current)

```gul
@imp std.http
@imp std.math

# Grouped
@imp python{numpy, pandas}

# Block style
@imp:
    std.http,
    std.math
```

**Rationale**: `@imp` decorator style is more consistent with foreign code blocks.

---

### Foreign Code Integration

#### v1.0 - v2.0 (Legacy)

```gul
cs python:
    def analyze(data):
        return sum(data) / len(data)

extern rust:
    fn compute(n: i32) -> i32 {
        n * n
    }
```

#### v3.0 (Current)

```gul
@python {
    def analyze(data):
        return sum(data) / len(data)
}

@rust {
    fn compute(n: i32) -> i32 {
        n * n
    }
}
```

**Rationale**: Decorator syntax is cleaner and more explicit.

---

## Complete v2.0 Syntax Reference

### Keywords (v2.0)

**Variables**:

- `const` - Immutable variable
- `mut` - Mutable variable
- `def` - Definition (v1.0 only)

**Functions**:

- `fn` - Function declaration
- `async` - Async function
- `asy` - Async (v1.0 only)

**Entry Points**:

- `main():` - Main entry point
- `mn:` - Alternative form

**Imports**:

- `import` - Import statement
- `imp` - Import (v1.0 only)

**Foreign Code**:

- `extern` - External code block
- `cs` - Cross-script (v1.0 only)

**Control Flow**:

- `if`, `elif`, `else`
- `for`, `while`, `loop`
- `break`, `continue`, `return`

**Error Handling**:

- `try`, `catch`, `finally`, `throw`
- `await`

### Complete v2.0 Example

```gul
# GUL v2.0 Complete Example

import std.http
import std.math

const API_URL = "https://api.example.com"
mut request_count = 0

fn calculate_stats(data: list) -> dict:
    const total = sum(data)
    const count = len(data)
    return {
        total: total,
        average: total / count
    }

async fetch_data(url: str) -> dict:
    mut response = await http.get(url)
    return response.json()

extern python:
    import numpy as np

    def analyze(data):
        arr = np.array(data)
        return {
            "mean": float(np.mean(arr)),
            "std": float(np.std(arr))
        }

main():
    print("Starting application...")

    const data = [1, 2, 3, 4, 5]
    const stats = calculate_stats(data)
    print("Stats:", stats)

    const analysis = analyze(data)
    print("Analysis:", analysis)

    mut api_data = await fetch_data(API_URL)
    request_count = request_count + 1
    print("API Response:", api_data)
    print("Total requests:", request_count)
```

---

## Migration Guide: v2.0 → v3.0

### Automated Migration

```bash
# Replace const with let
sed -i 's/^const /let /g' *.mn
sed -i 's/ const / let /g' *.mn

# Replace mut with var
sed -i 's/^mut /var /g' *.mn
sed -i 's/ mut / var /g' *.mn

# Replace main(): with mn:
sed -i 's/^main():/mn:/g' *.mn

# Replace import with @imp
sed -i 's/^import /@imp /g' *.mn
```

### Manual Changes Required

1. **Foreign code blocks**: Change `extern python:` to `@python {}`
2. **Import grouping**: Use new grouped syntax
3. **Review type annotations**: Ensure compatibility

---

## Deprecated Features

### v1.0 Features (Removed in v2.0)

- `def` keyword → Use `const` or `fn`
- `imp` keyword → Use `import`
- `asy` keyword → Use `async`
- `?variable` syntax → Use `mut variable`

### v2.0 Features (Deprecated in v3.0)

- `const` keyword → Use `let`
- `mut` keyword → Use `var`
- `main():` syntax → Use `mn:`
- `import` keyword → Use `@imp`
- `extern`/`cs` keywords → Use `@python`, `@rust`, etc.

**Note**: v2.0 syntax still works for backward compatibility but is not recommended.

---

## Ownership Model Evolution

### v2.0 Ownership Syntax

```gul
# Explicit ownership
const own x = value

# Reference borrowing
fn process(ref data):
    # Read-only access
    print(data)

# Mutable borrowing
fn modify(ref mut data):
    # Can modify
    data.append(item)

# Copy semantics
const y = copy x
```

### v3.0 Ownership (Planned)

```gul
# Ownership annotations
let own x = value

# Borrowing
fn process(ref data):
    print(data)

fn modify(ref var data):
    data.append(item)

# Explicit copy
let y = copy x
```

---

## Type System Evolution

### v2.0 Type Annotations

```gul
const name: str = "Alice"
mut count: int = 0
const data: list = [1, 2, 3]
mut result: any = 42

# Function types
fn add(a: int, b: int): int:
    return a + b

# Generic types (planned)
fn identity<T>(x: T): T:
    return x
```

### v3.0 Type Annotations (Current)

```gul
let name: str = "Alice"
var count: int = 0
let data: list = [1, 2, 3]
var result: any = 42

# Function types
fn add(a: int, b: int) -> int:
    return a + b

# Generic types (planned)
fn identity<T>(x: T) -> T:
    return x
```

---

## File Extension History

### v1.0

- `.gul` - All GUL files

### v2.0

- `.mn` - Main entry files
- `.def` - Definition files
- `.fnc` - Function files
- `.cs` - Cross-script files
- `.sct` - Secret files

### v3.0 (Current)

- `.mn` - **Recommended** for all files
- `.def`, `.fnc`, `.cs`, `.sct` - Legacy support

---

## Compiler Evolution

### v1.0 Compiler

- Basic lexer/parser
- Interpreted execution
- Python backend only

### v2.0 Compiler

- Improved type system
- Multi-language support
- Native code generation

### v3.0 Compiler (Current)

- Full v3.0 syntax support
- Backward compatible with v2.0
- 3 runtime backends (Python, JS, Rust)
- Package manager integration
- AI-assisted development

---

## Standard Library Evolution

### v1.0 (5 modules)

- std.io
- std.math
- std.http
- std.json
- std.fs

### v2.0 (10 modules)

- Added: std.db, std.time, std.crypto, std.regex, std.collections

### v3.0 (13 modules)

- Added: std.bytes, std.websocket, std.tcp, std.udp
- Total: **110+ functions**

---

## Package Ecosystem Growth

### v1.0

- 0 packages

### v2.0

- 25 packages (Rust, Python)

### v3.0 (Current)

- **58 packages** (Rust: 10, Python: 10, JavaScript: 10, Multi-language: 28)

---

## Breaking Changes Log

### v2.0 → v3.0

**Syntax Changes** (Backward compatible):

- `const` → `let` (const still works)
- `mut` → `var` (mut still works)
- `main():` → `mn:` (main(): still works)
- `import` → `@imp` (import still works)

**Removed Features**:

- None (full backward compatibility maintained)

**New Features**:

- Decorator-style foreign code blocks
- Grouped imports
- Enhanced package manager
- AI integration

### v1.0 → v2.0

**Breaking Changes**:

- `def` keyword removed
- `imp` keyword removed
- `asy` keyword removed
- `?variable` syntax removed

---

## Future Roadmap

### v3.1 (Planned)

- Enhanced ownership system
- Generic types
- Trait system
- Pattern matching improvements

### v4.0 (Future)

- JIT compilation
- Advanced type inference
- Distributed computing support
- WebAssembly target

---

## References

- [Current Syntax (v3.0)](../reference/syntax.md)
- [Language Specification](../reference/specification.md)
- [Migration Guide](../QUICK_REFERENCE.md)
- [Package Catalog](../reference/package-catalog.md)

---

**Last Updated**: 2025-12-18  
**Current Version**: v3.0  
**Status**: Production Ready

---

**Note**: This document is for historical reference only. All current documentation uses v3.0 syntax exclusively.
