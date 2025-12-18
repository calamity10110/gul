# GUL Language Syntax - Actual Implementation

**Version**: 0.13.0 | **Based on**: Real code analysis

## Supported Syntax Versions

GUL supports **multiple syntax versions** with full backward compatibility:

- **v3.0** (Recommended): `let`/`var`, `mn:`, `@imp`
- **v2.0** (Supported): `const`/`mut`, `main():`, `import`
- **Legacy** (Deprecated): `def`, `imp`, `asy`

## Variables

### v3.0 Syntax (Recommended)

```gul
let name = "Alice"      # Immutable
var count = 0           # Mutable
count = count + 1
```

### v2.0 Syntax (Still Works)

```gul
const name = "Alice"    # Immutable
mut count = 0           # Mutable
count = count + 1
```

### Type Annotations

```gul
let name: str = "Alice"
var counter: int = 0
const data: list = [1, 2, 3]
```

## Functions

### Synchronous Functions

```gul
fn greet(name: str) -> str:
    return "Hello, " + name
```

### Async Functions

```gul
# v3.0: No 'fn' keyword
async fetch_data(url: str) -> dict:
    response = await http.get(url)
    return response.json()
```

## Main Entry Point

### v3.0 Style (Recommended)

```gul
mn:
    print("Hello, GUL!")
```

### v2.0 Style (Still Works)

```gul
main():
    print("Hello, GUL!")
```

## Imports

### v3.0 Style

```gul
@imp std.io
@imp std.http

# Grouped
@imp python{numpy, pandas}
@imp rust{tokio, serde}

# Block style
@imp:
    std.math,
    std.collections
```

### v2.0 Style (Still Works)

```gul
import std.math
import std.io
```

## Foreign Code Blocks

### Python Integration

```gul
@python {
    def analyze_data(data):
        import numpy as np
        return np.mean(data)
}
```

### Rust Integration

```gul
@rust {
    fn fast_compute(n: i32) -> i32 {
        n * n
    }
}
```

## Control Flow

```gul
# If statements
if age >= 18:
    print("Adult")
elif age >= 13:
    print("Teenager")
else:
    print("Child")

# For loops
for i in 0..10:
    print(i)

# While loops
while count < 10:
    count = count + 1

# Try/Catch
try:
    print("Testing...")
catch error:
    print("Error:", error)
```

## Data Types

```gul
# Primitives
let num: int = 42
let pi: float = 3.14
let name: str = "GUL"
let active: bool = true

# Collections
let numbers: list = [1, 2, 3]
let data: dict = {name: "Alice", age: 25}

# Gradual typing
mut result: any = 42
```

## Bracket Equivalence (v2.1+)

All bracket types are equivalent:

```gul
print("text")
print["text"]
print{"text"}
```

## File Extensions

- `.mn` - Main/entry files (RECOMMENDED)
- `.def` - Definition files
- `.fnc` - Function files
- `.scrt` - Secret files (encrypted)
- `.cs` - Cross-script files

## Complete Example

```gul
# example.mn

# Imports
@imp std.http
@imp python{numpy}

# Variables
let API_URL = "https://api.example.com"
var request_count = 0

# Function
fn process_data(data: list) -> dict:
    return {
        "count": len(data),
        "sum": sum(data)
    }

# Async function
async fetch_api() -> dict:
    response = await http.get(API_URL)
    return response.json()

# Python integration
@python {
    def analyze(data):
        import numpy as np
        return float(np.mean(data))
}

# Main entry
mn:
    print("Starting application...")

    let data = [1, 2, 3, 4, 5]
    let stats = process_data(data)
    print("Stats:", stats)

    let avg = analyze(data)
    print("Average:", avg)

    let api_data = await fetch_api()
    print("API Response:", api_data)
```

## Migration Guide

### From v2.0 to v3.0

```gul
# v2.0                  # v3.0
const x = 5             let x = 5
mut y = 10              var y = 10
main():                 mn:
import std.math         @imp std.math
```

**Note**: v2.0 syntax still works! No need to migrate immediately.

## Keywords Reference

### Current (v3.0)

`let`, `var`, `fn`, `async`, `mn`, `@imp`, `@python`, `@rust`, `@sql`

### Supported (v2.0)

`const`, `mut`, `main`, `import`, `extern`

### Deprecated (Legacy)

`def`, `imp`, `asy`, `cs`

### Control Flow

`if`, `elif`, `else`, `for`, `while`, `loop`, `in`, `break`, `continue`, `return`

### Error Handling

`try`, `catch`, `finally`, `throw`, `await`

### Ownership (Planned)

`own`, `ref`, `copy`

---

**Last Updated**: 2025-12-18  
**Based on**: Actual code in src/ and examples/
