# GUL Data-Flow Programming Guide

\*\*Version: 0.14.0-dev | Syntax: v3.2 | Updated\*\*: 2026-01-08

---

## Overview

GUL supports **data-flow-driven, contract-based programming** where programs are assembled by matching input/output contracts between nodes. This enables visual, declarative programming with automatic dependency resolution.

---

## Core Concepts

### What is Data-Flow?

Data-flow programming focuses on the **movement and transformation of data** rather than control flow. Programs are defined as directed graphs where:

- **Nodes** represent operations/transformations
- **Edges** represent data dependencies
- **Execution** is driven by data availability

### Benefits

- ✅ **Visual Programming** - Easy to understand and debug
- ✅ **Parallelization** - Automatic concurrent execution
- ✅ **Composability** - Reusable components
- ✅ **Type Safety** - Contract-based validation

---

## Node Syntax (v3.2)

### Node Declaration

```gul
node add_node:
    (a: ref @int, b: ref @int),     # Inputs
    (result: @int)                   # Outputs

node multiply_node:
    (x: ref @int, y: ref @int),
    (result: @int)

node filter_node:
    (data: ref @list, threshold: ref @int),
    (filtered: @list)
```

### Node Implementation

```gul
# calculator.fnc
@fn @int add_impl(a, b):
    return a + b

@fn @int multiply_impl(x, y):
    return x * y

@fn @list filter_impl(data, threshold):
    let result = @list[]
    for item in data:
        if item > threshold:
            result.append(item)
    return result
```

---

## Data-Flow Main Block

### Basic Flow

```gul
@imp std.io

mn: [
    input(@int(5)) -> add_node.a
    input(@int(3)) -> add_node.b
    add_node.result -> multiply_node.x
    input(@int(2)) -> multiply_node.y
    multiply_node.result -> print
]
```

### Functional Style

```gul
mn:
    let data = @list[1, 2, 3, 4, 5]
    let threshold = @int(2)

    data
        -> filter(> threshold)
        -> map(x => x * 2)
        -> reduce((acc, x) => acc + x)
        -> print
```

---

## Type System with @ Prefix

### Primitive Types

```gul
@int       # Integer
@float     # Floating point
@str       # String
@bool      # Boolean
```

### Collection Types

```gul
@list[T]           # Generic list
@tuple(T, U, V)    # Tuple
@set{T}            # Set
@dict{K: V}        # Dictionary
```

### Function Types

```gul
node transform:
    (input: ref @int),
    (output: @str)

@fn @str transform_impl(n):
    return "Number: " + str(n)
```

---

## Advanced Patterns

### Pipeline Processing

```gul
@imp std{io, math}

# Data pipeline
mn:
    # Load data
    let numbers = @list[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    # Process pipeline
    numbers
        -> filter(x => x % 2 == 0)  # Even numbers
        -> map(x => x * x)          # Square them
        -> map(x => @float(x))      # Convert to float
        -> map(math.sqrt)           # Square root
        -> sum()                    # Sum all
        -> print                    # Output
```

### Parallel Processing

```gul
node parallel_task:
    (data: ref @list),
    (results: @list)

@fn @list parallel_impl(data):
    @python {
        from concurrent.futures import ThreadPoolExecutor
        with ThreadPoolExecutor() as executor:
            results = list(executor.map(process_item, data))
        return results
    }
    return python.results

mn:
    let data = @list[1, 2, 3, 4, 5]
    let processed = parallel_task(data)
    print(processed)
```

### Stream Processing

```gul
@imp std.stream

node stream_processor:
    (input: stream @int),
    (output: stream @int)

@fn stream_impl(stream):
    for item in stream:
        if item > 0:
            yield item * 2

mn:
    let data_stream = stream.from(@list[1, -2, 3, -4, 5])
    stream_processor(data_stream) -> print
```

---

## Contract Rules

### 1. Required Inputs

All required inputs must be connected:

```gul
node validate:
    (email: ref @str, age: ref @int),  # Both required
    (valid: @bool)

# Must provide both:
mn:
    validate(
        email: input(@str("user@example.com")),
        age: input(@int(25))
    ) -> print
```

### 2. Type Compatibility

Types must match or be compatible:

```gul
node add:
    (a: ref @int, b: ref @int),
    (result: @int)

# ✅ Valid
add(input(@int(5)), input(@int(3)))

# ❌ Invalid - type mismatch
# add(input(@str("5")), input(@int(3)))
```

### 3. Ownership Modes

```gul
node process:
    (data: borrow @list),   # Borrow - read only
    (result: @list)

node mutate:
    (data: ref @list),      # Ref - can modify
    (result: @list)

node consume:
    (data: move @list),     # Move - take ownership
    (result: @list)
```

---

## Complete Example: Data Analysis Pipeline

```gul
@imp std{io, math}
@imp python{pandas, numpy}

# Define nodes
node load_data:
    (filepath: ref @str),
    (data: @dict)

node clean_data:
    (raw_data: ref @dict),
    (clean_data: @dict)

node analyze_data:
    (data: ref @dict),
    (stats: @dict)

node visualize:
    (stats: ref @dict),
    (chart: @str)

# Implement nodes
@fn @dict load_impl(filepath):
    @python {
        import pandas as pd
        df = pd.read_csv(filepath)
        return df.to_dict()
    }
    return python.result

@fn @dict clean_impl(raw):
    @python {
        df = pd.DataFrame(raw)
        df = df.dropna()
        df = df[df['age'] > 0]
        return df.to_dict()
    }
    return python.result

@fn @dict analyze_impl(data):
    @python {
        df = pd.DataFrame(data)
        stats = {
            'mean': float(df['age'].mean()),
            'median': float(df['age'].median()),
            'count': len(df)
        }
        return stats
    }
    return python.stats

# Data flow
mn: [
    input(@str("data.csv"))
        -> load_data
        -> clean_data
        -> analyze_data
        -> print
]
```

---

## Visualization

### Simple Calculator Flow

```text
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  input(5)   │────▶│  add_node   │────▶│ multiply    │
└─────────────┘     │   a + b     │     │  result * y │────▶ print
┌─────────────┐     │   = 8       │     │   = 16      │
│  input(3)   │────▶│             │     │             │
└─────────────┘     └─────────────┘     └─────────────┘
                                    ┌─────────────┐
                                    │  input(2)   │
                                    └──────┬──────┘
                                           │
                                           ▼
```

### Complex Pipeline

```text
┌──────┐    ┌────────┐    ┌────────┐    ┌──────────┐    ┌───────┐
│ Load │───▶│ Clean  │───▶│ Filter │───▶│Transform │───▶│ Save  │
└──────┘    └────────┘    └────────┘    └──────────┘    └───────┘
               │             │              │
               ▼             ▼              ▼
           [Validate]    [Dedupe]       [Enrich]
```

---

## Best Practices

### 1. Keep Nodes Small

```gul
# ✅ Good - single responsibility
node validate_email:
    (email: ref @str),
    (valid: @bool)

node send_email:
    (to: ref @str, message: ref @str),
    (sent: @bool)

# ❌ Bad - too much in one node
node validate_and_send:
    (email: ref @str, message: ref @str),
    (sent: @bool)
```

### 2. Use Descriptive Names

```gul
# ✅ Good
node calculate_tax_amount
node filter_active_users
node transform_to_uppercase

# ❌ Bad
node calc
node filter
node transform
```

### 3. Document Contracts

```gul
# User registration node
# Inputs: email (validated), password (hashed)
# Outputs: user_id (int), success (bool)
node register_user:
    (email: ref @str, password: ref @str),
    (user_id: @int, success: @bool)
```

---

## Integration with MCP

Data-flow can be automated with MCP:

```bash
# Generate data-flow pipeline
gul-mcp generate "data pipeline: load → clean → analyze → save"

# AI will create:
# - Node definitions
# - Implementations
# - Data flow connections
# - Tests
```

---

## See Also

- [Quick Reference](../QUICK_REFERENCE.md) - Syntax guide
- [Standard Library](../api/standard-library.md) - Available functions
- [Integration Guide](integration.md) - Connect with other languages

---

**Last Updated**: 2026-01-08  
**Version**: 0.14.0-dev  
**Syntax\*\*: v3.2
