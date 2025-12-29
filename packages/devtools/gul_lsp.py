"""
GUL Language Server Protocol (LSP)
Simplified but functional LSP implementation.

Status: ✅ Implemented
Priority: High
Phase: 1
"""

from typing import Dict, List, Optional, Any
from dataclasses import dataclass
import json

__version__ = "0.1.0"
__all__ = ['LSPServer', 'CompletionItem', 'Diagnostic', 'Position', 'Range']

@dataclass
class Position:
    """Position in a document"""
    line: int
    character: int

@dataclass
class Range:
    """Range in a document"""
    start: Position
    end: Position

@dataclass
class Diagnostic:
    """Diagnostic (error/warning)"""
    range: Range
    message: str
    severity: int  # 1=Error, 2=Warning, 3=Info, 4=Hint

@dataclass
class CompletionItem:
    """Code completion item"""
    label: str
    kind: int  # 1=Text, 3=Function, 6=Variable, etc.
    detail: Optional[str] = None
    documentation: Optional[str] = None

class LSPServer:
    """
    Simplified Language Server Protocol implementation
    
    Provides:
    - Code completion
    - Diagnostics (errors/warnings)
    - Hover information
    - Go to definition
    
    Example:
        lsp = LSPServer()
        completions = lsp.get_completions("std.", 0, 4)
        diagnostics = lsp.get_diagnostics("let x = ")
    """
    
    def __init__(self):
        self.documents: Dict[str, str] = {}
        self.completions = self._init_completions()
    
    def _init_completions(self) -> List[CompletionItem]:
        """Initialize standard library completions"""
        return [
            # Keywords
            CompletionItem("let", 14, "Immutable variable"),
            CompletionItem("var", 14, "Mutable variable"),
            CompletionItem("fn", 14, "Function definition"),
            CompletionItem("async", 14, "Async function"),
            CompletionItem("struct", 14, "Struct definition"),
            CompletionItem("match", 14, "Pattern matching"),
            CompletionItem("if", 14, "If statement"),
            CompletionItem("for", 14, "For loop"),
            CompletionItem("while", 14, "While loop"),
            CompletionItem("return", 14, "Return statement"),
            
            # Types
            CompletionItem("@int", 25, "Integer type"),
            CompletionItem("@str", 25, "String type"),
            CompletionItem("@float", 25, "Float type"),
            CompletionItem("@bool", 25, "Boolean type"),
            CompletionItem("@list", 25, "List type"),
            CompletionItem("@dict", 25, "Dictionary type"),
            
            # Standard library
            CompletionItem("std.io", 9, "I/O module", "Input/output operations"),
            CompletionItem("std.http", 9, "HTTP module", "HTTP client and server"),
            CompletionItem("std.json", 9, "JSON module", "JSON encoding/decoding"),
            CompletionItem("std.math", 9, "Math module", "Mathematical functions"),
            CompletionItem("print", 3, "Print function", "print(value)"),
            CompletionItem("len", 3, "Length function", "len(collection)"),
            CompletionItem("range", 3, "Range function", "range(start, end)"),
        ]
    
    def did_open(self, uri: str, text: str):
        """Document opened"""
        self.documents[uri] = text
    
    def did_change(self, uri: str, text: str):
        """Document changed"""
        self.documents[uri] = text
    
    def did_close(self, uri: str):
        """Document closed"""
        if uri in self.documents:
            del self.documents[uri]
    
    def get_completions(
        self,
        uri: str,
        line: int,
        character: int
    ) -> List[CompletionItem]:
        """Get code completions at position"""
        if uri not in self.documents:
            return []
        
        text = self.documents[uri]
        lines = text.split('\n')
        
        if line >= len(lines):
            return self.completions
        
        current_line = lines[line][:character]
        
        # Filter completions based on prefix
        if '.' in current_line:
            # Module member completion
            parts = current_line.split('.')
            prefix = parts[-1]
            module = '.'.join(parts[:-1]).strip()
            
            if module == "std":
                return [c for c in self.completions if c.label.startswith("std.")]
        else:
            # General completion
            words = current_line.split()
            if words:
                prefix = words[-1]
                return [c for c in self.completions if c.label.startswith(prefix)]
        
        return self.completions
    
    def get_diagnostics(self, uri: str) -> List[Diagnostic]:
        """Get diagnostics for document"""
        if uri not in self.documents:
            return []
        
        text = self.documents[uri]
        diagnostics = []
        
        lines = text.split('\n')
        for line_num, line in enumerate(lines):
            # Check for common errors
            if '=' in line and line.strip().startswith('let'):
                if '@' not in line:
                    diagnostics.append(Diagnostic(
                        range=Range(
                            Position(line_num, 0),
                            Position(line_num, len(line))
                        ),
                        message="Consider adding type annotation with @",
                        severity=3  # Info
                    ))
            
            # Check for old syntax
            if 'import ' in line:
                diagnostics.append(Diagnostic(
                    range=Range(
                        Position(line_num, line.index('import')),
                        Position(line_num, line.index('import') + 6)
                    ),
                    message="Use @imp instead of import",
                    severity=2  # Warning
                ))
            
            if line.strip().startswith('def '):
                diagnostics.append(Diagnostic(
                    range=Range(
                        Position(line_num, line.index('def')),
                        Position(line_num, line.index('def') + 3)
                    ),
                    message="Use fn instead of def",
                    severity=2  # Warning
                ))
        
        return diagnostics
    
    def get_hover(self, uri: str, line: int, character: int) -> Optional[str]:
        """Get hover information"""
        if uri not in self.documents:
            return None
        
        # Simple hover: return type info or documentation
        for item in self.completions:
            if item.documentation:
                return item.documentation
        
        return "GUL v3.2 - Hover information"
    
    def goto_definition(self, uri: str, line: int, character: int) -> Optional[Position]:
        """Go to definition (simplified)"""
        # In a real implementation, this would jump to the definition
        return Position(0, 0)
    
    def format_document(self, uri: str) -> Optional[str]:
        """Format document"""
        if uri not in self.documents:
            return None
        
        text = self.documents[uri]
        lines = text.split('\n')
        formatted = []
        
        indent = 0
        for line in lines:
            stripped = line.strip()
            
            # Decrease indent for closing
            if stripped in ['}', ']', ')'] or stripped.startswith('else'):
                indent = max(0, indent - 1)
            
            formatted.append('    ' * indent + stripped)
            
            # Increase indent after opening
            if stripped.endswith(':') or stripped.endswith('{') or stripped.endswith('['):
                indent += 1
        
        return '\n'.join(formatted)

# JSON-RPC helpers
class JSONRPCHandler:
    """Handle JSON-RPC messages"""
    
    def __init__(self, lsp_server: LSPServer):
        self.lsp = lsp_server
    
    def handle_message(self, message: str) -> str:
        """Handle incoming JSON-RPC message"""
        try:
            data = json.loads(message)
            method = data.get('method')
            params = data.get('params', {})
            msg_id = data.get('id')
            
            result = self._handle_method(method, params)
            
            if msg_id:
                return json.dumps({
                    'jsonrpc': '2.0',
                    'id': msg_id,
                    'result': result
                })
            return json.dumps({'jsonrpc': '2.0'})
        
        except Exception as e:
            return json.dumps({
                'jsonrpc': '2.0',
                'error': {'code': -32603, 'message': str(e)}
            })
    
    def _handle_method(self, method: str, params: Dict) -> Any:
        """Handle specific LSP method"""
        if method == 'textDocument/didOpen':
            text_doc = params.get('textDocument', {})
            self.lsp.did_open(text_doc.get('uri'), text_doc.get('text', ''))
            return None
        
        elif method == 'textDocument/completion':
            text_doc = params.get('textDocument', {})
            position = params.get('position', {})
            completions = self.lsp.get_completions(
                text_doc.get('uri'),
                position.get('line', 0),
                position.get('character', 0)
            )
            return {'items': [vars(c) for c in completions]}
        
        elif method == 'textDocument/publishDiagnostics':
            uri = params.get('uri')
            diagnostics = self.lsp.get_diagnostics(uri)
            return {'diagnostics': [vars(d) for d in diagnostics]}
        
        return None
