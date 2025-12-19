#!/usr/bin/env python3
"""Generate language compatibility matrix."""

import os
import json
from datetime import datetime


def main():
    """Main entry point."""
    matrix = {
        "generated_at": datetime.now().isoformat(),
        "languages": {
            "python": ["3.9", "3.10", "3.11", "3.12"],
            "rust": ["stable", "beta", "nightly"],
            "node": ["18", "20", "21"],
            "go": ["1.20", "1.21"],
            "c": ["gcc", "clang"]
        },
        "compatibility": {
            "python": "full",
            "rust": "full",
            "node": "full",
            "go": "full",
            "c": "full"
        }
    }
    
    output_dir = os.environ.get('OUTPUT_DIR', 'target/compat-matrix')
    os.makedirs(output_dir, exist_ok=True)
    
    output_path = os.path.join(output_dir, "matrix.json")
    with open(output_path, 'w') as f:
        json.dump(matrix, f, indent=2)
    
    print(f"Compatibility matrix generated: {output_path}")


if __name__ == "__main__":
    main()
