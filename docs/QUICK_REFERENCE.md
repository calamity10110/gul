# GUL v3.0 Quick Reference

**All syntax examples use v3.0 exclusively.**

## Variables

```gul
let x = 10          # Immutable
var y = 20          # Mutable
y = y + 1
```

## Functions

```gul
fn add(a: int, b: int) -> int:
    return a + b

async fetch(url: str) -> dict:
    return await http.get(url)
```

## Main Entry

```gul
mn:
    print("Hello, GUL!")
```

## Imports

```gul
@imp std.http
@imp python{numpy}
```

## Control Flow

```gul
if x > 10:
    print("big")
elif x > 5:
    print("medium")
else:
    print("small")

for i in 0..10:
    print(i)

while x < 100:
    x = x * 2
```

## Data Types

```gul
let num: int = 42
let pi: float = 3.14
let name: str = "GUL"
let active: bool = true
let list: list = [1, 2, 3]
let dict: dict = {name: "Alice"}
```

## Foreign Code

```gul
@python {
    def analyze(data):
        import numpy as np
        return np.mean(data)
}

@rust {
    fn compute(n: i32) -> i32 {
        n * n
    }
}
```

## CLI Commands

```bash
# Package management
gul package list
gul package search <query>
gul package info <name>
gul package install <name>
gul package update <name>
gul package remove <name>
gul package audit
gul package outdated

# AI
gul ai status
gul ai set-provider <provider>

# Runtime
gul runtime python "<code>"
gul runtime js "<code>"
```

## Standard Library (13 Modules)

- `std.fs` - File system
- `std.path` - Path manipulation
- `std.env` - Environment
- `std.time` - Time/date
- `std.process` - Process management
- `std.random` - Random numbers
- `std.crypto` - Cryptography
- `std.collections` - Data structures
- `std.string` - String utilities
- `std.bytes` - Binary data
- `std.http` - HTTP (experimental)
- `std.websocket` - WebSocket (experimental)
- `std.tcp` - TCP sockets (experimental)
- `std.udp` - UDP sockets (experimental)

## Keywords

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

**Version**: v3.0 (Enforced)  
**Last Updated**: 2025-12-18
