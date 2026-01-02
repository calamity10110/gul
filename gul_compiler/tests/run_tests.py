import os
import subprocess
import glob
import sys

TEST_DIR = os.path.dirname(os.path.abspath(__file__))
# Adjust path to where binary is (workspace root target)
COMPILER = os.path.abspath(os.path.join(TEST_DIR, "../../target/debug/gulc"))

def run_test(file_path):
    print(f"Running {os.path.basename(file_path)}...")
    
    # Extract expectations
    expected_output = []
    with open(file_path, 'r') as f:
        for line in f:
            if "# EXPECT:" in line:
                expected_output.append(line.split("# EXPECT:", 1)[1].strip())
    
    # Compile
    exe_name = os.path.splitext(file_path)[0]
    # Quiet mode
    cmd = [COMPILER, file_path, "-o", exe_name, "--quiet"]
    result = subprocess.run(cmd, capture_output=True, text=True)
    if result.returncode != 0:
        print(f"Compilation failed:\n{result.stderr}")
        return False
        
    # Run
    result = subprocess.run([exe_name], capture_output=True, text=True)
    output = [line for line in result.stdout.strip().splitlines() if line]
    
    # Check output
    passed = True
    if len(expected_output) > 0:
        # Match expected lines
        if len(output) != len(expected_output):
             print(f"Output length mismatch.\nExpected: {expected_output}\nGot:      {output}")
             passed = False
        else:
             for i, (out, exp) in enumerate(zip(output, expected_output)):
                 if out.strip() != exp:
                     print(f"Line {i+1} mismatch. Expected '{exp}', got '{out.strip()}'")
                     passed = False
    
    if passed:
        print("PASS")
    else:
        print("FAIL")
    
    # Cleanup
    if os.path.exists(exe_name):
        os.remove(exe_name)
    if os.path.exists(exe_name + ".o"):
        os.remove(exe_name + ".o")
        
    return passed

if not os.path.exists(COMPILER):
    print(f"Compiler not found at {COMPILER}")
    sys.exit(1)

files = glob.glob(os.path.join(TEST_DIR, "*.gul"))
if not files:
    print("No test files found.")
    sys.exit(0)

passed_count = 0
for f in files:
    if run_test(f):
        passed_count += 1

print(f"\n{passed_count}/{len(files)} tests passed.")
if passed_count != len(files):
    sys.exit(1)
