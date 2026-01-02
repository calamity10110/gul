import os
import re
import sys
from pathlib import Path

class GulToRustTranspiler:
    def __init__(self):
        self.indent_stack = []
        self.output = []
        
    def transpile_file(self, content):
        self.indent_stack = []
        self.enum_stack = []
        self.wrapper_stack = [] # Stack of (indent, 'Statement'/'Expression') for closing braces
        
        self.STATEMENT_VARIANTS = {
            'LetStmt': 'LetDecl', 'VarStmt': 'VarDecl', 'FunctionDecl': 'FunctionDecl',
            'StructDecl': 'StructDecl', 'EnumDecl': 'EnumDecl',
            'IfStmt': 'IfStmt', 'WhileStmt': 'WhileStmt', 'ForStmt': 'ForStmt',
            'LoopStmt': 'LoopStmt', 'MatchStmt': 'MatchStmt',
            'BreakStmt': 'BreakStmt', 'ContinueStmt': 'ContinueStmt', 'ReturnStmt': 'ReturnStmt',
            'TryStmt': 'TryStmt',
            'ExpressionStmt': 'ExpressionStmt', 'AssignmentStmt': 'AssignmentStmt',
            'ImportStmt': 'ImportStmt', 'ForeignCodeBlock': 'ForeignCodeBlock', 'PassStmt': 'PassStmt'
        }
        self.EXPRESSION_VARIANTS = {
            'LiteralExpr': 'Literal', 'IdentifierExpr': 'Identifier',
            'BinaryOpExpr': 'BinaryOp', 'UnaryOpExpr': 'UnaryOp',
            'CallExpr': 'Call', 'IndexExpr': 'Index', 'AttributeExpr': 'Attribute',
            'ListExpr': 'List', 'TupleExpr': 'Tuple', 'SetExpr': 'Set', 'DictExpr': 'Dict',
            'LambdaExpr': 'Lambda', 'MatchExpr': 'Match',
            'TypeConstructorExpr': 'TypeConstructor', 'GroupedExpr': 'Grouped'
        } # (needs_semi, needs_comma)
        self.semi_stack = [] # (needs_semi, needs_comma)
        self.output = []
        self.paren_depth = 0
        lines = content.splitlines()
        for i, line in enumerate(lines):
            stripped = line.strip()
            if not stripped:
                self.output.append("")
                continue
            if stripped.startswith('#'):
                self.output.append(line.replace('#', '//', 1))
                continue
            
            indent = len(line) - len(line.lstrip())
            
            # Remove comments for block detection
            clean_line = stripped
            if '#' in clean_line:
                 clean_line = clean_line[:clean_line.find('#')].rstrip()
            
            # Update paren depth (simple string removal)
            temp_line = re.sub(r'".*?"', '', clean_line) # remove double quoted strings
            temp_line = re.sub(r"'.*?'", '', temp_line) # remove single quoted strings
            self.paren_depth += temp_line.count('(') - temp_line.count(')')

            
            is_block_start = clean_line.endswith(':') or clean_line.endswith('{')

            
            # Close blocks if indent decreased or sibling block starts
            pops_count = 0
            while self.indent_stack and self.indent_stack[-1][0] >= indent:
                popped_indent, is_explicit, closure_suffix, _ = self.indent_stack.pop() # Updated to 4-tuple
                if is_explicit:
                    pops_count += 1
                    
                if self.semi_stack:
                    needs_semi, needs_comma = self.semi_stack.pop()
                    if needs_comma:
                        suffix = "},"
                    elif needs_semi:
                        suffix = "};"
                    else:
                        suffix = "}"
                else:
                    suffix = "}"
                
                # Append closure suffix (e.g. ')') BEFORE semicolon/comma
                if closure_suffix:
                     suffix = suffix.replace('}', '}' + closure_suffix)
                
                self.output.append("    " * len(self.indent_stack) + suffix)
            
            # Ensure current indent is tracked if starting a block
            if is_block_start:
                explicit_brace = clean_line.endswith('{')
                closure_suffix = None
                block_type = 'other'
                
                if clean_line.startswith('struct '): block_type = 'struct'
                elif clean_line.startswith('enum '): block_type = 'enum'
                elif clean_line.startswith('fn '): block_type = 'fn'
                elif clean_line.startswith('impl '): block_type = 'impl'
                
                # Check for return wrapping requirements (Enum Variants)
                match_ret = re.search(r'return\s+([A-Za-z]+)\s*\{', clean_line)
                if match_ret:
                     struct_name = match_ret.group(1)
                     if struct_name in self.STATEMENT_VARIANTS:
                          variant = self.STATEMENT_VARIANTS[struct_name]
                          closure_suffix = ")"
                     elif struct_name in self.EXPRESSION_VARIANTS:
                          # Expression variants
                          closure_suffix = ")"
                
                self.indent_stack.append((indent, explicit_brace, closure_suffix, block_type))
                
                # Determine closure type
                needs_comma = False
                needs_semi = False
                
                # If inside parentheses (args), default to comma, no semi
                if self.paren_depth > 0:
                     needs_comma = True
                # Control flow - no suffix
                elif re.match(r'^(if|elif|else|while|for|match|fn|struct|enum|impl|try|catch|finally|mn)\b', clean_line):
                    pass
                # Match arms and struct fields need comma
                elif '=>' in clean_line:
                    needs_comma = True
                elif re.match(r'^[a-z_][a-z_0-9]*\s*:', clean_line) and not clean_line.startswith(('if', 'for', 'while', 'return')):
                     needs_comma = True
                # Expressions (let, return, assignment) need semicolon
                else:
                    needs_semi = True
                
                self.semi_stack.append((needs_semi, needs_comma))
            

            self.process_line(line, indent, pops_count)
            
        # Close all remaining blocks
        while len(self.indent_stack) > 0:
            self.indent_stack.pop() # Tuple
            if self.semi_stack:
                needs_semi, needs_comma = self.semi_stack.pop()
                if needs_comma:
                    suffix = "},"
                elif needs_semi:
                    suffix = "};"
                else:
                    suffix = "}"
            else:
                suffix = "}"
            self.output.append("    " * len(self.indent_stack) + suffix)
            
        return "\n".join(self.output)

    def preprocess_content(self, content):
        """Preprocess GUL source before transpiling"""
        # Protect string literals with escape sequences
        # GUL uses same escapes as Python/Rust, so they should pass through
        # But we need to ensure they don't trigger false regex matches
        return content


    def process_line(self, line, indent, pops_count=0):
        content = line.strip()
        indent_str = "    " * (indent // 4)
        
        # Consume explicit braces that match implicit pops
        if pops_count > 0:
            # We need to remove up to pops_count leading '}' characters
            remaining = pops_count
            new_content = ""
            # Separate potential comment
            actual_content = content
            comment_suffix = ""
            if '#' in content:
                idx = content.find('#')
                actual_content = content[:idx] # Don't strip yet, need exact chars
                comment_suffix = content[idx:]
            
            # Scan and skip '}'
            for char in actual_content:
                if remaining > 0 and char == '}':
                    remaining -= 1
                    continue
                new_content += char
            
            if new_content.strip() == "" and comment_suffix == "":
                 return # Line contained only consumed braces
            
            # Reconstruct content
            content = new_content.strip() + comment_suffix
            if content.startswith('#'): # Only comment remains
                 content = comment_suffix # Let comment logic handle it
            
            # If line empty now, verify it wasn't empty before?
            # If it becomes empty, it means it was just '}'s.
            if not content.strip():
                 return



        # Strip inline comment FIRST (before block detection)
        comment = ""
        if '#' in content:
            idx = content.find('#')
            comment = " //" + content[idx+1:]
            content = content[:idx].rstrip()
            if not content:  # Line was just a comment
                self.output.append(f"{indent_str}{comment.strip()}")
                return

        # Handle keywords that start a block
        is_block_start = content.endswith(':')
        if is_block_start:
            content = content[:-1].strip()

        # Control flow keywords (strict matching)
        if re.match(r'^(if|elif|else|while|for|match|try|catch|finally)\b', content):
            self.handle_control_flow(indent_str, content, is_block_start)
            return

        # Inline enum: enum Name: V1, V2, V3 -> enum Name { V1, V2, V3 }
        enum_match = re.match(r'^enum\s+(\w+):\s*(.+)$', content)
        if enum_match:
            name, variants = enum_match.groups()
            self.output.append(f"{indent_str}#[derive(Debug, Clone, PartialEq)]")
            self.output.append(f"{indent_str}pub enum {name} {{ {variants} }}")
            return

        # Enum/Struct/Impl block start (not inline) - make pub for export with derives
        if content.startswith('enum ') and is_block_start:
            self.output.append(f"{indent_str}#[derive(Debug, Clone, PartialEq)]")
            self.output.append(f"{indent_str}pub {content} {{")
            return
        if content.startswith('struct ') and is_block_start:
            self.output.append(f"{indent_str}#[derive(Debug, Clone, PartialEq)]")
            self.output.append(f"{indent_str}pub {content} {{")
            return
        if content.startswith('impl ') and is_block_start:
            self.output.append(f"{indent_str}{content} {{")
            return


        # Mod/import mapping - use wildcard imports to get all types
        if content.startswith(('import ', '@imp ')):
            parts = content.split(' ')
            if len(parts) >= 2:
                mod_path = parts[1].replace('.', '::').replace(';', '')
                # Standard library doesn't use crate:: prefix
                if mod_path == 'std::collections':
                    self.output.append(f"{indent_str}use std::collections::{{HashMap, HashSet}};")
                elif mod_path.startswith('std::'):
                    self.output.append(f"{indent_str}use {mod_path};")
                else:
                    # Strip 'compiler::' prefix since compiler/ is the crate root
                    if mod_path.startswith('compiler::'):
                        mod_path = mod_path[len('compiler::'):]
                    # Use wildcard import to get all types from local modules
                    self.output.append(f"{indent_str}use crate::{mod_path}::*;")
                return

        # Variable declarations

        if content.startswith(('let ', 'var ')):
            content = content.replace('var ', 'let mut ')
            content = content.replace('let ', 'let ')
            
        # Functions
        if content.startswith('fn '):
            content = 'pub ' + content
            content = content.replace(' -> ', '  ->  ')
            # Fix return type @str -> String etc happens in convert_types
            
        # Collections and Types
        content = self.convert_collections(content)
        content = self.convert_types(content)
        content = self.convert_syntax(content)
        
        # Suffix
        suffix = ""
        if not is_block_start:
            # Struct literal on assignment - ends with } but needs ;
            if content.endswith('}') and '=' in content and (content.startswith('let ') or content.startswith('let mut ')):
                suffix = ";"
            elif content.startswith('return ') and not content.strip().endswith('{'):
                suffix = ";"
            elif content.startswith('return ') and content.strip().endswith('{'):
                # Special case: return Struct { ...
                # Needs semicolon at END of struct block, but not here.
                # But transpiler tracks block nesting.
                # Logic block closure will append "}" or "};" based on semi_stack.
                # We need to tell indent_stack this block needs semi?
                # Indent logic: semi_stack.push((needs_semi, needs_comma)).
                # We must detect this is a RETURN block.
                # Line 91: needs_semi = True if startswith return?
                pass
            elif not any(content.endswith(c) for c in (';', '{', '}', ',', '!', '/')):
                 if content.startswith('//'):
                      suffix = ""
                 # Match arms: pattern => expr (BUT ignore dict!{...})
                 # Also ignore if line ends with => (continuation/block)
                 elif '=>' in content and 'dict!{' not in content and not content.strip().endswith('=>'):
                      suffix = ","
                 elif content.strip().endswith('=>'):
                       suffix = ""
                 # Struct field: name: Type - not a statement/declaration keyword
                 # Use word boundary check for keywords - add pub prefix for struct fields
                 elif re.match(r'^[a-z_][a-z_0-9]*\s*:', content) and not re.match(r'^(if|for|while|match|return|let|var|fn)\b', content):
                      suffix = ","
                      # Add pub prefix for struct field visibility ONLY if in struct block
                      current_block = self.indent_stack[-1][3] if self.indent_stack else 'other'
                      if current_block == 'struct' and not content.startswith('pub '):
                          content = 'pub ' + content
                 # Enum variants single (approx: inside enum block, no colon, single word)
                 elif re.match(r'^[A-Z][A-Za-z0-9_]*$', content):
                      suffix = ","
                 # Enum variants multiple on one line
                 elif re.match(r'^([A-Z][A-Za-z0-9_]*,\s*)+[A-Z][A-Za-z0-9_]*$', content):
                      suffix = ","
                 # Enum variants with data: Variant(Type)
                 elif re.match(r'^[A-Z][A-Za-z0-9_]*\s*\(.*?\)$', content):
                      suffix = ","
                 # Fallback: check if we are in an enum block (default to comma)
                 elif self.indent_stack and self.indent_stack[-1][3] == 'enum' and not content.startswith(('fn ', 'pub fn ')):
                      suffix = ","
                 else:
                      suffix = ";"


        
        # Avoid double brace if content already ends with {
        block_brace = " {"
        if content.rstrip().endswith('{'):
             block_brace = ""
        
        self.output.append(f"{indent_str}{content}{block_brace if is_block_start else suffix}{comment}")


    def handle_control_flow(self, indent, content, is_block_start):
        content = re.sub(r'\belif\b', 'else if', content)
        
        # Mapping try/catch/finally for bootstrap
        if content.startswith('try'):
            content = 'if true'
        elif content.startswith('catch'):
            content = re.sub(r'^catch\s+\w+', 'else if false', content)
            content = re.sub(r'^catch\b', 'else if false', content)
        elif content.startswith('finally'):
            content = 'if true'
            
        content = self.convert_collections(content)
        content = self.convert_types(content)
        content = self.convert_syntax(content)
        
        self.output.append(f"{indent}{content}{' {' if is_block_start else ''}")

    def convert_collections(self, line):
        # Skip if line contains string with @ inside (e.g., "gul_type == \"@list\"")
        # Simple check: if @list or @dict appears inside quotes, skip conversion for this line
        if re.search(r'"[^"]*@(list|dict)[^"]*"', line):
            return line
            
        # Map collection values (with explicit contents) - but NOT type annotations
        # Type annotations are preceded by : (e.g., "param: @list[String]")
        # Values are preceded by = or at start of expression (e.g., "= @list[]" or "@list[]")
        
        line = re.sub(r'@dict\s*\{', 'dict!{', line)
        # Convert dict!{"key": val} to HashMap::from([("key", val)])
        if 'dict!{' in line:
            def dict_replacer(match):
                 content = match.group(1)
                 # k: v -> (k, v)
                 # Handle "key": val
                 pairs = []
                 # Split by comma (naive, assumes no commas in values/keys - risky but okay for bootstrap config)
                 raw_pairs = content.split(',')
                 for pair in raw_pairs:
                      if ':' in pair:
                           k, v = pair.split(':', 1)
                           k_clean = k.strip()
                           # If k is string literal, add .to_string()
                           if re.match(r'^"[^"]*"$', k_clean):
                               k_clean = f'{k_clean}.to_string()'
                           pairs.append(f"({k_clean}, {v.strip()})")
                 if not pairs: return "HashMap::new()"
                 return f"HashMap::from([{', '.join(pairs)}])"
            
            line = re.sub(r'dict!\{([^}]*)\}', dict_replacer, line)
        
        # Only convert @list[] to vec![] when it's a VALUE (not a type annotation)
        # Type annotations: "name: @list[Type]" - keep as @list for convert_types
        # Values: "= @list[]" or "@list[1, 2]" - convert to vec![]
        line = re.sub(r'=\s*@list\s*\[', '= vec![', line)  # Assignment value
        line = re.sub(r'^\s*@list\s*\[', 'vec![', line)  # Start of line
        line = re.sub(r',\s*@list\s*\[', ', vec![', line)  # After comma (function arg)
        line = re.sub(r'\(\s*@list\s*\[', '(vec![', line)  # After open paren
        
        line = re.sub(r'@list\s*\[\]', 'vec![]', line)
        
        # For type annotations (after :), keep as types - handled by convert_types
        # For bare @list (no brackets), convert to Vec::new() only if it's a value context
        if re.search(r':\s*@(list|dict)\b', line):
            line = re.sub(r':\s*@list\b(?!\s*[\[(])', ': Vec<String>', line)
            line = re.sub(r':\s*@dict\b(?!\s*[{(])', ': HashMap<String, String>', line)
        else:
            line = re.sub(r'@list\b(?!(\s*[\[(]))', 'Vec::new()', line)
            line = re.sub(r'@dict\b(?!(\s*[{(]))', 'HashMap::new()', line)
        return line


    def convert_types(self, line):
        # Type aliases - use @ to distinguish from values where possible
        # Use usize for @int to support indexing (Rust requires usize for indices)
        line = re.sub(r'@int\b', 'usize', line)
        line = re.sub(r'i64\((\d+)\)', r'\1usize', line)  # i64(0) -> 0usize
        line = re.sub(r'usize\((\d+)\)', r'\1usize', line)  # usize(0) -> 0usize
        line = re.sub(r'@flt\b', 'f64', line)
        line = re.sub(r'@float\b', 'f64', line)
        line = re.sub(r'@str\b', 'String', line)
        line = re.sub(r'@bool\b', 'bool', line)
        
        # Types with generics
        line = re.sub(r'@list\b\s*\[', 'Vec[', line)
        line = re.sub(r'@dict\b\s*\[', 'HashMap[', line)
        line = re.sub(r'@box\b\s*\[', 'Box[', line) # Support for Box
        line = re.sub(r'@tuple\b\s*[\[\(](.*?)[\]\)]', r'(\1)', line) # Support for Tuple types
        
        line = re.sub(r'@list\b', 'Vec', line)
        line = re.sub(r'TokenType\.', 'TokenType::', line)
        line = re.sub(r'StmtType\.', 'StmtType::', line)
        line = re.sub(r'Precedence\.', 'Precedence::', line)
        line = re.sub(r'OpType\.', 'OpType::', line)
        line = re.sub(r'ExprType\.', 'ExprType::', line)
        line = re.sub(r'IRNodeType\.', 'IRNodeType::', line)
        line = re.sub(r'IRType\.', 'IRType::', line)
        
        line = re.sub(r'Expression\.', 'Expression::', line)
        line = re.sub(r'Statement\.', 'Statement::', line)
        line = re.sub(r'Parser\.', 'Parser::', line)
        
        # Collections
        line = re.sub(r'@dict\b', 'HashMap', line)
        line = re.sub(r'@set\b', 'HashSet', line)
        
        # Generic brackets
        line = re.sub(r'Vec\[(.*?)\]', r'Vec<\1>', line)
        line = re.sub(r'HashMap\[(.*?)\]', r'HashMap<\1>', line)
        line = re.sub(r'Box\[(.*?)\]', r'Box<\1>', line)
        
        return line

    def convert_syntax(self, line):
        # Docstrings - use // (not /// which is doc comment and causes issues)
        if '"""' in line:
            line = line.replace('"""', '// ')
            
        # Entry point mn: -> fn main()
        if line.strip() == 'mn':
            line = 'fn main()'
            
        # pass -> {} (empty block/no-op)
        if line.strip() == 'pass':
            line = '// pass'

        # Enum Variant wrapping for return statements
        match_ret = re.search(r'return\s+([A-Za-z]+)\s*\{', line)
        if match_ret:
             struct_name = match_ret.group(1)
             if struct_name in self.STATEMENT_VARIANTS:
                  variant = self.STATEMENT_VARIANTS[struct_name]
                  line = line.replace(f'return {struct_name}', f'return Statement::{variant}({struct_name}')
                  if line.strip().endswith(('}', '};')):
                       # Only replace the LAST brace
                       line = re.sub(r'\}(;?)\s*$', r'})\1', line)
             elif struct_name in self.EXPRESSION_VARIANTS:
                  variant = self.EXPRESSION_VARIANTS[struct_name]
                  line = line.replace(f'return {struct_name}', f'return Expression::{variant}({struct_name}')
                  if line.strip().endswith(('}', '};')):
                       # Only replace the LAST brace
                       line = re.sub(r'\}(;?)\s*$', r'})\1', line)
            
        # pass keyword
        if line.strip() == 'pass':
            line = '{}'
        elif re.search(r'=>\s*pass\b', line):
            line = re.sub(r'=>\s*pass\b', '=> {}', line)
        elif re.search(r':\s*pass\b', line):
            line = re.sub(r':\s*pass\b', ': {}', line)

        # Single line if/elif/else if with colon
        # Strip trailing colon from control flow (Normal blocks)
        # Check if it is NOT a single-line block (ends with colon)
        is_single_line = re.match(r'^\s*(if|elif|else\s+if)\s+.*(?<!:):(?!:).*[^{]$', line)
        
        if not is_single_line:
             if re.match(r'^\s*(if|else|while|for|fn|struct|enum|impl|try|catch|finally|mn)\b', line) and line.rstrip().endswith(':'):
                 line = line.rstrip()[:-1]
             elif re.match(r'^\s*(elif)\b', line) and line.rstrip().endswith(':'):
                 line = line.rstrip()[:-1]

        # Single line if/elif/else if with colon (e.g. if x: return y)
        if is_single_line:
            line = re.sub(r'^\s*(if|elif|else\s+if)\s+(.*?)(?<!:):(?!:)\s*(.*)', r'\1 \2 { \3 }', line)
            line = line.replace('elif ', 'else if ')

        # Single line else:
        if re.match(r'^\s*else\s*:\s*.+$', line):
            line = re.sub(r'^\s*else\s*:\s*(.+)$', r'else { \1 }', line)

        # Single line for:
        if re.match(r'^\s*for\s+.*:\s*.+$', line):
            line = re.sub(r'^\s*for\s+(.*?)\s+in\s+(.*?):\s*(.+)$', r'for \1 in \2 { \3 }', line)

        # Comments
        if '#' in line and '"' not in line:
            line = line.replace('#', '//')

        # 'not' keyword -> '!'
        line = re.sub(r'\bnot\b', '!', line)
        
        # 'or' -> '||', 'and' -> '&&'
        line = re.sub(r'\bor\b', '||', line)
        line = re.sub(r'\band\b', '&&', line)


        # @cast(Type, expr) -> (expr as Type) - simplified for bootstrap
        line = re.sub(r'@cast\((\w+),\s*(.*?)\)', r'(\2 as \1)', line)

        # f-strings replacement (robust)
        def fstring_replacer(match):
            content = match.group(1)
            # Find vars inside content
            vars = re.findall(r'\{(.*?)\}', content)
            # Replace {var} with {} in content
            template = re.sub(r'\{(.*?)\}', '{}', content)
            args = ", ".join(vars)
            if args:
                 return f'format!("{template}", {args})'
            return f'"{content}".to_string()'

        if 'f"' in line:
             line = re.sub(r'f"([^"]*)"', fstring_replacer, line)

        # Methods with self - convert params first
        line = line.replace('(ref self,', '(&mut self,').replace('(ref self)', '(&mut self)')
        line = line.replace('(self,', '(&self,').replace('(self)', '(&self)')
        
        # fn Type name(&self) -> fn name(&self) -> Type
        # Handle both &self and &mut self
        if '(&self)' in line or '(&mut self)' in line or '(&self,' in line or '(&mut self,' in line:
            # Pattern: fn @str name(&self) or fn String name(&self) -> fn name(&self) -> String
            line = re.sub(r'fn\s+([A-Za-z0-9_@]+)\s+([A-Za-z0-9_]+)\((&mut self|&self)', r'fn \2(\3) -> \1', line)


        # Normalizations
        line = re.sub(r'@(int|flt|float|bool|str)[\[\{](.*?)[\]\}]', r'@\1(\2)', line)
        line = line.replace('@bool(true)', 'true').replace('@bool(false)', 'false')
        line = line.replace('bool(true)', 'true').replace('bool(false)', 'false')
        
        # String repetition: "str" * n -> "str".repeat(n)
        line = re.sub(r'\"([^\"]*)\"\.to_string\(\)\s*\*\s*([a-zA-Z0-9_.]+(?:\(\))?)', r'"\1".repeat(\2)', line)
        line = re.sub(r'\"([^\"]*)\"\s*\*\s*([a-zA-Z0-9_.]+(?:\(\))?)', r'"\1".repeat(\2)', line)
        
        # String join: sep.join(list) -> list.join(sep)
        # Handle "\n".to_string().join(...)
        match_join = re.search(r'("[^"]*")\.to_string\(\)\.join\((.*?)\)', line)
        if match_join:
             sep = match_join.group(1)
             lst = match_join.group(2)
             line = line.replace(match_join.group(0), f"{lst}.join({sep})")
        
        # String("...") -> "...".to_string()
        line = re.sub(r'String\("([^"]*)"\)', r'"\1".to_string()', line)
        # @str("...") -> "...".to_string()
        line = re.sub(r'@str\("([^"]*)"\)', r'"\1".to_string()', line)
        
        # @box(...) -> Box::new(...)
        line = re.sub(r'@box\((.*?)\)', r'Box::new(\1)', line)
        
        # len(...) -> (...).len() - only if NOT already a .len() call
        line = re.sub(r'(?<!\.)\blen\((.*?)\)', r'(\1).len()', line)

        
        # Argument access sys.argv[1] -> sys::argv()[1]
        line = line.replace('sys.argv', 'sys::argv()')
        
        # print(...) -> println!(...)
        line = re.sub(r'(?<!\.)\bprint\((.*?)\)', r'println!("{}", \1)', line)
        # str(...) -> format!("{}", ...)
        line = re.sub(r'(?<!\.)\bstr\((.*?)\)', r'format!("{}", \1)', line)
        
        # String concatenation fixes (heuristic)
        # s + format!(...) -> s + &format!(...)
        line = re.sub(r'\+\s*format!', '+ &format!', line)
        # s + "...".to_string() -> s + &"...".to_string()
        line = re.sub(r'\+\s*"([^"]*)"\.to_string\(\)', r'+ &"\1".to_string()', line)
        
        # type(...) -> "unknown" (type introspection not available in Rust bootstrap)
        line = re.sub(r'(?<!\.)\btype\((.*?)\)', r'"unknown"', line)
        
        # .add() -> .push() for Vec
        line = line.replace('.add(', '.push(')
        
        # Token.type -> Token.token_type (Rust keyword avoidance)
        line = re.sub(r'\.type\b', '.token_type', line)
        
        # Python-style method names to Rust
        line = line.replace('.startswith(', '.starts_with(')
        line = line.replace('.endswith(', '.ends_with(')
        # substring is not a Rust method - will need custom handling
        # For now, leave substring calls as TODOs
        
        # source[idx] -> source.chars().nth(idx).unwrap().to_string()
        # Specific fix for lexer string indexing
        line = re.sub(r'\bsource\[(.*?)\]', r'source.chars().nth(\1).unwrap().to_string()', line)

        # Skip string conversion for match patterns (ending with =>)
        # and case statements, and likely formatting strings if complex
        # But generally convert "..." to "...".to_string() because GUL @str is String
        if not re.search(r'"[^"]*"\s*=>', line) and not re.search(r'case\s*"', line) and 'include_str' not in line:
             # Don't convert if already .to_string()
             # Use a negative lookahead/behind or just replace and fix double
             # Replace "foo" with "foo".to_string()
             # Avoid replacing inside includes or attributes if possible
             
             # Regex: "([^"]*)" not followed by .to_string
             # And not inside another string? (transpiler processes line by line)
             # Note: simple regex might break "str" + "str".
             # We already have string concat fix.
             
             # Let's try to target return values and assignments specifically first
             # return "..."
             line = re.sub(r'return\s+"([^"]+)"', r'return "\1".to_string()', line)
             # = "..."
             line = re.sub(r'=\s+"([^"]+)"', r'= "\1".to_string()', line)
             # ("...") func arg (basic) - Ignore macros like format!("...")
             # Use negative lookbehind (?<!!) to avoid replacing strings after ! (macros)
             # Matches ( "..." or ( "..." )
             line = re.sub(r'(?<!!)\(\s*"([^"]+)"\s*\)', r'("\1".to_string())', line)
             line = re.sub(r'(?<!!)\(\s*"([^"]+)"', r'("\1".to_string()', line)
             # Args after comma: , "..." -> , "...".to_string() (valid even in format!)
             line = re.sub(r',\s*"([^"]+)"', r', "\1".to_string()', line)
        
        # String concatenation: convert "str" + expr to format!("{}{}", "str", expr)
        # s.join(list) -> list.join(s)
        # Handle string literal join: ",".join(list)
        line = re.sub(r'"([^"]*)"\.join\((.*?)\)', r'\2.join("\1")', line)
        # Handle variable join: sep.join(list)
        # Only if strict pattern
        # line = re.sub(r'(\w+)\.join\((.*?)\)', r'\2.join(&\1)', line) # Dangerous if join is used otherwise
        
        # This is a simplified approach - replace + with .to_string() + &
        # Actually just wrap strings in format! for safety
        if ' + ' in line and '"' in line:
            # Simple approach: use .to_string() on string literals in concat
            line = re.sub(r'"([^"]*)" \+ ', r'"\1".to_string() + &', line)
        
        # Argument access sys.argv[1] -> sys::argv()[1]
        line = line.replace('sys.argv', 'sys::argv()')
        
        return line

def get_prelude():
    return """// Auto-generated from GUL source
#![allow(unused_variables, dead_code, unused_mut, unused_imports, non_snake_case)]

use std::collections::{HashMap, HashSet};
use std::fmt::Display;

pub trait GulString {
    fn add_gul<T: Display>(&self, other: T) -> String;
}

impl GulString for String {
    fn add_gul<T: Display>(&self, other: T) -> String {
        format!("{}{}", self, other)
    }
}

impl GulString for &str {
    fn add_gul<T: Display>(&self, other: T) -> String {
        format!("{}{}", self, other)
    }
}

#[macro_export]
macro_rules! dict {
    ($($key:expr => $val:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $( map.insert($key.to_string(), $val); )*
        map
    }};
    ($($key:ident : $val:expr),* $(,)?) => {{
        let mut map = HashMap::new();
        $( map.insert(stringify!($key).to_string(), $val); )*
        map
    }};
}

// Minimal sys module shim
pub mod sys {
    pub fn argv() -> Vec<String> {
        std::env::args().collect()
    }
}

// Minimal fs module shim
pub fn read_file(path: String) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

pub fn write_file(path: String, content: String) {
    let _ = std::fs::write(path, content);
}
"""

def transpile_directory(src_dir, dest_dir):
    src_path = Path(src_dir)
    dest_path = Path(dest_dir)
    transpiler = GulToRustTranspiler()
    
    # Files to exclude from bootstrap (complex patterns that need manual handling)
    exclude_files = {
        'flat_compiler.mn',  # Has complex escape sequences
    }
    
    gul_files = [f for f in src_path.glob('**/*.mn') if f.name not in exclude_files]




    print(f"Found {len(gul_files)} GUL files to transpile")
    
    # Track modules for main.rs
    modules = {}

    for gul_file in gul_files:
        rel_path = gul_file.relative_to(src_path)
        rust_file = dest_path / rel_path.with_suffix('.rs')
        rust_file.parent.mkdir(parents=True, exist_ok=True)

        
        # Track for module hierarchy
        parts = rel_path.with_suffix('').parts
        curr = modules
        for part in parts[:-1]:
            if part not in curr: curr[part] = {}
            curr = curr[part]
        curr[parts[-1]] = "file"

        print(f"Transpiling {rel_path} -> {rust_file.relative_to(dest_path)}")
        with open(gul_file, 'r') as f:
            content = f.read()
            
        rust_code = transpiler.transpile_file(content)
        
        with open(rust_file, 'w') as f:
            if rel_path.name == 'main.mn':
                f.write(get_prelude() + "\n")
            f.write(rust_code)
        print(f"✅ Generated {rust_file.relative_to(dest_path)}")

    # Generate mod declarations in main.rs or mod.rs
    generate_mods(dest_path, modules)

def generate_mods(dest_path, modules, prefix=""):
    main_rs_path = dest_path / 'main.rs'
    if main_rs_path.exists():
        with open(main_rs_path, 'r') as f:
            lines = f.readlines()
        
        # Insert mod declarations after shim modules
        insert_idx = -1
        for i, line in enumerate(lines):
            if 'pub fn write_file' in line:
                insert_idx = i + 3
                break
        
        if insert_idx != -1:
            mod_decls = []
            for mod_name in sorted(modules.keys()):
                if mod_name != 'main':
                    mod_decls.append(f"mod {mod_name};\n")
            lines.insert(insert_idx, "".join(mod_decls))
            with open(main_rs_path, 'w') as f:
                f.writelines(lines)

    # Recursive mod.rs generation for subdirectories
    for mod_name, content in modules.items():
        if isinstance(content, dict):
            dir_path = dest_path / mod_name
            with open(dir_path / 'mod.rs', 'w') as f:
                for sub_mod in sorted(content.keys()):
                    f.write(f"pub mod {sub_mod};\n")
            generate_mods(dir_path, content, prefix + mod_name + "::")

if __name__ == "__main__":
    src_dir = "compiler"
    dest_dir = "compiler_rust"
    if len(sys.argv) >= 2:
        src_dir = sys.argv[1]
    if len(sys.argv) >= 3:
        dest_dir = sys.argv[2]
        
    transpile_directory(src_dir, dest_dir)
