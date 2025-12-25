# GUL v3.1 Syntax Enforcement Plan

**Created**: December 24, 2024  
**Status**: In Progress  
**Version**: 3.1.0

---

## Overview

This plan outlines the steps to apply and enforce GUL v3.1 syntax across all project components: code, documentation, tests, examples, packages, compiler, tutorials, web UI, and TUI.

---

## v3.1 Syntax Summary

### Key Changes from v3.0

| Feature             | v3.0                  | v3.1                                       |
| ------------------- | --------------------- | ------------------------------------------ |
| Type annotation     | `@int(age) = 30` only | Both `@int(age) = 30` AND `age = @int(30)` |
| Function multiline  | Limited examples      | Full multiline support documented          |
| Async syntax        | `async fetch():`      | `async fetch(url):` with params            |
| Ownership table     | Text description      | Formatted table                            |
| Bracket equivalence | Implicit              | Explicitly documented                      |
| Error handling      | Not documented        | `try/catch/finally` patterns               |
| Comprehensions      | Not documented        | List comprehension examples                |

---

## Phase 1: Documentation (Priority: HIGH)

### 1.1 Core Documentation

- [x] `docs/QUICK_REFERENCE.md` - Updated to v3.1
- [ ] `docs/language_spec.md` - Update formal grammar for dual type annotation
- [ ] `docs/README.md` - Update examples to v3.1
- [ ] `docs/reference/syntax.md` - Add bracket equivalence section
- [ ] `docs/reference/ownership.md` - Convert to table format
- [ ] `docs/reference/structure.md` - Update node/function examples

### 1.2 Guides

- [ ] `docs/guides/introduction.md` - Update beginner examples
- [ ] `docs/guides/integration.md` - Update foreign code examples
- [ ] `docs/guides/dataflow.md` - Add simple pipeline examples
- [ ] `docs/guides/tui.md` - Update TUI examples
- [ ] `docs/guides/webui.md` - Update web examples

### 1.3 API Documentation

- [ ] `docs/api/standard-library.md` - Update std lib examples

---

## Phase 2: Parser & Compiler (Priority: HIGH)

### 2.1 Lexer Updates (`src/lexer/mod.rs`)

- [ ] Ensure `@type(value)` pattern is tokenized correctly
- [ ] Add tests for dual type annotation styles
- [ ] Validate bracket equivalence tokenization

### 2.2 Parser Updates (`src/parser.rs`)

- [ ] `parse_let_definition()` - Support both annotation styles:
  ```rust
  // Style 1: let @int(x) = 5
  // Style 2: let x = @int(5)
  ```
- [ ] `parse_function()` - Support multiline and single-line
- [ ] `parse_async_function()` - Require parameters in signature
- [ ] Add `parse_try_catch()` for error handling
- [ ] Add `parse_list_comprehension()` support

### 2.3 AST Updates (`src/ast.rs`)

- [ ] Add `TypeAnnotation` enum with `Decorator` and `Value` variants
- [ ] Add `TryCatch` statement type
- [ ] Add `ListComprehension` expression type

### 2.4 Semantic Analysis (`src/semantic/`)

- [ ] Validate type annotations in both styles
- [ ] Check ownership modes in node contracts
- [ ] Verify async function signatures

---

## Phase 3: Linter & Formatter (Priority: MEDIUM)

### 3.1 Linter Updates (`src/tools/linter.rs`)

- [ ] Add rule: `deprecated-type-style` (warning only, both valid)
- [ ] Add rule: `missing-async-params` - async without url param
- [ ] Add rule: `prefer-table-ownership` - suggest table format in docs
- [ ] Add rule: `bracket-consistency` - warn on mixed brackets
- [ ] Update `deprecated-main-syntax` to v3.1

### 3.2 Formatter Updates (`src/tools/formatter.rs`)

- [ ] Format multiline functions correctly
- [ ] Preserve type annotation style (don't convert)
- [ ] Format try/catch blocks
- [ ] Format list comprehensions

---

## Phase 4: Examples & Templates (Priority: MEDIUM)

### 4.1 Example Files (`examples/`)

Update all examples to v3.1 syntax:

- [ ] `examples/hello_world.mn`
- [ ] `examples/web_fetch.mn`
- [ ] `examples/dataflow_calculator.mn`
- [ ] `examples/sql_query.mn`
- [ ] `examples/ui_slider.mn`
- [ ] `examples/beginner_tutorial.mn`
- [ ] `examples/embedded_blink.mn`
- [ ] `examples/c_inline.mn`
- [ ] `examples/revised_syntax_demo.mn`
- [ ] `examples/v2_verify.mn` → rename to `examples/v31_verify.mn`

### 4.2 Templates (`templates/`)

- [ ] `templates/basic_project/main.mn`
- [ ] `templates/web_app/main.mn`
- [ ] `templates/ai_app/main.mn`
- [ ] `templates/embedded/main.mn`

---

## Phase 5: Tests (Priority: HIGH)

### 5.1 Core Tests (`tests/core/`)

- [ ] `tests/core/compiler_v21_test.mn` → `compiler_v31_test.mn`
- [ ] `tests/core/ui_components_test.mn` - Update to v3.1
- [ ] Add `tests/core/type_annotation_test.mn` - Test both styles
- [ ] Add `tests/core/bracket_equivalence_test.mn`
- [ ] Add `tests/core/error_handling_test.mn`

### 5.2 Integration Tests

- [ ] `tests/integration_test.rs` - Update assertions
- [ ] `tests/cli_test.rs` - Add v3.1 specific tests
- [ ] `tests/sql_integration_tests.rs` - Already passing

### 5.3 Parser Tests (`src/parser.rs`)

- [ ] Add `test_parse_dual_type_annotation()`
- [ ] Add `test_parse_try_catch()`
- [ ] Add `test_parse_list_comprehension()`
- [ ] Add `test_parse_multiline_function()`

### 5.4 Linter Tests (`src/tools/linter.rs`)

- [ ] Add tests for new lint rules
- [ ] Test v3.1 deprecation warnings

---

## Phase 6: Packages (`gul_packages/`)

### 6.1 Standard Library

- [ ] `gul_packages/std/main.mn` - Update to v3.1
- [ ] `gul_packages/math/main.mn`
- [ ] `gul_packages/http/main.mn`
- [ ] `gul_packages/json/main.mn`
- [ ] `gul_packages/io/main.mn`

### 6.2 Package Documentation

- [ ] Update all package README files
- [ ] Update example code in packages

---

## Phase 7: TUI & Web IDE (Priority: MEDIUM)

### 7.1 TUI IDE (`src/tools/tui_ide.rs`)

- [ ] Update syntax highlighter for v3.1
- [ ] Add v3.1 snippets
- [ ] Update autocomplete for dual type styles

### 7.2 Web IDE (`src/tools/web_ide.rs`)

- [ ] Update `v3_autocomplete` items
- [ ] Add v3.1 deprecation warnings
- [ ] Update example templates

### 7.3 Web Frontend (`web/`)

- [ ] Update playground examples
- [ ] Update documentation viewer
- [ ] Add v3.1 syntax highlighting

---

## Phase 8: CI/CD & Scripts (Priority: LOW)

### 8.1 Syntax Checker (`scripts/check_gul101_syntax.py`)

- [ ] Add v3.1 pattern detection
- [ ] Update deprecated patterns list
- [ ] Add bracket consistency check

### 8.2 GitHub Workflows

- [ ] Update `.github/workflows/ci.yml` - Run v3.1 tests
- [ ] Update `.github/workflows/documentation.yml`
- [ ] Add v3.1 migration check workflow

---

## Execution Order

```
Week 1: Phase 1 (Documentation) + Phase 2 (Parser Core)
Week 2: Phase 5 (Tests) + Phase 3 (Linter/Formatter)
Week 3: Phase 4 (Examples) + Phase 6 (Packages)
Week 4: Phase 7 (TUI/Web) + Phase 8 (CI/CD)
```

---

## Validation Checklist

After all phases complete:

- [ ] `cargo test --all-features` passes (500+ tests)
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] `cargo fmt --all -- --check` passes
- [ ] `python3 scripts/check_gul101_syntax.py docs/` - No warnings
- [ ] `gul run simple_test.mn` works
- [ ] `gul run examples/*.mn` - All examples run
- [ ] TUI IDE launches and syntax highlights correctly
- [ ] Web IDE displays v3.1 examples

---

## Files to Create

| File                            | Purpose                           |
| ------------------------------- | --------------------------------- |
| `tests/core/v31_syntax_test.mn` | Comprehensive v3.1 syntax tests   |
| `docs/MIGRATION_v31.md`         | Migration guide from v3.0 to v3.1 |
| `examples/v31_showcase.mn`      | Showcase all v3.1 features        |

---

## Deprecation Policy

- v3.0 syntax remains **fully supported** with no warnings
- v2.x syntax shows **deprecation warning**
- v1.x syntax **not supported** (parse error)

---

## Notes

- This is an **additive** update - no breaking changes
- Both type annotation styles have equal status
- Bracket equivalence is a core language feature
- Error handling (`try/catch`) may require runtime support

---

**Next Steps**: Start with Phase 1.1 (Core Documentation) and Phase 2.2 (Parser Updates)
