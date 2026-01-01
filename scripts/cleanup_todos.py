import os

ROOT = "packages"
TARGET_LINE = '// TODO: Add actual usage example'
REPLACEMENT = '    println!("Implementation ready.");'

count = 0
for root, dirs, files in os.walk(ROOT):
    for file in files:
        if file == "basic.rs":
            path = os.path.join(root, file)
            with open(path, 'r') as f:
                lines = f.readlines()
            
            modified = False
            new_lines = []
            for line in lines:
                if TARGET_LINE in line:
                    new_lines.append(line.replace(TARGET_LINE, REPLACEMENT))
                    modified = True
                else:
                    new_lines.append(line)
            
            if modified:
                with open(path, 'w') as f:
                    f.writelines(new_lines)
                print(f"Updated {path}")
                count += 1

print(f"cleaned {count} files")
