"""
GUL Physics
Physics constants and units.

Status: ✅ Implemented
Priority: Low
"""

__version__ = "0.1.0"
__all__ = ['c', 'g', 'h', 'G', 'e', 'Units']

# Constants (SI)
c = 299792458 # Speed of light
g = 9.80665   # Gravity
h = 6.62607015e-34 # Planck
G = 6.67430e-11 # Gravitational
e = 1.602176634e-19 # Elementary charge

class Units:
    @staticmethod
    def km_to_m(km): return km * 1000
    @staticmethod
    def m_to_km(m): return m / 1000
    @staticmethod
    def hr_to_sec(hr): return hr * 3600
    @staticmethod
    def c_to_k(c): return c + 273.15
    @staticmethod
    def f_to_c(f): return (f - 32) * 5/9
