# GUL Self-Hosting Compiler

**Status:** Phase 1 - Bootstrap (GUL → Rust Transpilation)  
**Version:** 0.13.0  
**Syntax:** v3.2  
**Started:** 2025-12-31

---

## 🎯 Project Goals

Build a **self-hosting compiler** where GUL can compile itself, eliminating dependency on the Rust bootstrap compiler.

### Three-Phase Roadmap

#### **Phase 1: Bootstrap (Current)**

- ✅ Write full compiler in GUL v3.2
- ✅ Transpile GUL compiler → Rust
- ✅ Compile transpiled Rust with rustc
- 🎯 **Goal:** GUL compiler that works via transpilation

#### **Phase 2: Self-Hosting**

- 🔄 GUL compiler can compile itself
- 🔄 Bootstrap from Phase 1 transpiled code
- 🎯 **Goal:** `gul compile compiler/*.mn` produces working compiler

#### **Phase 3: Independence**

- 🔮 Remove Rust bootstrap dependency
- 🔮 Pure GUL toolchain
- 🎯 **Goal:** Distribute GUL without requiring Rust

---

## 📁 Directory Structure

```
compiler/
├── README.md                 # This file
├── ARCHITECTURE.md           # Compiler architecture
├── main.mn                   # Main compiler driver
│
├── lexer/
│   ├── lexer.mn             # Tokenization logic
│   ├── token.mn             # Token definitions
│   └── char_stream.mn       # Character stream reader
│
├── parser/
│   ├── parser.mn            # Parser implementation
│   ├── precedence.mn        # Operator precedence
│   └── recovery.mn          # Error recovery
│
├── ast/
│   ├── nodes.mn             # AST node definitions
│   ├── types.mn             # Type representations
│   ├── visitor.mn           # AST visitor pattern
│   └── printer.mn           # AST pretty-printer (debugging)
│
├── semantic/
│   ├── analyzer.mn          # Semantic analysis
│   ├── type_checker.mn      # Type checking
│   ├── scope.mn             # Scope management
│   └── validator.mn         # Code validation
│
├── codegen/
│   ├── generator.mn         # Code generation orchestrator
│   ├── rust_backend.mn      # Rust code generation
│   ├── wasm_backend.mn      # WASM generation (future)
│   └── optimizer.mn         # Optimization passes
│
├── runtime/
│   ├── builtins.mn          # Built-in functions
│   ├── stdlib_bridge.mn     # Bridge to standard library
│   └── error.mn             # Error handling utilities
│
├── transpiler/
│   ├── gul_to_rust.mn       # GUL → Rust transpiler
│   ├── mappings.mn          # Type/syntax mappings
│   └── runtime_lib.rs       # Minimal Rust runtime support
│
└── tests/
    ├── test_lexer.mn        # Lexer tests
    ├── test_parser.mn       # Parser tests
    ├── test_semantic.mn     # Semantic tests
    ├── test_codegen.mn      # Code generation tests
    ├── test_integration.mn  # Full pipeline tests
    └── test_fixtures/       # Test input files
        ├── valid/
        └── invalid/
```

---

## 🏗️ Architecture Overview

### Compiler Pipeline

```
Source Code (.mn)
    ↓
┌─────────────────┐
│  LEXER          │  → Tokens
│  lexer.mn       │
└─────────────────┘
    ↓
┌─────────────────┐
│  PARSER         │  → AST
│  parser.mn      │
└─────────────────┘
    ↓
┌─────────────────┐
│  SEMANTIC       │  → Validated AST
│  analyzer.mn    │
└─────────────────┘
    ↓
┌─────────────────┐
│  CODE GEN       │  → Target Code
│  generator.mn   │     (Rust/WASM/etc)
└─────────────────┘
```

### Bootstrap Process (Phase 1)

```
1. Write compiler in GUL (compiler/*.mn)
2. Manually transpile to Rust (or use simple transpiler)
3. Compile with rustc → gul-compiler binary
4. Use gul-compiler to compile GUL programs
```

---

## 🔧 Design Principles

### 1. **Pure GUL First**

- Minimize `@rust {}` blocks
- Only use for: file I/O, system calls, critical performance
- Document all foreign code usage

### 2. **Compatibility**

- Match existing compiler behavior (src_bootstrap)
- Fix known bugs
- NO breaking changes to v3.2 syntax

### 3. **Error Handling**

- Use `Result<T, E>` for expected errors (parse errors, type errors)
- Use exceptions for panics (internal compiler errors)
- Rich error messages with source locations

### 4. **Testing**

- Unit tests for each module
- Comparison tests with bootstrap compiler
- Golden tests (expected output)
- Self-compilation test (Phase 2)

### 5. **Performance**

- **Phase 1:** Correctness only
- **Phase 2:** Profile and optimize hot paths
- **Phase 3:** Match or exceed Rust bootstrap performance

---

## 📚 Dependencies

### Standard Library Modules Used

```gul
@imp std.io          # File reading/writing
@imp std.fs          # Filesystem operations
@imp std.collections # HashMap, Vec, etc.
@imp std.json        # AST serialization (debugging)
@imp std.time        # Performance profiling
```

### Minimal Foreign Code

```gul
# Only where absolutely necessary:
@rust {
    use std::fs::File;
    use std::io::Read;
    
    pub fn read_file_fast(path: &str) -> Result<String, String> {
        // High-performance file reading
    }
}
```

---

## 🧪 Testing Strategy

### 1. **Unit Tests** (per module)

```gul
# tests/test_lexer.mn
@imp compiler.lexer.lexer
@imp std.testing

fn test_tokenize_integers():
    let tokens = lexer.tokenize("123 456")
    assert_eq(tokens.len(), 2)
    assert_eq(tokens[0].type, TokenType.Integer)
    assert_eq(tokens[0].value, "123")
```

### 2. **Comparison Tests**

```bash
# Compare with bootstrap compiler
./test_compare.sh program.mn
```

### 3. **Integration Tests**

```gul
# tests/test_integration.mn
fn test_full_compilation():
    let source = "@imp std.io\nmn:\n    print(\"Hello\")"
    let result = compiler.compile(source)
    assert(result.is_ok())
```

### 4. **Golden Tests**

```
tests/test_fixtures/valid/hello.mn     → tests/golden/hello.output
```

---

## 🚀 Getting Started

### Compile the Compiler (Phase 1)

```bash
# Step 1: Transpile GUL compiler to Rust
cd compiler
python3 transpiler/transpile.py . > ../target/gul_compiler.rs

# Step 2: Compile transpiled Rust
cd ../target
rustc gul_compiler.rs -o gul-compiler

# Step 3: Use the compiler
./gul-compiler ../examples/hello_world.mn
```

### Self-Compilation Test (Phase 2 Goal)

```bash
# Use GUL compiler to compile itself
./gul-compiler compiler/main.mn -o gul-compiler-v2

# Verify it works
./gul-compiler-v2 examples/hello_world.mn
```

---

## 📈 Progress Tracking

### Phase 1 Checklist

#### Foundation

- [ ] Token definitions (ast/token.mn)
- [ ] AST node definitions (ast/nodes.mn)
- [ ] Error types (runtime/error.mn)

#### Lexer (Week 1-2)

- [ ] Character stream reader
- [ ] Token recognition
- [ ] Lexer tests
- [ ] Comparison with bootstrap lexer

#### Parser (Week 3-4)

- [ ] Recursive descent parser
- [ ] Operator precedence
- [ ] Error recovery
- [ ] Parser tests

#### Semantic Analysis (Week 5-6)

- [ ] Scope management
- [ ] Type checking
- [ ] Validation rules
- [ ] Semantic tests

#### Code Generation (Week 7-8)

- [ ] Rust backend
- [ ] Transpiler implementation
- [ ] Optimization passes
- [ ] Code generation tests

#### Integration (Week 9-10)

- [ ] Main compiler driver
- [ ] CLI argument parsing
- [ ] Full pipeline tests
- [ ] Performance profiling

#### Bootstrap (Week 11-12)

- [ ] Transpile compiler to Rust
- [ ] Build bootstrap binary
- [ ] Verify all tests pass
- [ ] Documentation

---

## 🎓 Learning Resources

### For Contributors

- [GUL v3.2 Syntax](../docs/QUICK_REFERENCE.md)
- [Language Specification](../docs/reference/specification.md)
- [Bootstrap Compiler Source](../src_bootstrap/)
- [Compiler Design Patterns](ARCHITECTURE.md)

### Recommended Reading

- "Crafting Interpreters" by Robert Nystrom
- "Modern Compiler Implementation" by Andrew Appel
- "Engineering a Compiler" by Cooper & Torczon

---

## 🤝 Contributing

All compiler code must:

1. Be written in **pure GUL v3.2** syntax
2. Include **unit tests**
3. Match **bootstrap compiler behavior**
4. Have **clear documentation**
5. Pass **all existing tests**

---

## 📝 Notes

### Why Self-Hosting?

1. **Dog-fooding**: Best way to test GUL is to write real software in it
2. **Independence**: No dependency on Rust ecosystem
3. **Optimization**: Compiler understands its own language best
4. **Legitimacy**: Serious languages are self-hosted
5. **Learning**: Great educational project

### Challenges

1. **Bootstrapping paradox**: Need a compiler to build the compiler
2. **Performance**: Interpreted GUL may be slower than compiled Rust
3. **Tooling**: Need good debugging for compiler written in GUL
4. **Testing**: Must maintain compatibility with existing ecosystem

---

**Next Steps:** Start with `lexer/token.mn` and `lexer/lexer.mn`
