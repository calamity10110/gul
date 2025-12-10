# GUL Language Specification v2.0

**GUL** (GUL Universal Language) is a unified, multi-paradigm language designed for modern full-stack development.

## 1. File Structure

GUL uses a strict file organization system to enforce separation of concerns:

- **.def** (Definitions): Imports, constants, global variables, data structures.
- **.fnc** (Functions): Logic, algorithms, and process flows.
- **.mn** (Main): The single entry point for the application.
- **.scrt** (Secrets): Secure, encrypted credentials (stripped from public builds).
- **.cs** (Cross-Script): Embedded foreign code (Python, Rust, SQL, etc.).

---

## 2. Definitions (.def)

The `.def` file is the universal declaration file. It handles imports, constants, and global state.

### 2.1 Imports

Flexible syntax supporting grouping `[]`, `{}`, `()`.

```gul
# Recommended syntax
import std{io, math}
import python{numpy, random}

# Loading external data definitions
import matrix.4d from list.txt
```

### 2.2 Constants

Immutable by default using `def`.

```gul
const MAX_GUESSES = 5
const API_VERSION = "2.0"
```

### 2.3 Global State

Global mutable state must be marked with `@global` and `?`.

```gul
@global ?game_state = {
    high_score: 0,
    games_played: 0
}
```

### 2.4 Multi-Dimensional Structures (Lisp-Style)

Built-in list system supporting 1D-4D (x, y, z, t) structures.

```gul
# 4D list definition
const matrix4d = list[
    x0[y0[z0[t0]]]
]
```

---

## 3. Functions (.fnc)

The `.fnc` file contains all application logic.

### 3.1 Function Declarations

Unified syntax for sync `@fn` and async `@asy` functions.

```gul
# Synchronous function
fn calculate(x: int) -> int:
    return x * 2

# Async function
async fetch_data(url):
    data = await http.get(url)
    return data
```

### 3.2 List Operations

Native Lisp-style list manipulation functions:

- `car(list)`: Head
- `cdr(list)`: Tail
- `cons(a, list)`: Construct
- `map(fn, list)`: Apply
- `fold(fn, init, list)`: Reduce
- `slice(list, x, y, z, t)`: Multi-dimensional slicing

### 3.3 Control Flow

Supports standard controls with annotation-based syntax options.

```gul
while count > 0:
    print(count)
    count = count - 1

if x < 5:
    return x
else:
    return 0
```

---

### 3. First-Class UI Components

UI elements are part of the language syntax:

```gul
# Create UI components inline
ui.print(&[button{text="Click Me", color="blue"}])
ui.print(&[slider{min=0, max=100, value=50}])
ui.print(&[chart{type="bar", data=values}])
ui.print(&[table{headers=["Name", "Age"], rows=data}])
```

---

## 4. Main Entry (.mn)

The `.mn` file is the sole entry point.

```gul
main():
    await play_game()
    print("Done!")
    print(f"Games played: {game_state.games_played}")
```

---

## 5. Ownership Model

GUL enforces a strong ownership model without Garbage Collection.

- **own**: Exclusive ownership (move semantics).
- **ref**: Borrowed reference (immutable).
- **copy**: Explicit duplication.

```gul
const data = list[1, 2, 3]
def ref view = data     # Allowed
def copy backup = data  # Duplicates data
```

---

## 6. Multi-Language Integration (.cs)

Embed foreign code directly with automatic binding generation.

```gul
# Python Integration
fn generate_secret() -> int:
    extern python {
        fn get_random():
            import random
            return random.randint(1, 100)
    }

    return get_random()
```

Supported languages: Python, Rust, JS, SQL, C, Go, Java.

---

## 7. Secrets (.scrt)

Secure file for sensitive data. Encrypted on disk.

```gul
# api_keys.scrt
const STRIPE_KEY = "sk_test_..."
```

---

## 8. Development Style

- **Indentation**: Python-style significant whitespace.
- **Annotations**: `@int`, `@str`, `@global`, `@fn` used for clarity and compiler hints.
- **Mutability**: `?variable` indicates mutable state.
