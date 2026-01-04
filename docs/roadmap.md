# GUL Project Roadmap

**Vision**: GUL is a data-flow driven, ML-native programming language that compiles to efficient native code.

## Version History

| Version | Status | Key Features |
|---------|--------|--------------|
| v3.0 | Released | New syntax (`@fn`, `mn:`, `@imp`), pipeline operator `|>` |
| v3.1 | Released | Collections API (List, Set, Dict), f-strings, match |
| v3.2 | Current | Ownership semantics, Cranelift codegen, tuple support |

---

## Q1 2026: Core Compiler Stability

### Milestone 1: Dual Compiler Parity

- [x] Fix `gul_stable` compilation errors
- [ ] Migrate `gul_stable` to Cranelift codegen
- [ ] Shared test suite passing on both compilers
- [ ] Shared C runtime (`stdlib.c`) for both compilers

### Milestone 2: CI/CD Consolidation ✅

- [x] Single unified CI workflow for compilers (`ci.yml`)
- [x] Automated test execution for shared tests
- [ ] Cross-platform builds (Linux, macOS, Windows)

> **See Also**: [testfiles.md](./testfiles.md) for complete test documentation.

---

## Q2 2026: ML & Data-Flow Features

### Milestone 3: Tensor Primitives

- [ ] `@tensor` type with shape inference
- [ ] Basic tensor operations (add, mul, matmul)
- [ ] SIMD acceleration via Cranelift

### Milestone 4: Auto-Differentiation

- [ ] `@grad` decorator for automatic gradients
- [ ] Tape-based differentiation engine
- [ ] Integration with tensor primitives

---

## Q3 2026: Ecosystem Expansion

### Milestone 5: Standard Library

- [ ] `std.flow` - Channels and async pipelines
- [ ] `std.simd` - SIMD vector operations
- [ ] `std.tensor` - Core tensor library
- [ ] `std.alloc` - Arena allocators

### Milestone 6: Package Ecosystem

- [ ] `gul-dataframe` - Lazy data processing
- [ ] `gul-http` - HTTP client/server
- [ ] `gul-sql` - Database connectivity
- [ ] Package registry and `gul install`

---

## Q4 2026: Developer Experience

### Milestone 7: Tooling

- [ ] Language Server (LSP) implementation
- [ ] `gul doc` - API documentation generator
- [ ] `gul fmt` - Code formatter
- [ ] VS Code extension

### Milestone 8: Performance

- [ ] Profile-guided optimization
- [ ] Incremental compilation
- [ ] Build caching

---

## Long-Term Vision

### 2027+

- **GUL v4.0**: Full self-hosting (compiler written in GUL)
- **GPU Support**: CUDA/Metal backends for tensors
- **Distributed**: Native actor model for distributed computing
- **WebAssembly**: Full WASM target support

---

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for how to contribute to GUL development.

### Priority Areas

1. Cranelift codegen improvements
2. Test coverage expansion
3. Documentation improvements
4. Package ecosystem growth
