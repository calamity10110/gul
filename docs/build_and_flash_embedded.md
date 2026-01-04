---
description: How to build and flash embedded firmware/examples
---

# Embedded Build & Flash Workflow

The GUL compiler (`gul build`) for firmware is currently in a prototype state and acts as a validator. To run code on the ESP32-S3, you should use the Rust crates in `packages/embedded`.

## Option 1: Run Embedded Package Examples

The most reliable way to verify hardware is to run the examples provided in the embedded packages.

1. **Navigate to the package:**

   ```bash
   cd packages/embedded/gul-gpio
   ```

2. **Run an example (builds and flashes):**

   ```bash
   cargo run --example basic --target xtensa-esp32s3-espidf
   # OR if .cargo/config.toml is set:
   cargo run --example basic
   ```

   *Note: Ensure you have `espflash` installed (`cargo install espflash`) and your device is connected.*

## Option 2: GUL CLI (Experimental)

To validate the `main.mn` script syntax:

```bash
gul build firmware/esp32-s3-amoled/main.mn --target esp32s3
```

*Currently, this command only validates the GUL script and does not produce a flashable binary.*

## Prerequisites

- **Rust & Cargo**: Standard installation.
- **ESP-IDF Tools**: `espup install` and `. $HOME/export-esp.sh` (or appropriate env vars).
- **espflash**: `cargo install espflash`.
