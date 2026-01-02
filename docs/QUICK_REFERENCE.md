# GUL v3.2 Quick Reference

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-30

---

## Table of Contents

1. [Type System](#type-system)
2. [Variables](#variables)
3. [Collections](#collections)
4. [Functions](#functions)
5. [Structs](#structs)
6. [Control Flow](#control-flow)
7. [Foreign Code](#foreign-code)
8. [MCP Commands](#mcp-commands)

---

## Type System

### Primitive Types (@ prefix)

```gul
let name = "Alice"            # Default String
let name2 = @str("Alice")     # Explicit String
let age = @int(30)            # Integer
let score = @float(95.5)      # Float also accepts @flt(95)
let active = @bool(true)      # Boolean
```

## Variables

```gul
let x = @int(10)     # Immutable 
var y = @int(20)     # Mutable

y = 30               # OK
# x = 15             # Error: immutable
```

---

## Collections

### Collection Types

```gul
let numbers = @list[1, 2, 3]           # List (immutable)
let tags = @set{"a", "b", "c"}         # Set (immutable)
let user = @dict{name: "Bob", age: 25} # Dictionary (immutable)

var numbers = @list[1, 2, 3]           # List (mutable)
var tags = @set{"a", "b", "c"}         # Set (mutable)
var user = @dict{name: "Bob", age: 25} # Dictionary (mutable)
```

### Lists

```gul
var items = @list(1, 2, 3)

items.add(4)
items.insertbefore(0, 0)
let val = items[0]
```

### Sets

```gul
var tags = @set{"rust", "python"}
tags.add("go")
if "rust" in tags:
    print("Found")
```

### Dictionaries

```gul
var cfg = @dict{host: "localhost", port: 8080}
cfg.add(ssl: true)
let p = cfg["port"]
```

---

## Functions

### Basic

```gul
fn greet(name: str) -> str:
    return "Hello, " + name

fn add(a: int, b: int) -> int:
    return a + b
```

### Arrow Functions

```gul
let double = (x) => x * 2
let add = (a, b) => a + b
```

### Async Functions

```gul
async fetch_data(url: str) -> dict:
    let response = await http.get(url)
    return response.json()
```

### Input/Output

```gul
fn greet(name(std.input)):
    print("Hello " + name)
```

---

## Structs

```gul
struct Point:
    x: float
    y: float

    fn distance(self) -> float:
        return math.sqrt(self.x^2 + self.y^2)

# Usage
let p = Point{x: 3.0, y: 4.0}
print(p.distance())
```

---

## Control Flow

### If/Else

```gul
if x > 10:
    print("Large")
elif x > 5:
    print("Medium")
else:
    print("Small")
```

### Loops

```gul
while count < 10:
    count = count + 1

for item in items:
    print(item)

for i in 0..10:
    print(i)
```

### Match

```gul
match value:
    1 => print("One")
    _ => print("Other")
```

---

## Foreign Code

### Python

```gul
@imp python{pandas}

@python {
    import pandas as pd
    df = pd.read_csv("data.csv")
    mean = df['age'].mean()
}

let result = python.mean
```

### Rust

```gul
@rust {
    fn fib(n: u64) -> u64 {
        // Rust code
        n
    }
}
```

### SQL

```gul
@sql {
    SELECT * FROM users
}
```

---

## MCP Commands

```bash
# Generate code
gul-mcp generate "REST API"

# Auto tasks
gul-mcp auto all

# Dashboard
gul-mcp tui
```

---

## Entry Point

```gul
mn:
    print("Hello, World!")
```

## Error Handling

```gul
try:
    risky()
catch e:
    print(e)
finally:
    cleanup()

# Result Type
match divide(10, 0):
    Ok(v) => print(v)
    Err(e) => print(e)
```
