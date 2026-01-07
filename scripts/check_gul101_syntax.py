#!/usr/bin/env python3
"""
GUL v3.1 Syntax Checker for Documentation
Enforces v3.1 ownership syntax and patterns in all .md and .mn files
"""

import os
import re
import sys
from pathlib import Path

# ============================================
# v3.1 Deprecated Patterns (from v2.x)
# ============================================
DEPRECATED_PATTERNS = [
    # Old ownership as standalone keywords (not in node contracts)
    (r'^\s*own\s+(\w+)\s*=', 'let $1 = or use "own" in node contracts'),
    (r'^\s*ref\s+(\w+)\s*=', 'let $1 = or use "ref" in node contracts'),
    (r'\bcopy\s+@', '"gives @" ownership mode'),
    
    # Old function syntax (def instead of fn)
    (r'\bdef\s+(\w+)\s*\(', 'fn $1('),
    
    # Old import syntax (imp without @)
    (r'(?<![@@])imp\s+\w', '@imp'),
    
    # Old entry point (main() instead of mn:)
    (r'\bmain\s*\(\s*\)\s*:', 'mn:'),
    
    # Old async syntax (asy instead of async)
    (r'\basy\s+', 'async '),
    
    # Old const/mut (v2.x)
    (r'\bconst\s+(\w+)\s*=', 'let $1 ='),
    (r'\bmut\s+(\w+)\s*=', 'var $1 ='),
]

# ============================================
# v3.1 Valid Patterns (for validation)
# ============================================
VALID_PATTERNS = [
    # Node system
    r'node\s+\w+\s*[\(\[\{]',       # Node declarations
    r're_in:',                      # Required inputs
    r're_out:',                     # Required outputs
    
    # Ownership modes
    r'borrow\s+@\w+',               # Borrow mode
    r'ref\s+@\w+',                  # Ref mode in contracts
    r'move\s+@\w+',                 # Move mode
    r'kept\s+@\w+',                 # Kept mode
    
    # v3.1 imports
    r'@imp\s+',                     # New import style
    
    # Variables (both annotation styles)
    r'let\s+@\w+\([^)]+\)\s*=',     # let @type(name) = value
    r'let\s+\w+\s*=\s*@\w+\(',     # let name = @type(value)
    r'let\s+\w+\s*=',               # let name = value
    r'var\s+@\w+\([^)]+\)\s*=',     # var @type(name) = value
    r'var\s+\w+\s*=\s*@\w+\(',     # var name = @type(value)
    r'var\s+\w+\s*=',               # var name = value
    
    # Decorators
    r'@grad\s*',                    # Autograd decorator
    
    # Autograd Builtins
    r'gul_autograd_begin\(',
    r'gul_make_var\(',
    r'gul_backward\(',
    r'gul_print_float\(',
    
    # Functions
    r'fn\s+\w+\s*[\(\[\{]',         # fn name(...)
    r'async\s+\w+\s*[\(\[\{]',      # async name(...)
    
    # Main entry
    r'mn:',                         # Sequential main
    r'mn:\s*\[',                    # Graph-style main
    
    # Error handling
    r'\btry:',                      # Try block
    r'\bcatch\s+\w*:',              # Catch block
    r'\bfinally:',                  # Finally block
    
    # Bracket equivalence (all valid)
    r'\w+\([^)]*\)',                # func()
    r'\w+\[[^\]]*\]',               # func[]
    r'\w+\{[^}]*\}',                # func{}
]

# ============================================
# v3.1 Best Practices (warnings only)
# ============================================
BEST_PRACTICE_PATTERNS = [
    # Suggest async functions have parameters
    (r'async\s+\w+\s*\(\s*\):', 'Consider adding parameters to async functions'),
    
    # Suggest using graph-style for data pipelines
    (r'mn:\s*\n\s+\w+\s*->', 'Consider using graph-style: mn: [ ... ]'),
]

def check_file(filepath: Path, strict: bool = False) -> list:
    """Check a file for deprecated syntax."""
    issues = []
    warnings = []
    
    try:
        content = filepath.read_text()
    except Exception as e:
        return [(0, f"Could not read file: {e}", "error")]
    
    lines = content.split('\n')
    in_gul_code_block = False  # Only check GUL code blocks
    in_other_code_block = False  # Skip other code blocks
    in_foreign_block = False
    in_triple_quote = False  # Track triple-quoted strings (contain foreign code)
    foreign_brace_depth = 0
    is_mn_file = filepath.suffix in ['.mn', '.gul']
    
    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        
        # Track triple-quoted strings (often contain foreign code like python.exec)
        triple_quote_count = line.count('"""') + line.count("'''")
        if triple_quote_count % 2 == 1:
            in_triple_quote = not in_triple_quote
        if in_triple_quote:
            continue
        
        # Track markdown code blocks
        if stripped.startswith('```'):
            if stripped == '```gul' or stripped == '```':
                in_gul_code_block = not in_gul_code_block
            elif stripped.startswith('```') and len(stripped) > 3:
                # Other language code block (```python, ```rust, etc.)
                in_other_code_block = not in_other_code_block
            else:
                # Closing block
                in_gul_code_block = False
                in_other_code_block = False
            continue
        
        # Skip non-GUL code blocks in markdown
        if in_other_code_block:
            continue
        
        # Track foreign code blocks (@python {, @rust {, @c {, @sql {, @cs lang:)
        if re.search(r'@(python|rust|c|sql|js|go|java)\s*[\{:]', line):
            in_foreign_block = True
            foreign_brace_depth = 1
            continue
        
        # Old cross-language syntax: @cs lang:
        if re.search(r'@cs\s+\w+:', line):
            in_foreign_block = True
            foreign_brace_depth = 1
            continue
        
        if in_foreign_block:
            foreign_brace_depth += line.count('{') - line.count('}')
            if foreign_brace_depth <= 0:
                in_foreign_block = False
                foreign_brace_depth = 0
            continue
            
        # Skip comments (but not in code blocks)
        if stripped.startswith('#') and not in_gul_code_block:
            continue
        
        # Check deprecated patterns
        for pattern, suggestion in DEPRECATED_PATTERNS:
            if re.search(pattern, line):
                issues.append((i, f"Deprecated: → {suggestion}", "error"))
        
        # Check best practices (warnings only)
        for pattern, suggestion in BEST_PRACTICE_PATTERNS:
            if re.search(pattern, line):
                warnings.append((i, f"Suggestion: {suggestion}", "warning"))
    
    if strict:
        return issues + warnings
    return issues

def check_directory(directory: Path, extensions: list = None, strict: bool = False) -> dict:
    """Check all files in directory."""
    if extensions is None:
        extensions = ['.md', '.mn', '.gul']
        
    results = {}
    
    # Files to exclude (historical/legacy documentation)
    exclude_files = {'devhistory.md', 'CHANGELOG.md', 'MIGRATION.md', 'MIGRATION_v31.md'}
    exclude_dirs = {'target', 'node_modules', '.git', '__pycache__'}
    
    for ext in extensions:
        for filepath in directory.rglob(f'*{ext}'):
            # Skip excluded directories
            if any(excl in filepath.parts for excl in exclude_dirs):
                continue
            # Skip excluded files
            if filepath.name in exclude_files:
                continue
                
            issues = check_file(filepath, strict)
            if issues:
                results[str(filepath)] = issues
    
    return results

def print_results(results: dict) -> None:
    """Print check results with colors."""
    if not results:
        print("✅ No deprecated v3.1 syntax found!")
        return
    
    error_count = 0
    warning_count = 0
    
    print(f"\n{'='*60}")
    print("GUL v3.1 Syntax Check Results")
    print(f"{'='*60}\n")
    
    for filepath, issues in sorted(results.items()):
        print(f"📄 {filepath}:")
        for line, msg, level in issues:
            if level == "error":
                print(f"   ❌ Line {line}: {msg}")
                error_count += 1
            else:
                print(f"   ⚠️  Line {line}: {msg}")
                warning_count += 1
        print()
    
    print(f"{'='*60}")
    print(f"Summary: {error_count} errors, {warning_count} warnings in {len(results)} files")
    print(f"{'='*60}")

def main():
    """Main entry point."""
    import argparse
    
    parser = argparse.ArgumentParser(
        description='GUL v3.1 Syntax Checker',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python check_gul101_syntax.py docs/
  python check_gul101_syntax.py examples/ --strict
  python check_gul101_syntax.py . --extensions .mn .gul
        """
    )
    parser.add_argument('path', nargs='?', default='.', help='Path to check')
    parser.add_argument('--fix', action='store_true', help='Attempt to fix issues (not implemented)')
    parser.add_argument('--strict', action='store_true', help='Include warnings and exit with error if issues found')
    parser.add_argument('--extensions', nargs='+', default=['.md', '.mn', '.gul'], help='File extensions to check')
    parser.add_argument('--quiet', action='store_true', help='Minimal output')
    
    args = parser.parse_args()
    path = Path(args.path)
    
    if not path.exists():
        print(f"Error: Path '{path}' does not exist")
        return 1
    
    if path.is_file():
        issues = check_file(path, args.strict)
        if issues:
            print(f"\n{path}:")
            for line, msg, level in issues:
                prefix = "❌" if level == "error" else "⚠️"
                print(f"  {prefix} Line {line}: {msg}")
            return 1 if args.strict else 0
        elif not args.quiet:
            print(f"✅ {path}: No issues found")
    else:
        results = check_directory(path, args.extensions, args.strict)
        
        if not results:
            if not args.quiet:
                print("✅ No deprecated v3.1 syntax found!")
            return 0
        
        print_results(results)
        
        if args.strict:
            return 1
    
    return 0

if __name__ == '__main__':
    sys.exit(main())
