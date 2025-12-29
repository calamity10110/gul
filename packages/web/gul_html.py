"""
GUL HTML Builder
HTML generation library.

Status: ✅ Implemented
Priority: Medium
"""

from typing import List, Dict, Optional, Union, Any

__version__ = "0.1.0"
__all__ = ['HTML', 'tag', 'div', 'span', 'a', 'p', 'h1']

class Element:
    """HTML element"""
    
    def __init__(self, tag: str, *children, **attrs):
        self.tag = tag
        self.children = children
        self.attrs = attrs
        self._void_elements = {
            'area', 'base', 'br', 'col', 'embed', 'hr', 'img', 'input',
            'link', 'meta', 'param', 'source', 'track', 'wbr'
        }
    
    def __str__(self) -> str:
        return self.render()
    
    def render(self) -> str:
        """Render to HTML string"""
        props = self._format_attrs()
        opening = f"<{self.tag}{props}>"
        
        if self.tag in self._void_elements:
            return opening
        
        content = "".join(str(c) for c in self.children)
        return f"{opening}{content}</{self.tag}>"
    
    def _format_attrs(self) -> str:
        """Format attributes"""
        if not self.attrs:
            return ""
        
        parts = []
        for key, value in self.attrs.items():
            # Handle special cases: class_, for_, id_
            key = key.rstrip('_')
            
            if value is True:
                parts.append(f" {key}")
            elif value is False:
                continue
            else:
                value = str(value).replace('"', '&quot;')
                parts.append(f' {key}="{value}"')
        
        return "".join(parts)

class HTML:
    """
    HTML builder
    
    Example:
        html = HTML.div(
            HTML.h1("Hello World", class_="title"),
            HTML.p("This is GUL HTML"),
            HTML.a("Click me", href="/link")
        )
        print(html.render())
    """
    
    @staticmethod
    def tag(name: str, *children, **attrs) -> Element:
        return Element(name, *children, **attrs)
    
    @staticmethod
    def div(*children, **attrs) -> Element:
        return Element("div", *children, **attrs)
    
    @staticmethod
    def span(*children, **attrs) -> Element:
        return Element("span", *children, **attrs)
    
    @staticmethod
    def p(*children, **attrs) -> Element:
        return Element("p", *children, **attrs)
    
    @staticmethod
    def a(*children, **attrs) -> Element:
        return Element("a", *children, **attrs)
    
    @staticmethod
    def h1(*children, **attrs) -> Element:
        return Element("h1", *children, **attrs)
    
    @staticmethod
    def h2(*children, **attrs) -> Element:
        return Element("h2", *children, **attrs)
    
    @staticmethod
    def ul(*children, **attrs) -> Element:
        return Element("ul", *children, **attrs)
    
    @staticmethod
    def li(*children, **attrs) -> Element:
        return Element("li", *children, **attrs)
    
    @staticmethod
    def img(**attrs) -> Element:
        return Element("img", **attrs)
    
    @staticmethod
    def input(**attrs) -> Element:
        return Element("input", **attrs)
    
    @staticmethod
    def form(*children, **attrs) -> Element:
        return Element("form", *children, **attrs)
    
    @staticmethod
    def button(*children, **attrs) -> Element:
        return Element("button", *children, **attrs)

# Short aliases
tag = HTML.tag
div = HTML.div
span = HTML.span
a = HTML.a
p = HTML.p
h1 = HTML.h1
