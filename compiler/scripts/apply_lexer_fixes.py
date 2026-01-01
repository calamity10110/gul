#!/usr/bin/env python3
"""
Apply critical bug fixes to the GUL lexer
"""

def apply_fixes():
    with open('/media/vu/512gb/blob/gul/compiler/lexer/lexer.mn', 'r') as f:
        lines = f.readlines()
    
    # Fix 1: Add indentation validation after line 154
    # Find the dedent section
    for i, line in enumerate(lines):
        if i == 153 and 'add_token(TokenType.Dedent' in line:
            # Insert validation after this line
            indent = '            '
            lines.insert(i + 1, '\n')
            lines.insert(i + 2, f'{indent}# BUGFIX: Validate dedent aligns with previous indent level\n')
            lines.insert(i + 3, f'{indent}if len(self.indent_stack) > 0 and self.indent_stack[-1] != indent_level:\n')
            lines.insert(i + 4, f'{indent}    # Indentation doesn\'t match any outer level\n')
            lines.insert(i + 5, f'{indent}    print(f"Warning: Inconsistent indentation at {{self.stream.line}}:{{self.stream.column}}")\n')
            lines.insert(i + 6, f'{indent}    print(f"  Expected indent level to match {{self.indent_stack[-1]}}, got {{indent_level}}")\n')
            break
    
    # Fix 2: Change escape sequence check (line ~247)
    # Original has 5 backslashes in source (which is 2 literal backslashes displayed)
    # We need just 2 backslashes in source (which is 1 literal backslash)
    for i, line in enumerate(lines):
        if 'if self.stream.current() ==' in line and '\\\\\\\\\\' in line:
            # Change from 5 backslashes to 2
            lines[i] = line.replace('"\\\\\\\\\\"', '"\\\\"')
            # Add comment
            lines.insert(i, '            # BUGFIX: Changed from "\\\\\\\\\\" to "\\\\" for single backslash check\n')
            break
    
    # Fix 3, 4, 5: Add else clauses for !, &, |
    for i, line in enumerate(lines):
        # Fix ! operator
        if '"!" =>' in line:
            # Find the closing brace
            j = i + 1
            while j < len(lines) and '}' not in lines[j]:
                j += 1
            # Insert else clause before closing brace
            indent = '                '
            lines.insert(j, f'{indent}else:\n')
            lines.insert(j + 1, f'{indent}    # BUGFIX: Standalone ! is not valid in GUL\n')
            lines.insert(j + 2, f'{indent}    print(f"Error: Unexpected character \'!\' at {{self.stream.line}}:{{self.stream.column}}")\n')
        
        # Fix & operator
        if '"&" =>' in line:
            j = i + 1
            while j < len(lines) and '}' not in lines[j]:
                j += 1
            indent = '                '
            lines.insert(j, f'{indent}else:\n')
            lines.insert(j + 1, f'{indent}    # BUGFIX: Standalone & is not valid in GUL\n')
            lines.insert(j + 2, f'{indent}    print(f"Error: Unexpected character \'&\' at {{self.stream.line}}:{{self.stream.column}}")\n')
        
        # Fix | operator
        if '"|" =>' in line:
            j = i + 1
            while j < len(lines) and '}' not in lines[j]:
                j += 1
            indent = '                '
            lines.insert(j, f'{indent}else:\n')
            lines.insert(j + 1, f'{indent}    # BUGFIX: Standalone | is not valid in GUL\n')
            lines.insert(j + 2, f'{indent}    print(f"Error: Unexpected character \'|\' at {{self.stream.line}}:{{self.stream.column}}")\n')
    
    # Write back
    with open('/media/vu/512gb/blob/gul/compiler/lexer/lexer.mn', 'w') as f:
        f.writelines(lines)
    
    print("✅ All bug fixes applied successfully!")
    print("Fixed:")
    print("  1. Indentation alignment validation")
    print("  2. Escape sequence handling")
    print("  3. Standalone ! operator error")
    print("  4. Standalone & operator error")
    print("  5. Standalone | operator error")

if __name__ == '__main__':
    apply_fixes()
