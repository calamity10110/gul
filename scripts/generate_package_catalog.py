"""
Generate PACKAGES_IMPLEMENTED.md by scanning the codebase.
"""
import os
import re

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
PACKAGES_DIR = os.path.join(ROOT, "packages")
DOC_FILE = os.path.join(ROOT, "docs", "PACKAGES_IMPLEMENTED.md")

def scan_packages():
    packages = []
    
    for root, _, files in os.walk(PACKAGES_DIR):
        for file in files:
            if not file.endswith(".py") or file == "__init__.py":
                continue
                
            path = os.path.join(root, file)
            rel_path = os.path.relpath(path, ROOT)
            
            with open(path, "r", encoding="utf-8") as f:
                content = f.read()
                
            # Extract info
            name_match = re.search(r'GUL\s+(.*?)\n', content)
            desc_match = re.search(r'\n(.*?)\.\n', content) # First line of desc
            status_match = re.search(r'Status:\s+(.*)', content)
            
            # Fallback name from filename
            name = name_match.group(1).strip() if name_match else os.path.splitext(file)[0].replace("_", "-")
            if not name.startswith("gul-") and "gul" in file:
                 name = "gul-" + name.lower().replace(" ", "-")
            
            desc = desc_match.group(1).strip() if desc_match else "No description"
            status = status_match.group(1).strip() if status_match else "Unknown"
            
            packages.append({
                "name": name,
                "description": desc,
                "status": status,
                "path": rel_path,
                "category": os.path.basename(root)
            })
            
    return sorted(packages, key=lambda x: x['name'])

def generate_markdown(packages):
    total = 180
    implemented = len(packages)
    percent = (implemented / total) * 100
    
    md = [
        "# GUL Package Catalog - Implemented Packages",
        "",
        "**Version**: 0.13.0  ",
        "**Syntax**: v3.2  ",
        "**Last Updated**: 2025-12-28",
        "",
        "---",
        "",
        "## 📊 Implementation Status",
        "",
        f"**Total Packages Planned**: {total}  ",
        f"**Implemented**: {implemented}  ",
        f"**Progress**: {percent:.1f}%",
        "",
        "---",
        "",
        f"## ✅ IMPLEMENTED PACKAGES ({implemented})",
        ""
    ]
    
    # Group by category
    by_category = {}
    for p in packages:
        cat = p['category'].capitalize()
        if cat not in by_category:
            by_category[cat] = []
        by_category[cat].append(p)
        
    for cat in sorted(by_category.keys()):
        md.append(f"### {cat} ({len(by_category[cat])} packages)")
        md.append("")
        
        for pkg in by_category[cat]:
            icon = "✅" if "Implemented" in pkg['status'] or "Production" in pkg['status'] else "🚧"
            md.append(f"**{pkg['name']}** {icon}")
            md.append(f"- **Status**: {pkg['status']}")
            md.append(f"- **Description**: {pkg['description']}")
            md.append(f"- **Location**: `{pkg['path']}`")
            md.append("")
        
        md.append("---")
        md.append("")
        
    return "\n".join(md)

if __name__ == "__main__":
    pkgs = scan_packages()
    content = generate_markdown(pkgs)
    with open(DOC_FILE, "w", encoding="utf-8") as f:
        f.write(content)
    print(f"Generated {DOC_FILE} with {len(pkgs)} packages.")
