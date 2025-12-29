
import os

def generate_kb():
    root = "packages"
    packages = []
    
    # Walk directory
    for dirpath, dirnames, filenames in os.walk(root):
        # Rust Packages
        if "Cargo.toml" in filenames:
            pkg_name = os.path.basename(dirpath)
            packages.append({
                "name": pkg_name,
                "type": "Rust Crate",
                "path": dirpath,
                "status": "Implemented"
            })
            # Don't traverse deeper into Rust crates
            dirnames[:] = []
            continue

        # Python & GUL Packages
        for f in filenames:
            if f.endswith(".py") and f != "__init__.py":
                packages.append({
                    "name": f[:-3],
                    "type": "Python Module",
                    "path": os.path.join(dirpath, f),
                    "status": "Implemented"
                })
            elif f.endswith(".mn"):
                packages.append({
                    "name": f[:-3],
                    "type": "Pure GUL",
                    "path": os.path.join(dirpath, f),
                    "status": "Implemented"
                })

    # Sort packages
    packages.sort(key=lambda x: (x['type'], x['name']))

    # Generate Markdown
    md = "# Project Knowledge Base\n\n"
    md += "**Generated**: Automatically by scripts/knowlege_base_gen.py\n"
    md += "**Scope**: All Packages in `packages/`\n\n"
    md += "## Package Inventory\n\n"
    md += "| Package Name | Type | Location | Status |\n"
    md += "|--------------|------|----------|--------|\n"
    
    for p in packages:
        md += f"| `{p['name']}` | {p['type']} | `{p['path']}` | {p['status']} |\n"

    md += "\n## Architecture Rules\n"
    md += "1. **System Core**: Must be implemented in **Rust** (High Performance).\n"
    md += "2. **Cloud/Extensions**: Can be Python (Rapid Integration).\n"
    md += "3. **Logic/App**: Must be Pure GUL (.mn).\n"
    md += "4. **NO DUPLICATES**: A package must exist in only one form.\n"

    with open("docs/PROJECT_KNOWLEDGE_BASE.md", "w") as f:
        f.write(md)
    
    print(f"Knowledge Base generated with {len(packages)} packages.")

if __name__ == "__main__":
    generate_kb()
