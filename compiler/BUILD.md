# Building and Using the GUL Compiler

## 🎯 Quick Start

The GUL compiler is written in GUL and transpiles to Rust for bootstrapping.

### Build the Compiler

```bash
cd /media/vu/512gb/blob/gul
./compiler/scripts/build_compiler.sh
```

This will:

1. Transpile all GUL compiler code to Rust
2. Create a Cargo project
3. Compile with `rustc`
4. Generate `compiler_rust/target/release/gul-compile`

### Use the Compiler

Once built, you can compile GUL programs:

```bash
# Compile a GUL file to Rust
./compiler_rust/target/release/gul-compile input.mn -o output.rs

# Then compile the Rust to binary
rustc output.rs -o program
./program
```

---

## 📋 Manual Build (if script fails)

If the automated build fails, you can manually transpile and fix issues:

### Step 1: Transpile to Rust

```bash
python3 compiler/scripts/bootstrap_transpiler.py
```

This creates `compiler_rust/` with all `.rs` files.

### Step 2: Review and Fix

The bootstrap transpiler is simple and may need manual fixes:

```bash
cd compiler_rust
# Edit any .rs files that have syntax errors
# Common fixes needed:
# - String interpolation (f-strings)
# - Complex match expressions
# - Type annotations
```

### Step 3: Compile

```bash
rustc --edition 2021 main.rs -o gul-compile
```

Or use Cargo:

```bash
cargo build --release
```

---

## 🧪 Testing the Compiler

### Create a Test File (`test.mn`)

```gul
# Simple GUL program
let x = @int(42)
let y = x + 10

fn greet(name: str) -> str:
    return "Hello, " + name

mn:
    let message = greet("World")
    print(message)
    print(y)
```

### Compile and Run

```bash
# Compile with GUL compiler
./compiler_rust/target/release/gul-compile test.mn -o test.rs

# Compile Rust output
rustc test.rs -o test

# Run!
./test
```

**Expected Output:**

```
Hello, World
52
```

---

## 📝 Compiler Options

```bash
gul-compile [OPTIONS] <input.mn>

Options:
  -o, --output <file>     Output file (default: input.rs)
  --no-semantic          Skip semantic analysis
  --verbose              Verbose output
  --emit-ast             Print AST (debugging)
  --help                 Show help
```

---

## 🔬 Development Workflow

### Modify the Compiler

1. Edit GUL source in `compiler/`
2. Rebuild: `./compiler/scripts/build_compiler.sh`
3. Test with example programs
4. Iterate!

### Add New Features

1. Update AST nodes in `compiler/ast/nodes.mn`
2. Add lexer tokens in `compiler/lexer/token.mn`
3. Add parser logic in `compiler/parser/parser.mn`
4. Add semantic checks in `compiler/semantic/analyzer.mn`
5. Add code generation in `compiler/codegen/rust_backend.mn`
6. Rebuild and test

---

## 🐛 Troubleshooting

### Build Errors

**Problem:** Transpiler generates invalid Rust  
**Solution:** The bootstrap transpiler is minimal. Check `compiler_rust/*.rs` files and fix syntax manually.

**Problem:** Missing dependencies  
**Solution:** Add to `Cargo.toml` or use `rustc` directly with `--extern`

**Problem:** Type mismatches  
**Solution:** GUL types may not map perfectly to Rust. Add explicit casts.

### Runtime Errors

**Problem:** Compiler crashes  
**Solution:** Check semantic analyzer - may have type errors

**Problem:** Generated code doesn't compile  
**Solution:** Code generator may have bugs. Check `compiler/codegen/rust_backend.mn`

---

## 📊 Testing

### Run Lexer Tests

```bash
# Once GUL interpreter/compiler works:
gul-compile compiler/tests/test_lexer.mn
rustc test_lexer.rs
./test_lexer
```

### Run All Tests

```bash
./compiler/scripts/run_tests.sh
```

---

## 🚀 Self-Hosting

Once the compiler works, we can achieve self-hosting:

```bash
# Compile the compiler with itself!
./compiler_rust/target/release/gul-compile compiler/main.mn -o compiler_v2.rs
rustc compiler_v2.rs -o gul-compile-v2

# Now this second-generation compiler should be identical
./gul-compile-v2 test.mn -o test.rs
```

**This proves the compiler is self-hosting!** 🎉

---

## 📦 Files Structure

```
compiler/
├── lexer/              Tokenization
│   ├── token.mn
│   └── lexer.mn
├── parser/             Parsing
│   ├── parser.mn
│   └── statement_parser.mn
├── ast/                AST nodes
│   └── nodes.mn
├── semantic/           Type checking
│   └── analyzer.mn
├── codegen/            Code generation
│   └── rust_backend.mn
├── main.mn             Compiler driver
├── tests/              Test suite
│   └── test_lexer.mn
└── scripts/            Build scripts
    ├── bootstrap_transpiler.py
    └── build_compiler.sh
```

---

## 🎓 How It Works

### Compilation Pipeline

```
GUL Source File
      ↓
   [Lexer]  → Tokens
      ↓
   [Parser] → AST
      ↓
  [Semantic] → Type-checked AST
      ↓
  [Codegen]  → Rust Code
      ↓
   [rustc]   → Machine Code
```

### Bootstrapping

```
Phase 1: GUL Compiler (written in GUL)
         ↓ (Python transpiler)
         Rust Code
         ↓ (rustc)
         Binary Compiler

Phase 2: Use Binary Compiler to compile itself
         → Self-hosting achieved!

Phase 3: Replace Rust backend with LLVM
         → No more Rust intermediate!
```

---

## 🔮 Future Enhancements

- [ ] LLVM backend (direct machine code)
- [ ] Optimization passes
- [ ] Better error messages
- [ ] Incremental compilation
- [ ] IDE integration (LSP)
- [ ] Package manager integration
- [ ] WebAssembly target
- [ ] Multiple backends (LLVM, Cranelift, custom)

---

## 💡 Tips

1. **Start simple** - Test with small programs first
2. **Check generated Rust** - The `.rs` output shows what went wrong
3. **Use verbose mode** - See each compilation phase
4. **Read the source** - The compiler is written in clean GUL!
5. **Contribute** - Found a bug? Fix it in the GUL source!

---

## 📚 Resources

- Compiler source: `compiler/`
- Documentation: `compiler/*.md`
- Examples: `examples/`
- Tests: `compiler/tests/`

---

**Ready to compile GUL with GUL?** 🚀

Run: `./compiler/scripts/build_compiler.sh`
