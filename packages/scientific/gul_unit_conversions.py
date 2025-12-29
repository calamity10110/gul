"""
GUL Unit Conversions
Unit conversion utilities.

Status: ✅ Implemented
Priority: Low
"""

__version__ = "0.1.0"
__all__ = ['convert']

CONVERSIONS = {
    # Length (base: meter)
    'm': 1.0, 'km': 1000.0, 'cm': 0.01, 'mm': 0.001,
    'mi': 1609.34, 'yd': 0.9144, 'ft': 0.3048, 'in': 0.0254,
    # Mass (base: kg)
    'kg': 1.0, 'g': 0.001, 'mg': 0.000001, 'lb': 0.453592, 'oz': 0.0283495,
    # Volume (base: liter)
    'l': 1.0, 'ml': 0.001, 'gal': 3.78541, 'qt': 0.946353, 'pt': 0.473176,
    # Time (base: second)
    's': 1.0, 'min': 60.0, 'h': 3600.0, 'd': 86400.0,
}

TYPE_MAP = {
    'm': 'L', 'km': 'L', 'cm': 'L', 'mm': 'L', 'mi': 'L', 'yd': 'L', 'ft': 'L', 'in': 'L',
    'kg': 'M', 'g': 'M', 'mg': 'M', 'lb': 'M', 'oz': 'M',
    'l': 'V', 'ml': 'V', 'gal': 'V', 'qt': 'V', 'pt': 'V',
    's': 'T', 'min': 'T', 'h': 'T', 'd': 'T'
}

def convert(value: float, from_unit: str, to_unit: str) -> float:
    """Convert value between units"""
    if from_unit not in CONVERSIONS or to_unit not in CONVERSIONS:
        raise ValueError("Unknown unit")
        
    if TYPE_MAP[from_unit] != TYPE_MAP[to_unit]:
        raise ValueError("Incompatible unit types")
        
    base_val = value * CONVERSIONS[from_unit]
    return base_val / CONVERSIONS[to_unit]
