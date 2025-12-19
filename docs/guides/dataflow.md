# GUL Data-Flow Programming Guide

**Version**: 3.0 | **Status**: Implemented

---

## Overview

GUL supports **data-flow-driven, contract-based programming** where programs are assembled by matching input/output contracts between nodes.

---

## Syntax

### Node Declaration (.def files)

```gul
node <name> {
    re_in: @type[ param: Label, ... ],   # Required inputs
    re_out: @type[ param: Label, ... ],  # Required outputs
    opt_in: @type[ param: Label, ... ],  # Optional inputs
    opt_out: @type[ param: Label, ... ], # Optional outputs
}
```

**Example:**

```gul
node add {
    re_in: @int[ a: A, b: B ],
    re_out: @int[ result: Sum ],
}

node multiply {
    re_in: @int[ x: X, y: Y ],
    re_out: @int[ result: Product ],
}
```

### Node Implementation (.fnc files)

```gul
fn add(@int[a, b]) -> @int:
    return result : a + b

fn multiply(@int[x, y]) -> @int:
    return result : x * y
```

### Data-Flow Main Block (.mn files)

```gul
mn: [
    input(@int(5)) : add: a
    input(@int(3)) : add: b
    add: result : multiply: x
    input(@int(2)) : multiply: y
    print: multiply: result
]
```

---

## Type Annotations

```gul
@int       # Integer type
@float     # Floating point
@str       # String
@bool      # Boolean
@list      # List
@dict      # Dictionary
@any       # Any type (gradual typing)
```

---

## Traits

Traits define semantic meaning, not structure:

```gul
trait serialize   # Can be serialized
trait trainable   # Can be trained (ML)
trait stream      # Streaming data
trait async       # Asynchronous execution
trait sync        # Synchronous execution
```

---

## Contract Rules

1. **Required inputs** must be connected to:

   - External input
   - Another node's output

2. **Required outputs** must connect to:

   - External output (print)
   - Another node's input

3. **Type compatibility**:

   - Exact match
   - Covariant (int → float)
   - Trait-based

4. **Ownership**:
   - `own` - Full ownership
   - `ref` - Borrowing
   - `borrow` - Immutable reference

---

## Complete Example

```gul
# calculator.def
node add {
    re_in: @int[ a: A, b: B ],
    re_out: @int[ result: Sum ],
}

node multiply {
    re_in: @int[ x: X, y: Y ],
    re_out: @int[ result: Product ],
}

# calculator.fnc
fn add(@int[a, b]) -> @int:
    return result : a + b

fn multiply(@int[x, y]) -> @int:
    return result : x * y

# main.mn
mn: [
    input(@int(5)) : add: a
    input(@int(3)) : add: b
    add: result : multiply: x
    input(@int(2)) : multiply: y
    print: multiply: result  # Output: 16
]
```

---

## Data Flow Visualization

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  input(5)   │────▶│     add     │     │  multiply   │
└─────────────┘     │  a ──▶ result ────▶ x ──▶ result ──▶ print
┌─────────────┐     │  b ──┘      │     │  y ──┘      │
│  input(3)   │────▶│             │     │             │
└─────────────┘     └─────────────┘     └─────────────┘
                                   ┌─────────────┐
                                   │  input(2)   │
                                   └─────────────┘
```

---

**Last Updated**: 2025-12-18
