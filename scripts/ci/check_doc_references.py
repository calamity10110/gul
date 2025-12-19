#!/usr/bin/env python3
"""Check documentation cross-references for broken links."""

import os
import re
import sys
from pathlib import Path


def find_markdown_files(docs_dir: str) -> list:
    """Find all markdown files in the docs directory."""
    return list(Path(docs_dir).rglob("*.md"))


def extract_references(content: str) -> list:
    """Extract all markdown links and references from content."""
    # Match [text](link) pattern
    link_pattern = r'\[([^\]]+)\]\(([^)]+)\)'
    return re.findall(link_pattern, content)


def check_internal_links(docs_dir: str) -> list:
    """Check all internal links in documentation."""
    errors = []
    md_files = find_markdown_files(docs_dir)
    
    for md_file in md_files:
        try:
            content = md_file.read_text(encoding='utf-8')
            references = extract_references(content)
            
            for text, link in references:
                # Skip external links
                if link.startswith(('http://', 'https://', '#', 'mailto:')):
                    continue
                    
                # Check if internal file exists
                if link.startswith('file://'):
                    continue
                    
                target = md_file.parent / link.split('#')[0]
                if not target.exists() and not link.startswith('/'):
                    errors.append(f"{md_file}: broken link to '{link}'")
        except Exception as e:
            errors.append(f"{md_file}: error reading file - {e}")
    
    return errors


def main():
    """Main entry point."""
    docs_dir = os.environ.get('DOCS_DIR', 'docs')
    
    if not os.path.exists(docs_dir):
        print(f"Documentation directory '{docs_dir}' not found")
        sys.exit(0)  # Don't fail if docs don't exist yet
    
    errors = check_internal_links(docs_dir)
    
    if errors:
        print("Documentation reference errors found:")
        for error in errors:
            print(f"  - {error}")
        # Warning only, don't fail the build
        print(f"\nTotal: {len(errors)} issues found")
    else:
        print("All documentation references are valid!")
    
    sys.exit(0)


if __name__ == "__main__":
    main()
