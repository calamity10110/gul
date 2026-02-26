# Architecture Refactor: Pipeline-Centric Reorganization — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Reorganize `src_bootstrap/` from a flat 21-module structure into a pipeline-centric hierarchy (`frontend/`, `backend/`, `runtime/`, `domains/`, `tools/`, `platform/`, `interop/`, `mcp/`, `tui/`, `autonomous/`, `memory/`, `benchmarks/`) so a new developer can understand the codebase in 10 minutes.

**Architecture:** Three-phase approach: (1) Split oversized files in-place using Rust's `mod.rs` convention so no import paths change, (2) Create the new grouping directories and move everything, (3) Update all import paths. Each phase ends with a green test run.

**Tech Stack:** Rust (MSVC toolchain), cargo. Test command: `cargo +stable-x86_64-pc-windows-msvc test --lib`

**Design doc:** `docs/plans/2026-02-25-architecture-refactor-design.md`

**Baseline:** 490 lib tests passing, 0 failures

---

## Phase 1: Split Oversized Files In-Place

> In this phase, we split `lexer/mod.rs`, `parser.rs`, and `interpreter.rs` into sub-modules.
> Because `mod foo;` resolves to both `foo.rs` and `foo/mod.rs`, external import paths stay unchanged.

---

### Task 1: Run baseline tests

**Files:** None

**Step 1: Verify baseline**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib 2>&1 | tail -3`
Expected: `test result: ok. 490 passed; 0 failed; 1 ignored`

**Step 2: Commit baseline marker**

```bash
git add -A
git commit -m "chore: verify baseline before architecture refactor"
```

---

### Task 2: Split lexer — extract `tokens.rs`

The Token enum (lines 5-130) and its impl blocks (lines 131-185) should move to a dedicated file.

**Files:**
- Create: `src_bootstrap/lexer/tokens.rs`
- Modify: `src_bootstrap/lexer/mod.rs`

**Step 1: Create `tokens.rs`**

Extract from `src_bootstrap/lexer/mod.rs`:
- Lines 3 (the `use std::fmt;`)
- Lines 5-185 (Token enum + `impl Token` + `impl fmt::Display for Token`)

Place into `src_bootstrap/lexer/tokens.rs`. At the top, add:

```rust
use std::fmt;
```

**Step 2: Update `lexer/mod.rs`**

At the top of `mod.rs`, replace the extracted code with:

```rust
// Lexer module - tokenizes source code

pub mod tokens;

pub use tokens::Token;
```

Keep everything from the `Lexer` struct onward (line ~187+). The Lexer uses `Token` directly, so `use crate::lexer::Token` paths throughout the codebase remain valid thanks to `pub use`.

**Step 3: Run tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib 2>&1 | tail -3`
Expected: `test result: ok. 490 passed; 0 failed; 1 ignored`

**Step 4: Commit**

```bash
git add src_bootstrap/lexer/tokens.rs src_bootstrap/lexer/mod.rs
git commit -m "refactor: extract Token enum to lexer/tokens.rs"
```

---

### Task 3: Split lexer — extract `scanner.rs`

The main scanning logic (Lexer struct and tokenize impl) moves to scanner.rs, leaving mod.rs as a thin re-export hub.

**Files:**
- Create: `src_bootstrap/lexer/scanner.rs`
- Modify: `src_bootstrap/lexer/mod.rs`

**Step 1: Create `scanner.rs`**

Extract from `mod.rs`:
- The `Lexer` struct definition and entire `impl Lexer` block (lines ~187-716)

Place into `src_bootstrap/lexer/scanner.rs`. At the top, add:

```rust
use super::tokens::Token;
```

**Step 2: Update `lexer/mod.rs`**

The mod.rs should now look like:

```rust
// Lexer module - tokenizes source code

pub mod tokens;
pub mod scanner;

pub use tokens::Token;
pub use scanner::Lexer;

#[cfg(test)]
mod tests {
    // ... existing tests unchanged ...
}
```

**Step 3: Run tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib 2>&1 | tail -3`
Expected: `test result: ok. 490 passed; 0 failed; 1 ignored`

**Step 4: Commit**

```bash
git add src_bootstrap/lexer/scanner.rs src_bootstrap/lexer/mod.rs
git commit -m "refactor: extract Lexer struct to lexer/scanner.rs"
```

---

### Task 4: Convert `parser.rs` to directory module

Before splitting parser internals, convert it from a single file to a directory module.

**Files:**
- Rename: `src_bootstrap/parser.rs` → `src_bootstrap/parser/mod.rs`

**Step 1: Create directory and move**

```bash
mkdir -p src_bootstrap/parser
mv src_bootstrap/parser.rs src_bootstrap/parser/mod.rs
```

**Step 2: Run tests** (should pass — Rust resolves `mod parser;` to `parser/mod.rs`)

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib 2>&1 | tail -3`
Expected: `test result: ok. 490 passed; 0 failed; 1 ignored`

**Step 3: Commit**

```bash
git add src_bootstrap/parser/mod.rs
# git rm tracks the old file automatically since we used mv
git add -u
git commit -m "refactor: convert parser.rs to parser/mod.rs"
```

---

### Task 5: Split parser — extract `expressions.rs`

Expression parsing methods move to their own file.

**Files:**
- Create: `src_bootstrap/parser/expressions.rs`
- Modify: `src_bootstrap/parser/mod.rs`

**Step 1: Identify expression methods**

These methods handle expression parsing (approximate lines from original parser.rs):
- `parse_expression` (~line 974)
- `parse_lambda_or_logical_or` (~line 978)
- `extract_params_from_expr` (~line 998)
- `parse_logical_or` (~line 1019)
- `parse_logical_and` (~line 1035)
- `parse_equality` (~line 1051)
- `parse_comparison` (~line 1072)
- `parse_addition` (~line 1098)
- `parse_multiplication` (~line 1119)
- `parse_power` (~line 1144)
- `parse_unary` (~line 1160)
- `parse_postfix` (~line 1187)
- `parse_primary` (~line 1275)
- `parse_paren_or_lambda` (~line 201)

**Step 2: Create `expressions.rs`**

Move the expression methods into a trait impl or a separate `impl Parser` block:

```rust
// Expression parsing methods for Parser

use crate::ast::{Type, *};
use crate::lexer::Token;
use super::Parser;

impl Parser {
    // paste all expression methods here
    // (parse_expression, parse_lambda_or_logical_or, etc.)
}
```

Note: In Rust, `impl` blocks can be split across files as long as they're in the same crate. The methods reference `self.current_token()`, `self.advance()`, etc. which remain in `mod.rs`.

**Step 3: Update `parser/mod.rs`**

Add at the top (after the existing `use` statements):

```rust
mod expressions;
```

Remove the extracted methods from `mod.rs`.

**Step 4: Run tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib 2>&1 | tail -3`
Expected: `test result: ok. 490 passed; 0 failed; 1 ignored`

**Step 5: Commit**

```bash
git add src_bootstrap/parser/expressions.rs src_bootstrap/parser/mod.rs
git commit -m "refactor: extract expression parsing to parser/expressions.rs"
```

---

### Task 6: Split parser — extract `statements.rs`

Statement parsing methods move to their own file.

**Files:**
- Create: `src_bootstrap/parser/statements.rs`
- Modify: `src_bootstrap/parser/mod.rs`

**Step 1: Identify statement methods**

These methods handle statement parsing:
- `parse_statement` (~line 125)
- `parse_annotation_statement` (~line 218)
- `parse_global_def` (~line 243)
- `parse_mutable_assignment` (~line 282)
- `parse_import` (~line 310)
- `parse_single_import_path` (~line 344)
- `parse_struct` (~line 389)
- `parse_try_catch` (~line 485)
- `parse_definition` (~line 562)
- `parse_function` (~line 610)
- `parse_block` (~line 710)
- `is_block_end` (~line 734)
- `parse_custom_block` (~line 739)
- `parse_main` (~line 803)
- `parse_if` (~line 851)
- `parse_loop` (~line 880)
- `parse_for` (~line 890)
- `parse_while` (~line 916)
- `parse_return` (~line 931)
- `parse_expression_statement` (~line 946)
- `parse_const_definition` (~line 1449)
- `parse_mut_definition` (~line 1485)
- `parse_let_definition` (~line 1522)
- `parse_var_definition` (~line 1558)
- `parse_at_import` (~line 1597)
- `parse_at_ui` (~line 1734)
- `parse_at_lang_block` (~line 1783)
- `parse_test_annotation` (~line 1923)
- `parse_extern_block` (~line 1935)

**Step 2: Create `statements.rs`**

```rust
// Statement parsing methods for Parser

use crate::ast::{Type, *};
use crate::lexer::Token;
use super::Parser;

impl Parser {
    // paste all statement methods here
}
```

**Step 3: Update `parser/mod.rs`**

Add `mod statements;` and remove the extracted methods. The `mod.rs` should now contain only:
- `use` statements
- `Parser` struct + `new()`, `current_token()`, `peek()`, `advance()`, `expect()`, `skip_newlines()`
- `parse()`, `parse_def_file()`, `parse_fnc_file()`, `parse_mn_file()`, `parse_tokens_strict()`
- `mod expressions;` and `mod statements;` declarations

**Step 4: Run tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib 2>&1 | tail -3`
Expected: `test result: ok. 490 passed; 0 failed; 1 ignored`

**Step 5: Commit**

```bash
git add src_bootstrap/parser/statements.rs src_bootstrap/parser/mod.rs
git commit -m "refactor: extract statement parsing to parser/statements.rs"
```

---

### Task 7: Split parser — extract `types.rs`

Type annotation parsing moves to its own file.

**Files:**
- Create: `src_bootstrap/parser/types.rs`
- Modify: `src_bootstrap/parser/mod.rs` (or `statements.rs` if that's where it ended up)

**Step 1: Extract type methods**

- `parse_type` (~line 440)
- `parse_type_annotation` (~line 529)

**Step 2: Create `types.rs`**

```rust
// Type annotation parsing methods for Parser

use crate::ast::{Type, *};
use crate::lexer::Token;
use super::Parser;

impl Parser {
    // paste type parsing methods here
}
```

**Step 3: Update source — remove from wherever they currently live, add `mod types;`**

**Step 4: Run tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib 2>&1 | tail -3`
Expected: `test result: ok. 490 passed; 0 failed; 1 ignored`

**Step 5: Commit**

```bash
git add src_bootstrap/parser/types.rs src_bootstrap/parser/mod.rs
git commit -m "refactor: extract type parsing to parser/types.rs"
```

---

### Task 8: Split parser — extract `tests.rs`

Move the parser test block to a dedicated file.

**Files:**
- Create: `src_bootstrap/parser/tests.rs`
- Modify: `src_bootstrap/parser/mod.rs`

**Step 1: Extract tests**

Move the entire `#[cfg(test)] mod tests { ... }` block (lines ~1998-2387) to `parser/tests.rs`.

The file should contain just the inner module content (without the outer `mod tests` wrapper):

```rust
use crate::lexer::Lexer;
use super::Parser;

#[test]
fn test_parse_simple_definition() {
    // ... existing test code ...
}
// ... all other tests ...
```

**Step 2: Update `parser/mod.rs`**

Replace the `#[cfg(test)] mod tests { ... }` block with:

```rust
#[cfg(test)]
mod tests;
```

**Step 3: Run tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib parser 2>&1 | tail -5`
Expected: All parser tests pass

**Step 4: Commit**

```bash
git add src_bootstrap/parser/tests.rs src_bootstrap/parser/mod.rs
git commit -m "refactor: extract parser tests to parser/tests.rs"
```

---

### Task 9: Convert `interpreter.rs` to directory module and split

**Files:**
- Rename: `src_bootstrap/interpreter.rs` → `src_bootstrap/interpreter/mod.rs`
- Create: `src_bootstrap/interpreter/values.rs`
- Create: `src_bootstrap/interpreter/eval.rs`
- Create: `src_bootstrap/interpreter/exec.rs`
- Create: `src_bootstrap/interpreter/builtins.rs`

**Step 1: Create directory and move**

```bash
mkdir -p src_bootstrap/interpreter
mv src_bootstrap/interpreter.rs src_bootstrap/interpreter/mod.rs
```

**Step 2: Run tests** (should pass)

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib 2>&1 | tail -3`
Expected: `test result: ok. 490 passed; 0 failed; 1 ignored`

**Step 3: Extract `values.rs`**

Move from `mod.rs`:
- `ControlFlow` enum (~line 9-14)
- `Value` enum (~line 22-36)
- `impl PartialEq for Value` (~line 38-57)
- `impl fmt::Debug for Value` (~line 59-77)

Into `interpreter/values.rs`:

```rust
use crate::ast::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub enum ControlFlow {
    Next,
    Return(Value),
    Break,
    Continue,
}

#[derive(Clone)]
pub enum Value {
    // ... all variants ...
}

impl PartialEq for Value { /* ... */ }
impl fmt::Debug for Value { /* ... */ }
```

Add to `mod.rs`:

```rust
pub mod values;
pub use values::{ControlFlow, Value};
```

**Step 4: Extract `builtins.rs`**

Move the `register_builtins` method (~line 114-307) into `interpreter/builtins.rs`:

```rust
use super::{Interpreter, values::Value};

impl Interpreter {
    pub(crate) fn register_builtins(&mut self) {
        // ... all builtin registration code ...
    }
}
```

Add `mod builtins;` to `mod.rs`.

**Step 5: Extract `eval.rs`**

Move the `evaluate` method (~line 578-end before tests) into `interpreter/eval.rs`:

```rust
use crate::ast::*;
use super::{Interpreter, values::{Value, ControlFlow}};

impl Interpreter {
    pub(crate) fn evaluate(&mut self, expr: &Expression) -> Result<Value, String> {
        // ... all expression evaluation code ...
    }
}
```

Add `mod eval;` to `mod.rs`.

**Step 6: Extract `exec.rs`**

Move the `execute` and `execute_statements` methods (~line 103-577) into `interpreter/exec.rs`:

```rust
use crate::ast::*;
use super::{Interpreter, values::{Value, ControlFlow}};

impl Interpreter {
    pub(crate) fn execute_statements(&mut self, stmts: &[Statement]) -> Result<ControlFlow, String> {
        // ...
    }

    pub(crate) fn execute(&mut self, stmt: &Statement) -> Result<ControlFlow, String> {
        // ...
    }
}
```

Add `mod exec;` to `mod.rs`.

**Step 7: Verify `mod.rs` is now small**

`interpreter/mod.rs` should now contain only:
- `pub mod values; pub mod eval; pub mod exec; mod builtins;`
- `pub use values::{ControlFlow, Value};`
- The `Interpreter` struct and `impl Default`
- `new()` and `run()` methods

**Step 8: Run tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib 2>&1 | tail -3`
Expected: `test result: ok. 490 passed; 0 failed; 1 ignored`

**Step 9: Commit**

```bash
git add src_bootstrap/interpreter/
git add -u
git commit -m "refactor: split interpreter.rs into mod/values/eval/exec/builtins"
```

---

### Task 10: Phase 1 checkpoint

**Step 1: Run full test suite**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib 2>&1 | tail -3`
Expected: `test result: ok. 490 passed; 0 failed; 1 ignored`

**Step 2: Verify file count**

```bash
find src_bootstrap -name '*.rs' | wc -l
```

Expected: ~145+ files (was 136, added ~10 new split files)

---

## Phase 2: Create Grouping Directories and Move Modules

> This is the big coordinated move. All files move at once, lib.rs is rewritten, and all import paths are updated.
> The code will NOT compile between steps — that's expected. Only test at the end of this phase.

---

### Task 11: Create new directory structure

**Files:**
- Create directories: `frontend/`, `frontend/semantic/`, `backend/`, `backend/codegen/`, `backend/vm/`, `domains/`

**Step 1: Create directories**

```bash
cd src_bootstrap
mkdir -p frontend/semantic
mkdir -p backend/codegen
mkdir -p backend/vm
mkdir -p domains
```

No commit yet — we'll commit after the full move.

---

### Task 12: Move frontend modules

**Step 1: Move ast.rs**

```bash
mv src_bootstrap/ast.rs src_bootstrap/frontend/ast.rs
```

**Step 2: Move lexer/**

```bash
mv src_bootstrap/lexer src_bootstrap/frontend/lexer
```

**Step 3: Move parser/**

```bash
mv src_bootstrap/parser src_bootstrap/frontend/parser
```

**Step 4: Move semantic.rs → frontend/semantic/analyzer.rs**

```bash
mv src_bootstrap/semantic.rs src_bootstrap/frontend/semantic/analyzer.rs
```

**Step 5: Move ownership/ → frontend/semantic/ownership/**

```bash
mv src_bootstrap/ownership src_bootstrap/frontend/semantic/ownership
```

**Step 6: Move traits/ → frontend/semantic/traits/**

```bash
mv src_bootstrap/traits src_bootstrap/frontend/semantic/traits
```

**Step 7: Create `frontend/mod.rs`**

```rust
//! Frontend: source code → AST pipeline

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod semantic;
```

**Step 8: Create `frontend/semantic/mod.rs`**

```rust
//! Semantic analysis, type checking, and ownership validation

pub mod analyzer;
pub mod ownership;
pub mod traits;

// Re-export the main analysis function
pub use analyzer::{SemanticAnalyzer, SymbolTable, Symbol, analyze};
```

---

### Task 13: Move backend modules

**Step 1: Move interpreter/**

```bash
mv src_bootstrap/interpreter src_bootstrap/backend/interpreter
```

**Step 2: Move compiler/ → backend/codegen/ (the subdirectory files)**

```bash
mv src_bootstrap/compiler/blocks.rs src_bootstrap/backend/codegen/blocks.rs
mv src_bootstrap/compiler/builder.rs src_bootstrap/backend/codegen/builder.rs
mv src_bootstrap/compiler/codegen.rs src_bootstrap/backend/codegen/emit.rs
```

**Step 3: Move compiler.rs → backend/codegen/mod.rs**

```bash
mv src_bootstrap/compiler.rs src_bootstrap/backend/codegen/mod.rs
rmdir src_bootstrap/compiler  # should be empty now
```

**Step 4: Update `backend/codegen/mod.rs`**

Change the sub-module declarations:

```rust
pub mod blocks;
pub mod builder;
pub mod emit;   // was: pub mod codegen;
```

Update imports from:
```rust
use crate::ast::Program;
use crate::lexer::Lexer;
use crate::parser::Parser;
```
to:
```rust
use crate::frontend::ast::Program;
use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;
```

Also update the `crate::semantic::analyze` call to `crate::frontend::semantic::analyze`.

**Step 5: Move runtime/vm.rs → backend/vm/mod.rs**

```bash
mv src_bootstrap/runtime/vm.rs src_bootstrap/backend/vm/mod.rs
```

Update `runtime/mod.rs` to remove `pub mod vm;` (it's no longer there).

**Step 6: Create `backend/mod.rs`**

```rust
//! Backend: AST → execution pipeline

pub mod interpreter;
pub mod codegen;
pub mod vm;
```

---

### Task 14: Move domain modules

**Step 1: Move domain directories**

```bash
mv src_bootstrap/advanced src_bootstrap/domains/advanced
mv src_bootstrap/embedded src_bootstrap/domains/embedded
mv src_bootstrap/ai src_bootstrap/domains/ai
mv src_bootstrap/dataflow src_bootstrap/domains/dataflow
```

**Step 2: Create `domains/mod.rs`**

```rust
//! Domain-specific language features

pub mod advanced;
pub mod embedded;
pub mod ai;
pub mod dataflow;
```

---

### Task 15: Rewrite `lib.rs`

Replace the entire module declaration section in `lib.rs` with the new structure.

**Before** (current — 21 top-level `pub mod` declarations):

```rust
pub mod advanced;
pub mod ai;
pub mod ast;
pub mod autonomous;
pub mod benchmarks;
pub mod compiler;
pub mod dataflow;
pub mod embedded;
pub mod interop;
pub mod interpreter;
pub mod lexer;
pub mod mcp;
pub mod memory;
pub mod ownership;
pub mod parser;
pub mod platform;
pub mod runtime;
pub mod semantic;
pub mod stdlib;
pub mod tools;
pub mod traits;
pub mod tui;
```

**After** (new — grouped hierarchy with re-exports):

```rust
// === Pipeline groups ===
pub mod frontend;
pub mod backend;

// === Runtime & support ===
pub mod runtime;
pub mod stdlib;

// === Domains ===
pub mod domains;

// === Tools & infrastructure ===
pub mod tools;
pub mod platform;
pub mod interop;
pub mod mcp;
pub mod tui;
pub mod autonomous;
pub mod memory;
pub mod benchmarks;

// === Backward-compatible re-exports ===
// These allow `use crate::ast::*` etc. to keep working in external code.
// IMPORTANT: These are TEMPORARY for Phase 2 migration.
// They will be REMOVED in Phase 3 (Task 17) after all imports are updated.
pub use frontend::ast;
pub use frontend::lexer;
pub use frontend::parser;
pub use frontend::semantic;
pub use frontend::semantic::ownership;
pub use frontend::semantic::traits;
pub use backend::interpreter;
pub use backend::codegen as compiler;
pub use domains::advanced;
pub use domains::ai;
pub use domains::embedded;
pub use domains::dataflow;
```

**Why temporary re-exports?** This lets us compile and test immediately after the move, before updating all 29 import paths. We remove them in Task 17.

---

### Task 16: Fix remaining module references

Some modules that moved have internal `use crate::` paths that need updating.

**Files to update** (complete list from exploration):

| File (new path) | Old import | New import |
|---|---|---|
| `frontend/parser/mod.rs` | `use crate::ast::{Type, *};` | `use crate::frontend::ast::{Type, *};` |
| `frontend/parser/mod.rs` | `use crate::lexer::Token;` | `use crate::frontend::lexer::Token;` |
| `frontend/parser/expressions.rs` | `use crate::ast::{Type, *};` | `use crate::frontend::ast::{Type, *};` |
| `frontend/parser/expressions.rs` | `use crate::lexer::Token;` | `use crate::frontend::lexer::Token;` |
| `frontend/parser/statements.rs` | `use crate::ast::{Type, *};` | `use crate::frontend::ast::{Type, *};` |
| `frontend/parser/statements.rs` | `use crate::lexer::Token;` | `use crate::frontend::lexer::Token;` |
| `frontend/parser/types.rs` | `use crate::ast::{Type, *};` | `use crate::frontend::ast::{Type, *};` |
| `frontend/parser/types.rs` | `use crate::lexer::Token;` | `use crate::frontend::lexer::Token;` |
| `frontend/parser/tests.rs` | `use crate::lexer::Lexer;` | `use crate::frontend::lexer::Lexer;` |
| `frontend/semantic/analyzer.rs` | `use crate::ast::*;` | `use crate::frontend::ast::*;` |
| `frontend/semantic/ownership/checker.rs` | `use crate::ast::Ownership;` | `use crate::frontend::ast::Ownership;` |
| `frontend/semantic/ownership/checker.rs` (tests) | `use crate::ast::Type;` | `use crate::frontend::ast::Type;` |
| `frontend/semantic/traits/builtin.rs` | `use crate::ast::BuiltinTrait;` | `use crate::frontend::ast::BuiltinTrait;` |
| `backend/interpreter/mod.rs` | `use crate::ast::*;` | `use crate::frontend::ast::*;` |
| `backend/interpreter/values.rs` | `use crate::ast::*;` | `use crate::frontend::ast::*;` |
| `backend/interpreter/eval.rs` | `use crate::ast::*;` | `use crate::frontend::ast::*;` |
| `backend/interpreter/exec.rs` | `use crate::ast::*;` | `use crate::frontend::ast::*;` |
| `backend/codegen/mod.rs` | (already updated in Task 13) | — |
| `backend/codegen/blocks.rs` | `use crate::ast::{Program, Statement};` | `use crate::frontend::ast::{Program, Statement};` |
| `backend/codegen/blocks.rs` (tests) | `use crate::ast::Expression;` | `use crate::frontend::ast::Expression;` |
| `backend/codegen/emit.rs` | `use crate::ast::*;` | `use crate::frontend::ast::*;` |
| `backend/vm/mod.rs` | `use crate::ast::Ownership;` | `use crate::frontend::ast::Ownership;` |
| `backend/vm/mod.rs` | `use crate::interpreter::Value;` | `use crate::backend::interpreter::Value;` |
| `domains/dataflow/contracts.rs` | `use crate::ast::{...};` | `use crate::frontend::ast::{...};` |
| `domains/dataflow/executor.rs` | `use crate::interpreter::Value;` | `use crate::backend::interpreter::Value;` |
| `domains/dataflow/graph.rs` | `use crate::ast::{...};` | `use crate::frontend::ast::{...};` |
| `domains/dataflow/ir.rs` | `use crate::ast::{...};` | `use crate::frontend::ast::{...};` |
| `domains/dataflow/mod.rs` | `use crate::ast::{...};` | `use crate::frontend::ast::{...};` |
| `domains/dataflow/validator.rs` | `use crate::ast::PortRef;` | `use crate::frontend::ast::PortRef;` |
| `stdlib.rs` | `use crate::interpreter::Value;` | `use crate::backend::interpreter::Value;` |
| `stdlib/nodes.rs` | `use crate::ast::{...};` | `use crate::frontend::ast::{...};` |
| `stdlib/tcp.rs` | `use crate::ast::Value;` | `use crate::frontend::ast::Value;` |
| `stdlib/udp.rs` | `use crate::ast::Value;` | `use crate::frontend::ast::Value;` |
| `stdlib/websocket.rs` | `use crate::ast::Value;` | `use crate::frontend::ast::Value;` |

**Strategy:** Use find-and-replace across `src_bootstrap/`:

```
use crate::ast::  →  use crate::frontend::ast::
use crate::lexer  →  use crate::frontend::lexer
use crate::parser →  use crate::frontend::parser
use crate::semantic →  use crate::frontend::semantic
use crate::interpreter →  use crate::backend::interpreter
```

**IMPORTANT:** Do NOT update paths in:
- `lib.rs` (uses `pub use` re-exports)
- `main.rs` (uses `gul_lang::` paths, updated separately)
- `runtime/mod.rs` (remove `pub mod vm;` since vm moved)

**Step: Run tests after ALL import updates**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib 2>&1 | tail -3`
Expected: `test result: ok. 490 passed; 0 failed; 1 ignored`

**Commit:**

```bash
git add -A
git commit -m "refactor: move modules into frontend/backend/domains hierarchy"
```

---

### Task 17: Remove temporary re-exports (clean break)

Now that all internal `crate::` paths are updated, remove the temporary re-exports from `lib.rs`.

**Files:**
- Modify: `src_bootstrap/lib.rs`

**Step 1: Remove re-export lines**

Delete these lines from `lib.rs`:

```rust
pub use frontend::ast;
pub use frontend::lexer;
pub use frontend::parser;
pub use frontend::semantic;
pub use frontend::semantic::ownership;
pub use frontend::semantic::traits;
pub use backend::interpreter;
pub use backend::codegen as compiler;
pub use domains::advanced;
pub use domains::ai;
pub use domains::embedded;
pub use domains::dataflow;
```

**Step 2: Run tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib 2>&1 | tail -3`
Expected: `test result: ok. 490 passed; 0 failed; 1 ignored`

If tests fail, some import was missed in Task 16. Fix them before proceeding.

**Step 3: Commit**

```bash
git add src_bootstrap/lib.rs
git commit -m "refactor: remove temporary re-exports, clean import paths"
```

---

## Phase 3: Update External References

> `main.rs`, tests, examples, and benchmarks reference `gul_lang::parser`, `gul_lang::lexer`, etc.
> These need updating to the new paths.

---

### Task 18: Update `main.rs`

**Files:**
- Modify: `src_bootstrap/main.rs`

**Changes:**

| Line | Old | New |
|------|-----|-----|
| 1 | `use gul_lang::{autonomous, benchmarks, compiler, tools};` | `use gul_lang::{autonomous, benchmarks, backend::codegen as compiler, tools};` |
| 237 | `gul_lang::lexer::Lexer` | `gul_lang::frontend::lexer::Lexer` |
| 239 | `gul_lang::parser::Parser` | `gul_lang::frontend::parser::Parser` |
| 242 | `gul_lang::interpreter::Interpreter` | `gul_lang::backend::interpreter::Interpreter` |
| 272 | `gul_lang::platform::package_support::PackageManager` | (unchanged — platform didn't move) |
| 399 | `gul_lang::ai::{AIManager, AIProvider}` | `gul_lang::domains::ai::{AIManager, AIProvider}` |
| 428 | `gul_lang::interop::python_runtime::PythonRuntime` | (unchanged — interop didn't move) |
| 438 | `gul_lang::interop::js_runtime::JavaScriptRuntime` | (unchanged) |
| 446 | `gul_lang::interop::rust_loader::RustLoader` | (unchanged) |
| 515 | `gul_lang::tui::GulTuiApp` | (unchanged — tui didn't move) |
| 597 | `gul_lang::mcp::cli::execute_cli()` | (unchanged — mcp didn't move) |
| 697 | `gul_lang::lexer::Lexer` | `gul_lang::frontend::lexer::Lexer` |
| 699 | `gul_lang::parser::Parser` | `gul_lang::frontend::parser::Parser` |
| 702 | `gul_lang::interpreter::Interpreter` | `gul_lang::backend::interpreter::Interpreter` |

**Step: Run full build**

Run: `cargo +stable-x86_64-pc-windows-msvc build 2>&1 | tail -5`
Expected: `Finished` with no errors

**Commit:**

```bash
git add src_bootstrap/main.rs
git commit -m "refactor: update main.rs to new module paths"
```

---

### Task 19: Update external files

**Files to update:**

1. `benches/performance.rs`:
   - `use gul_lang::lexer::Lexer;` → `use gul_lang::frontend::lexer::Lexer;`
   - `use gul_lang::parser::Parser;` → `use gul_lang::frontend::parser::Parser;`

2. `packages/testing/gul-test/src/lib.rs`:
   - `use gul_lang::{interpreter::Interpreter, lexer::Lexer, parser::Parser};`
   → `use gul_lang::{backend::interpreter::Interpreter, frontend::lexer::Lexer, frontend::parser::Parser};`

3. `src_bootstrap/bin/gul-mcp.rs`:
   - Check if it uses any moved modules (likely `gul_lang::mcp::server` which didn't move — probably no change needed)

**Step: Run full test suite**

Run: `cargo +stable-x86_64-pc-windows-msvc test 2>&1 | tail -5`
Expected: All tests pass

**Commit:**

```bash
git add benches/ packages/ src_bootstrap/bin/
git commit -m "refactor: update external references to new module paths"
```

---

### Task 20: Update `lib.rs` doc comments

**Files:**
- Modify: `src_bootstrap/lib.rs`

Update the module documentation at the top to reflect the new structure:

```rust
//! ## Module Overview
//!
//! ### Frontend (source → AST)
//! - [`frontend::ast`] - Abstract Syntax Tree definitions
//! - [`frontend::lexer`] - Tokenization and lexical analysis
//! - [`frontend::parser`] - Parse tokens into AST
//! - [`frontend::semantic`] - Semantic analysis, type checking, ownership
//!
//! ### Backend (AST → execution)
//! - [`backend::interpreter`] - Runtime interpretation and execution
//! - [`backend::codegen`] - Code generation and compilation
//! - [`backend::vm`] - Bytecode virtual machine
//!
//! ### Runtime & Standard Library
//! - [`runtime`] - OS/IO operations, filesystem, HTTP, database
//! - [`stdlib`] - Standard library (std.io, std.http, etc.)
//!
//! ### Domains
//! - [`domains::advanced`] - Symbolic math, physics, chemistry
//! - [`domains::embedded`] - ESP32, RP2040, HAL
//! - [`domains::ai`] - AI integration
//! - [`domains::dataflow`] - Reactive dataflow graphs
//!
//! ### Tools & Platform
//! - [`tools`] - Debugger, linter, formatter, IDE
//! - [`platform`] - WASM, mobile, packages
//! - [`interop`] - Python, JavaScript, Rust, C, SQL FFI
//! - [`mcp`] - Model Context Protocol (AI agents)
//! - [`tui`] - Terminal User Interface IDE
```

**Commit:**

```bash
git add src_bootstrap/lib.rs
git commit -m "docs: update lib.rs documentation for new module structure"
```

---

### Task 21: Final verification

**Step 1: Run lib tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib 2>&1 | tail -3`
Expected: `test result: ok. 490 passed; 0 failed; 1 ignored`

**Step 2: Run all tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test 2>&1 | tail -5`
Expected: All test suites pass

**Step 3: Run clippy**

Run: `cargo +stable-x86_64-pc-windows-msvc clippy 2>&1 | tail -10`
Expected: No errors (warnings are OK)

**Step 4: Verify directory structure**

```bash
ls src_bootstrap/frontend/
ls src_bootstrap/backend/
ls src_bootstrap/domains/
```

Expected:
```
frontend/: ast.rs  lexer/  mod.rs  parser/  semantic/
backend/:  codegen/  interpreter/  mod.rs  vm/
domains/:  advanced/  ai/  dataflow/  embedded/  mod.rs
```

**Step 5: Final commit**

```bash
git add -A
git commit -m "refactor: complete pipeline-centric architecture reorganization

Reorganized src_bootstrap/ from flat 21-module structure to grouped hierarchy:
- frontend/ (ast, lexer, parser, semantic)
- backend/ (interpreter, codegen, vm)
- domains/ (advanced, embedded, ai, dataflow)

Split oversized files:
- parser.rs (2,387 lines) → 5 files
- lexer/mod.rs (1,228 lines) → 3 files
- interpreter.rs (971 lines) → 5 files

All 490 tests passing. Zero behavior changes."
```

---

## Summary

| Phase | Tasks | What Happens |
|-------|-------|-------------|
| 1: Split | Tasks 1-10 | Split lexer, parser, interpreter into sub-modules in-place |
| 2: Move | Tasks 11-17 | Create frontend/backend/domains, move everything, update imports |
| 3: External | Tasks 18-21 | Update main.rs, tests, benches, docs; final verification |

**Total tasks:** 21
**Estimated new files:** ~15
**Estimated deleted files:** 3 (original large files replaced by directories)
**Lines of code moved:** ~4,600 (parser + lexer + interpreter)
**Import paths updated:** ~29
**Tests that must pass at end:** 490+ lib tests
