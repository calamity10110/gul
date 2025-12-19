# GUL Quick Reference

**v3.0 Syntax Cheat Sheet**

---

## Variables

```gul
let name = "Alice"    # Immutable
var count = 0         # Mutable
```

## Functions

```gul
fn add(a, b):
    return a + b

async fetch():
    return await http.get(url)
```

## Main Entry

```gul
mn:
    print("Hello!")
```

## Imports

```gul
@imp std.http
@imp python{numpy, pandas}
```

## Foreign Code

```gul
@python { import numpy as np }
@rust { fn compute() -> i32 { 42 } }
@c { int add(int a, int b) { return a+b; } }
```

## Data-Flow (Node System)

```gul
node add {
    re_in: @int[ a: A, b: B ],
    re_out: @int[ result: Sum ],
}

fn add(@int[a, b]) -> @int:
    return result : a + b

mn: [
    input(@int(5)) : add: a
    input(@int(3)) : add: b
    print: add: result
]
```

## Control Flow

```gul
if x > 0:
    print("positive")
elif x < 0:
    print("negative")
else:
    print("zero")

for item in items:
    print(item)

while running:
    process()
```

## Types

```gul
@int, @float, @str, @bool
@list, @dict, @any
```

## Traits

```gul
trait serialize
trait trainable
trait stream
trait async
trait sync
```

## CLI Commands

```bash
gul run file.mn      # Run
gul build file.mn    # Build
gul check file.mn    # Check
gul test             # Test
gul fmt file.mn      # Format
gul tui              # TUI IDE
```

---

**Docs**: [Full Documentation](README.md)
