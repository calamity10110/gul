# GUL Quick Reference

**v3.1 Syntax Cheat Sheet** | **GUL 101**

---

## Variables

```gul
# Immutable bindings
let name = "Alice"
let age = @int(30)           # Type annotation (value style)
let @int(count) = 0          # Type annotation (decorator style)

# Mutable bindings
var total = 0
var message = @str("Hello")  # Type annotation (value style)
var @float(price) = 19.99    # Type annotation (decorator style)
```

> **Note**: Both type annotation styles are equivalent. Choose one for consistency in your project.

---

## Functions

```gul
# Single-line (expression body - implicit return)
fn add(a, b): a + b
async fetch(url): http.get(url)

# Multi-line (block body - explicit return/await)
fn add(a, b):
    return a + b

async fetch(url):
    await http.get(url)

# With explicit types
fn add(@int(a, b)):
    return @int(a + b)
```

---

## Bracket Equivalence (v3.0+)

```gul
# All bracket types are equivalent in GUL
func(a, b)   # Parentheses
func[a, b]   # Brackets
func{a, b}   # Braces

# Same for collections
list1 = [1, 2, 3]
list2 = (1, 2, 3)
list3 = {1, 2, 3}

# Same for named parameters
config = {host: "localhost", port: 8080}
config = [host: "localhost", port: 8080]
config = (host: "localhost", port: 8080)
```

---

## Ownership (GUL 101)

### Ownership Modes

| Mode     | Access    | Ownership                |
| -------- | --------- | ------------------------ |
| `borrow` | Mutable   | Stays with upstream node |
| `ref`    | Read-only | Stays upstream           |
| `move`   | Mutable   | Transfers to downstream  |
| `kept`   | Read-only | Transfers downstream     |

> Ownership is compiler-dictated based on usage patterns.

### In Node Contracts

```gul
node process(data, config) {
    re_in:
        data: borrow @Image(input.png)   # Mutable, no transfer
        config: ref @Config(settings)     # Read-only, no transfer
    re_out:
        result: @Image(output.png)        # Output ownership
}
```

### In Function Signatures

```gul
fn consume(data: move @Image(a.png)):    # Ownership transfers in
    process(data)

fn view(data: ref @Image(a.png)):        # Read-only, ownership stays upstream
    display(data)
```

---

## Main Entry

```gul
# Sequential style
mn:
    print("Hello!")
    result = compute()
    print(result)

# Graph style (data-flow)
mn: [
    input(5) -> double -> print
]
```

---

## Data-Flow (Node System)

```gul
# Define a node
node add {
    re_in:
        a: borrow @int
        b: move @int
    re_out:
        result: @int
}

# Implement the node
fn add(@int(a, b)):
    return @int(a + b)

# Simple pipeline
mn: [
    input(5) -> double -> print
]

# Explicit connections
mn: [
    input(@int(5)) : add : a
    input(@int(3)) : add : b
    add : result -> print
]
```

---

## Imports

```gul
@imp std.http
@imp std.math
@imp python{numpy, pandas}
@imp rust{serde, tokio}
```

---

## Foreign Code

```gul
@python {
    import numpy as np
    result = np.array([1, 2, 3])
}

@rust {
    fn compute() -> i32 { 42 }
}

@c {
    int add(int a, int b) { return a + b; }
}

@sql {
    SELECT * FROM users WHERE age > 18
}
```

---

## Common Patterns

### Error Handling

```gul
try:
    data = await fetch(url)
catch e:
    print("Error:", e)
finally:
    cleanup()
```

### List Comprehension

```gul
squares = [x^2 for x in range(10)]
evens = [x for x in numbers if x % 2 == 0]
```

### Structs

```gul
struct User:
    name: @str
    age: @int

    fn greet(self):
        print("Hello, " + self.name)

# Usage
user = User{name: "Alice", age: 30}
user.greet()
```

---

## Control Flow

```gul
# Conditionals
if x > 0:
    print("positive")
elif x < 0:
    print("negative")
else:
    print("zero")

# For loop
for item in items:
    print(item)

# While loop
while running:
    process()

# Loop with break
loop:
    data = read()
    if data == nil:
        break
    process(data)
```

---

## Types

```gul
# Primitives
@int, @float, @str, @bool

# Collections
@list<T>, @dict<K,V>, @set<T>

# Special
@any, @nil, @fn<Args, Return>
```

---

## Traits

```gul
trait Serialize:
    fn to_json(self): @str
    fn from_json(data: @str): Self

trait Trainable:
    fn train(self, data): Self
    fn predict(self, input): @any

# Apply trait
struct Model with Trainable:
    weights: @list<@float>
```

---

## CLI Commands

```bash
gul run file.mn       # Run a file
gul build file.mn     # Build binary
gul check file.mn     # Syntax check
gul test              # Run tests
gul fmt file.mn       # Format code
gul lint file.mn      # Lint code
gul tui               # Launch TUI IDE
gul new project       # Create new project
gul install pkg       # Install package
```

---

## Quick Examples

### Hello World

```gul
mn:
    print("Hello, World!")
```

### Web Fetch

```gul
@imp std.http

async mn:
    response = await http.get("https://api.example.com/data")
    print(response.json())
```

### Data Pipeline

```gul
mn: [
    read_csv("data.csv")
        -> filter(row: row.age > 18)
        -> transform(row: row.name.upper())
        -> write_csv("output.csv")
]
```

---

**Docs**: [Full Documentation](README.md) | [Language Spec](language_spec.md) | [Tutorials](guides/introduction.md)
