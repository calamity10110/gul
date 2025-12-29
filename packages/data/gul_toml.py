"""
GUL TOML
TOML parser and generator.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Dict, Any, Optional
import json

__version__ = "0.1.0"
__all__ = ['load', 'loads', 'dump', 'dumps']

# Since Python 3.11, tomllib is built-in for parsing
try:
    import tomllib
except ImportError:
    # Basic fallback implementation or rely on user having 'toml' installed
    # For now, we'll implement a basic subset if not available
    tomllib = None

class TOMLParser:
    """Basic TOML parser fallback"""
    def parse(self, text: str) -> Dict[str, Any]:
        data = {}
        current_section = data
        
        for line in text.split('\n'):
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            
            # Section: [section]
            if line.startswith('[') and line.endswith(']'):
                section = line[1:-1]
                data[section] = {}
                current_section = data[section]
                continue
            
            # Key-Value: key = value
            if '=' in line:
                key, value = line.split('=', 1)
                key = key.strip()
                value = self._parse_value(value.strip())
                current_section[key] = value
        
        return data
    
    def _parse_value(self, value: str) -> Any:
        # String
        if value.startswith('"') and value.endswith('"'):
            return value[1:-1]
        
        # Boolean
        if value == 'true': return True
        if value == 'false': return False
        
        # Array (simplified)
        if value.startswith('[') and value.endswith(']'):
            items = value[1:-1].split(',')
            return [self._parse_value(item.strip()) for item in items if item.strip()]
        
        # Number
        try:
            return int(value)
        except ValueError:
            try:
                return float(value)
            except ValueError:
                return value

def loads(text: str) -> Dict[str, Any]:
    """Load TOML from string"""
    if tomllib:
        return tomllib.loads(text)
    return TOMLParser().parse(text)

def load(fp) -> Dict[str, Any]:
    """Load TOML from file"""
    if tomllib:
        return tomllib.load(fp)
    return loads(fp.read())

def dumps(data: Dict[str, Any]) -> str:
    """Dump TOML to string"""
    lines = []
    
    # Simple values first
    for k, v in data.items():
        if isinstance(v, dict):
            continue
        lines.append(f"{k} = {_dump_value(v)}")
    
    # Sections
    for k, v in data.items():
        if isinstance(v, dict):
            lines.append(f"\n[{k}]")
            for sk, sv in v.items():
                lines.append(f"{sk} = {_dump_value(sv)}")
    
    return "\n".join(lines)

def dump(data: Dict[str, Any], fp):
    """Dump TOML to file"""
    fp.write(dumps(data))

def _dump_value(value: Any) -> str:
    if isinstance(value, str):
        return f'"{value}"'
    if isinstance(value, bool):
        return str(value).lower()
    if isinstance(value, list):
        items = [_dump_value(item) for item in value]
        return f"[{', '.join(items)}]"
    return str(value)
