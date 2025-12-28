# Syntax

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# GUL v3.2 Syntax Reference

**ENFORCED**: This document uses **v3.2 syntax only**. While v2.0 syntax still works for backward compatibility, all new code should use v3.2.

## 1. Variables

### Immutable Variables

```gul
let name = "Alice"
let age = 25
let pi = 3.14159
```

### Mutable Variables

```gul
var count = 0
count = count + 1

var items = [1, 2, 3]
items.push(4)
```

### Type Annotations

```gul
let name: str = "Alice"
var counter: int = 0
let data: list = [1, 2, 3]
```

## 2. Functions

### Synchronous Functions

```gul
fn greet(name: str) -> str:
    return "Hello, " + name

fn add(a: int, b: int) -> int:
    return a + b
```

### Async Functions

```gul
async fetch_data(url: str) -> dict:
    response = await http.get(url)
    return response.json()
```

## 3. Main Entry Point

```gul
mn:
    print("Hello, GUL!")
    let x = 10
    print(x)
```

## 4. Imports

```gul
@imp std.io
@imp std.http

# Grouped imports
@imp python{numpy, pandas}
@imp rust{tokio, serde}

# Block style
@imp:
    std.math,
    std.collections
```

## 5. Control Flow

### If Statements

```gul
if age >= 18:
    print("Adult")
elif age >= 13:
    print("Teenager")
else:
    print("Child")
```

### For Loops

```gul
for i in 0..10:
    print(i)

for item in items:
    print(item)
```

### While Loops

```gul
while count < 10:
    count = count + 1
```

### Loop

```gul
loop:
    if done:
        break
```

## 6. Error Handling

```gul
try:
    let result = risky_operation()
    print(result)
catch error:
    print("Error:", error)
finally:
    cleanup()
```

## 7. Data Types

### Primitives

```gul
let num: int = 42
let pi: float = 3.14159
let name: str = "GUL"
let active: bool = true
```

### Collections

```gul
let numbers: list = [1, 2, 3, 4, 5]
let user: dict = {name: "Alice", age: 25}
```

### Gradual Typing

```gul
var result: any = 42
result = "now a string"
```

## 8. Foreign Code Integration

### Python

```gul
@python {
    fn analyze_data(data):
        import numpy as np
        return np.mean(data)
}
```

### Rust

```gul
@rust {
    fn fast_compute(n: i32) -> i32 {
        n * n
    }
}
```

### SQL

```gul
@sql {
    SELECT * FROM users WHERE age > 18
}
```

## 9. Comments

```gul
# Single line comment

#[
Multi-line
comment
]#
```

## 10. Operators

### Arithmetic

`+`, `-`, `*`, `/`, `%`, `^`

### Comparison

`==`, `!=`, `<`, `>`, `<=`, `>=`

### Logical

`&&`, `||`, `!`

### Bitwise

`&`, `|`, `<<`, `>>`

## Complete Example

```gul
# app.mn

@imp std.http
@imp python{numpy}

let API_URL = "https://api.example.com"
var request_count = 0

fn process_data(data: list) -> dict:
    return {
        "count": len(data),
        "sum": sum(data)
    }

async fetch_api() -> dict:
    response = await http.get(API_URL)
    return response.json()

@python {
    fn analyze(data):
        import numpy as np
        return float(np.mean(data))
}

mn:
    print("Starting...")

    let data = @list[1, 2, 3, 4, 5]
    let stats = process_data(data)
    print("Stats:", stats)

    let avg = analyze(data)
    print("Average:", avg)
```

## v3.2 Keywords

**Variables**: `let`, `var`

**Functions**: `fn`, `async`

**Entry**: `mn:`

**Imports**: `@imp`

**Foreign**: `@python`, `@rust`, `@sql`

**Control**: `if`, `elif`, `else`, `for`, `while`, `loop`, `in`, `break`, `continue`, `return`

**Error**: `try`, `catch`, `finally`, `throw`, `await`

## File Extension

Use `.mn` for all GUL files.

---

**Version**: v3.2 (Enforced)  
**Last Updated**: 2025-12-28
