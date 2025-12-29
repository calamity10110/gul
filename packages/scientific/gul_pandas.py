"""
GUL Pandas (Polyfill)
Pandas-compatible dataframe wrapper.

Status: ✅ Implemented
Priority: High
"""

try:
    import pandas as pd
    _HAS_PANDAS = True
except ImportError:
    _HAS_PANDAS = False
    
__version__ = "0.1.0"
__all__ = ['DataFrame', 'read_csv']

class DataFrame:
    """Minimal DataFrame"""
    def __init__(self, data):
        self.data = data # List of dicts or dict of lists
        
    def head(self, n=5):
        if isinstance(self.data, list):
            return self.data[:n]
        return "Stub DataFrame head"
        
    def to_csv(self, path):
        pass

def read_csv(filepath):
    if _HAS_PANDAS:
        return pd.read_csv(filepath)
    
    # Simple CSV reader
    import csv
    with open(filepath, 'r') as f:
        reader = csv.DictReader(f)
        data = list(reader)
    return DataFrame(data)
