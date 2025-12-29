"""
GUL Raytracer
Simple Raytracer core.

Status: ✅ Implemented
Priority: Low
"""

import math
from .gul_3d import Vec3, dot, normalize

__version__ = "0.1.0"
__all__ = ['Sphere', 'Ray', 'Hit']

class Ray:
    def __init__(self, origin: Vec3, direction: Vec3):
        self.origin = origin
        self.direction = direction

class Hit:
    def __init__(self, t, point, normal):
        self.t = t
        self.point = point
        self.normal = normal

class Sphere:
    def __init__(self, center: Vec3, radius: float, color: tuple):
        self.center = center
        self.radius = radius
        self.color = color
        
    def intersect(self, r: Ray):
        oc = r.origin - self.center
        a = dot(r.direction, r.direction)
        b = 2.0 * dot(oc, r.direction)
        c = dot(oc, oc) - self.radius**2
        discriminant = b*b - 4*a*c
        
        if discriminant > 0:
            root = math.sqrt(discriminant)
            temp = (-b - root) / (2*a)
            if temp > 0:
                p = r.origin + r.direction * temp
                n = (p - self.center).normalize()
                return Hit(temp, p, n)
            temp = (-b + root) / (2*a)
            if temp > 0:
                p = r.origin + r.direction * temp
                n = (p - self.center).normalize()
                return Hit(temp, p, n)
        return None
