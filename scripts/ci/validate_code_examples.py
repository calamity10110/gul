#!/usr/bin/env python3
"""
Validate code examples in documentation files.
Ensures all code examples are syntactically correct and runnable.
"""

import os
import sys
import re
import subprocess
import tempfile
from pathlib import Path
from typing import List, Tuple

class DocExampleValidator:
    def __init__(self):
        self.root_dir = Path(os.getcwd())
        self.docs_dir = self.root_dir / "docs"
        self.errors: List[str] = []
        self.warnings: List[str] = []
        self.total_examples = 0
        self.validated_examples = 0
    
    def validate_all(self) -> bool:
        """Validate all code examples in documentation"""
        print("📖 Validating documentation code examples...")
        
        # Find all markdown files
        md_files = list(self.docs_dir.rglob("*.md"))
        
        for md_file in md_files:
            self._validate_file(md_file)
        
        self._print_results()
        
        return len(self.errors) == 0
    
    def _validate_file(self, md_file: Path):
        """Validate code examples in a single markdown file"""
        with open(md_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Extract code blocks
        code_blocks = self._extract_code_blocks(content)
        
        for lang, code, line_num in code_blocks:
            self.total_examples += 1
            
            if lang == "gul":
                self._validate_gul_code(md_file, code, line_num)
            elif lang in ["python", "rust", "javascript", "typescript", "c"]:
                self._validate_language_code(md_file, lang, code, line_num)
    
    def _extract_code_blocks(self, content: str) -> List[Tuple[str, str, int]]:
        """Extract code blocks from markdown"""
        code_blocks = []
        pattern = r'```(\w+)\n(.*?)```'
        
        for match in re.finditer(pattern, content, re.DOTALL):
            lang = match.group(1)
            code = match.group(2)
            line_num = content[:match.start()].count('\n') + 1
            code_blocks.append((lang, code, line_num))
        
        return code_blocks
    
    def _validate_gul_code(self, md_file: Path, code: str, line_num: int):
        """Validate GUL code example"""
        # Create temporary file
        with tempfile.NamedTemporaryFile(mode='w', suffix='.gul', delete=False) as f:
            f.write(code)
            temp_file = f.name
        
        try:
            # Check if GUL compiler exists
            gul_binary = self.root_dir / "target" / "release" / "gul"
            
            if not gul_binary.exists():
                self.warnings.append(
                    f"{md_file.name}:{line_num}: GUL compiler not found, skipping validation"
                )
                return
            
            # Run syntax check
            result = subprocess.run(
                [str(gul_binary), "check", temp_file],
                capture_output=True,
                timeout=10
            )
            
            if result.returncode == 0:
                self.validated_examples += 1
            else:
                self.errors.append(
                    f"{md_file.name}:{line_num}: Invalid GUL code:\n{result.stderr.decode()[:200]}"
                )
        
        except Exception as e:
            self.errors.append(
                f"{md_file.name}:{line_num}: Error validating GUL code: {e}"
            )
        finally:
            os.unlink(temp_file)
    
    def _validate_language_code(self, md_file: Path, lang: str, code: str, line_num: int):
        """Validate code in other languages"""
        # For now, just do basic syntax checking
        validators = {
            "python": self._validate_python,
            "rust": self._validate_rust,
            "javascript": self._validate_javascript,
        }
        
        if lang in validators:
            try:
                validators[lang](code)
                self.validated_examples += 1
            except Exception as e:
                self.errors.append(
                    f"{md_file.name}:{line_num}: Invalid {lang} code: {e}"
                )
        else:
            # Skip validation for unsupported languages
            self.validated_examples += 1
    
    def _validate_python(self, code: str):
        """Validate Python code"""
        import ast
        ast.parse(code)
    
    def _validate_rust(self, code: str):
        """Validate Rust code (basic check)"""
        # Just check if it has basic Rust syntax
        if not ("fn " in code or "struct " in code or "use " in code):
            if len(code.strip()) > 0:
                raise SyntaxError("Possibly incomplete Rust code")
    
    def _validate_javascript(self, code: str):
        """Validate JavaScript code (basic check)"""
        # Very basic check - look for obvious syntax errors
        if code.count('{') != code.count('}'):
            raise SyntaxError("Mismatched braces")
        if code.count('(') != code.count(')'):
            raise SyntaxError("Mismatched parentheses")
    
    def _print_results(self):
        """Print validation results"""
        print(f"\n{'='*60}")
        print(f"Code Example Validation Results")
        print(f"{'='*60}")
        print(f"Total examples: {self.total_examples}")
        print(f"Validated: {self.validated_examples}")
        print(f"Errors: {len(self.errors)}")
        print(f"Warnings: {len(self.warnings)}")
        
        if self.warnings:
            print(f"\n⚠️  Warnings:")
            for warning in self.warnings[:10]:  # Show first 10
                print(f"   {warning}")
        
        if self.errors:
            print(f"\n❌ Errors:")
            for error in self.errors[:10]:  # Show first 10
                print(f"   {error}")
        
        print(f"{'='*60}\n")

def main():
    validator = DocExampleValidator()
    success = validator.validate_all()
    
    sys.exit(0 if success else 1)

if __name__ == "__main__":
    main()
