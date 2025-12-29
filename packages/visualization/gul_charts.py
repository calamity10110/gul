"""
GUL Charts
SVG Chart Generator.

Status: ✅ Implemented
Priority: Medium
"""

from .gul_svg import SVG
from typing import List, Tuple

__version__ = "0.1.0"
__all__ = ['BarChart', 'LineChart']

class BarChart:
    def __init__(self, width=600, height=400):
        self.svg = SVG(width, height)
        self.width = width
        self.height = height
        self.padding = 40
        
    def plot(self, data: List[float], labels: List[str] = None):
        if not data: return
        
        max_val = max(data)
        count = len(data)
        bar_width = (self.width - 2 * self.padding) / count * 0.8
        spacing = (self.width - 2 * self.padding) / count
        
        # Axis
        self.svg.line(self.padding, self.height - self.padding, self.width - self.padding, self.height - self.padding) # X
        self.svg.line(self.padding, self.padding, self.padding, self.height - self.padding) # Y
        
        for i, val in enumerate(data):
            h = (val / max_val) * (self.height - 2 * self.padding)
            x = self.padding + i * spacing + spacing * 0.1
            y = self.height - self.padding - h
            
            self.svg.rect(x, y, bar_width, h, fill="#4CAF50")
            if labels and i < len(labels):
                self.svg.text(x, self.height - self.padding + 15, labels[i], font_size=10)
                
    def save(self, filename: str):
        self.svg.save(filename)

class LineChart:
    def __init__(self, width=600, height=400):
        self.svg = SVG(width, height)
        self.width = width
        self.height = height
        self.padding = 40
        
    def plot(self, data: List[float]):
        if not data: return
        
        max_val = max(data)
        min_val = min(data)
        range_val = max_val - min_val if max_val != min_val else 1
        
        step_x = (self.width - 2 * self.padding) / (len(data) - 1)
        
        points = []
        for i, val in enumerate(data):
            x = self.padding + i * step_x
            y = self.height - self.padding - ((val - min_val) / range_val) * (self.height - 2 * self.padding)
            points.append((x, y))
            self.svg.circle(x, y, 3, fill="red")
            
        for i in range(len(points) - 1):
            self.svg.line(points[i][0], points[i][1], points[i+1][0], points[i+1][1], stroke="blue", stroke_width=2)
            
    def save(self, filename: str):
        self.svg.save(filename)
