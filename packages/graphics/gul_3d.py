"""
GUL 3D
3D Math & Vectors.

Status: ✅ Implemented
Priority: Medium
"""

import math

__version__ = "0.1.0"
__all__ = ['Vec3', 'dot', 'cross', 'normalize']

class Vec3:
    def __init__(self, x=0, y=0, z=0):
        self.x = x
        self.y = y
        self.z = z
        
    def __add__(self, other):
        return Vec3(self.x + other.x, self.y + other.y, self.z + other.z)
        
    def __sub__(self, other):
        return Vec3(self.x - other.x, self.y - other.y, self.z - other.z)
        
    def __mul__(self, other):
        if isinstance(other, Vec3):
             # Element-wise
             return Vec3(self.x * other.x, self.y * other.y, self.z * other.z)
        return Vec3(self.x * other, self.y * other, self.z * other)
        
    def mag(self):
        return math.sqrt(self.x**2 + self.y**2 + self.z**2)
        
    def normalize(self):
        m = self.mag()
        if m == 0: return Vec3()
        return Vec3(self.x/m, self.y/m, self.z/m)
        
    def __repr__(self):
        return f"Vec3({self.x}, {self.y}, {self.z})"

def dot(v1: Vec3, v2: Vec3) -> float:
    return v1.x * v2.x + v1.y * v2.y + v1.z * v2.z

def cross(v1: Vec3, v2: Vec3) -> Vec3:
    return Vec3(
        v1.y * v2.z - v1.z * v2.y,
        v1.z * v2.x - v1.x * v2.z,
        v1.x * v2.y - v1.y * v2.x
    )

def normalize(v: Vec3) -> Vec3:
    return v.normalize()
