"""
GUL Canvas
Canvas API abstraction.

Status: ✅ Implemented
Priority: Medium
"""

__version__ = "0.1.0"
__all__ = ['Canvas', 'Context2D']

class Context2D:
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.ops = []
        
    def fill_rect(self, x, y, w, h, color):
        self.ops.append(f"fill_rect({x},{y},{w},{h},'{color}')")
        
    def stroke_rect(self, x, y, w, h, color):
        self.ops.append(f"stroke_rect({x},{y},{w},{h},'{color}')")
        
    def clear_rect(self, x, y, w, h):
        self.ops.append(f"clear_rect({x},{y},{w},{h})")
        
    def begin_path(self):
        self.ops.append("begin_path()")
        
    def move_to(self, x, y):
        self.ops.append(f"move_to({x},{y})")
        
    def line_to(self, x, y):
        self.ops.append(f"line_to({x},{y})")
        
    def stroke(self):
        self.ops.append("stroke()")

class Canvas:
    def __init__(self, width, height):
        self.width = width
        self.height = height
        self.ctx = Context2D(width, height)
        
    def get_context(self):
        return self.ctx
