#!/bin/bash
# GUL Update Script
# Builds and installs the GUL compiler with all features

set -e

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║      GUL Update & Install Script       ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""

# Get script directory (project root)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo -e "${YELLOW}📦 Step 1: Clean previous build...${NC}"
cargo clean 2>/dev/null || true

echo -e "${YELLOW}🔧 Step 2: Building GUL (release mode)...${NC}"
cargo build --release

echo -e "${YELLOW}📥 Step 3: Installing GUL binary...${NC}"

# Install using cargo install
cargo install --path . --force

echo ""
echo -e "${GREEN}✅ GUL has been updated successfully!${NC}"
echo ""

# Show version info
echo -e "${BLUE}Installed commands:${NC}"
gul --help 2>/dev/null | head -20 || echo "  (run 'gul --help' to see commands)"

echo ""
echo -e "${GREEN}Usage:${NC}"
echo "  gul ide             # Launch TUI IDE"
echo "  gul ide --mode web  # Launch Web IDE"
echo "  gul build <file>    # Build a project"
echo "  gul run <file>      # Run a program"
echo ""
