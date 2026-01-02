# Specification

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-30

---

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
- **Interoperability**: Seamless integration with existing ecosystems (Python, Rust, C, JavaScript)
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
- `fn`, `async`, `return`, `await` (Functions)
- `mn` (Entry point)
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
(1, 2)              # Tuple
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

**Lists (4-D Support):**

```gul
let numbers = @list[1, 2, 3]
let matrix = @list[[1, 2], [3, 4]]
```

**Dictionaries:**

```gul
let config = @dict{host: "localhost", port: 8080}
```

**Sets:**

```gul
let tags = @set{"a", "b"}
```

### 4.3 Type Annotations

```gul
let name: str = "Alice"
var count: int = 0
```

### 4.4 Type Inference

GUL infers types where possible:

```gul
let x = 10         # Inferred as int
let name = "Alice" # Inferred as str
```

---

## 5. Ownership Model

### 5.1 Overview

GUL uses a move-by-default system for non-primitive types, ensuring memory safety without GC.

### 5.2 Ownership Modes

- **`borrow`**: Read-only reference (default for reading).
- **`ref`**: Mutable reference.
- **`move`**: Transfer ownership.
- **`kept`**: Create a copy.

### 5.3 Examples

```gul
fn process(data: borrow @list):
    print(data[0])

fn consume(data: move @list):
    # 'data' is owned here, caller loses it
```

### 5.4 Move Semantics

Assignment transfers ownership for non-copy types:

```gul
let list1 = @list[1, 2, 3]
let list2 = list1  # list1 is now invalid (moved)
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
    _ => print("Other")
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
