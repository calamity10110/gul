# GUL v3.2 Cheatsheet 🚀

## Entry Point

```gul
mn:
    std.io.println("Hello, GUL!")
```

## Variables

```gul
let x = 10          # Immutable (Constant)
var y = 20          # Mutable
let z: int = 30     # Explicit Type
?count = 0          # Quick Mutable Syntax
```

## Types

- `int`, `float`, `str`, `bool`, `any`
- `list<T>`, `map<K,V>`

## Control Flow

```gul
if x > 5:
    std.io.println("Big")
else:
    std.io.println("Small")

loop:               # Infinite loop
    break

for i in range(10):
    continue
```

## Functions

```gul
fn add(a: int, b: int) -> int:
    return a + b

# Async Function
asy fetch_data(url: str):
    await http.get(url)
```

## Ownership (Key Concept!)

```gul
fn process(own data: str):  # Takes ownership (data moved)
    ...

fn view(ref data: str):     # Borrows data (read-only)
    ...

fn modify(mut data: str):   # Mutable borrow
    ...
```

## Modules

```gul
# Standard Import
imp std.io
imp std.math

# Grouped
imp {
    std.io,
    std.net
}

# Annotated Import
@imp(version="1.0")
package utils
```

## AI Native

```gul
@ai(model="gpt-4")
fn summarize(text: str) -> str
```
