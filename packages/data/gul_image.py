"""
GUL Image
Basic Image processing (BMP/PPM support without heavy deps).

Status: ✅ Implemented
Priority: Low
"""

from typing import Tuple, List, Union
import struct

__version__ = "0.1.0"
__all__ = ['Image', 'Color']

Color = Tuple[int, int, int]

class Image:
    """
    Simple Image Processor (Pure Python)
    Supports creating and saving generic RGB images to BMP/PPM.
    """
    
    def __init__(self, width: int, height: int, color: Color = (0, 0, 0)):
        self.width = width
        self.height = height
        self.pixels = [color] * (width * height)
        
    def put_pixel(self, x: int, y: int, color: Color):
        if 0 <= x < self.width and 0 <= y < self.height:
            self.pixels[y * self.width + x] = color
            
    def get_pixel(self, x: int, y: int) -> Color:
        if 0 <= x < self.width and 0 <= y < self.height:
            return self.pixels[y * self.width + x]
        return (0, 0, 0)
        
    def fill(self, color: Color):
        self.pixels = [color] * (self.width * self.height)
        
    def save(self, filename: str):
        if filename.endswith(".ppm"):
            self._save_ppm(filename)
        elif filename.endswith(".bmp"):
            self._save_bmp(filename)
        else:
            raise ValueError("Unsupported format. Use .ppm or .bmp")
            
    def _save_ppm(self, filename: str):
        with open(filename, 'w') as f:
            f.write(f"P3\n{self.width} {self.height}\n255\n")
            for y in range(self.height):
                for x in range(self.width):
                    r, g, b = self.get_pixel(x, y)
                    f.write(f"{r} {g} {b} ")
                f.write("\n")
                
    def _save_bmp(self, filename: str):
        # Basic BMP Header
        file_size = 54 + 3 * self.width * self.height
        
        with open(filename, 'wb') as f:
            # Bitmap File Header (14 bytes)
            f.write(b'BM')
            f.write(struct.pack('<I', file_size))
            f.write(b'\x00\x00')
            f.write(b'\x00\x00')
            f.write(b'\x36\x00\x00\x00') # Offset to pixel data
            
            # DIB Header (40 bytes)
            f.write(b'\x28\x00\x00\x00') # Header size
            f.write(struct.pack('<I', self.width))
            f.write(struct.pack('<I', self.height))
            f.write(b'\x01\x00') # Planes
            f.write(b'\x18\x00') # Bits per pixel (24)
            f.write(b'\x00\x00\x00\x00') # Compression
            f.write(struct.pack('<I', 3 * self.width * self.height)) # Image size
            f.write(b'\x00\x00\x00\x00') # X PPM
            f.write(b'\x00\x00\x00\x00') # Y PPM
            f.write(b'\x00\x00\x00\x00') # Colors used
            f.write(b'\x00\x00\x00\x00') # Import colors
            
            # Pixel Data (Bottom-up, BGR)
            padding = (4 - (self.width * 3) % 4) % 4
            for y in range(self.height - 1, -1, -1):
                for x in range(self.width):
                    r, g, b = self.get_pixel(x, y)
                    f.write(struct.pack('BBB', b, g, r))
                f.write(b'\x00' * padding)

def new(width: int, height: int, color: Color = (255, 255, 255)) -> Image:
    return Image(width, height, color)
