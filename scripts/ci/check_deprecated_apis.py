#!/usr/bin/env python3
"""Check for deprecated API usage in the codebase."""

import os
import re
import sys
from pathlib import Path


DEPRECATED_PATTERNS = [
    (r'\bconst\s+', "Use 'let' instead of 'const'"),
    (r'\bmut\s+', "Use 'var' instead of 'mut'"),
    (r'\bmain\(\):', "Use 'mn:' instead of 'main():'"),
    (r'\bimport\s+', "Use '@imp' instead of 'import'"),
    (r'\bextern\s+', "Use '@python/@rust/@c' instead of 'extern'"),
]


def check_file(file_path: Path) -> list:
    """Check a file for deprecated API usage."""
    issues = []
    
    try:
        content = file_path.read_text(encoding='utf-8')
        lines = content.split('\n')
        
        for i, line in enumerate(lines, 1):
            for pattern, message in DEPRECATED_PATTERNS:
                if re.search(pattern, line):
                    issues.append({
                        "file": str(file_path),
                        "line": i,
                        "message": message,
                        "content": line.strip()[:50]
                    })
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
    
    return issues


def main():
    """Main entry point."""
    src_dir = os.environ.get('SRC_DIR', 'examples')
    
    if not os.path.exists(src_dir):
        print(f"Directory '{src_dir}' not found")
        sys.exit(0)
    
    all_issues = []
    
    for file_path in Path(src_dir).rglob("*.mn"):
        issues = check_file(file_path)
        all_issues.extend(issues)
    
    if all_issues:
        print(f"Found {len(all_issues)} deprecated API usages:")
        for issue in all_issues:
            print(f"  {issue['file']}:{issue['line']}: {issue['message']}")
    else:
        print("No deprecated API usage found!")
    
    # Don't fail the build, just warn
    sys.exit(0)


if __name__ == "__main__":
    main()
