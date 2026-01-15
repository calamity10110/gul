# GUL v3.2 Migration Guide

This guide helps you update existing GUL code to v3.2 syntax.

## Quick Reference

### Variables

**v2.0**: `let x = 10`, `var y = 20`  
**v3.2**: `@const x = 10`, `@var y = 20`

### Functions

**v2.0**: `fn add(a, b) -> int: return a + b`  
**v3.2**: `@fn add(a, b)(result): result = a + b`

### Arrow Functions

**v2.0**: `let double = (x) => x * 2`
**v3.2**: `@const double(x) => x * 2`

### Async Functions  

**v2.0**: `async fn fetch(url) -> dict: return await http.get(url)`  
**v3.2**: `async fetch(url)(result): result = await http.get(url)` (or `@async`)

### Comments

**v2.0**: `# comment`  
**v3.2**: `// comment` or `# comment` (both supported)

### Parallel Loops

**v3.2 NEW**: `also_for i in 0..10: process(i)`  
**v3.2 NEW**: `also_while condition: step()`

### Entry Point

**v2.0**: `main(): ...` then `main()`  
**v3.2**: `mn: ...` (no call needed, or use `fn main()` with `main()`)

### UI Components  

**v3.2 NEW**: `@ui button{text: "Click", color: "blue"}`

## Common Patterns

### Simple Function with Output

```gul
@fn calculate(x, y)(result):
    result = x + y
```

### Named Arrow Function

```gul
@const double(x) => x * 2
@const add(a, b) => a + b
```

### Async/Await

```gul
async fetch_data(url)(data):
    data = await http.get(url)

mn:
    @const result = await fetch_data("https://api.example.com")
```

### Parallel Processing

```gul
also_for item in items:
    process(item)  // Runs in parallel
```

## Type Constructors

Type constructors like `@int()`, `@str()`, `@list[]`, `@dict{}` are used for:

- Explicit type conversion: `@str(42)` → `"42"`  
- Collection creation: `@list[1, 2, 3]`
- In type annotations: `x: int`

**Note**: Type constructors may not work in all expression contexts; use simple assignment when possible.

## Tips

1. Use `@const` for immutable values (preferred over deprecated `let`)
2. Use `@var` for mutable values
3. Functions with outputs should assign to output parameters, not use `return`
4. The `@fn` decorator is required for function definitions
5. `async` functions don't need the `fn` keyword
6. Named arrow functions: `@const name(params) => expression`

## Deprecated Features

- `let` → use `@const`
- `var` without `@` → use `@var`  
- `def` → use `@const` or `@var`
- `imp` → use `@imp`
- `main():` → use `mn:` or `fn main()` with call
- `own`, `ref` in parameter lists → use in node contracts only
- `let name = (x) => x * 2` → use `@const name(x) => x * 2`
