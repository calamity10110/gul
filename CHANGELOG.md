# Changelog

All notable changes to GUL will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Comprehensive documentation suite (66+ files)
- Rust documentation with detailed module descriptions
- AI-optimized knowledgebase
- Complete README with 60+ documentation links
- Learning tracks (Beginner → Advanced)
- FAQ and troubleshooting guides

### Changed

- Reorganized project directory structure
- Cleaned up root directory (moved test files, docs)
- Enhanced lib.rs with comprehensive rustdoc comments

### Fixed

- All compiler warnings resolved
- Documentation 100% aligned (version, syntax)
- semantic module properly declared

## [0.13.0] - 2025-12-30

### Added

- **v3.2 Syntax**: Modern `let`/`var` keywords, `@imp`, `@type` constructors
- **180+ Packages**: Comprehensive ecosystem across 22 categories
- **MCP Server**: AI-powered development integration
- **13 Standard Library Modules**: Complete stdlib implementation
- **Embedded Support**: ESP32-S3 and RP2040 targets
- **Foreign Code Integration**: Python, Rust, JavaScript, SQL
- **Ownership System**: `borrow`, `ref`, `move`, `kept` modes
- **Async/Await**: Built-in cooperative multitasking
- **TUI IDE**: Terminal user interface
- **Web IDE**: Browser-based development environment

### Changed

- Migrated from v2.0 to v3.2 syntax
- Updated all examples to v3.2 syntax
- Enhanced error handling with Result types
- Improved type inference system

### Fixed

- Memory leaks in interpreter
- Parser edge cases with nested structures
- Foreign code execution stability
- Embedded GPIO driver issues

### Security

- Added cryptographic stdlib module
- Implemented secure secrets management
- Enhanced memory safety checks

## [0.12.0] - 2025-12-xx

### Added

- Initial release
- Basic compiler and interpreter
- Core standard library
- Python integration
- Simple package system
- Documentation framework

### Known Issues

- Limited stdlib coverage
- No async support
- Basic error messages
- Performance needs optimization

---

## Release Tags

- [0.13.0]: https://github.com/calamity10110/gul/releases/tag/v0.13.0
- [0.12.0]: https://github.com/calamity10110/gul/releases/tag/v0.12.0

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for how to contribute to this project.
