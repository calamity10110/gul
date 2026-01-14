# GUL Downloads

Download pre-built GUL binaries for your platform.

## Latest Release: v0.14.0-dev

### Desktop & Server Platforms

#### Linux (x86_64)

```bash
wget https://github.com/calamity10110/gul/releases/download/v0.14.0-dev/gul-v0.14.0-dev-linux-x86_64.tar.gz
tar xzf gul-v0.14.0-dev-linux-x86_64.tar.gz
sudo mv gul /usr/local/bin/
sudo mv gul-mcp /usr/local/bin/
```

Verify installation:

```bash
gul --version
```

#### Windows (x86_64)

1. Download: [gul-v0.14.0-dev-windows-x86_64.zip](https://github.com/calamity10110/gul/releases/download/v0.14.0-dev/gul-v0.14.0-dev-windows-x86_64.zip)
2. Extract the ZIP file
3. Add the extracted directory to your PATH

Verify installation:

```powershell
gul --version
```

#### macOS (Universal Binary)

```bash
wget https://github.com/calamity10110/gul/releases/download/v0.14.0-dev/gul-v0.14.0-dev-macos-universal.tar.gz
tar xzf gul-v0.14.0-dev-macos-universal.tar.gz
sudo mv gul /usr/local/bin/
sudo mv gul-mcp /usr/local/bin/
```

Verify installation:

```bash
gul --version
```

---

### Embedded Platforms

#### Raspberry Pi (ARMv7 - 32-bit)

For Raspberry Pi 2, 3, 4 running 32-bit OS:

```bash
wget https://github.com/calamity10110/gul/releases/download/v0.14.0-dev/gul-v0.14.0-dev-rpi-armv7.tar.gz
tar xzf gul-v0.14.0-dev-rpi-armv7.tar.gz
sudo mv gul /usr/local/bin/
sudo mv gul-mcp /usr/local/bin/
```

#### Raspberry Pi (ARM64 - 64-bit)

For Raspberry Pi 3, 4, 5 running 64-bit OS:

```bash
wget https://github.com/calamity10110/gul/releases/download/v0.14.0-dev/gul-v0.14.0-dev-rpi-arm64.tar.gz
tar xzf gul-v0.14.0-dev-rpi-arm64.tar.gz
sudo mv gul /usr/local/bin/
sudo mv gul-mcp /usr/local/bin/
```

#### ESP32-S3

For ESP32-S3 microcontrollers:

```bash
wget https://github.com/calamity10110/gul/releases/download/v0.14.0-dev/gul-v0.14.0-dev-esp32s3.zip
unzip gul-v0.14.0-dev-esp32s3.zip
```

Flash to ESP32-S3:

```bash
esptool.py --chip esp32s3 --port /dev/ttyUSB0 write_flash 0x10000 gul-esp32s3.bin
```

---

## Platform Support Matrix

| Platform | Architecture | Binary | Auto-Built | Tested |
|----------|-------------|--------|-----------|--------|
| **Linux** | x86_64 | `gul`, `gul-mcp` | ✅ Yes | ✅ Yes |
| **Windows** | x86_64 | `gul.exe`, `gul-mcp.exe` | ✅ Yes | ✅ Yes |
| **macOS** | Universal (Intel + Apple Silicon) | `gul`, `gul-mcp` | ✅ Yes | ✅ Yes |
| **Raspberry Pi** | ARMv7 (32-bit) | `gul`, `gul-mcp` | ✅ Yes | ⚠️  Limited |
| **Raspberry Pi** | ARM64 (64-bit) | `gul`, `gul-mcp` | ✅ Yes | ⚠️  Limited |
| **ESP32-S3** | Xtensa | `gul-esp32s3.bin` | ✅ Yes | ⚠️  Experimental |

---

## Build from Source

If pre-built binaries aren't available for your platform:

### Prerequisites

- Rust 1.70+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- Git

### Build Steps

```bash
# Clone repository
git clone https://github.com/calamity10110/gul.git
cd gul

# Build release
cargo build --release

# Install
sudo cp target/release/gul /usr/local/bin/
sudo cp target/release/gul-mcp /usr/local/bin/
```

### Cross-Compilation

For embedded targets, see [docs/guides/CROSS_COMPILATION.md](../guides/CROSS_COMPILATION.md).

---

## Checksums

SHA256 checksums for v0.14.0-dev:

```
# Linux
<checksum> gul-v0.14.0-dev-linux-x86_64.tar.gz

# Windows  
<checksum> gul-v0.14.0-dev-windows-x86_64.zip

# macOS
<checksum> gul-v0.14.0-dev-macos-universal.tar.gz

# Raspberry Pi
<checksum> gul-v0.14.0-dev-rpi-armv7.tar.gz
<checksum> gul-v0.14.0-dev-rpi-arm64.tar.gz

# ESP32-S3
<checksum> gul-v0.14.0-dev-esp32s3.zip
```

Verify downloads:

```bash
sha256sum -c checksums.txt
```

---

## Nightly Builds

Nightly builds are available from the [Actions tab](https://github.com/calamity10110/gul/actions) after each commit to `main`.

---

## Version History

- **v0.14.0-dev** (Latest) - Multi-platform release, MCP integration, v3.2 syntax
- **v0.12.0** - Enhanced foreign code integration
- **v0.11.0** - Package management system
- See [CHANGELOG.md](../../CHANGELOG.md) for full history

---

## Support

- **Issues**: [GitHub Issues](https://github.com/calamity10110/gul/issues)
- **Discussions**: [GitHub Discussions](https://github.com/calamity10110/gul/discussions)
- **Documentation**: [docs.gul-lang.org](https://docs.gul-lang.org)
