
import re
import sys

def clean_base(file_path):
    with open(file_path, 'r') as f:
        content = f.read()

    # Regex for Statement base (multiline allowed)
    # base: Statement {\s*node: (creation),\s*stmt_type: .*?},
    # Note: .*? with re.DOTALL matches newlines
    
    # Pattern 1: node first (relaxed node value matching anything up to comma)
    pattern1 = r'base:\s*Statement\s*\{\s*node:\s*([^,]+),\s*stmt_type:\s*[^}]+\s*\},'
    content = re.sub(pattern1, r'node: \1,', content, flags=re.DOTALL)
    
    # Pattern 2: stmt_type first? Unlikely in my code style.
    
    # Pattern 3: Expression base (relaxed)
    pattern3 = r'base:\s*Expression\s*\{\s*node:\s*([^,]+),\s*expr_type:\s*[^}]+\s*\},'
    content = re.sub(pattern3, r'node: \1,', content, flags=re.DOTALL)
    
    # Remove ASTNode specific patterns as they are covered by generic patterns above, or keep for safety?
    # If generic works, it covers ASTNode{...} too (balanced braces issue? regex non-greedy usually stops at comma?)
    # [^,]+ matches ASTNode{...} ONLY if it contains no commas.
    # But ASTNode{line: 1, column: 1} CONTAINS COMMA.
    # So [^,]+ WILL FAIL.
    
    # Better Regex for Node Value: Match balanced braces? Or just until , stmt_type?
    # node: (.*?), stmt_type:
    # Use non-greedy dot match?
    pattern1 = r'base:\s*Statement\s*\{\s*node:\s*(.*?),\s*stmt_type:\s*[^}]+\s*\},'
    content = re.sub(pattern1, r'node: \1,', content, flags=re.DOTALL)
    
    pattern3 = r'base:\s*Expression\s*\{\s*node:\s*(.*?),\s*expr_type:\s*[^}]+\s*\},'
    content = re.sub(pattern3, r'node: \1,', content, flags=re.DOTALL)

    with open(file_path, 'w') as f:
        f.write(content)

clean_base('compiler/parser/parser.mn')
