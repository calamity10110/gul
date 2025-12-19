#!/usr/bin/env python3
"""Generate TUI screenshots for documentation."""

import os
import sys


def main():
    """Main entry point."""
    output_dir = os.environ.get('OUTPUT_DIR', 'docs/assets/screenshots/tui')
    os.makedirs(output_dir, exist_ok=True)
    
    # TUI screenshots require a display, which may not be available in CI
    # This is a placeholder that creates empty marker files
    
    screenshots = [
        "main_view",
        "file_tree",
        "editor",
        "command_palette",
    ]
    
    for name in screenshots:
        marker_path = os.path.join(output_dir, f"{name}.txt")
        with open(marker_path, 'w') as f:
            f.write(f"Placeholder for {name} TUI screenshot\n")
        print(f"Created placeholder: {marker_path}")
    
    print("TUI screenshot placeholders created!")
    print("Note: Actual TUI screenshots require a graphical environment")


if __name__ == "__main__":
    main()
