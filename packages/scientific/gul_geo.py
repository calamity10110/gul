"""
GUL Geo
Geolocation utilities.

Status: ✅ Implemented
Priority: Low
"""

import math

__version__ = "0.1.0"
__all__ = ['distance', 'Point']

class Point:
    def __init__(self, lat, lon):
        self.lat = lat
        self.lon = lon

def distance(p1: Point, p2: Point) -> float:
    """Haversine distance in km"""
    R = 6371 # Earth radius km
    dlat = math.radians(p2.lat - p1.lat)
    dlon = math.radians(p2.lon - p1.lon)
    a = math.sin(dlat/2)**2 + math.cos(math.radians(p1.lat)) * math.cos(math.radians(p2.lat)) * math.sin(dlon/2)**2
    c = 2 * math.atan2(math.sqrt(a), math.sqrt(1-a))
    return R * c
