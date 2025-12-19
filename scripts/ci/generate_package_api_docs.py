#!/usr/bin/env python3
"""Generate package API documentation from GUL packages."""

import os
import json
from pathlib import Path


def generate_package_docs(packages_dir: str, output_dir: str):
    """Generate API documentation for all packages."""
    os.makedirs(output_dir, exist_ok=True)
    
    packages_path = Path(packages_dir)
    if not packages_path.exists():
        print(f"Packages directory '{packages_dir}' not found")
        return
    
    # Find all package metadata files
    for pkg_dir in packages_path.iterdir():
        if pkg_dir.is_dir():
            metadata_file = pkg_dir / "package.json"
            if metadata_file.exists():
                try:
                    with open(metadata_file, 'r') as f:
                        metadata = json.load(f)
                    generate_single_package_doc(metadata, pkg_dir, output_dir)
                except Exception as e:
                    print(f"Error processing {pkg_dir}: {e}")


def generate_single_package_doc(metadata: dict, pkg_dir: Path, output_dir: str):
    """Generate documentation for a single package."""
    name = metadata.get('name', pkg_dir.name)
    version = metadata.get('version', '0.0.0')
    description = metadata.get('description', 'No description')
    
    doc_content = f"""# {name}

**Version**: {version}

{description}

## Installation

```gul
@imp pkg.{name}
```

## API

See package source for API details.
"""
    
    output_path = Path(output_dir) / f"{name}.md"
    with open(output_path, 'w') as f:
        f.write(doc_content)
    
    print(f"Generated docs for {name}")


def main():
    """Main entry point."""
    packages_dir = os.environ.get('PACKAGES_DIR', 'packages')
    output_dir = os.environ.get('OUTPUT_DIR', 'docs/api/generated/packages')
    
    generate_package_docs(packages_dir, output_dir)
    print("Package API documentation generated!")


if __name__ == "__main__":
    main()
