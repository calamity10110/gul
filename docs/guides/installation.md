# Installation

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# Installation Guide

Welcome to the GUL installation guide! This document will walk you through installing GUL on various platforms and setting up your development environment.

## 📋 Prerequisites

### System Requirements

- **Operating System**: Linux, macOS, Windows, or BSD
- **RAM**: Minimum 2GB, 4GB+ recommended
- **Disk Space**: 500MB for GUL + dependencies
- **Internet Connection**: Required for package installation

### Required Dependencies

GUL requires Rust 1.70 or later. You can install Rust using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, ensure Rust is in your PATH:

```bash
source $HOME/.cargo/env
rustc --version
```

## 🚀 Installation Methods

### Method 1: Install from Source (Recommended for Development)

#### Step 1: Clone the Repository

```bash
git clone https://github.com/gul-lang/gul.git
cd gul
```

#### Step 2: Build GUL

```bash
cargo build --release
```

This will compile GUL with optimizations. The binary will be located at `target/release/gul`.

#### Step 3: Install System-Wide (Optional)

```bash
cargo install --path .
```

This installs the `gul` binary to `~/.cargo/bin/`, which should be in your PATH.

#### Step 4: Verify Installation

```bash
gul --version
```

You should see output like:

```bash
gul 0.13.0
```

### Method 2: Install from Cargo (When Published)

Once GUL is published to crates.io, you can install it directly:

```bash
cargo install gul
```

### Method 3: Download Pre-Built Binaries (Coming Soon)

Pre-built binaries for major platforms will be available from the releases page:

```bash
# Linux
wget https://github.com/gul-lang/gul/releases/download/v0.13.0/gul-linux-x86_64.tar.gz
tar -xzf gul-linux-x86_64.tar.gz
sudo mv gul /usr/local/bin/

# macOS
wget https://github.com/gul-lang/gul/releases/download/v0.13.0/gul-macos-x86_64.tar.gz
tar -xzf gul-macos-x86_64.tar.gz
sudo mv gul /usr/local/bin/

# Windows (using PowerShell)
Invoke-WebRequest -Uri "https://github.com/gul-lang/gul/releases/download/v0.13.0/gul-windows-x86_64.zip" -OutFile "gul.zip"
Expand-Archive -Path gul.zip -DestinationPath C:\Program Files\GUL
```

## 🔧 Platform-Specific Instructions

### Linux

#### Ubuntu/Debian

Install build dependencies:

```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev git curl
```

Then follow the standard installation from source.

#### Fedora/RHEL/CentOS

```bash
sudo dnf install gcc pkg-config openssl-devel git curl
```

#### Arch Linux

```bash
sudo pacman -S base-devel openssl git curl
```

### macOS

#### Using Homebrew (Coming Soon)

```bash
brew install gul
```

#### Manual Installation

Install Xcode Command Line Tools:

```bash
xcode-select --install
```

Then follow the standard installation from source.

### Windows

#### Using WSL2 (Recommended)

1. Install WSL2 with Ubuntu
2. Follow the Linux installation instructions

#### Native Windows

1. Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
2. Install [Rust for Windows](https://www.rust-lang.org/tools/install)
3. Clone and build:

```powershell
git clone https://github.com/gul-lang/gul.git
cd gul
cargo build --release
```

### FreeBSD/OpenBSD

```bash
pkg install rust git curl
```

Then follow the standard installation from source.

## 🎨 IDE Setup

### TUI IDE

The GUL TUI IDE is built-in and ready to use:

```bash
gul ide
# or explicitly
gul ide --mode tui
```

### Web IDE

Launch the web-based IDE:

```bash
gul ide --mode web
```

This will start a local web server default: <http://localhost:8080>

### VS Code Integration (Coming Soon)

Install the GUL extension from the VS Code marketplace:

1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "GUL Language"
4. Click Install

### Vim/Neovim Integration (Coming Soon)

```bash
git clone https://github.com/gul-lang/vim-gul.git ~/.vim/pack/plugins/start/vim-gul
```

## 📦 Package Manager Setup

### Initialize Package Configuration

Create a package manifest for your project:

```bash
gul init
```

This creates a `gul.toml` file with project metadata.

### Install Packages

Install packages from the GUL registry:

```bash
gul install http
gul install database
gul install ui
```

### Configure Registry (Optional)

Set a custom package registry:

```bash
export GUL_REGISTRY=https://custom-registry.example.com
```

Or add to `gul.toml`:

```toml
[registry]
url = "https://custom-registry.example.com"
```

## 🔑 Environment Configuration

### Environment Variables

GUL respects the following environment variables:

- `GUL_HOME`: GUL installation directory (default: `~/.mn`)
- `GUL_REGISTRY`: Package registry URL
- `GUL_CACHE_DIR`: Package cache directory
- `GUL_LOG_LEVEL`: Logging level (trace, debug, info, warn, error)

Example `.bashrc`/`.zshrc` configuration:

```bash
export GUL_HOME=$HOME/.mn
export GUL_REGISTRY=https://packages.mn-lang.org
export GUL_CACHE_DIR=$HOME/.cache/gul
export GUL_LOG_LEVEL=info
export PATH=$PATH:$HOME/.cargo/bin
```

### Shell Completion

Generate shell completion scripts:

```bash
# Bash
gul completion bash > ~/.local/share/bash-completion/completions/gul

# Zsh
gul completion zsh > ~/.zfunc/_gul

# Fish
gul completion fish > ~/.config/fish/completions/gul.fish
```

## ✅ Verification

### Test Your Installation

Create a simple test file `hello.mn`:

```gul
main:
    print("Hello, GUL!")
```

Run it:

```bash
gul run hello.mn
```

Expected output:

```bash
Hello, GUL!
```

### Run Built-in Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test integration_tests
```

### Check System Information

```bash
gul --version
gul --help
```

## 🐛 Troubleshooting

### Common Issues

#### "gul: command not found"

**Solution**: Ensure `~/.cargo/bin` is in your PATH:

```bash
echo 'export PATH=$HOME/.cargo/bin:$PATH' >> ~/.bashrc
source ~/.bashrc
```

#### Build Errors on Linux

**Solution**: Install missing dependencies:

```bash
sudo apt install build-essential pkg-config libssl-dev
```

#### Permission Denied on macOS

**Solution**: Use sudo for system-wide installation:

```bash
sudo cargo install --path .
```

#### Windows: "VCRUNTIME140.dll not found"

**Solution**: Install [Visual C++ Redistributable](https://aka.ms/vs/17/release/vc_redist.x64.exe)

#### "Cannot connect to package registry"

**Solution**: Check your internet connection and firewall settings. Try:

```bash
curl -I https://packages.mn-lang.org
```

### Performance Issues

If GUL runs slowly:

1. **Use Release Builds**: Always use `--release` flag
2. **Increase Memory**: Close other applications
3. **Check Disk Space**: Ensure adequate free space
4. **Update Rust**: `rustup update stable`

### Getting Help

If you encounter issues not covered here:

- **Documentation**: [gul-lang.org/docs](https://gul-lang.org/docs)
- **GitHub Issues**: [github.com/gul-lang/gul/issues](https://github.com/gul-lang/gul/issues)
- **Community Forum**: [forum.mn-lang.org](https://forum.mn-lang.org)
- **Discord**: [discord.gg/gul-lang](https://discord.gg/gul-lang)

## 📚 Next Steps

Now that GUL is installed:

1. **Read the Quick Start**: [tutorials/quickstart.md](../tutorials/quickstart.md)
2. **Write Your First Program**: [tutorials/first-program.md](../tutorials/first-program.md)
3. **Explore Examples**: Check the `examples/` directory
4. **Learn the Syntax**: [reference/syntax.md](../reference/syntax.md)

## 🔄 Updating GUL

### From Source

```bash
cd gul
git pull origin main
cargo build --release
cargo install --path .
```

### From Cargo

```bash
cargo install gul --force
```

### Check for Updates

```bash
gul check-update
```

## 🗑️ Uninstallation

### Remove GUL Binary

```bash
cargo uninstall gul
```

### Remove All Files

```bash
rm -rf ~/.mn
rm -rf ~/.cache/gul
```

---

**Last Updated**: 2025-12-28  
**Version: 0.13.0  
**License**: MIT

For more information, visit [gul-lang.org](https://gul-lang.org)
