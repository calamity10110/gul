#!/usr/bin/env python3
"""
GUL Interpreter - Full Implementation
Interprets GUL code to run the compiler.
"""

import sys
import re
from pathlib import Path
from typing import Any, Dict, List, Optional

class GulFunction:
    """Represents a GUL function"""
    def __init__(self, name, params, body, closure):
        self.name = name
        self.params = params  # List of (name, type) tuples
        self.body = body  # List of lines
        self.closure = closure  # Environment at definition time

class GulStructDefinition:
    """Represents a GUL struct definition"""
    def __init__(self, name, fields, methods):
        self.name = name
        self.fields = fields # List of field names
        self.methods = methods # Dict of name -> GulFunction

class GulStruct:
    """Represents a GUL struct instance"""
    def __init__(self, struct_name, fields):
        self.struct_name = struct_name
        self.fields = fields  # Dict of field_name: value

class GulInterpreter:
    def __init__(self, argv=None):
        self.vars = {}
        self.functions = {}
        self.structs = {}  # struct definitions
        self.enums = {}
        self.indent_stack = [0]
        
        # Flags
        self.debug = False # Start false, enable based on argv maybe? 
        
        # Import system
        self.loaded_modules = {}  # module_path -> module namespace
        self.current_file = None  # Track current file being executed
        
        # Built-ins
        self.vars['print'] = print
        self.vars['len'] = len
        self.vars['range'] = range
        self.vars['str'] = str
        self.vars['int'] = int
        self.vars['float'] = float
        self.vars['bool'] = bool
        self.vars['list'] = list
        self.vars['dict'] = dict
        self.vars['set'] = set
        self.vars['tuple'] = tuple
        self.vars['True'] = True
        self.vars['False'] = False
        self.vars['None'] = None
        
        # File I/O built-ins
        self.vars['read_file'] = self.builtin_read_file
        self.vars['write_file'] = self.builtin_write_file
        self.vars['file_exists'] = self.builtin_file_exists
        
        # System module with argv as a proper list
        class SysModule:
            def __init__(self, argv):
                self.argv = argv or []
        
        self.vars['sys'] = SysModule(argv if argv else sys.argv[1:])
        
        # Check for --debug flag
        if '--debug' in self.vars['sys'].argv:
            self.debug = True
            print("🐞 Debug mode enabled")
    
    def run_file(self, filename):
        if self.debug: print(f"🚀 Running: {filename}\n")
        else: print(f"🚀 Running: {filename}\n")
        
        self.current_file = filename
        
        with open(filename, 'r') as f:
            lines = f.readlines()
        
        self.execute_block(lines, 0, len(lines))
        
        print("\n✅ Complete!")
    
    def load_module(self, module_path):
        """Load a GUL module by path (e.g., ['compiler', 'lexer', 'lexer'])"""
        module_key = '.'.join(module_path)
        
        # Check if already loaded
        if module_key in self.loaded_modules:
            return self.loaded_modules[module_key]
        
        # Convert module path to file path
        # compiler.lexer.lexer -> compiler/lexer/lexer.mn
        file_path = '/'.join(module_path) + '.mn'
        
        # Try relative to current file's directory
        if self.current_file:
            base_dir = Path(self.current_file).parent
            full_path = base_dir / file_path
        else:
            full_path = Path(file_path)
        
        if not full_path.exists():
            # Try from current working directory
            full_path = Path(file_path)
        
        if not full_path.exists():
            print(f"Warning: Could not find module {module_key} at {file_path}")
            return {}
            
        if self.debug: print(f"  [Import] Loading module: {module_key} from {full_path}")
        
        # Save current state
        saved_file = self.current_file
        saved_vars = dict(self.vars)
        
        # Load and execute module
        self.current_file = str(full_path)
        module_namespace = {}
        
        try:
            with open(full_path, 'r') as f:
                lines = f.readlines()
            
            # Execute module
            self.execute_block(lines, 0, len(lines))
            
            # Extract exported items (functions, structs, etc.)
            # Everything defined at module level is exported
            for name, value in self.vars.items():
                if name not in saved_vars:
                    module_namespace[name] = value
            
            # Cache module
            self.loaded_modules[module_key] = module_namespace
            
        except ReturnValue:
            # Return statements in modules are ignored (only valid in functions)
            pass
        finally:
            # Restore state (but keep imported items)
            self.current_file = saved_file
            # Merge module namespace into current vars
            for name, value in module_namespace.items():
                self.vars[name] = value
        
        return module_namespace
    
    def execute_block(self, lines, start, end):
        """Execute a block of lines"""
        i = start
        while i < end:
            line = lines[i]
            stripped = line.strip()
            indent = len(line) - len(line.lstrip())
            
            # Skip empty lines and comments
            if not stripped or stripped.startswith('#'):
                i += 1
                continue
            
            if self.debug: print(f"Processing line {i+1}: {stripped}")
            
            # Handle different statement types
            try:
                # Import
                if stripped.startswith('@imp'):
                    # Parse import: @imp module.path
                    import_line = stripped[4:].strip()
                    module_path = import_line.split('.')
                    self.load_module(module_path)
                    i += 1
                    continue
                
                # Function definition
                if stripped.startswith('fn '):
                    i = self.handle_function_def(lines, i)
                    continue
                
                # Struct definition
                if stripped.startswith('struct '):
                    i = self.handle_struct_def(lines, i)
                    continue
                
                # Enum definition
                if stripped.startswith('enum '):
                    i = self.handle_enum_def(lines, i)
                    continue
                
                # Impl block
                if stripped.startswith('impl '):
                    i = self.handle_impl(lines, i)
                    continue
                
                # If statement
                if stripped.startswith('if '):
                    i = self.handle_if(lines, i)
                    continue

                # Match statement
                if stripped.startswith('match '):
                    i = self.handle_match(lines, i)
                    continue
                
                # While loop
                if stripped.startswith('while '):
                    i = self.handle_while(lines, i)
                    continue
                
                # Main entry point (mn:)
                if stripped == 'mn:':
                    if self.debug: print("Found main entry point 'mn:'")
                    # Find block
                    base_indent = len(lines[start]) - len(lines[start].lstrip())
                    indented_start = i + 1
                    indented_end = indented_start
                    
                    # Find end of block
                    while indented_end < len(lines):
                        line = lines[indented_end]
                        if line.strip() and not line.strip().startswith('#'):
                            line_indent = len(line) - len(line.lstrip())
                            if line_indent <= base_indent:
                                break
                        indented_end += 1
                        
                    # Execute block
                    if self.debug: print(f"Executing main block lines {indented_start+1}-{indented_end+1}")
                    self.execute_block(lines, indented_start, indented_end)
                    i = indented_end
                    continue
                
                # For loop
                if stripped.startswith('for '):
                    i = self.handle_for(lines, i)
                    continue
                
                # Try/catch block
                if stripped.startswith('try:'):
                    i = self.handle_try(lines, i)
                    continue
                
                # Return statement
                if stripped.startswith('return'):
                    value = None
                    if len(stripped) > 6:
                        value = self.eval_expr(stripped[6:].strip())
                    raise ReturnValue(value)
                
                # Break/Continue
                if stripped == 'break':
                    raise BreakLoop()
                if stripped == 'continue':
                    raise ContinueLoop()
                
                # Let/var declaration
                if stripped.startswith('let ') or stripped.startswith('var '):
                    self.execute_line(stripped)
                    i += 1
                    continue
                
                # Assignment or expression
                self.execute_line(stripped)
                i += 1
                
            except ReturnValue:
                raise
            except (BreakLoop, ContinueLoop):
                raise
            except Exception as e:
                print(f"Error on line {i+1}: {e}")
                raise
        
        return i
    
    def handle_function_def(self, lines, start):
        """Handle function definition"""
        line = lines[start].strip()
        
        # Parse: fn [decorator] name(params) [-> return_type]:
        # Examples:
        # fn create(config: CompilerConfig) -> Compiler:
        # fn @dict create_token(...) -> Token:
        
        # Regex breakdown:
        # fn\s+                  # Match 'fn '
        # (?:@\w+\s+)?           # Optional decorator like @dict
        # (\w+)                  # Function name
        # \s*\(                  # Opening paren
        # (.*?)                  # Parameters (non-greedy)
        # \)                     # Closing paren
        # (?:\s*->\s*(.+))?      # Optional return type
        # \s*:                   # Colon at end
        
        match = re.match(r'fn\s+(?:@\w+\s+)?(\w+)\s*\((.*?)\)(?:\s*->\s*(.+))?\s*:', line)
        if not match:
            # Try looser match for debugging
            if line.startswith('fn '):
                print(f"Warning: Could not parse function signature: {line}")
            return start + 1
        
        name, params_str, return_type = match.groups()
        
        # Debug - useful to see what's being defined
        # print(f"Defined function: {name}") 
        
        # Parse parameters
        params = []
        if params_str.strip():
            # Basic comma split (doesn't handle nested types in params well yet, but sufficient for now)
            # A better approach would be to parse matching parens/brackets
            current_param = ""
            bracket_level = 0
            for char in params_str:
                if char == ',' and bracket_level == 0:
                    param_str = current_param.strip()
                    p_name = param_str
                    p_type = None
                    
                    if ':' in param_str:
                        p_name, p_type = param_str.split(':', 1)
                        p_name = p_name.strip()
                        p_type = p_type.strip()
                    
                    # Strip modifiers: ref, mut
                    for mod in ['ref', 'mut', 'borrow', 'move', 'kept']:
                        if p_name.startswith(mod + ' '):
                            p_name = p_name[len(mod)+1:].strip()
                            
                    params.append((p_name, p_type))
                    
                    current_param = ""
                else:
                    if char in '[({': bracket_level += 1
                    if char in '])}': bracket_level -= 1
                    current_param += char
            
            # Last param
            if current_param.strip():
                param_str = current_param.strip()
                p_name = param_str
                p_type = None
                
                if ':' in param_str:
                    p_name, p_type = param_str.split(':', 1)
                    p_name = p_name.strip()
                    p_type = p_type.strip()
                
                # Strip modifiers
                for mod in ['ref', 'mut', 'borrow', 'move', 'kept']:
                    if p_name.startswith(mod + ' '):
                        p_name = p_name[len(mod)+1:].strip()
                        
                params.append((p_name, p_type))
        
        # Find function body (indented block after the definition)
        base_indent = len(lines[start]) - len(lines[start].lstrip())
        body_start = start + 1
        body_end = body_start
        
        # Find where body ends (dedent or end of file)
        while body_end < len(lines):
            line = lines[body_end]
            if line.strip() and not line.strip().startswith('#'):
                line_indent = len(line) - len(line.lstrip())
                if line_indent <= base_indent:
                    break
            body_end += 1
        
        # Store function
        body = lines[body_start:body_end]
        self.functions[name] = GulFunction(name, params, body, dict(self.vars))
        self.vars[name] = self.functions[name]
        
        return body_end

    def handle_impl(self, lines, start):
        """Handle impl block"""
        line = lines[start].strip()
        # impl StructName:
        match = re.match(r'impl\s+(\w+)\s*:', line)
        if not match:
             print(f"Warning: Could not parse impl block: {line}")
             return start + 1
        
        struct_name = match.group(1)
        
        # Find struct def
        struct_def = self.structs.get(struct_name)
        if not struct_def:
             print(f"Warning: impl for unknown struct {struct_name}")
             # We can still parse/skip the block
        
        # Find block
        base_indent = len(lines[start]) - len(lines[start].lstrip())
        block_start = start + 1
        i = block_start
        
        while i < len(lines):
            line = lines[i]
            if not line.strip() or line.strip().startswith('#'):
                i += 1
                continue
                
            line_indent = len(line) - len(line.lstrip())
            if line_indent <= base_indent:
                break
                
            # Parse method
            # Re-use handle_function_def logic but customized for methods?
            # Or just parse manually since handle_function_def adds to self.functions
            
            stripped = line.strip()
            if stripped.startswith('fn '):
                 # Manually parse function to avoid polluting global namespace
                 # Parse signature
                 f_match = re.match(r'fn\s+(?:@\w+\s+)?(\w+)\s*\((.*?)\)(?:\s*->\s*(.+))?\s*:', stripped)
                 if f_match:
                      m_name, params_str, ret_type = f_match.groups()
                      
                      # Parse Params (simplified copy from handle_function_def)
                      params = []
                      if params_str.strip():
                            current_param = ""
                            bracket_level = 0
                            for char in params_str:
                                if char == ',' and bracket_level == 0:
                                    p_parts = current_param.strip().split(':', 1)
                                    p_name = p_parts[0].strip()
                                    # Strip modifiers
                                    for mod in ['ref', 'mut', 'borrow', 'move', 'kept']:
                                        if p_name.startswith(mod + ' '): p_name = p_name[len(mod)+1:].strip()
                                    params.append((p_name, None))
                                    current_param = ""
                                else:
                                    if char in '[({': bracket_level += 1
                                    if char in '])}': bracket_level -= 1
                                    current_param += char
                            if current_param.strip():
                                p_parts = current_param.strip().split(':', 1)
                                p_name = p_parts[0].strip()
                                for mod in ['ref', 'mut', 'borrow', 'move', 'kept']:
                                    if p_name.startswith(mod + ' '): p_name = p_name[len(mod)+1:].strip()
                                params.append((p_name, None))
                      
                      # Find body
                      m_base_indent = line_indent
                      m_body_start = i + 1
                      m_body_end = m_body_start
                      while m_body_end < len(lines):
                           m_line = lines[m_body_end]
                           if m_line.strip() and not m_line.strip().startswith('#'):
                                m_indent = len(m_line) - len(m_line.lstrip())
                                if m_indent <= m_base_indent:
                                     break
                           m_body_end += 1
                      
                      m_body = lines[m_body_start:m_body_end]
                      
                      # Register method
                      if struct_def:
                           # Capture closure variables properly? For now assuming stateless or passing vars via args
                           struct_def.methods[m_name] = GulFunction(m_name, params, m_body, dict(self.vars))
                      
                      i = m_body_end
                      continue
            
            i += 1
            
        return i
    
    def handle_struct_def(self, lines, start):
        """Handle struct definition"""
        line = lines[start].strip()
        match = re.match(r'struct\s+(\w+)\s*:', line)
        if not match:
            return start + 1
        
        struct_name = match.group(1)
        fields = []
        methods = {}
        
        base_indent = len(lines[start]) - len(lines[start].lstrip())
        i = start + 1
        
        while i < len(lines):
            sline = lines[i]
            # Check for end of block
            if sline.strip() and not sline.strip().startswith('#'):
                line_indent = len(sline) - len(sline.lstrip())
                if line_indent <= base_indent:
                    break
                
                # Check for method definition
                stripped = sline.strip()
                if stripped.startswith('fn '):
                    # Extract function name for retrieval
                    fn_match = re.match(r'fn\s+(?:@\w+\s+)?(\w+)', stripped)
                    if fn_match:
                        fn_name = fn_match.group(1)
                        # Parse function using existing handler
                        # This adds it to self.functions
                        new_i = self.handle_function_def(lines, i)
                        
                        # Move from global functions to struct methods
                        if fn_name in self.functions:
                            methods[fn_name] = self.functions.pop(fn_name)
                        
                        i = new_i
                        continue
            
                # Parse field: name: type
                if ':' in stripped:
                    parts = stripped.split(':', 1)
                    name = parts[0].strip()
                    fields.append(name)
            i += 1
        
        self.structs[struct_name] = GulStructDefinition(struct_name, fields, methods)
        return i
    
    def handle_enum_def(self, lines, start):
        """Handle enum definition"""
        line = lines[start].strip()
        match = re.match(r'enum\s+(\w+)\s*:', line)
        if not match:
            return start + 1
        
        enum_name = match.group(1)
        variants = []
        
        base_indent = len(lines[start]) - len(lines[start].lstrip())
        i = start + 1
        
        while i < len(lines):
            eline = lines[i]
            if eline.strip() and not eline.strip().startswith('#'):
                line_indent = len(eline) - len(eline.lstrip())
                if line_indent <= base_indent:
                    break
                variants.append(eline.strip())
            i += 1
        
        self.enums[enum_name] = variants
        return i
    
    def handle_if(self, lines, start):
        """Handle if statement"""
        line = lines[start].strip()
        
        # Parse condition
        match = re.match(r'if\s+(.+?):', line)
        if not match:
            return start + 1
        
        condition_expr = match.group(1)
        condition = self.eval_expr(condition_expr)
        
        # Find then block
        base_indent = len(lines[start]) - len(lines[start].lstrip())
        then_start = start + 1
        then_end = then_start
        
        while then_end < len(lines):
            sline = lines[then_end]
            if sline.strip() and not sline.strip().startswith('#'):
                line_indent = len(sline) - len(sline.lstrip())
                if line_indent <= base_indent:
                    break
            then_end += 1
        
        # Check for elif/else
        else_start = then_end
        else_end = then_end
        
        if else_start < len(lines):
            else_line = lines[else_start].strip()
            if else_line.startswith('elif ') or else_line.startswith('else:'):
                # Handle elif/else recursively
                while else_end < len(lines):
                    eline = lines[else_end]
                    if eline.strip() and not eline.strip().startswith('#'):
                        line_indent = len(eline) - len(eline.lstrip())
                        if line_indent < base_indent:
                            break
                    else_end += 1
        
        # Execute appropriate branch
        try:
            if condition:
                self.execute_block(lines, then_start, then_end)
            elif else_start < else_end:
                # Execute else/elif (skip the else: line itself)
                self.execute_block(lines, else_start + 1, else_end)
        except (BreakLoop, ContinueLoop, ReturnValue):
            raise
        
        return else_end if else_end > then_end else then_end

    def handle_match(self, lines, start):
        """Handle match statement"""
        line = lines[start].strip()
        
        match_stmt = re.match(r'match\s+(.+?):', line)
        if not match_stmt:
            return start + 1
            
        target_expr = match_stmt.group(1)
        target_val = self.eval_expr(target_expr)
        
        base_indent = len(lines[start]) - len(lines[start].lstrip())
        current_idx = start + 1
        
        matched_case_executed = False
        
        while current_idx < len(lines):
            line = lines[current_idx]
            stripped = line.strip()
            
            # Skip empty lines/comments
            if not stripped or stripped.startswith('#'):
                current_idx += 1
                continue
                
            # Check indentation to exit match block
            line_indent = len(line) - len(line.lstrip())
            if line_indent <= base_indent:
                break
                
            # Identify case: pattern => ...
            if '=>' in stripped:
                # Split at FIRST => 
                pattern_part, body_part = stripped.split('=>', 1)
                pattern_str = pattern_part.strip()
                body_str = body_part.strip()
                
                # Check if this case matches
                is_match = False
                
                # Default case
                if pattern_str == '_' or pattern_str == 'else':
                    is_match = True
                else:
                    # Evaluate pattern
                    # Check for literals explicitly to handle strings safely
                    # e.g. "foo" => ...
                    val = self.eval_expr(pattern_str)
                    if val == target_val:
                        is_match = True
                
                # Calculate case block range
                case_start = current_idx + 1
                case_end = case_start
                
                # Find extent of this case's block (indented further than case line)
                case_indent = line_indent
                
                # Look ahead for block
                while case_end < len(lines):
                    cline = lines[case_end]
                    cstripped = cline.strip()
                    if cstripped and not cstripped.startswith('#'):
                        c_indent = len(cline) - len(cline.lstrip())
                        # If indent matches case ident, it's next case
                        if c_indent <= case_indent:
                             break
                    case_end += 1
                
                if is_match and not matched_case_executed:
                    matched_case_executed = True
                    try:
                        # Execute inline body if present and not just '{'
                        if body_str and body_str != '{':
                             self.execute_line(body_str)
                        
                        # Execute block if exists
                        if case_end > case_start:
                             self.execute_block(lines, case_start, case_end)
                    except (BreakLoop, ContinueLoop, ReturnValue):
                         raise

                # Advance current_idx to end of this case block
                current_idx = case_end
                
            else:
                # Line inside match but not a case? Skip or error.
                # Could be comment or whitespace handled above.
                current_idx += 1
                
        return current_idx
    
    def handle_while(self, lines, start):
        """Handle while loop"""
        line = lines[start].strip()
        
        match = re.match(r'while\s+(.+?):', line)
        if not match:
            return start + 1
        
        condition_expr = match.group(1)
        
        # Find loop body
        base_indent = len(lines[start]) - len(lines[start].lstrip())
        body_start = start + 1
        body_end = body_start
        
        while body_end < len(lines):
            bline = lines[body_end]
            if bline.strip() and not bline.strip().startswith('#'):
                line_indent = len(bline) - len(bline.lstrip())
                if line_indent <= base_indent:
                    break
            body_end += 1
        
        # Execute loop
        try:
            while self.eval_expr(condition_expr):
                try:
                    self.execute_block(lines, body_start, body_end)
                except ContinueLoop:
                    continue
                except BreakLoop:
                    break
        except ReturnValue:
            raise
        
        return body_end
    
    def handle_for(self, lines, start):
        """Handle for loop"""
        line = lines[start].strip()
        
        match = re.match(r'for\s+(\w+)\s+in\s+(.+?):', line)
        if not match:
            return start + 1
        
        var_name, iterable_expr = match.groups()
        iterable = self.eval_expr(iterable_expr)
        
        # Find loop body
        base_indent = len(lines[start]) - len(lines[start].lstrip())
        body_start = start + 1
        body_end = body_start
        
        while body_end < len(lines):
            bline = lines[body_end]
            if bline.strip() and not bline.strip().startswith('#'):
                line_indent = len(bline) - len(bline.lstrip())
                if line_indent <= base_indent:
                    break
            body_end += 1
        
        # Execute loop
        try:
            for value in iterable:
                self.vars[var_name] = value
                try:
                    self.execute_block(lines, body_start, body_end)
                except ContinueLoop:
                    continue
                except BreakLoop:
                    break
        except ReturnValue:
            raise
        
        return body_end
    
    def handle_try(self, lines, start):
        """Handle try/catch block"""
        # Find try block
        base_indent = len(lines[start]) - len(lines[start].lstrip())
        try_start = start + 1
        try_end = try_start
        
        # Find end of try block (where catch starts)
        while try_end < len(lines):
            line = lines[try_end]
            if line.strip() and not line.strip().startswith('#'):
                line_indent = len(line) - len(line.lstrip())
                if line_indent <= base_indent:
                    if line.strip().startswith('catch'):
                        break
                    elif line_indent < base_indent:
                        break
            try_end += 1
        
        # Find catch block if exists
        catch_start = try_end
        catch_end = catch_start
        
        if catch_start < len(lines) and lines[catch_start].strip().startswith('catch'):
            catch_start += 1
            catch_end = catch_start
            while catch_end < len(lines):
                line = lines[catch_end]
                if line.strip() and not line.strip().startswith('#'):
                    line_indent = len(line) - len(line.lstrip())
                    if line_indent <= base_indent:
                        break
                catch_end += 1
        
        # Execute try block
        try:
            self.execute_block(lines, try_start, try_end)
        except (ReturnValue, BreakLoop, ContinueLoop):
            raise
        except Exception as e:
            # Execute catch block if exists
            if catch_start < catch_end:
                # Bind exception to variable if specified
                # For now, just execute catch block
                self.execute_block(lines, catch_start, catch_end)
        
        return catch_end if catch_end > try_end else try_end
    
    def execute_line(self, line):
        """Execute a single line"""
        # Let declaration
        if line.startswith('let '):
            match = re.match(r'let\s+(\w+)(?:\s*:\s*\S+)?\s*=\s*(.+)', line)
            if match:
                name, expr = match.groups()
                value = self.eval_expr(expr)
                self.vars[name] = value
                return
        
        # Var declaration
        if line.startswith('var '):
            match = re.match(r'var\s+(\w+)(?:\s*:\s*\S+)?\s*=\s*(.+)', line)
            if match:
                name, expr = match.groups()
                value = self.eval_expr(expr)
                self.vars[name] = value
                return
        
        # Assignment (Robust detection)
        if '=' in line:
            assign_idx = -1
            in_quote = False
            quote_char = ''
            parens = 0
            
            for i, char in enumerate(line):
                if char in ['"', "'"]:
                    if not in_quote:
                        in_quote = True
                        quote_char = char
                    elif char == quote_char:
                        # Simple escape check
                        if i > 0 and line[i-1] != '\\':
                            in_quote = False
                
                if in_quote: continue
                
                if char == '(': parens += 1
                elif char == ')': parens -= 1
                
                if parens > 0: continue
                
                if char == '=':
                    # Check context to ensure it's assignment =
                    prev = line[i-1] if i > 0 else ' '
                    next_c = line[i+1] if i+1 < len(line) else ' '
                    
                    if prev in ['!', '<', '>', '=', '/']: continue
                    if next_c in ['=', '>']: continue
                    
                    assign_idx = i
                    break
            
            if assign_idx != -1:
                parts = [line[:assign_idx], line[assign_idx+1:]]
                target = parts[0].strip()
                value_expr = parts[1].strip()
            
                # Handle attribute /index assignment
                if '.' in target or '[' in target:
                    # Implement attribute assignment
                    if '.' in target and '[' not in target: # Simple dot access for now
                        obj_name, attr = target.rsplit('.', 1)
                        value = self.eval_expr(value_expr)
                        
                        obj = self.eval_expr(obj_name)
                        if hasattr(obj, 'fields') and isinstance(obj.fields, dict):
                             obj.fields[attr] = value
                        elif hasattr(obj, attr): # Python object
                             setattr(obj, attr, value)
                        else:
                             print(f"Error: Cannot set attribute {attr} on {obj}")
                    else:
                         print(f"Warning: Complex assignment to {target} not fully supported yet")
                else:
                    value = self.eval_expr(value_expr)
                    self.vars[target] = value
                return
        
        # Expression (might be function call)
        self.eval_expr(line)
    
    def eval_expr(self, expr):
        """Evaluate an expression"""
        # if self.debug: print("DEBUG: eval_expr input:", expr)
        
        expr = expr.strip()
        
        # Empty
        if not expr:
            return None
        
        # Binary operations (Check FIRST using robust parsing)
        # Sort ops by precedence (lowest binding power first for splitting)
        # Logical > Comparison > Add/Sub > Mul/Div
        ops_by_precedence = [
            ['||', 'or'],
            ['&&', 'and'],
            ['==', '!=', '<=', '>=', '<', '>', 'in'],
            ['+', '-'],
            ['*', '/']
        ]
        
        for ops in ops_by_precedence:
            split_res = self.parse_binary_op(expr, ops)
            if split_res:
                left_expr, op, right_expr = split_res
                left = self.eval_expr(left_expr)
                right = self.eval_expr(right_expr)
                return self.apply_op(left, op, right)

        # Unary operations (not, -)
        # Must be checked AFTER binary to allow splitting "not A and B" at "and"
        for op in ['not ', '-']: # Space for word ops
            if expr.startswith(op):
                # not A -> not arg
                # -A -> - arg
                arg_expr = expr[len(op):].strip()
                # Special handling for '-' to avoid matching identifiers starting with - (impossible)
                # But 'not ' includes space. '-' does not.
                # If op is '-', we should ensure it's not part of something else?
                # But we are at start of string (startswith).
                # If expr is "-5". op="-". arg="5".
                # If expr is "-var". op="-". arg="var".
                operand = self.eval_expr(arg_expr)
                return self.apply_unary_op(op.strip(), operand)

        # String literals
        if expr.startswith('"') and expr.endswith('"'):
            # Basic integrity check (crude): ensure not split like "A" + "B"
            # Since parse_binary_op didn't find ops at depth 0, this assumes safe
            # return expr[1:-1]
            import codecs
            try:
                return codecs.decode(expr[1:-1], 'unicode_escape')
            except:
                return expr[1:-1]
        if expr.startswith("'") and expr.endswith("'"):
            import codecs
            try:
                return codecs.decode(expr[1:-1], 'unicode_escape')
            except:
                return expr[1:-1]
        
        # Number literals
        if expr.replace('.', '').replace('-', '').replace('e', '').replace('+', '').isdigit():
            try:
                if '.' in expr or 'e' in expr:
                    return float(expr)
                return int(expr)
            except:
                pass
        
        # Boolean
        if expr == 'true' or expr == 'True':
            return True
        if expr == 'false' or expr == 'False':
            return False
        if expr == 'None':
            return None
        
        # Number literals
        # Check for float logic (contains . or e)
        is_number = False
        try:
            # Quick check if it looks like number
            if expr[0].isdigit() or (expr.startswith('-') and len(expr) > 1 and expr[1].isdigit()):
                is_number = True
            elif expr.startswith('.'): # .5
                is_number = True
        except: pass
        
        if is_number:
            try:
                if '.' in expr or 'e' in expr or 'E' in expr:
                    return float(expr)
                return int(expr)
            except ValueError:
                pass # Not a number, continue

        
        # List literal
        if expr.startswith('@list[') or (expr.startswith('[') and expr.endswith(']')):
             # Handle generic list syntax if allowed, or just @list
             # Determine content
             inner = ""
             if expr.startswith('@list['):
                 inner = expr[6:-1]
             else:
                 inner = expr[1:-1]
                 
             if not inner.strip(): return []
             # Use split_args to handle nested lists
             return [self.eval_expr(x.strip()) for x in self.split_args(inner)]
        
        # Dict literal
        if expr.startswith('@dict{') or (expr.startswith('{') and ':' in expr and '=' not in expr):
             # Distinguish from struct: Struct uses '=' for fields usually? No, struct definition uses ':'.
             # Struct instantiation: Name{field: val}.
             # Dict: {key: val}.
             # If starts with @dict, safe.
             # If just {, ambiguous. GUL uses Name{...} for structs.
             # So {k:v} is dict.
             # BUT Struct literal logic is: if '{' in expr and '=' not in expr.
             # Wait. Struct instantiation: Lexer{stream: ...}. Using ':'.
             # My interpreter Struct Construction logic checks: if '{' in expr and '=' not in expr.
             # This CONFLICTS with Dict {k:v}.
             # However, struct construction requires Name before {.
             # Regex `(\w+)\s*\{`.
             # Dict starts with {.
             # So we must check startswith '{' or '@dict'.
             
             inner = ""
             if expr.startswith('@dict{'):
                 inner = expr[6:-1]
             elif expr.startswith('{'):
                 inner = expr[1:-1]
                 
             if not inner.strip(): return {}
             
             d = {}
             for part in self.split_args(inner):
                 if ':' in part:
                     k, v = part.split(':', 1)
                     d[self.eval_expr(k.strip())] = self.eval_expr(v.strip())
             return d

        # Type constructors
        if expr.startswith('@'):
            return self.eval_type_constructor(expr)
        
        # Struct construction
        if '{' in expr and '=' not in expr:
            # Check for StructName{...}
            # Find first {
            try:
                open_brace = expr.index('{')
                if open_brace > 0:
                     struct_name = expr[:open_brace].strip()
                     # Verify name is identifier
                     if struct_name.isidentifier():
                         # Find matching }
                         balance = 0
                         close_brace = -1
                         for i in range(open_brace, len(expr)):
                             if expr[i] == '{': balance += 1
                             elif expr[i] == '}': 
                                 balance -= 1
                                 if balance == 0:
                                     close_brace = i
                                     break
                        
                         if close_brace != -1:
                             fields_str = expr[open_brace+1:close_brace]
                             fields = {}
                             for pair in self.split_args(fields_str):
                                 if ': 'in pair:
                                     k, v = pair.split(':', 1)
                                     val = self.eval_expr(v.strip())
                                     if self.debug: print(f"DEBUG: Field {k.strip()} [{v.strip()}] -> {type(val)}: {val}")
                                     fields[k.strip()] = val
                                 elif ':' in pair: # Allow tight colon
                                     k, v = pair.split(':', 1)
                                     val = self.eval_expr(v.strip())
                                     if self.debug: print(f"DEBUG: Field {k.strip()} [{v.strip()}] -> {type(val)}: {val}")
                                     fields[k.strip()] = val
                             
                             struct_obj = GulStruct(struct_name, fields)
                             if self.debug: print(f"DEBUG: Struct {struct_name} created (id={id(struct_obj)}) with fields: {list(fields.keys())}")
                             return struct_obj
            except Exception as e:
                print(f"DEBUG: Error constructing struct: {e}")
                pass
        
        # Function call
        if '(' in expr and ')' in expr:
            # Simple check, but call might be obj.method()
            # If ( is after [ or ., it's call. 
            pass # Let eval_call handle it or fall through?
            # Actually eval_expr currently checks parens logic below or above?
            # In previous view, "Function call" was at 686.
            pass

        # Index access (var[index])
        if expr.endswith(']'):
            # Find matching [ scanning backward
            balance = 0
            found_open = -1
            for i in range(len(expr) - 1, -1, -1):
                char = expr[i]
                if char == ']':
                    balance += 1
                elif char == '[':
                    balance -= 1
                    if balance == 0:
                        found_open = i
                        break
            
            if found_open != -1:
                # obj[index]
                obj_expr = expr[:found_open]
                index_expr = expr[found_open+1:-1]
                
                # If obj_expr is empty, it's list literal? [1,2] matches startswith checks earlier.
                # But check just in case
                if obj_expr.strip():
                     # Evaluate obj (unless it's a type constructor like @list[...])
                     if not obj_expr.strip().startswith('@'): 
                        obj = self.eval_expr(obj_expr)
                        index = self.eval_expr(index_expr)
                        
                        try:
                            return obj[index]
                        except Exception as e:
                            if self.debug: print(f"Index error: {e} on {obj_expr}[{index_expr}]")
                            pass
        
        # Function call check (moved/re-verified context)
        if '(' in expr and expr.endswith(')'):
             return self.eval_call(expr)
        
        # Attribute access (check for dots not in function calls)
        if '.' in expr and '(' not in expr:
            parts = expr.split('.', 1)
            obj_name = parts[0].strip()
            attr_path = parts[1].strip()
            
            # Get the base object
            if obj_name in self.vars:
                obj = self.vars[obj_name]
            else:
                return expr  # Not a variable, return as is
            
            # DEBUG
            if self.debug: print(f"DEBUG: Accessing {attr_path} on {obj_name} ({type(obj)})")
            
            # Navigate attribute path
            for attr in attr_path.split('.'):
                attr = attr.strip()
                if self.debug: print(f"  DEBUG: attr={attr}, obj={obj}")
                
                if isinstance(obj, GulStruct) and attr in obj.fields:
                    obj = obj.fields[attr]
                elif isinstance(obj, dict) and attr in obj:
                    obj = obj[attr]
                elif hasattr(obj, attr):
                    obj = getattr(obj, attr)
                else:
                    return expr
            return obj
        
        # Variable lookup
        if expr in self.vars:
            return self.vars[expr]
        
        # Unknown or Failed - raise error to help debug
        raise Exception(f"Failed to evaluate expression: '{expr}'")
    
    def eval_type_constructor(self, expr):
        """Evaluate type constructor like @int(x)"""
        match = re.match(r'@(\w+)\((.*)\)', expr)
        if match:
            type_name, arg_expr = match.groups()
            arg = self.eval_expr(arg_expr)
            
            if type_name == 'int':
                return int(arg)
            elif type_name == 'float':
                return float(arg)
            elif type_name == 'str':
                return str(arg)
            elif type_name == 'bool':
                return bool(arg)
            elif type_name == 'list':
                return list(arg) if arg else []
        
        return None
    
    def eval_call(self, expr):
        """Evaluate function call"""
        # if self.debug: print("DEBUG: eval_call input:", expr)
        
        # Handle method calls: obj.method(args)
        # Must have a dot before the opening paren
        if '.' in expr and '(' in expr:
            try:
                # Find the dot that separates object and method (handle parens in args)
                # Reverse search for ( to find start of args
                # Actually, check logic: obj.method(arg)
                paren_idx = expr.index('(')
                dot_idx = expr.rindex('.', 0, paren_idx)
                
                obj_expr = expr[:dot_idx]
                rest = expr[dot_idx+1:]
                
                match = re.match(r'(\w+)\((.*)\)', rest)
                if match:
                    method_name, args_str = match.groups()
                    # if self.debug: print(f"DEBUG: Eval call parsed: obj={obj_expr}, method={method_name}, args={args_str}")
                    
                    # Evaluate object
                    obj = None
                    is_static = False
                    
                    if obj_expr in self.structs:
                        obj = self.structs[obj_expr] # GulStructDefinition
                        is_static = True
                    else:
                        obj = self.eval_expr(obj_expr)
                    
                    args = []
                    if args_str.strip():
                        args = [self.eval_expr(a.strip()) for a in self.split_args(args_str)]
                    
                    # Mapping for list methods
                    if isinstance(obj, list) and method_name == 'add':
                        method_name = 'append'
                    
                    # Call method
                    # 1. Check Python methods (for builtins/lists/etc)
                    if hasattr(obj, method_name):
                        method = getattr(obj, method_name)
                        return method(*args)
                    
                    # 2. Check Struct Instance methods
                    if isinstance(obj, GulStruct):
                        struct_def = self.structs.get(obj.struct_name)
                        if self.debug: 
                            if struct_def:
                                print(f"DEBUG: Checking methods for struct {obj.struct_name}: {list(struct_def.methods.keys())}")
                            else:
                                print(f"DEBUG: Struct def not found for {obj.struct_name}")
                        
                        if struct_def and method_name in struct_def.methods:
                            func = struct_def.methods[method_name]
                            # Pass 'self' (obj) as first arg?
                            # Check function signature
                            # Assuming standard GUL method signature: fn method(self, ...)
                            # We prepend obj to args
                            return self.call_gul_function(func, [obj] + args)
                            
                    # 3. Check Struct Static methods
                    if isinstance(obj, GulStructDefinition):
                        if method_name in obj.methods:
                            func = obj.methods[method_name]
                            # Static method, no 'self' prepended (unless passed explicitly?)
                            # Usually static methods don't take self.
                            return self.call_gul_function(func, args)

            except (ValueError, AttributeError) as e:
                # Not a method call, fall through to regular function call
                if self.debug: print(f"DEBUG: Method call failed: {e}")
                pass
        
        # Regular function call
        match = re.match(r'([a-zA-Z_]\w*)\((.*)\)$', expr)
        if match:
            func_name, args_str = match.groups()
            
            # Get function
            if func_name in self.vars:
                func = self.vars[func_name]
            else:
                return None
            
            # Parse arguments
            args = []
            if args_str.strip():
                for arg in self.split_args(args_str):
                    args.append(self.eval_expr(arg.strip()))
            
            # Call function
            if callable(func):
                return func(*args)
            elif isinstance(func, GulFunction):
                return self.call_gul_function(func, args)
        
        return None
    
    def call_gul_function(self, func, args):
        """Call a GUL-defined function"""
        if self.debug: 
            print(f"DEBUG: Executing GUL function '{func.name}' with {len(args)} args")
            print(f"DEBUG: Args values: {args}")
        
        # Save current vars
        saved_vars = dict(self.vars)
        
        # Set up function environment
        self.vars.update(func.closure)
        
        # Bind parameters
        for (param_name, _), arg_value in zip(func.params, args):
            self.vars[param_name] = arg_value
        
        # Execute function body
        result = None
        try:
            self.execute_block(func.body, 0, len(func.body))
        except ReturnValue as ret:
            result = ret.value
        
        # Restore vars
        self.vars = saved_vars
        
        return result
    
    def split_args(self, args_str):
        """Split comma-separated arguments (respecting nesting and quotes)"""
        args = []
        current = ""
        depth = 0
        in_quote = False
        quote_char = ""
        
        for i, char in enumerate(args_str):
            if char in ["'", '"']:
                if not in_quote:
                    in_quote = True
                    quote_char = char
                elif char == quote_char:
                    # Check escape
                    if i > 0 and args_str[i-1] != '\\':
                        in_quote = False
            
            if not in_quote:
                if char in '([{':
                    depth += 1
                elif char in ')]}':
                    depth -= 1
                elif char == ',' and depth == 0:
                    args.append(current.strip())
                    current = ""
                    continue
            current += char
        
        if current.strip():
            args.append(current.strip())
        
        return args
    
    def builtin_read_file(self, filepath):
        """Read file contents"""
        # Let exceptions propagate to be caught by interpreter's try/catch
        with open(filepath, 'r') as f:
            return f.read()
    
    def builtin_write_file(self, filepath, content):
        """Write content to file"""
        try:
            with open(filepath, 'w') as f:
                f.write(content)
            return True
        except Exception as e:
            print(f"Error writing file {filepath}: {e}")
            return False
    
    def builtin_file_exists(self, filepath):
        """Check if file exists"""
        from pathlib import Path
        return Path(filepath).exists()
    
    def parse_binary_op(self, expr, ops):
        """Find top-level binary operator split point respecting nesting"""
        depth = 0
        in_quote = False
        quote_char = ''
        
        last_op_index = -1
        last_op_len = 0
        last_op_str = ''
        
        i = 0
        while i < len(expr):
            char = expr[i]
            
            if in_quote:
                if char == quote_char and (i == 0 or expr[i-1] != '\\'):
                    in_quote = False
            else:
                if char in '"\'':
                    in_quote = True
                    quote_char = char
                elif char in '([{':
                    depth += 1
                elif char in ')]}':
                    depth -= 1
                elif depth == 0:
                    # Check for ops
                    for op in ops:
                        # Check " op " context
                        if expr.startswith(op, i):
                            # BINARY op must have left operand, so start > 0
                            # But if strict space matching ' + ' is used, i > 0 is implicit if expr trimmed
                            # However, our op list includes '+', not ' + '.
                            # We must enforce i > 0 to avoid matching unary - at start
                            if i == 0:
                                continue
                                
                            is_match = True
                            op_len = len(op)
                            
                            # Check start
                            if op[0].isalnum(): # Keywords like 'and'
                                if i > 0 and expr[i-1].isalnum(): is_match = False
                            
                            # Check end
                            if op[-1].isalnum():
                                if i + op_len < len(expr) and expr[i+op_len].isalnum(): is_match = False
                                
                            if is_match:
                                last_op_index = i
                                last_op_len = op_len
                                last_op_str = op
                                break
            i += 1
            
        if last_op_index != -1:
            return (expr[:last_op_index].strip(), last_op_str, expr[last_op_index+last_op_len:].strip())
        return None

    def apply_unary_op(self, op, operand):
        """Apply unary operator"""
        if op == 'not':
            return not operand
        elif op == '-':
            # Ensure operand is number
            try:
                return -operand
            except:
                return None
        return None

    def apply_op(self, left, op, right):
        """Apply binary operator"""
        if op == '+':
            if self.debug: print(f"DEBUG: + op: left={left!r} ({type(left)}), right={right!r} ({type(right)})")
            if isinstance(left, str) or isinstance(right, str):
                return str(left) + str(right)
            return left + right
        elif op == '-':
            return left - right
        elif op == '*':
            return left * right
        elif op == '/':
            return left / right if right != 0 else float('inf')
        elif op == '==':
            return left == right
        elif op == '!=':
            return left != right
        elif op == '<':
            return left < right
        elif op == '>':
            return left > right
        elif op == '<=':
            return left <= right
        elif op == '>=':
            return left >= right
        elif op in ['&&', 'and']:
            return left and right
        elif op in ['||', 'or']:
            return left or right
        elif op == 'in':
            return left in right
        return None

# Exception classes for control flow
class ReturnValue(Exception):
    def __init__(self, value):
        self.value = value

class BreakLoop(Exception):
    pass

class ContinueLoop(Exception):
    pass

def main():
    if len(sys.argv) < 2:
        print("GUL Interpreter v0.2.0")
        print("Usage: python3 gul_interpreter.py <file.mn>")
        return
    
    filename = sys.argv[1]
    
    if not Path(filename).exists():
        print(f"Error: File not found: {filename}")
        return
    
    # Pass all command-line arguments to interpreter
    interpreter = GulInterpreter(argv=sys.argv[1:])  # Skip script name
    try:
        interpreter.run_file(filename)
    except Exception as e:
        print(f"\n❌ Error: {e}")
        import traceback
        traceback.print_exc()

if __name__ == '__main__':
    main()
