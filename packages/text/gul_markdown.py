"""
GUL Markdown
Markdown processing and rendering.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Dict, Optional, List
import re

__version__ = "0.1.0"
__all__ = ['Markdown', 'markdown']

class Markdown:
    """
    Simple Markdown processor
    
    Example:
        md = Markdown()
        html = md.render("# Hello World")
    """
    
    def render(self, text: str) -> str:
        """Render markdown to HTML"""
        lines = text.split('\n')
        html_lines = []
        
        in_code_block = False
        
        for line in lines:
            line = line.rstrip()
            
            # Code blocks
            if line.startswith('```'):
                if in_code_block:
                    html_lines.append('</pre>')
                    in_code_block = False
                else:
                    html_lines.append('<pre>')
                    in_code_block = True
                continue
            
            if in_code_block:
                html_lines.append(self._escape(line))
                continue
            
            # Headers
            if line.startswith('#'):
                level = len(line.split()[0])
                content = line[level:].strip()
                html_lines.append(f'<h{level}>{self._process_inline(content)}</h{level}>')
                continue
            
            # Lists
            if line.strip().startswith('- '):
                content = line.strip()[2:]
                html_lines.append(f'<li>{self._process_inline(content)}</li>')
                continue
            
            # Paragraphs
            if line:
                html_lines.append(f'<p>{self._process_inline(line)}</p>')
        
        return '\n'.join(html_lines)
    
    def _process_inline(self, text: str) -> str:
        """Process inline formatting (bold, italic, links)"""
        # Bold
        text = re.sub(r'\*\*(.*?)\*\*', r'<strong>\1</strong>', text)
        
        # Italic
        text = re.sub(r'\*(.*?)\*', r'<em>\1</em>', text)
        
        # Code
        text = re.sub(r'`(.*?)`', r'<code>\1</code>', text)
        
        # Links
        text = re.sub(r'\[(.*?)\]\((.*?)\)', r'<a href="\2">\1</a>', text)
        
        return text
    
    def _escape(self, text: str) -> str:
        """Escape HTML characters"""
        return text.replace('&', '&amp;').replace('<', '&lt;').replace('>', '&gt;')

_md = Markdown()

def render(text: str) -> str:
    """Quick render"""
    return _md.render(text)
