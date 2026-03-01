# Pipe Operator (`|>`) Design

**Date**: 2026-02-27
**Version**: 0.14.0-dev
**Status**: Approved
**Approach**: Syntactic sugar (parser desugaring)

## Summary

Add a pipe operator `|>` to GUL that enables left-to-right function chaining. The operator is desugared in the parser into regular function calls — no changes to semantic analysis, codegen, or the interpreter.

## Syntax

Three forms based on how the piped value is placed:

```gul
# Form 1: No-arg — pipe as sole argument
data |> len              # → len(data)

# Form 2: Default — pipe as first argument
data |> filter(is_active)   # → filter(data, is_active)
data |> map(transform)      # → map(data, transform)

# Form 3: Placeholder — pipe to explicit position
data |> join(", ", _)       # → join(", ", data)
data |> insert(0, _, list)  # → insert(0, data, list)
```

### Chaining

Left-to-right evaluation, left-associative:

```gul
const result = users
    |> filter(is_active)
    |> map(get_name)
    |> sort()
    |> join(", ")
# Desugars to: join(sort(map(filter(users, is_active), get_name)), ", ")
```

### Precedence

Lower than all arithmetic/comparison operators, higher than assignment. `a + b |> f` is `f(a + b)`, not `a + f(b)`.

### Placeholder Rules

- `_` may appear exactly once on the right side of `|>`
- Multiple `_` is a compile error
- If `_` is present, it marks the insertion point
- If `_` is absent, the value is prepended as the first argument

## Error Propagation

The `?` operator works on pipe stages to unwrap `Result` types:

```gul
const data = raw_input
    |> validate()?          # Err → early return
    |> parse_json()?        # Err → early return
    |> transform()
    |> serialize()
```

Desugars to:

```gul
const _t1 = validate(raw_input)?
const _t2 = parse_json(_t1)?
const _t3 = transform(_t2)
const data = serialize(_t3)
```

## Async (Deferred)

Async pipes (`|> await f()`) are out of scope for v1. Piping into async functions produces a compile error with a clear message: "async functions cannot be used in pipe expressions yet; use `await` with a regular call instead."

## Compiler Changes

All changes are in the **Lexer** and **Parser** only.

### Lexer

Add `|>` as a new two-character token (`TokenKind::PipeOp`). The lexer already handles `|` — check for `>` following it.

### Parser

1. Add `PipeOp` to the operator precedence table (lowest precedence, left-associative)
2. When parsing a pipe expression `lhs |> rhs`:
   - If `rhs` is a bare identifier (no parens): rewrite to `rhs(lhs)`
   - If `rhs` is a function call without `_`: prepend `lhs` as first argument
   - If `rhs` is a function call with `_`: replace `_` with `lhs`
   - If `rhs` has `?`: wrap the rewritten call in error propagation
3. For chained pipes, process left-to-right (left-associative)

### What Does NOT Change

- Semantic analyzer — sees only regular function calls
- Code generator — no new IR nodes
- Interpreter — no new evaluation logic

## Testing Strategy

- **Lexer tests**: `|>` tokenization, distinguish from `|` and `>`
- **Parser tests**: All three forms, chaining, placeholder, error propagation
- **Integration tests**: End-to-end pipe expressions with stdlib functions
- **Error tests**: Multiple `_` placeholders, async in pipe (compile error), invalid RHS

## Examples

```gul
# Data processing pipeline
const report = raw_data
    |> parse_csv()?
    |> filter(row_valid)
    |> map(normalize)
    |> group_by("category")
    |> summarize()

# String processing
const slug = title
    |> trim()
    |> lower()
    |> replace(" ", "-", _)
    |> truncate(50)

# With placeholder for non-first-arg position
const html = markdown_text
    |> render_to_html(options, _)
    |> wrap_in_template("main", _)
```
