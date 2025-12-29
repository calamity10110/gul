"""
GUL YAML Parser
YAML parsing and serialization.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Any, Dict, List, Union
import re

__version__ = "0.1.0"
__all__ = ['parse_yaml', 'dump_yaml', 'YAMLParser']

class YAMLParser:
    """Simple YAML parser"""
    
    def parse(self, content: str) -> Union[Dict, List, Any]:
        """Parse YAML content"""
        lines = content.strip().split('\n')
        return self._parse_lines(lines, 0)[0]
    
    def _parse_lines(self, lines: List[str], start_indent: int) -> tuple:
        """Parse lines with indentation"""
        result = {}
        i = 0
        current_key = None
        
        while i < len(lines):
            line = lines[i]
            
            if not line.strip() or line.strip().startswith('#'):
                i += 1
                continue
            
            indent = len(line) - len(line.lstrip())
            
            if indent < start_indent:
                break
            
            if indent > start_indent:
                i += 1
                continue
            
            # Key-value pair
            if ':' in line:
                key, value = line.split(':', 1)
                key = key.strip()
                value = value.strip()
                
                if value:
                    result[key] = self._parse_value(value)
                else:
                    # Nested object or list
                    if i + 1 < len(lines):
                        next_line = lines[i + 1]
                        next_indent = len(next_line) - len(next_line.lstrip())
                        
                        if next_line.strip().startswith('-'):
                            # List
                            items, consumed = self._parse_list(lines[i + 1:], next_indent)
                            result[key] = items
                            i += consumed
                        elif next_indent > indent:
                            # Nested object
                            nested, consumed = self._parse_lines(lines[i + 1:], next_indent)
                            result[key] = nested
                            i += consumed
            
            i += 1
        
        return result, i
    
    def _parse_list(self, lines: List[str], start_indent: int) -> tuple:
        """Parse list"""
        items = []
        i = 0
        
        while i < len(lines):
            line = lines[i]
            
            if not line.strip():
                i += 1
                continue
            
            indent = len(line) - len(line.lstrip())
            
            if indent < start_indent:
                break
            
            if line.strip().startswith('-'):
                value = line.strip()[1:].strip()
                
                if value:
                    items.append(self._parse_value(value))
                else:
                    # Nested structure
                    if i + 1 < len(lines):
                        next_indent = len(lines[i + 1]) - len(lines[i + 1].lstrip())
                        if next_indent > indent:
                            nested, consumed = self._parse_lines(lines[i + 1:], next_indent)
                            items.append(nested)
                            i += consumed
            
            i += 1
        
        return items, i
    
    def _parse_value(self, value: str) -> Any:
        """Parse value"""
        value = value.strip()
        
        # String in quotes
        if (value.startswith('"') and value.endswith('"')) or \
           (value.startswith("'") and value.endswith("'")):
            return value[1:-1]
        
        # Number
        if re.match(r'^-?\d+$', value):
            return int(value)
        
        if re.match(r'^-?\d+\.\d+$', value):
            return float(value)
        
        # Boolean
        if value.lower() == 'true':
            return True
        if value.lower() == 'false':
            return False
        
        # Null
        if value.lower() in ['null', '~', '']:
            return None
        
        return value
    
    def dump(self, data: Union[Dict, List], indent: int = 0) -> str:
        """Dump data to YAML"""
        lines = []
        prefix = '  ' * indent
        
        if isinstance(data, dict):
            for key, value in data.items():
                if isinstance(value, (dict, list)):
                    lines.append(f"{prefix}{key}:")
                    lines.append(self.dump(value, indent + 1))
                else:
                    lines.append(f"{prefix}{key}: {self._format_value(value)}")
        
        elif isinstance(data, list):
            for item in data:
                if isinstance(item, (dict, list)):
                    lines.append(f"{prefix}-")
                    lines.append(self.dump(item, indent + 1))
                else:
                    lines.append(f"{prefix}- {self._format_value(item)}")
        
        return '\n'.join(lines)
    
    def _format_value(self, value: Any) -> str:
        """Format value for YAML"""
        if value is None:
            return 'null'
        if isinstance(value, bool):
            return 'true' if value else 'false'
        if isinstance(value, str) and (' ' in value or ':' in value):
            return f'"{value}"'
        return str(value)

def parse_yaml(content: str) -> Any:
    """Quick YAML parse"""
    parser = YAMLParser()
    return parser.parse(content)

def dump_yaml(data: Any) -> str:
    """Quick YAML dump"""
    parser = YAMLParser()
    return parser.dump(data)
