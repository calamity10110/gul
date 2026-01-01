import os
import re

ROOT = "packages"

def check_cargo_toml(path):
    with open(path, 'r') as f:
        content = f.read()
    
    issues = []
    if 'name =' not in content: issues.append("Missing name")
    if 'version =' not in content: issues.append("Missing version")
    # if 'description =' not in content: issues.append("Missing description") # Optional but recommended
    # if 'license =' not in content: issues.append("Missing license") # Optional
    return issues

def audit():
    total_packages = 0
    issues_found = 0
    
    print(f"{'Package':<40} | {'Status':<10} | {'Issues'}")
    print("-" * 80)
    
    for root, dirs, files in os.walk(ROOT):
        if "Cargo.toml" in files:
            total_packages += 1
            toml_path = os.path.join(root, "Cargo.toml")
            problems = check_cargo_toml(toml_path)
            
            # structural checks
            if not os.path.exists(os.path.join(root, "src")):
                problems.append("Missing src/")
            # readme check (already done by docgen, but verify)
            if not os.path.exists(os.path.join(root, "README.md")):
                problems.append("Missing README.md")

            if problems:
                issues_found += 1
                print(f"{os.path.basename(root):<40} | FAIL       | {', '.join(problems)}")
            else:
                pass 
                # print(f"{os.path.basename(root):<40} | OK         |")

    print("-" * 80)
    print(f"Total Packages: {total_packages}")
    print(f"Packages with Issues: {issues_found}")
    print(f"Health: {((total_packages - issues_found)/total_packages)*100:.1f}%")

if __name__ == "__main__":
    audit()
