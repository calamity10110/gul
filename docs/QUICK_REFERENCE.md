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
let discount = 0.2            # Default Float
let age = @int(30)            # Integer
let score = @float(95.5)      # Float also accepts @flt(95)
let active = @bool(true)      # Boolean
let tuple = (1, 2)            # Tuple (treated as List)
```

### String Interpolation

```gul
let name = "Alice"
let msg = f"Hello {name}"     # F-String
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
let numbers = @list[1, 2, 3]        # Literal immutable
var alpha = @list["a", "b", "c"]   # String mutable
let matrix = @list[[1, 2], [3, 4]]  # Multi-dimensional immutable
var matrix2 = @list[[1, 2], [3, 4], [5, 6]]  # Multi-dimensional mutable

# Methods
# Methods
numbers.insertbefore(99) # Insert at begin
numbers.insertbefore(0, 99) # Insert at index, count from begin
numbers.insertafter(0, 99) # Insert at index, count from end 
numbers.insertafter(99) # insert to end
numbers.remove(2)           # Remove by value
numbers.pop()               # Remove from end
numbers.pop(0)              # Remove at index
numbers.clear()             # Empty list
numbers.contains("C")       # Membership verify
numbers.len()               # Length property
numbers[0]  # First element
numbers[-1]  # Last element
```

### Sets

```gul
let tags = @set{"a", "b"}        # Literal immutable
var tags = @set{"a", "b"}        # convert to mutable

# Methods
# Methods
tags.add("c")             # Add element
tags.contains("C")        # Membership verify
tags.remove("b")          # Remove element
tags.clear()              # Empty set
tags.len()                # Length property
```

### Dictionaries

```gul
let config = @dict{host: "localhost", port: 8080}  # Immutable
var cfg = @dict{host: "localhost", port: 8080}  # Mutable

# Methods
# Methods
config.contains("port")     # Membership verify
config.len()                # Length property

# Access
cfg[key]  # By identifier
cfg["key"]  # By string

# Insertion methods (maintains order)
cfg.insertbefore(position, key: value)  # Insert at position/default begin
cfg.insertbefore(target_key, key: value)  # Insert before key
cfg.insertafter(key: value)  # Insert at end (default)
cfg.insertafter(target_key, key: value)  # Insert after key
cfg.add(key: value)  # Append at end

# Removal
cfg.remove(position)  # By position
cfg.remove(key)  # By key
cfg.remove(key: value)  # By key-value pair

```

---

## Functions

### Basic

```gul
@fn  greet(name: str) -> str:
    return "Hello, " + name

@fn  add(a: int, b: int) -> int:
    return a + b

@fn  process(data: borrow list, config: move dict):
    # Ownership: borrow (read-only), move (consume)
    pass
```

### Arrow Functions

```gul
let double = (x) => x * 2
let add = (a, b) => a + b
```

### Async Functions

```gul
@async fetch_data(url: str) -> dict:
    let response = await http.get(url)
    return response.json()
```

### Input/Output

```gul
@fn  greet(name(std.input)):
    print("Hello " + name)
```

---

## Structs

```gul
struct Point:
    x: float
    y: float

    @fn  distance(self) -> float:
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
    "hello" => print("Greeting")
    val => print("Captured: " + val) # Variable binding
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
    @fn  fib(n: u64) -> u64 {
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
