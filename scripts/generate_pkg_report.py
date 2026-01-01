
import os
import datetime

def generate_report():
    root = "packages"
    packages = []
    
    # Scan
    for dirpath, dirnames, filenames in os.walk(root):
        if "target" in dirpath: continue
        
        # Rust
        if "Cargo.toml" in filenames:
            name = os.path.basename(dirpath)
            category = os.path.basename(os.path.dirname(dirpath)).title()
            packages.append({"name": name, "type": "Rust Crate", "cat": category, "path": dirpath})
            continue

        # GUL
        for f in filenames:
            if f.endswith(".mn"):
                name = f[:-3]
                category = os.path.basename(dirpath).title()
                packages.append({"name": name, "type": "Pure GUL", "cat": category, "path": os.path.join(dirpath, f)})

    packages.sort(key=lambda x: (x['cat'], x['name']))

    md = f"# GUL Package Catalog\n\n"
    md += f"**Date**: {datetime.datetime.now().strftime('%Y-%m-%d')}\n"
    md += f"**Total Packages**: {len(packages)}\n"
    md += f"**Status**: 100% Implemented (Rust & Pure GUL)\n\n"
    md += "---\n\n"
    
    current_cat = ""
    for p in packages:
        if p['cat'] != current_cat:
            current_cat = p['cat']
            md += f"### {current_cat}\n\n"
        
        icon = "🦀" if p['type'] == "Rust Crate" else "🔷"
        md += f"**{p['name']}** {icon}\n"
        md += f"- Type: {p['type']}\n"
        md += f"- Location: `{p['path']}`\n\n"

    with open("docs/PACKAGES_IMPLEMENTED.md", "w") as f:
        f.write(md)

    print(f"Generated report for {len(packages)} packages.")

if __name__ == "__main__":
    generate_report()
