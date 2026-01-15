# GUL v2.0 vs v3.2 Comparison

## Quick Syntax Comparison

| Feature | v2.0 Syntax | v3.2 Syntax | Status |
|---------|-------------|-------------|--------|
| **Variables (Immutable)** | `let x = 10` | `@const x = 10` | `let` deprecated |
| **Variables (Mutable)** | `var x = 10` | `@var x = 10` | `var` works but `@var` preferred |
| **Function Definition** | `fn add(a, b) -> int:` | `@fn add(a, b)(result):` | Both work |
| **Function Return** | `return a + b` | `result = a + b` | Outputs preferred |
| **Async Functions** | `async fn fetch() -> dict:` | `async fetch()(data):` | `fn` optional |
| **Comments** | `# comment` | `// comment` or `# comment` | Both supported |
| **Imports** | `imp std.io` | `@imp std.io` | `imp` deprecated |
| **Entry Point** | `main(): ...` + `main()` | `mn: ...` | Both work |
| **Parallel For** | N/A | `also_for i in items:` | NEW in v3.2 |
| **Parallel While** | N/A | `also_while condition:` | NEW in v3.2 |
| **UI Components** | N/A | `@ui button{text: "Click"}` | NEW in v3.2 |
| **Await** | `await expression` | `await expression` | Enhanced |

## Feature Details

### 1. Variable Declarations

**v2.0**:

```gul
let name = "Alice"     # Immutable
var count = 0          # Mutable
```

**v3.2**:

```gul
@const name = "Alice"  // Immutable (preferred)
@var count = 0         // Mutable (preferred)
```

---

### 2. Function Definitions

**v2.0**:

```gul
fn calculate(x, y) -> int:
    return x + y
```

**v3.2** (Multiple Styles):

```gul
// Style 1: Explicit outputs
@fn calculate(x, y)(result):
    result = x + y

// Style 2: Arrow function
@const calculate = (x, y) => x + y

// Style 3: Multiple outputs
@fn divide(a, b)(quotient, remainder):
    quotient = a / b
    remainder = a % b
```

---

### 3. Async Programming

**v2.0**:

```gul
async fn fetch_data(url) -> dict:
    let response = await http.get(url)
    return response.json()
```

**v3.2**:

```gul
// Shortcut: 'fn' keyword optional!
async fetch_data(url)(data):
    data = await http.get(url)
```

---

### 4. Parallel Processing

**v2.0**:

```gul
# No built-in parallel loops
for i in items:
    process(i)  # Sequential only
```

**v3.2**:

```gul
// Built-in parallel execution
also_for i in items:
    process(i)  // Runs in parallel!

@var count = 0
also_while count < limit:
    step()
    count = count + 1
```

---

### 5. UI Development

**v2.0**:

```gul
# No built-in UI syntax
ui.button({"text": "Click", "color": "blue"})
```

**v3.2**:

```gul
// Declarative UI syntax
@ui button{
    text: "Click",
    color: "blue",
    onClick: handler
}
```

---

### 6. Comments

**v2.0**:

```gul
# Only Python-style comments
```

**v3.2**:

```gul
// C-style comments (NEW!)
# Python-style comments (still works)
```

---

## Migration Path

### Easy Migration (Automatic)

These features work in both versions:

- Basic syntax (if, for, while, match)
- Function calls
- Type system
- Foreign code blocks

### Requires Updates (Deprecation Warnings)

These show warnings but still work:

- `let` → use `@const`
- `var` → use `@var`
- `imp` → use `@imp`
- `def` → use `@const` or `@var`

### New Features (v3.2 Only)

- `also_for` / `also_while`
- `@ui` components
- `async` without `fn` keyword
- Function outputs: `@fn name()(output):`
- `//` comments

---

## Performance Impact

| Feature | v2.0 | v3.2 | Notes |
|---------|------|------|-------|
| Variable access | Same | Same | No change |
| Function calls | Same | Same | Output syntax is compiler transformation |
| Parallel loops | N/A | **Faster** | Automatic parallelization |
| Async/await | Same | Same | Just syntax sugar |

---

## Compatibility

### Backward Compatibility

✅ **All v2.0 code runs on v3.2** (with deprecation warnings)

### Forward Compatibility  

❌ **v3.2 features don't work on v2.0** (use migration guide)

---

## When to Upgrade?

**Upgrade to v3.2 if you:**

- Want modern, clearer syntax
- Need parallel processing (`also_for` / `also_while`)
- Are building UI applications (`@ui` syntax)
- Want better code organization (explicit outputs)
- Prefer C-style `//` comments

**Stay on v2.0 if:**

- You have a large existing codebase
- Your team isn't ready for new syntax
- You need maximum stability (though v3.2 is production-ready!)

---

## Quick Migration Checklist

- [ ] Replace `let` with `@const`
- [ ] Replace `var` with `@var`  
- [ ] Replace `imp` with `@imp`
- [ ] Update functions to use outputs: `@fn name()(result):`
- [ ] Consider using `async` shortcut (drop `fn` keyword)
- [ ] Use `also_for` / `also_while` for parallel operations
- [ ] Add `@ui` components where applicable
- [ ] Update `#` comments to `//` (optional)

---

## Resources

- **Full Migration Guide**: [v32_migration.md](guides/v32_migration.md)
- **Feature Overview**: [GUL_V32_OVERVIEW.md](GUL_V32_OVERVIEW.md)
- **Interactive Walkthrough**: Run `gul run walkthrough_v32.mn`
- **Documentation**: [README.md](../README.md)

**GUL v3.2** - Cleaner, faster, better! 🚀
