# GUL Language Specification v3.2

**Version**: 0.13.0
**Date**: 2025-12-30
**Status**: Production Ready

---

## Table of Contents

### Implemented Sections

1. [Language Overview](#1-language-overview)
2. [Language Structure](#2-language-structure)
3. [Lexical Structure](#3-lexical-structure)
4. [Types](#4-types)
5. [Ownership Model](#5-ownership-model)
6. [Variables](#6-variables)
7. [Expressions](#7-expressions)
8. [Statements](#8-statements)

---

This specification is based on the comprehensive GUL [knowledgebase](knowledgebase.md) and defines the complete language syntax, semantics, and behavior. For implementation details, see the [Compiler Guide](../guides/compiler.md).

For a quick reference, see [QUICK_REFERENCE.md](../QUICK_REFERENCE.md).

---

## 1. Language Overview

### 1.1 Purpose of GUL

GUL aims to be a "universal" language, suitable for:

- Systems programming
- Web development
- Data science
- Scientific computing
- Embedded systems
- IoT applications

### 1.2 Design Philosophy

**Core Principles:**

- **Readability First**: Syntax should be obvious and clean
- **Safety by Default**: Memory safety without GC pause times
- **Interoperability**: Seamless integration with existing ecosystems (Python, Rust, C, JavaScript, SQL)
- **Performance**: Native compilation for maximum speed
- **Simplicity**: Python-like syntax with Rust-like safety

### 1.3 Key Philosophies

- **Simplicity:** Indentation-based, readable syntax
- **Performance:** Native compilation via Rust-based toolchain
- **Safety:** Strong ownership model (`borrow`, `move`, `kept`) without garbage collection
- **Autonomy:** Built-in AI and self-organization features
- **Universality:** Runs on Desktop, Web (WASM), and Embedded devices

### 1.4 Compilation Pipeline Summary

```text
Source → Lexer → Parser → AST → Semantic Analysis → IR → CodeGen (LLVM/Rust/WASM)
```

### 1.5 Execution Model

Compiled to native machine code or WebAssembly. No VM required for pure GUL code.

### 1.6 Supported Paradigms

- **Imperative**: Procedural programming with clear control flow
- **Functional**: Lisp-style list operations, first-class functions
- **Object-Oriented**: Structs with methods
- **Async/Concurrent**: Native async/await support

---

## 2. Language Structure

### 2.1 File Types (v3.2)

GUL uses a single-file authoring approach (`.mn`) which the compiler can optionally separate during build/publish.

| Extension | Name              | Purpose                     | Content                                |
| --------- | ----------------- | --------------------------- | -------------------------------------- |
| `.mn`     | Main              | Entry point and user code   | `mn:` block, functions, imports        |
| `.sct`    | Secret Credential | API keys, passwords, tokens | Ignored by compiler; stub on publish   |

### 2.2 Module System

Modules map to file paths:

- `@imp std.math` imports `std/math.mn` or the `std/math` package
- Folders are packages
- Each package can have a `package.toml` manifest

### 2.3 Naming Rules

- **Variables/Functions**: `snake_case` (e.g., `my_variable`, `calculate_sum`)
- **Types/Structs**: `PascalCase` (e.g., `MyStruct`, `UserData`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `MAX_RETRY`, `API_VERSION`)
- **Private**: Prefix with `_` (e.g., `_internal_function`)

### 2.4 Code Blocks & Indentation

GUL uses significant indentation (4 spaces recommended). Blocks are denoted by `:`.

```gul
if x > 0:
    print("Positive")
    process(x)
```

### 2.5 Comments

- **Single line**: `#` comment
- **Multi-line**: `#[ ... ]#` block comment

```gul
# This is a single-line comment

#[
This is a
multi-line comment
]#
```

---

## 3. Lexical Structure

### 3.1 Character Set

UTF-8 encoding is required for all source files.

### 3.2 Tokens

The lexer produces keywords, identifiers, literals, operators, delimiters, and special tokens.

### 3.3 Identifiers

- Start with letter or `_`
- Followed by letters, numbers, or `_`
- Case-sensitive

### 3.4 Keywords (v3.2)

**Primary Keywords:**

- `let`, `var` (Variables)
- `@fn`, `return`, `@async`, `await` (Functions)
- `mn:` (Entry point)
- `struct`, `enum` (Types)
- `if`, `elif`, `else`, `match` (Conditional)
- `for`, `while`, `loop`, `break`, `continue` (Loops)
- `try`, `catch`, `finally`, `throw` (Error Handling)
- `borrow`, `ref`, `move`, `kept` (Ownership)
- `@imp` (Import)
- `@python`, `@rust`, `@sql`, `@js` (Foreign Blocks)

**Constants:** `true`, `false`, `null`

**Deprecated Keywords:** `def`, `imp`, `main`, `cs`

### 3.5 Literals

**String Literals:**

```gul
"hello"
"""multi-line"""
f"formatted {var}"
```

**Numeric Literals:**

```gul
123          # Integer
12.34        # Float 
0xFF         # Hex
```

**Collection Literals:**

```gul
(1, 2)              # Set
[1, 2, 3]           # List
{key: value}        # Dictionary
```

---

## 4. Types

### 4.1 Primitive Types

- **`int`**: 64-bit signed integer (`@int(10)`)
- **`float`**: 64-bit IEEE 754 float (`@float(10.5)`), @flt(24)
- **`bool`**: Boolean (`@bool(true)`)
- **`str`**: UTF-8 string (`@str("hello")`)

### 4.2 Collection Types

**Lists (Dynamic Arrays):**

```gul
let numbers = @list[1, 2, 3]        # Literal immutable
var numbers = @list[1, 2, 3]        # Literal mutable
let matrix = @list[[1, 2], [3, 4]]  # Multi-dimensional immutable
var matrix = @list[[1, 2], [3, 4]]  # Multi-dimensional mutable

#### Methods
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

**Dictionaries (Key-Value Maps):**

```gul
let config = @dict{host: "localhost", port: 8080}  # Immutable
var cfg = @dict{host: "localhost", port: 8080}  # Mutable

#### Methods
config.contains("port")     # Membership verify
config.len()                # Length property
config[key]  # access by identifier
config(host)
config["key"]  # access by string
config("host")

#### Insertion methods (maintains order)
cfg.insertbefore(position, key: value)  # Insert at position/default begin
cfg.insertbefore(target_key, key: value)  # Insert before key
cfg.insertafter(key: value)  # Insert at end (default)
cfg.insertafter(target_key, key: value)  # Insert after key
cfg.add(key: value)  # Append at end

#### Removal
cfg.remove(position)  # By position
cfg.remove(key)  # By key
cfg.remove(key: value)  # By key-value pair

```

**Sets (Unique Collections):**

```gul
let tags = @set{"a", "b"}
var tags = @set{"a", "b"}

#### Methods
tags.add("c")             # Add element
tags.contains("C")        # Membership verify
tags.remove("b")          # Remove element
tags.clear()              # Empty set
tags.len()                # Length property
```

### 4.3 Type Annotations

```gul
let name = "Alice" # Inferred as String
let name = @str("Alice") # Explicit String with @str
let name: str = "Alice" # Explicit String with type annotation
let x = 10         # Inferred as float
var count: int = 0 # Explicit Int with type annotation
let tags = @int(24) # Explicit Int with @int
```

### 4.4 Type Inference

GUL infers types where possible:

```gul
let x = 10         # Inferred as float
let name = "Alice" # Inferred as str
```

### 4.5 Type Coercion

```gul
let x = 10         # float
var y = "20"       # str
let z = x + y      # type mismatch error
let z = x + @flt(y) # convert y to float and match
```

---

## 5. Ownership Model

### 5.1 Overview

GUL uses a move-by-default system for non-primitive types, ensuring memory safety without GC.

### 5.2 Ownership Modes

- **`borrow`**: not transfer ownership, immutable Read-only copy (default for reading).
- **`kept`**: transfer ownership to downstream, Immutable Read-only data.
- **`ref`**: not transfer ownership, mutable copy.
- **`move`**: transfer ownership to downstream, mutable data.

### 5.3 Examples

```gul
@fn  process(data: borrow @list):
    #/ immutable 'data' is not owned by process here, upstream caller keeps it
    print(data[0])
@fn  consume(data: kept @list):
    #/ immutable 'data' is copied here, downstream caller owns it
    data.push(4)
@fn  consume(data: ref @list):
    #/ mutable 'data' is copied here, downstream caller owns it
    data.push(4)
@fn  consume(data: move @list):
    #/ mutable 'data' is owned by consume here, upstream caller loses it
    data.push(4)
```

### 5.4 Move Semantics

Assignment transfers ownership for non-copy types:

```gul
let list1 = @list[1, 2, 3]
let list2 = list1  # list1 is now invalid (removed from memory)
```

---

## 6. Variables

### 6.1 Declarations

```gul
let name = "Alice"      # Immutable
var count = 0           # Mutable
```

### 6.2 Mutability

```gul
let x = 10
# x = 20  # Error: immutable

var y = 10
y = 20    # OK
```

### 6.3 Global Variables

```gul
@global var config = {}
```

---

## 7. Expressions

### 7.1 Arithmetic

Standard operators: `+`, `-`, `*`, `/`, `%`, `^`.

### 7.2 Comparison

`==`, `!=`, `<`, `>`, `<=`, `>=`.

### 7.3 Logical

`&&`, `||`, `!`.

---

## 8. Statements

### 8.1 Control Flow

**If/Else:**

```gul
if x > 0:
    print("Pos")
elif x < 0:
    print("Neg")
else:
    print("Zero")
```

**Loops:**

```gul
for i in 0..10:
    print(i)

while count < 10:
    count = count + 1
```

**Match:**

```gul
match value:
    1 => print("One")
    "hello" => print("Greeting")
    val => print("Captured value: " + val)
    _ => print("Other")
```

**Variable Binding:**

Identifiers in match arms bind the matched value to a new variable, inheriting the type (Float, Int, String, etc.).

```gul
let x = 3.14
match x:
    3.14 => print("Pi")
    other => print("Not Pi: " + other) // other is float
```

Pipeline Operator (|>)

```gul
let x = 3.14
x |> print
```

### 8.2 Imports

```gul
@imp std.io
@imp python{pandas}
```

### 8.3 Entry Point

```gul
mn:
    print("Start")
```

---

## 9. Testing & Verification

### 9.1 Test File Location

| Path | Description |
| ------ | ----------- |
| `compilers/shared/tests/` | Cross-compiler compatibility tests |
| `compilers/nightly/tests/` | Nightly-specific feature tests |

### 9.2 Running Tests

```bash
# Run nightly test suite
cd compilers/nightly && python3 tests/run_tests.py

# Run specific test
./target/release/gulc tests/test_control_flow.gul -o test && ./test
```

### 9.3 CI/CD

Automated testing via GitHub Actions:

- `.github/workflows/ci.yml` - Main compiler tests

See [testfiles.md](../testfiles.md) for complete test documentation.
