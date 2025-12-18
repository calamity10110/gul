# GUL Language Syntax Reference v3.0

**GUL** (GUL Universal Language) is a unified, multi-paradigm language designed for modern full-stack development.

## Current Implementation Status

This document reflects the **actual implemented syntax** in GUL v0.13.0.

## 1. Keywords

### v3.0 Keywords (Current)

- `let` - Immutable variable declaration
- `var` - Mutable variable declaration
- `fn` - Function declaration
- `async` - Async function
- `import` / `use` - Module imports
- `struct` - Structure definition
- `mn` - Main entry point block

### Control Flow

- `if`, `elif`, `else` - Conditionals
- `for`, `while`, `loop` - Loops
- `in` - Iterator keyword
- `break`, `continue` - Loop control
- `return` - Function return

### Async/Error Handling

- `await` - Await async operations
- `try`, `catch`, `finally` - Exception handling
- `throw` - Throw exceptions

### Ownership (Planned)

- `own` - Exclusive ownership
- `ref` - Borrowed reference
- `copy` - Explicit duplication

### Annotations

- `@global`, `@static`, `@local` - Scope modifiers

### Deprecated (Backward Compatible)

- `const` → use `let`
- `mut` → use `var`
- `def` → use `let` or `fn`
- `imp` → use `import`
- `asy` → use `async`
- `extern` → use `@python`, `@rust`, etc.

## 2. Variables

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

## 3. Functions

### Synchronous Functions

```gul
fn greet(name):
    return "Hello, " + name

fn add(a: int, b: int) -> int:
    return a + b
```

### Async Functions

```gul
async fetch_data(url):
    response = await http.get(url)
    return response.json()
```

## 4. Data Types

### Primitive Types

- `int` - Integer (i64)
- `float` - Floating point (f64)
- `string` - UTF-8 string
- `bool` - Boolean (true/false)

### Collection Types

- `list` - Dynamic array: `[1, 2, 3]`
- `dict` - Hash map: `{name: "Alice", age: 25}`

### Special Types

- `any` - Gradual typing
- `null` - Null value

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

## 6. Imports

### Standard Library

```gul
import std{http, math}
import std.fs
```

### Python Integration

```gul
import python{numpy, pandas}
```

## 7. Main Entry Point

```gul
mn:
    print("Hello, GUL!")
    await run_app()
```

## 8. Operators

### Arithmetic

`+`, `-`, `*`, `/`, `%`, `^`

### Comparison

`==`, `!=`, `<`, `>`, `<=`, `>=`

### Logical

`&&` (and), `||` (or), `!` (not)

### Bitwise

`&`, `|`, `<<`, `>>`

### Assignment

`=`

### Other

`->` (function return type), `.` (member access), `:` (type annotation)

## 9. Comments

```gul
# Single line comment

#[
Multi-line
comment
]#
```

## 10. Indentation

GUL uses **significant whitespace** (Python-style):

- Indent with 4 spaces (or 1 tab = 4 spaces)
- Consistent indentation required
- Blocks defined by indentation level

## 11. UI Components (Experimental)

```gul
# UI sprite syntax
let button = ^&^[button{text="Click", color="blue"}]

# Alternative syntax
ui![button{text="Click", color="blue"}]
```

## 12. Scientific Units (Experimental)

```gul
let speed = 10 m/s
let acceleration = 9.81 m/s^2
let mass = 5 kg
```

## 13. Type Annotations

```gul
fn calculate(x: int, y: float) -> float:
    return x * y

let numbers: list = [1, 2, 3]
let user: dict = {name: "Alice"}
```

## 14. String Literals

```gul
let simple = "Hello"
let multiline = "Line 1
Line 2
Line 3"
```

## 15. Boolean Literals

```gul
let is_active = true
let is_disabled = false
```

## Implementation Notes

- **Lexer**: Tokenizes source code with indentation tracking
- **Parser**: Builds AST from tokens
- **Interpreter**: Executes AST (current implementation)
- **Compiler**: Planned for future releases

## Migration Guide

### From v2.0 to v3.0

```gul
# Old (v2.0)          # New (v3.0)
const x = 5           let x = 5
mut y = 10            var y = 10
def add(a, b):        fn add(a, b):
asy fetch():          async fetch():
```

## See Also

- [Language Specification](specification.md) - Complete language spec
- [Standard Library](../api/standard-library.md) - Built-in functions
- [Examples](../../examples/) - Code examples
