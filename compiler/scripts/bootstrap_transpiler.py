#!/usr/bin/env python3
"""
Simple GUL to Rust Transpiler (Bootstrap Version)
Converts the GUL compiler code to Rust so we can compile it.

This is a minimal transpiler just for bootstrapping - not production quality.
Once the GUL compiler is working, it will replace this.
"""

import re
import sys
from pathlib import Path

class GulToRustTranspiler:
    def __init__(self):
        self.indent_level = 0
        self.output = []
        
    def transpile_file(self, gul_code):
        """Transpile GUL code to Rust"""
        lines = gul_code.split('\n')
        
        for line in lines:
            self.process_line(line)
        
        return '\n'.join(self.output)
    
    def process_line(self, line):
        """Process a single line of GUL code"""
        stripped = line.strip()
        
        # Skip empty lines and comments
        if not stripped or stripped.startswith('#'):
            self.output.append(line.replace('#', '//'))
            return
        
        # Imports
        if stripped.startswith('@imp'):
            self.transpile_import(stripped)
            return
        
        # Struct definitions
        if stripped.startswith('struct '):
            self.transpile_struct_start(stripped)
            return
        
        # Enum definitions
        if stripped.startswith('enum '):
            self.transpile_enum_start(stripped)
            return
        
        # Function definitions
        if stripped.startswith('fn '):
            self.transpile_function(stripped)
            return
        
        # Let/var declarations
        if stripped.startswith('let '):
            self.transpile_let(line)
            return
        
        if stripped.startswith('var '):
            self.transpile_var(line)
            return
        
        # Match expressions
        if stripped.startswith('match '):
            self.transpile_match(line)
            return
        
        # Control flow
        if stripped.startswith(('if ', 'elif ', 'else:', 'while ', 'for ', 'return ')):
            self.transpile_control_flow(line)
            return
        
        # Default: pass through with minor modifications
        self.output.append(self.convert_syntax(line))
    
    def transpile_import(self, line):
        """Convert @imp to use"""
        # @imp std.collections -> use std::collections;
        module = line.replace('@imp ', '').strip()
        module = module.replace('.', '::')
        self.output.append(f"use {module};")
    
    def transpile_struct_start(self, line):
        """Convert struct definition"""
        # struct Name: -> struct Name {
        self.output.append(line.replace(':', ' {'))
    
    def transpile_enum_start(self, line):
        """Convert enum definition"""
        # enum Name: -> enum Name {
        self.output.append(line.replace(':', ' {'))
    
    def transpile_function(self, line):
        """Convert function definition"""
        # fn name(params) -> type: -> fn name(params) -> type {
        line = line.replace('ref self', '&mut self')
        line = line.replace('self', '&self')
        line = line.replace(':', ' {', 1)
        line = self.convert_types(line)
        self.output.append(line)
    
    def transpile_let(self, line):
        """Convert let declaration"""
        converted = self.convert_types(line)
        converted = self.convert_syntax(converted)
        if not converted.rstrip().endswith(';'):
            converted = converted.rstrip() + ';'
        self.output.append(converted)
    
    def transpile_var(self, line):
        """Convert var to let mut"""
        converted = line.replace('var ', 'let mut ')
        converted = self.convert_types(converted)
        converted = self.convert_syntax(converted)
        if not converted.rstrip().endswith(';'):
            converted = converted.rstrip() + ';'
        self.output.append(converted)
    
    def transpile_match(self, line):
        """Convert match expression"""
        converted = self.convert_syntax(line)
        self.output.append(converted)
    
    def transpile_control_flow(self, line):
        """Convert control flow statements"""
        line = line.replace('elif ', 'else if ')
        line = line.replace('else:', 'else {')
        line = self.convert_syntax(line)
        self.output.append(line)
    
    def convert_types(self, line):
        """Convert GUL types to Rust types"""
        # @int -> i64, @str -> String, etc.
        line = line.replace('@int', 'i64')
        line = line.replace('@float', 'f64')
        line = line.replace('@str', 'String')
        line = line.replace('@bool', 'bool')
        line = line.replace('@list', 'Vec')
        line = line.replace('@dict', 'HashMap')
        line = line.replace('@set', 'HashSet')
        line = line.replace('@tuple', '')  # Remove @tuple prefix
        return line
    
    def convert_syntax(self, line):
        """Convert GUL syntax to Rust syntax"""
        # f"..." -> format!(...)
        line = re.sub(r'f"([^"]*)"', lambda m: self.convert_fstring(m.group(1)), line)
        
        # not -> !
        line = re.sub(r'\bnot\b', '!', line)
        
        # and -> &&, or -> ||
        line = re.sub(r'\band\b', '&&', line)
        line = re.sub(r'\bor\b', '||', line)
        
        # None -> None (Rust's Option::None)
        line = re.sub(r'\bNone\b', 'Option::None', line)
        
        # true/false are same
        
        # : at end of line -> {
        if line.rstrip().endswith(':') and not line.strip().startswith('#'):
            line = line.rstrip()[:-1] + ' {'
        
        return line
    
    def convert_fstring(self, content):
        """Convert f-string to format!"""
        # Simple conversion - just wrap in format!
        # This won't handle complex expressions perfectly
        return f'format!("{content}")'

def transpile_gul_file(input_file, output_file):
    """Transpile a GUL file to Rust"""
    print(f"Transpiling {input_file} -> {output_file}")
    
    with open(input_file, 'r') as f:
        gul_code = f.read()
    
    transpiler = GulToRustTranspiler()
    rust_code = transpiler.transpile_file(gul_code)
    
    # Add standard prelude
    prelude = """// Auto-generated from GUL source
#![allow(unused_variables, dead_code, unused_mut)]

use std::collections::{HashMap, HashSet, VecDeque};

"""
    
    with open(output_file, 'w') as f:
        f.write(prelude + rust_code)
    
    print(f"✅ Generated {output_file}")

def transpile_compiler():
    """Transpile the entire GUL compiler"""
    compiler_dir = Path('compiler')
    output_dir = Path('compiler_rust')
    output_dir.mkdir(exist_ok=True)
    
    # Find all .mn files
    gul_files = list(compiler_dir.rglob('*.mn'))
    
    print(f"Found {len(gul_files)} GUL files to transpile")
    print()
    
    for gul_file in gul_files:
        # Calculate relative path
        rel_path = gul_file.relative_to(compiler_dir)
        
        # Create output path (change .mn to .rs)
        rust_file = output_dir / rel_path.with_suffix('.rs')
        rust_file.parent.mkdir(parents=True, exist_ok=True)
        
        # Transpile
        try:
            transpile_gul_file(gul_file, rust_file)
        except Exception as e:
            print(f"❌ Error transpiling {gul_file}: {e}")
    
    print()
    print(f"✅ Transpiled {len(gul_files)} files to {output_dir}")
    print()
    print("Next steps:")
    print("  1. cd compiler_rust")
    print("  2. Create Cargo.toml")
    print("  3. rustc main.rs")

if __name__ == '__main__':
    if len(sys.argv) > 1:
        # Transpile specific file
        input_file = sys.argv[1]
        output_file = sys.argv[2] if len(sys.argv) > 2 else input_file.replace('.mn', '.rs')
        transpile_gul_file(input_file, output_file)
    else:
        # Transpile entire compiler
        transpile_compiler()
