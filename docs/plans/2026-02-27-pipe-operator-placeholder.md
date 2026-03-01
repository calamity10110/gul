# Pipe Operator Placeholder (`_`) Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add placeholder `_` support to the existing pipe operator `|>` so `data |> f(x, _)` desugars to `f(x, data)`.

**Architecture:** The `|>` token and basic desugaring (always-first-arg) already exist in both stable and nightly compilers at `compilers/{stable,nightly}/parser/parser.rs:365-390`. We enhance the existing `Pipeline` match arm to detect `_` identifiers in call arguments and replace them with the piped value. Both compilers have identical code and get identical changes.

**Tech Stack:** Rust, compilers/stable and compilers/nightly parser modules.

---

### Task 1: Add placeholder detection to stable compiler parser

**Files:**
- Modify: `compilers/stable/parser/parser.rs:365-390`

**Step 1: Write the failing test**

No dedicated test file exists for the stable compiler parser in the Rust test framework. We'll verify manually, then add a `.mn` test file in Task 3. For now, ensure existing tests still pass.

**Step 2: Run existing tests to establish baseline**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib`
Expected: 493 passed, 0 failed

**Step 3: Modify the Pipeline match arm in stable parser**

In `compilers/stable/parser/parser.rs`, replace lines 365-390 (the `Pipeline` handling) with:

```rust
else if token.token_type == TokenType::Pipeline {
    // Pipeline Operator: left |> right
    // Supports three forms:
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
                // Replace the placeholder with the piped value
                call_expr.arguments[pos] = left;
            } else {
                // No placeholder: insert as first argument
                call_expr.arguments.insert(0, left);
            }
            return Expression::Call(call_expr);
        },
        Expression::Identifier(ident_expr) => {
            return Expression::Call(CallExpr{
                node: ident_expr.node.clone(),
                callee: Box::new(Expression::Identifier(ident_expr)),
                arguments: vec![left],
                keyword_args: HashMap::new()
            });
        },
        _ => {
            return Expression::Call(CallExpr{
                node: ASTNode{line: 1, column: 1},
                callee: Box::new(right),
                arguments: vec![left],
                keyword_args: HashMap::new()
            });
        }
    }
}
```

**Step 4: Run tests to verify nothing broke**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib`
Expected: 493 passed, 0 failed

**Step 5: Commit**

```bash
git add compilers/stable/parser/parser.rs
git commit -m "feat(pipe): add placeholder _ support to stable compiler pipeline operator"
```

---

### Task 2: Add placeholder detection to nightly compiler parser

**Files:**
- Modify: `compilers/nightly/parser/parser.rs:367-392`

**Step 1: Apply the same change to nightly parser**

The nightly parser at lines 367-392 has identical pipeline handling code. Apply the exact same replacement as Task 1.

**Step 2: Run tests to verify nothing broke**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib`
Expected: 493 passed, 0 failed

**Step 3: Commit**

```bash
git add compilers/nightly/parser/parser.rs
git commit -m "feat(pipe): add placeholder _ support to nightly compiler pipeline operator"
```

---

### Task 3: Add pipe operator test examples

**Files:**
- Create: `examples/pipe_operator.mn`

**Step 1: Create a GUL example file demonstrating all pipe forms**

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
    print(a)  # Expected: 10

    # Form 2: Default first-arg pipe
    const b = 10 |> add(5)
    print(b)  # Expected: 15

    # Form 3: Placeholder pipe
    const c = 42 |> wrap("[", _, "]")
    print(c)  # Expected: [42]

    # Chaining
    const d = 3 |> double |> double |> double
    print(d)  # Expected: 24

    # Mixed chaining with placeholder
    const e = 100 |> add(50) |> wrap("<<", _, ">>")
    print(e)  # Expected: <<150>>

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

**Step 1: Update status from Approved to Implemented**

Change line 6 from:
```
**Status**: Approved
```
to:
```
**Status**: Implemented (placeholder support added 2026-02-27)
```

**Step 2: Commit**

```bash
git add docs/plans/2026-02-27-pipe-operator-design.md
git commit -m "docs: mark pipe operator design as implemented"
```
