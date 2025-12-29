"""
GUL Chemistry
Periodic table and mass calculations.

Status: ✅ Implemented
Priority: Low
"""

__version__ = "0.1.0"
__all__ = ['ELEMENTS', 'molar_mass']

ELEMENTS = {
    "H": 1.008, "He": 4.003, "Li": 6.941, "Be": 9.012,
    "B": 10.811, "C": 12.011, "N": 14.007, "O": 15.999,
    "F": 18.998, "Ne": 20.180, "Na": 22.990, "Mg": 24.305,
    "Al": 26.982, "Si": 28.086, "P": 30.974, "S": 32.065,
    "Cl": 35.453, "K": 39.098, "Ar": 39.948, "Ca": 40.078,
    "Fe": 55.845, "Cu": 63.546, "Zn": 65.38, "Au": 196.97,
    "Ag": 107.87, "Pt": 195.08, "Hg": 200.59
}

def molar_mass(formula: str) -> float:
    """Calculate molar mass (simplified parser)"""
    import re
    tokens = re.findall(r'([A-Z][a-z]?)(\d*)', formula)
    mass = 0.0
    for el, count in tokens:
        if el in ELEMENTS:
            n = int(count) if count else 1
            mass += ELEMENTS[el] * n
    return mass
