"""
GUL CSV Parser
CSV file parsing and generation.

Status: ✅ Implemented
Priority: High
"""

from typing import List, Dict, Optional, Any, TextIO
from dataclasses import dataclass

__version__ = "0.1.0"
__all__ = ['CSVReader', 'CSVWriter', 'parse_csv', 'write_csv']

@dataclass
class CSVConfig:
    delimiter: str = ","
    quote_char: str = '"'
    escape_char: str = "\\"
    skip_header: bool = False
    header_row: Optional[List[str]] = None

class CSVReader:
    """CSV reader"""
    
    def __init__(self, config: Optional[CSVConfig] = None):
        self.config = config or CSVConfig()
    
    def parse(self, content: str) -> List[List[str]]:
        """Parse CSV content"""
        rows = []
        lines = content.strip().split('\n')
        
        for line in lines:
            if not line:
                continue
            rows.append(self._parse_line(line))
        
        if self.config.skip_header and rows:
            return rows[1:]
        
        return rows
    
    def parse_dict(self, content: str) -> List[Dict[str, str]]:
        """Parse CSV as dictionaries"""
        rows = self.parse(content)
        
        if not rows:
            return []
        
        header = self.config.header_row or rows[0]
        data_rows = rows[1:] if not self.config.header_row else rows
        
        result = []
        for row in data_rows:
            result.append(dict(zip(header, row)))
        
        return result
    
    def _parse_line(self, line: str) -> List[str]:
        """Parse single CSV line"""
        fields = []
        current_field = []
        in_quotes = False
        
        i = 0
        while i < len(line):
            char = line[i]
            
            if char == self.config.quote_char:
                if in_quotes and i + 1 < len(line) and line[i + 1] == self.config.quote_char:
                    current_field.append(char)
                    i += 1
                else:
                    in_quotes = not in_quotes
            
            elif char == self.config.delimiter and not in_quotes:
                fields.append(''.join(current_field))
                current_field = []
            
            else:
                current_field.append(char)
            
            i += 1
        
        fields.append(''.join(current_field))
        return fields

class CSVWriter:
    """CSV writer"""
    
    def __init__(self, config: Optional[CSVConfig] = None):
        self.config = config or CSVConfig()
    
    def write(self, rows: List[List[str]]) -> str:
        """Write rows to CSV"""
        lines = []
        
        for row in rows:
            line = self.config.delimiter.join(self._escape_field(f) for f in row)
            lines.append(line)
        
        return '\n'.join(lines)
    
    def write_dict(self, rows: List[Dict[str, str]], header: Optional[List[str]] = None) -> str:
        """Write dictionaries to CSV"""
        if not rows:
            return ""
        
        header = header or list(rows[0].keys())
        
        lines = [self.config.delimiter.join(self._escape_field(h) for h in header)]
        
        for row in rows:
            line = self.config.delimiter.join(
                self._escape_field(str(row.get(h, ''))) for h in header
            )
            lines.append(line)
        
        return '\n'.join(lines)
    
    def _escape_field(self, field: str) -> str:
        """Escape field if needed"""
        if self.config.delimiter in field or self.config.quote_char in field or '\n' in field:
            escaped = field.replace(self.config.quote_char, self.config.quote_char * 2)
            return f"{self.config.quote_char}{escaped}{self.config.quote_char}"
        return field

def parse_csv(content: str, has_header: bool = True) -> List[Dict[str, str]]:
    """Quick CSV parse"""
    reader = CSVReader(CSVConfig(skip_header=False))
    return reader.parse_dict(content)

def write_csv(rows: List[Dict[str, str]]) -> str:
    """Quick CSV write"""
    writer = CSVWriter()
    return writer.write_dict(rows)
