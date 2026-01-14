# GUL v3.2 Quick Reference

**Version**: 0.14.0-dev | **Syntax**: v3.2 | **Updated**: 2026-01-08

---

## Table of Contents

1. [Type System](#type-system)
2. [Variables](#variables)
3. [Collections](#collections)
4. [Functions](#functions)
5. [Structs](#structs)
6. [Control Flow](#control-flow)
7. [Foreign Code](#foreign-code)
8. [Autograd](#autograd)
9. [MCP Commands](#mcp-commands)

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

// C-Style comments supported (v3.2)
# Legacy comments also supported
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
len(numbers)               # Length (Builtin)
numbers.len()               # Length (Method)
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
len(tags)                 # Length (Builtin)
tags.len()                # Length (Method)
```

### Dictionaries

```gul
let config = @dict{host: "localhost", port: 8080}  # Immutable
var cfg = @dict{host: "localhost", port: 8080}  # Mutable

# Access
cfg["host"] or cfg.host

# Methods
cfg.insert("key", "value")
cfg.remove("key")
cfg.get("key")
len(cfg)                  # Length (Builtin)
cfg.len()                 # Length (Method)
```

### Tables (@tabl)

```gul
# Table with named columns and rows
let data = @tabl {
    (name, age, score):
    alice: {"Alice", 30, 95.5}
    bob:   {"Bob", 25, 88.0}
}

# Access
data.col_count
data.row_count
```

---

## Functions

### Basic

```gul
@fn  greet(name: str) -> str:
    return "Hello, " + name

@fn  add(a: int, b: int)(res):
    res = a + b

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
# Async shortcut (v3.2)
async fetch_data(url: str)(res):
    res = await http.get(url)
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

for i in 0..10:
    print(i)

# Parallel (also_for, also_while)
also_for i in 0..10:
    process(i)
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

---

## Autograd

Automatic differentiation support for machine learning.

```gul
@fn train():
    autograd_begin()
    
    let x = make_var(10.0)
    let y = make_var(20.0)
    let z = var_add(x, y)
    
    # Forward pass value
    print(var_val(z)) # 30.0
    
    # Backward pass
    backward(z)
    
    # Get gradients
    print(var_grad(x)) # 1.0
    
    autograd_end()
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
