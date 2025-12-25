#!/usr/bin/env python3
"""
Generate API documentation for all packages.

This script generates API reference documentation from package sources.
"""

import os
import sys
import json
import tomllib
from pathlib import Path
from datetime import datetime


def load_toml(path: Path) -> dict | None:
    """Load and parse a TOML file."""
    try:
        with open(path, "rb") as f:
            return tomllib.load(f)
    except Exception:
        return None


def extract_doc_comments(file_path: Path) -> list[dict]:
    """Extract documentation comments from a GUL source file."""
    docs = []
    
    try:
        content = file_path.read_text()
        lines = content.split("\n")
        
        current_doc = []
        for i, line in enumerate(lines):
            stripped = line.strip()
            
            # Doc comments start with ///
            if stripped.startswith("///"):
                current_doc.append(stripped[3:].strip())
            elif stripped.startswith("//!"):
                # Module-level doc comment
                current_doc.append(stripped[3:].strip())
            elif current_doc:
                # End of doc comment, look for what it documents
                if "fn " in stripped or "let " in stripped or "pub " in stripped:
                    docs.append({
                        "line": i + 1,
                        "doc": "\n".join(current_doc),
                        "signature": stripped.split("{")[0].strip()
                    })
                current_doc = []
    except Exception:
        pass
    
    return docs


def generate_package_docs(package_dir: Path) -> dict:
    """Generate documentation for a single package."""
    package_name = package_dir.name
    
    doc = {
        "name": package_name,
        "version": "0.1.0",
        "description": "",
        "modules": [],
        "functions": [],
        "types": []
    }
    
    # Load manifest
    for manifest_name in ["gul.toml", "package.toml", "Cargo.toml"]:
        manifest_path = package_dir / manifest_name
        if manifest_path.exists():
            data = load_toml(manifest_path)
            if data:
                pkg = data.get("package", {})
                doc["version"] = pkg.get("version", "0.1.0")
                doc["description"] = pkg.get("description", "")
            break
    
    # Scan source files
    for source_file in package_dir.rglob("*.mn"):
        module_name = source_file.stem
        docs = extract_doc_comments(source_file)
        
        if docs:
            doc["modules"].append({
                "name": module_name,
                "file": str(source_file.relative_to(package_dir)),
                "items": docs
            })
    
    # Also scan for .gul files
    for source_file in package_dir.rglob("*.gul"):
        module_name = source_file.stem
        docs = extract_doc_comments(source_file)
        
        if docs:
            doc["modules"].append({
                "name": module_name,
                "file": str(source_file.relative_to(package_dir)),
                "items": docs
            })
    
    return doc


def generate_markdown(package_doc: dict) -> str:
    """Generate markdown documentation for a package."""
    lines = []
    
    lines.append(f"# {package_doc['name']}")
    lines.append("")
    lines.append(f"Version: {package_doc['version']}")
    lines.append("")
    
    if package_doc["description"]:
        lines.append(package_doc["description"])
        lines.append("")
    
    if package_doc["modules"]:
        lines.append("## Modules")
        lines.append("")
        
        for module in package_doc["modules"]:
            lines.append(f"### {module['name']}")
            lines.append("")
            lines.append(f"Source: `{module['file']}`")
            lines.append("")
            
            for item in module["items"]:
                if item.get("signature"):
                    lines.append(f"#### `{item['signature']}`")
                    lines.append("")
                if item.get("doc"):
                    lines.append(item["doc"])
                    lines.append("")
    
    return "\n".join(lines)


def main():
    """Main entry point."""
    project_root = Path(__file__).parent.parent.parent
    output_dir = project_root / "docs" / "api" / "packages"
    
    # Ensure output directory exists
    output_dir.mkdir(parents=True, exist_ok=True)
    
    all_packages = []
    
    # Process gul_packages
    gul_packages_dir = project_root / "gul_packages"
    if gul_packages_dir.exists():
        for package_dir in gul_packages_dir.iterdir():
            if not package_dir.is_dir():
                continue
            
            print(f"Processing {package_dir.name}...")
            
            package_doc = generate_package_docs(package_dir)
            all_packages.append(package_doc)
            
            # Generate markdown
            markdown = generate_markdown(package_doc)
            output_file = output_dir / f"{package_dir.name}.md"
            output_file.write_text(markdown)
    
    # Generate index
    index_lines = ["# Package API Reference", ""]
    index_lines.append(f"Generated: {datetime.now().isoformat()}")
    index_lines.append("")
    index_lines.append("## Packages")
    index_lines.append("")
    index_lines.append("| Package | Version | Description |")
    index_lines.append("|---------|---------|-------------|")
    
    for pkg in sorted(all_packages, key=lambda x: x["name"]):
        desc = pkg["description"][:50] + "..." if len(pkg["description"]) > 50 else pkg["description"]
        index_lines.append(f"| [{pkg['name']}]({pkg['name']}.md) | {pkg['version']} | {desc} |")
    
    index_file = output_dir / "index.md"
    index_file.write_text("\n".join(index_lines))
    
    # Save JSON index
    json_index = output_dir / "packages.json"
    with open(json_index, "w") as f:
        json.dump({
            "generated": datetime.now().isoformat(),
            "packages": all_packages
        }, f, indent=2)
    
    print(f"✅ Generated documentation for {len(all_packages)} packages")
    print(f"   Output: {output_dir}")


if __name__ == "__main__":
    main()
