# Cross-Compilation

**Version**: 0.14.0-dev | **Updated**: 2026-01-08

> This page is a work in progress. See the [Platforms Guide](../docs/targets/platforms.md) for supported targets.

## Overview

GUL supports cross-compilation to multiple targets including embedded systems (ESP32-S3, RP2040), WebAssembly, and standard desktop platforms.

## Building for a Target

```bash
# Build for a specific target
gul build --target <target-triple>

# Example: build for WebAssembly
gul build --target wasm32-unknown-unknown

# Example: build for ARM embedded
gul build --target thumbv7em-none-eabihf
```

## Supported Targets

See [docs/targets/platforms.md](../docs/targets/platforms.md) for the full list of supported platforms.
