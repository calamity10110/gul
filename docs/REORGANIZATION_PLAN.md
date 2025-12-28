# GUL Documentation Reorganization Plan

**Date**: 2025-12-28  
**Version**: v3.2  
**Status**: Complete Overhaul

---

## 📋 Summary

Comprehensive documentation update to v3.2 with MCP features across all directories.

---

## 📁 Directory Structure

```
docs/
├── README.md                 # Main hub - UPDATED ✅
├── INDEX.md                  # Complete index - NEW ✅
├── QUICK_REFERENCE.md        # Cheat sheet - UPDATED ✅
├── devhistory.md             # History - Keep as-is
├── V32_UPDATE_COMPLETE.md    # v3.2 changes - NEW ✅
│
├── api/                      # API Documentation
│   ├── standard-library.md   # Core modules
│   ├── http.md              # HTTP API
│   ├── database.md          # DB operations
│   ├── math-science.md      # Math/science
│   ├── filesystem.md        # File ops
│   ├── ui-components.md     # UI
│   └── MCP_COMPLETE.md      # MCP API - NEW ✅
│
├── guides/                   # Learning Guides
│   ├── quickstart.md        # 5-min start
│   ├── first-program.md     # Hello World
│   ├── MCP_QUICKSTART.md    # MCP setup - NEW ✅
│   ├── installation.md      # Install
│   ├── web-development.md   # Web apps
│   ├── data-analysis.md     # Data science
│   ├── iot-embedded.md      # Hardware
│   ├── tui.md              # Terminal IDE
│   ├── webui.md            # Web IDE
│   ├── dataflow.md         # Node programming
│   ├── compiler.md         # Compilation
│   ├── integration.md      # Integration
│   ├── creating-packages.md # Packages
│   └── course.md           # Full course
│
├── reference/                # Language Reference
│   ├── syntax.md            # Complete syntax
│   ├── types.md             # Type system
│   ├── ownership.md         # Memory model
│   ├── specification.md     # Full spec
│   ├── knowledgebase.md     # Comprehensive
│   ├── structure.md         # Program structure
│   └── package-catalog.md   # All packages
│
└── targets/                  # Platform Support
    └── platforms.md          # OS/arch support
```

---

## 🎯 Update Strategy

### Phase 1: Core Documentation ✅

- [x] agents.md
- [x] docs/README.md
- [x] docs/QUICK_REFERENCE.md
- [x] docs/INDEX.md

### Phase 2: API Documentation

- [ ] Update all api/\*.md to v3.2
- [ ] Add MCP examples
- [ ] Standardize format

### Phase 3: Guides

- [ ] Update syntax in all guides
- [ ] Add MCP integration examples
- [ ] Ensure consistency

### Phase 4: Reference

- [ ] Update language reference
- [ ] Document @ prefix system
- [ ] Update ownership docs

### Phase 5: Cleanup

- [ ] Remove outdated files
- [ ] Fix broken links
- [ ] Add cross-references

---

## 📝 Standard Format for All Docs

### Header Template

```markdown
# [Title]

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

[Brief description]

---

## Table of Contents

[...]

---

## Content

[...]

---

**Last Updated**: 2025-12-28
```

### Code Block Format

````markdown
````gul
# Always use v3.2 syntax
let x = @int(10)
let items = @list[1, 2, 3]
\```
````
````

---

## 🔄 Bulk Update Commands

### Find old syntax

```bash
cd docs
grep -r "let.*=" --include="*.md" | grep -v "@"
```

### Update patterns

```bash
# Update type syntax
sed -i 's/let \([a-z_]*\) = \[\([^]]*\)\]/let \1 = @list[\2]/g' **/*.md
sed -i 's/let \([a-z_]*\) = {\([^}]*\)}/let \1 = @dict{\2}/g' **/*.md
```

---

## ✅ Checklist

### All Files Must Have:

- [x] v3.2 syntax in all examples
- [x] @ prefix for types
- [x] Updated version number
- [x] Current date
- [x] Consistent formatting
- [x] Working code examples
- [x] MCP references where relevant

---

## 📊 Progress

Total files: 37  
Updated: 5  
Remaining: 32

---

## 🎯 Priority Files

### High Priority

1. docs/README.md ✅
2. docs/QUICK_REFERENCE.md ✅
3. docs/INDEX.md ✅
4. guides/quickstart.md
5. guides/MCP_QUICKSTART.md ✅
6. reference/syntax.md
7. api/standard-library.md

### Medium Priority

- All guides/
- All api/
- reference/types.md

### Low Priority

- devhistory.md (historical)
- targets/platforms.md

---

**Status**: Phase 1 Complete ✅  
**Next**: Batch update remaining files
