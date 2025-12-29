"""
GUL SVG
SVG Builder.

Status: ✅ Implemented
Priority: Medium
"""

from typing import List, Optional

__version__ = "0.1.0"
__all__ = ['SVG', 'create_svg']

class SVG:
    def __init__(self, width: int = 800, height: int = 600):
        self.width = width
        self.height = height
        self.elements = []
        
    def add(self, tag: str, **attrs):
        attr_str = " ".join([f'{k.replace("-", "_")}="{v}"' for k, v in attrs.items()])
        self.elements.append(f"<{tag} {attr_str} />")
        
    def rect(self, x, y, width, height, fill="black", stroke="none"):
        self.add("rect", x=x, y=y, width=width, height=height, fill=fill, stroke=stroke)
        
    def circle(self, cx, cy, r, fill="black", stroke="none"):
        self.add("circle", cx=cx, cy=cy, r=r, fill=fill, stroke=stroke)
        
    def line(self, x1, y1, x2, y2, stroke="black", stroke_width=1):
        self.add("line", x1=x1, y1=y1, x2=x2, y2=y2, stroke=stroke, style=f"stroke-width:{stroke_width}")
        
    def text(self, x, y, content, fill="black", font_size=12):
        self.elements.append(f'<text x="{x}" y="{y}" fill="{fill}" font-size="{font_size}">{content}</text>')
        
    def render(self) -> str:
        body = "\n".join(self.elements)
        return f'<svg width="{self.width}" height="{self.height}" xmlns="http://www.w3.org/2000/svg">\n{body}\n</svg>'
        
    def save(self, filename: str):
        with open(filename, "w") as f:
            f.write(self.render())

def create_svg(filename: str, width: int, height: int):
    return SVG(width, height)
