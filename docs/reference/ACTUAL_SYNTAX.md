# GUL Actual Syntax - v3.0 Only

**This document uses v3.0 syntax exclusively.**

For historical v2.0 syntax, see [Development History](../devhistory.md).

---

## Current Syntax (v3.0)

GUL v3.0 uses modern, clean keywords:

| Feature      | v3.0 Syntax                      |
| ------------ | -------------------------------- |
| Immutable    | `let`                            |
| Mutable      | `var`                            |
| Main entry   | `mn:`                            |
| Imports      | `@imp`                           |
| Foreign code | `@python`, `@rust`, `@c`, `@sql` |

---

## Variables

```gul
let name = "Alice"      # Immutable
var count = 0           # Mutable
count = count + 1
```

## Functions

```gul
fn add(a: int, b: int) -> int:
    return a + b

async fetch(url: str) -> dict:
    return await http.get(url)
```

## Main Entry

```gul
mn:
    print("Hello, GUL!")
```

## Imports

```gul
@imp std.http
@imp python{numpy}
```

## Foreign Code

```gul
@python {
    def analyze(data):
        import numpy as np
        return np.mean(data)
}

@rust {
    fn compute(n: i32) -> i32 {
        n * n
    }
}
```

## Complete Example

```gul
@imp std.http
@imp python{numpy}

let API_URL = "https://api.example.com"
var request_count = 0

fn process_data(data: list) -> dict:
    return {
        "count": len(data),
        "sum": sum(data)
    }

async fetch_api() -> dict:
    response = await http.get(API_URL)
    return response.json()

@python {
    def analyze(data):
        import numpy as np
        return float(np.mean(data))
}

mn:
    print("Starting...")

    let data = [1, 2, 3, 4, 5]
    let stats = process_data(data)
    print("Stats:", stats)

    let avg = analyze(data)
    print("Average:", avg)
```

---

**For v2.0 syntax and migration guide, see**: [devhistory.md](../devhistory.md)

**Last Updated**: 2025-12-18  
**Version**: v3.0
