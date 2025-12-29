"""
GUL XML Parser
XML parsing and generation.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Dict, List, Optional, Any
from dataclasses import dataclass

__version__ = "0.1.0"
__all__ = ['XMLParser', 'XMLElement', 'parse_xml', 'to_xml']

@dataclass
class XMLElement:
    """XML element"""
    tag: str
    attributes: Dict[str, str]
    text: Optional[str] = None
    children: List['XMLElement'] = None
    
    def __post_init__(self):
        if self.children is None:
            self.children = []
    
    def find(self, tag: str) -> Optional['XMLElement']:
        """Find first child with tag"""
        for child in self.children:
            if child.tag == tag:
                return child
        return None
    
    def find_all(self, tag: str) -> List['XMLElement']:
        """Find all children with tag"""
        return [c for c in self.children if c.tag == tag]
    
    def get_text(self) -> str:
        """Get element text"""
        return self.text or ''
    
    def to_dict(self) -> Dict:
        """Convert to dictionary"""
        result = {'tag': self.tag}
        if self.attributes:
            result['attributes'] = self.attributes
        if self.text:
            result['text'] = self.text
        if self.children:
            result['children'] = [c.to_dict() for c in self.children]
        return result

class XMLParser:
    """XML parser"""
    
    def parse(self, content: str) -> XMLElement:
        """Parse XML content"""
        content = content.strip()
        
        # Remove XML declaration
        if content.startswith('<?xml'):
            content = content[content.index('?>') + 2:].strip()
        
        return self._parse_element(content)[0]
    
    def _parse_element(self, content: str) -> tuple:
        """Parse single element"""
        if not content.startswith('<'):
            return None, content
        
        # Get tag name and attributes
        end = content.index('>')
        tag_content = content[1:end]
        
        # Self-closing tag
        if tag_content.endswith('/'):
            tag_parts = tag_content[:-1].strip().split(None, 1)
            tag = tag_parts[0]
            attrs = self._parse_attributes(tag_parts[1] if len(tag_parts) > 1 else '')
            return XMLElement(tag, attrs), content[end + 1:]
        
        # Parse tag and attributes
        tag_parts = tag_content.split(None, 1)
        tag = tag_parts[0]
        attrs = self._parse_attributes(tag_parts[1] if len(tag_parts) > 1 else '')
        
        remaining = content[end + 1:]
        
        # Parse content until closing tag
        children = []
        text_parts = []
        close_tag = f'</{tag}>'
        
        while remaining and not remaining.startswith(close_tag):
            if remaining.startswith('<'):
                child, remaining = self._parse_element(remaining)
                if child:
                    children.append(child)
            else:
                # Text content
                next_tag = remaining.find('<')
                if next_tag == -1:
                    text_parts.append(remaining)
                    break
                text_parts.append(remaining[:next_tag])
                remaining = remaining[next_tag:]
        
        # Skip closing tag
        if remaining.startswith(close_tag):
            remaining = remaining[len(close_tag):]
        
        text = ''.join(text_parts).strip() or None
        
        return XMLElement(tag, attrs, text, children), remaining
    
    def _parse_attributes(self, attr_str: str) -> Dict[str, str]:
        """Parse attributes"""
        attrs = {}
        if not attr_str:
            return attrs
        
        i = 0
        while i < len(attr_str):
            # Skip whitespace
            while i < len(attr_str) and attr_str[i].isspace():
                i += 1
            
            if i >= len(attr_str):
                break
            
            # Get attribute name
            name_start = i
            while i < len(attr_str) and attr_str[i] not in '= \t\n':
                i += 1
            
            name = attr_str[name_start:i]
            
            # Skip = and whitespace
            while i < len(attr_str) and attr_str[i] in '= \t\n':
                i += 1
            
            # Get value
            if i < len(attr_str):
                quote = attr_str[i]
                if quote in '"\'':
                    i += 1
                    value_start = i
                    while i < len(attr_str) and attr_str[i] != quote:
                        i += 1
                    value = attr_str[value_start:i]
                    i += 1
                else:
                    value_start = i
                    while i < len(attr_str) and not attr_str[i].isspace():
                        i += 1
                    value = attr_str[value_start:i]
                
                attrs[name] = value
        
        return attrs
    
    def generate(self, element: XMLElement, indent: int = 0) -> str:
        """Generate XML from element"""
        prefix = '  ' * indent
        
        # Start tag with attributes
        attrs = ' '.join(f'{k}="{v}"' for k, v in element.attributes.items())
        start = f'{prefix}<{element.tag}'
        if attrs:
            start += f' {attrs}'
        
        # Self-closing if no content
        if not element.text and not element.children:
            return f'{start}/>'
        
        start += '>'
        
        # Text content
        if element.text and not element.children:
            return f'{start}{element.text}</{element.tag}>'
        
        # Children
        lines = [start]
        if element.text:
            lines.append(f'{prefix}  {element.text}')
        
        for child in element.children:
            lines.append(self.generate(child, indent + 1))
        
        lines.append(f'{prefix}</{element.tag}>')
        
        return '\n'.join(lines)

def parse_xml(content: str) -> XMLElement:
    """Quick XML parse"""
    parser = XMLParser()
    return parser.parse(content)

def to_xml(element: XMLElement) -> str:
    """Quick XML generation"""
    parser = XMLParser()
    return parser.generate(element)
