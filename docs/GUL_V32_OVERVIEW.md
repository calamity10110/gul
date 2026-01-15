# GUL v3.2 Feature Overview

## What is GUL?

**GUL (GUL Universal Language)** is a modern, multi-paradigm programming language that combines:

- Python's simplicity and readability
- Rust's safety and performance  
- JavaScript's async capabilities
- Universal deployment (Desktop, Web, Embedded)

**Current Version**: 0.14.0-dev (Syntax v3.2)

---

## GUL v3.2 Major Features

### 1. **Modern Variable System**

```gul
// Immutable by default
@const name = "Alice"
@const count = 42

// Mutable when needed
@var counter = 0
counter = counter + 1
```

**Why?** Clear distinction between immutable and mutable data, improving code safety.

---

### 2. **Functions with Explicit Outputs**

**Old way (v2.0)**:

```gul
fn add(a, b) -> int:
    return a + b
```

**New way (v3.2)**:

```gul
@fn add(a, b)(result):
    result = a + b
```

**Benefits**:

- Named outputs make dataflow explicit
- Supports multi-output functions: `@fn calc(x)(out1, out2):`
- Better for visual programming and node-based systems

---

### 3. **Async/Await Shortcut**

**Old way**:

```gul
async fn fetch(url) -> dict:
    return await http.get(url)
```

**New way (v3.2)**:

```gul
async fetch(url)(data):
    data = await http.get(url)
```

**Note**: The `fn` keyword is optional after `async`!

---

### 4. **Parallel Processing** (NEW!)

Execute loops in parallel with ease:

```gul
// Parallel for loop
also_for i in 0..1000:
    process_item(i)

// Parallel while loop
@var count = 0
also_while count < limit:
    process_next()
    count = count + 1
```

**Use case**: Data processing, batch operations, parallel computations

---

### 5. **Modern Comment Styles**

```gul
// C-style comments (NEW in v3.2)
# Python-style comments (still supported)
```

Both styles work - use what you prefer!

---

### 6. **UI Component Syntax** (NEW!)

Declarative UI with property blocks:

```gul
@ui button{
    text: "Click Me",
    color: "blue",
    onClick: handle_click
}

@ui text{
    content: "Hello World",
    size: 18
}
```

**Benefits**: Clean, declarative syntax for UI development

---

### 7. **Arrow Functions**

Concise function expressions:

```gul
// Named arrow functions (v3.2 preferred)
@const double(x) => x * 2
@const add(a, b) => a + b

// Use them like any function
@const result = double(5)  // 10
@const sum = add(3, 4)     // 7
```

---

### 8. **Function Node Introspection** (Advanced)

Access function metadata:

```gul
@fn calculate(input_val)(output_val):
    output_val = input_val * 2

// Access node properties
@const fn_node = calculate
fn_node.input_1   // First input parameter
fn_node.output_1  // First output parameter
```

**Use case**: Visual programming, dataflow systems, meta-programming

---

## Core GUL Features (All Versions)

### Multi-Paradigm Support

```gul
// Imperative
for i in 0..10:
    print(i)

// Functional
numbers.map(x => x * 2).filter(x => x > 10)

// Object-Oriented
struct User:
    name: str
    age: int
    
    @fn greet(self)(msg):
        msg = "Hello, I'm " + self.name
```

---

### Foreign Code Integration

Embed other languages directly:

**Python**:

```gul
@python {
    import pandas as pd
    df = pd.read_csv("data.csv")
    result = df['age'].mean()
}
```

**Rust**:

```gul
@rust {
    fn fast_fibonacci(n: u64) -> u64 {
        // High-performance Rust code
        todo!()
    }
}
```

**SQL**:

```gul
@sql {
    SELECT * FROM users WHERE age > 18
}
```

---

### Type System

```gul
// Gradual typing - explicit when needed
@const name: str = "Alice"
@const age: int = 25

// Type inference
@const score = 95.5  // inferred as float
```

---

### Pattern Matching

```gul
match status_code:
    200 => print("OK")
    404 => print("Not Found")
    code => print("Error: " + @str(code))
```

---

### Ownership System

```gul
@fn process(data: borrow list):  // Read-only
    print(data[0])

@fn modify(data: ref list):      // Mutable reference
    data.append(10)

@fn consume(data: move list):    // Transfer ownership
    // Original becomes invalid
```

---

## Platform Support

- **Desktop**: Linux, macOS, Windows
- **Web**: WebAssembly (WASM)
- **Embedded**: ESP32-S3, RP2040
- **Cloud**: Native compilation for server deployment

---

## Quick Start

**Install**:

```bash
git clone https://github.com/calamity10110/gul.git
cd gul
cargo build --release
```

**Run**:

```bash
gul run hello.mn
```

**Example** (`hello.mn`):

```gul
@imp std.io

@fn greet(name)(msg):
    msg = "Hello, " + name

mn:
    @const greeting = greet("World")
    println(greeting)
```

---

## Why GUL v3.2?

1. **Clearer Code**: Explicit outputs make dataflow visible
2. **Better Parallelism**: `also_for` / `also_while` make parallel programming simple
3. **Modern Syntax**: Support for both `//` and `#` comments
4. **UI-First**: Built-in `@ui` component syntax
5. **Backwards Compatible**: Old code still works with deprecation warnings

---

## Resources

- **Documentation**: [docs/](docs/)
- **Quick Reference**: [QUICK_REFERENCE.md](docs/QUICK_REFERENCE.md)
- **Migration Guide**: [v32_migration.md](docs/guides/v32_migration.md)
- **Examples**: [examples/](examples/)
- **Tests**: Run `cargo test --lib` (490 tests passing)

---

## Next Steps

1. Try the interactive walkthrough: `gul run walkthrough_v32.mn`
2. Read the [migration guide](docs/guides/v32_migration.md)
3. Explore [examples/](examples/) for real-world code
4. Check out the [package ecosystem](docs/reference/package-catalog.md) (180+ packages!)

**GUL v3.2** - Write once, run everywhere! 🚀
