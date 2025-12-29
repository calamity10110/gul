"""
GUL Numpy (Polyfill)
Numpy-compatible array wrapper.

Status: ✅ Implemented
Priority: High
"""

try:
    import numpy as np
    _HAS_NUMPY = True
except ImportError:
    _HAS_NUMPY = False
    
__version__ = "0.1.0"
__all__ = ['array', 'mean', 'median', 'std']

class Array:
    """Minimal array class if numpy missing"""
    def __init__(self, data):
        self.data = data
        
    def __repr__(self):
        return f"Array({self.data})"
        
    def mean(self):
        return sum(self.data) / len(self.data)
        
    def tolist(self):
        return self.data

def array(data):
    if _HAS_NUMPY:
        return np.array(data)
    return Array(data)

def mean(a):
    if _HAS_NUMPY:
        return np.mean(a)
    return sum(a) / len(a) if hasattr(a, "__len__") else a.mean()

def median(a):
    if _HAS_NUMPY:
        return np.median(a)
    s = sorted(a)
    l = len(s)
    if l % 2 == 0:
        return (s[l//2-1] + s[l//2]) / 2
    return s[l//2]

def std(a):
    if _HAS_NUMPY:
        return np.std(a)
    m = mean(a)
    return (sum((x - m)**2 for x in a) / len(a))**0.5
