# Pipe Operator Placeholder (`_`) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add placeholder `_` support to the existing pipe operator `|>` so `data |> f(x, _)` desugars to `f(x, data)`.

**Architecture:** The `|>` token and basic desugaring (always-first-arg) already exist in both compilers at `compilers/{stable,nightly}/parser/parser.rs:367-392`. Both files are identical (just synced). We enhance the `Pipeline` match arm to detect `_` identifiers in call arguments and replace them with the piped value. The change is ~10 lines per file.

**Tech Stack:** Rust, `compilers/stable/parser/parser.rs`, `compilers/nightly/parser/parser.rs`

---

### Task 1: Verify baseline

**Step 1: Run tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib`
Expected: 493 passed, 0 failed, 1 ignored

---

### Task 2: Add placeholder `_` detection to both compiler parsers

**Files:**
- Modify: `compilers/stable/parser/parser.rs:367-392`
- Modify: `compilers/nightly/parser/parser.rs:367-392`

**Step 1: Replace the Pipeline match arm in BOTH parsers**

Find this code (identical in both files):

```rust
        else if token.token_type == TokenType::Pipeline {
            // Pipeline Operator: left |> right
            let right = self.parse_expression(precedence);

            match right {
                 Expression::Call(mut call_expr) => {
                     call_expr.arguments.insert(0, left);
                     return Expression::Call(call_expr);
                 },
```

Replace with:

```rust
        else if token.token_type == TokenType::Pipeline {
            // Pipeline Operator: left |> right
            // Three forms:
            //   left |> func          -> func(left)
            //   left |> func(a, b)    -> func(left, a, b)
            //   left |> func(a, _, b) -> func(a, left, b)
            let right = self.parse_expression(precedence);

            match right {
                 Expression::Call(mut call_expr) => {
                     // Check if any argument is the placeholder `_`
                     let placeholder_pos = call_expr.arguments.iter().position(|arg| {
                         matches!(arg, Expression::Identifier(ident) if ident.name == "_")
                     });
                     if let Some(pos) = placeholder_pos {
                         // Replace placeholder with piped value
                         call_expr.arguments[pos] = left;
                     } else {
                         // No placeholder: insert as first argument
                         call_expr.arguments.insert(0, left);
                     }
                     return Expression::Call(call_expr);
                 },
```

The `Identifier` and wildcard `_` arms below remain unchanged.

**Step 2: Run tests to verify nothing broke**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib`
Expected: 493 passed, 0 failed, 1 ignored

**Step 3: Commit**

```bash
git add compilers/stable/parser/parser.rs compilers/nightly/parser/parser.rs
git commit -m "feat(pipe): add placeholder _ support to pipeline operator in both compilers"
```

---

### Task 3: Add pipe operator example file

**Files:**
- Create: `examples/pipe_operator.mn`

**Step 1: Create example demonstrating all three pipe forms**

```gul
# Pipe Operator Examples - GUL v3.2

# Helper functions for testing
@fn double(x)(res):
    res = x * 2

@fn add(a, b)(res):
    res = a + b

@fn wrap(prefix, value, suffix)(res):
    res = prefix + str(value) + suffix

mn:
    # Form 1: No-arg pipe (bare identifier)
    const a = 5 |> double
    print(a)

    # Form 2: Default first-arg pipe
    const b = 10 |> add(5)
    print(b)

    # Form 3: Placeholder pipe
    const c = 42 |> wrap("[", _, "]")
    print(c)

    # Chaining
    const d = 3 |> double |> double |> double
    print(d)

    # Mixed chaining with placeholder
    const e = 100 |> add(50) |> wrap("<<", _, ">>")
    print(e)

    print("All pipe operator tests passed!")
```

**Step 2: Commit**

```bash
git add examples/pipe_operator.mn
git commit -m "feat(pipe): add pipe operator example with placeholder demos"
```

---

### Task 4: Update design doc status

**Files:**
- Modify: `docs/plans/2026-02-27-pipe-operator-design.md:6`

**Step 1: Change status**

Replace: `**Status**: Approved`
With: `**Status**: Implemented (placeholder support added 2026-03-01)`

**Step 2: Commit**

```bash
git add docs/plans/2026-02-27-pipe-operator-design.md
git commit -m "docs: mark pipe operator design as implemented"
```
