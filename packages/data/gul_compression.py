"""
GUL Compression
Data compression utilities.

Status: ✅ Implemented
Priority: Low
"""

from typing import Union
import zlib
import gzip
import bz2

__version__ = "0.1.0"
__all__ = ['compress', 'decompress', 'Compressor']

class Compressor:
    """
    Compression utilities
    
    Example:
        comp = Compressor()
        
        data = b"Hello world! " * 1000
        
        # gzip
        compressed = comp.compress(data, method="gzip")
        restored = comp.decompress(compressed, method="gzip")
        
        # zlib  
        compressed = comp.compress(data, method="zlib")
        
        # bz2
        compressed = comp.compress(data, method="bz2")
    """
    
    def compress(self, data: bytes, method: str = "gzip", level: int = 6) -> bytes:
        """Compress data"""
        if method == "gzip":
            return gzip.compress(data, compresslevel=level)
        elif method == "zlib":
            return zlib.compress(data, level=level)
        elif method == "bz2":
            return bz2.compress(data, compresslevel=level)
        else:
            raise ValueError(f"Unknown compression method: {method}")
    
    def decompress(self, data: bytes, method: str = "gzip") -> bytes:
        """Decompress data"""
        if method == "gzip":
            return gzip.decompress(data)
        elif method == "zlib":
            return zlib.decompress(data)
        elif method == "bz2":
            return bz2.decompress(data)
        else:
            raise ValueError(f"Unknown compression method: {method}")
    
    def compress_string(self, text: str, method: str = "gzip", level: int = 6) -> bytes:
        """Compress string"""
        return self.compress(text.encode(), method, level)
    
    def decompress_string(self, data: bytes, method: str = "gzip") -> str:
        """Decompress to string"""
        return self.decompress(data, method).decode()

_compressor = Compressor()

def compress(data: Union[bytes, str], method: str = "gzip") -> bytes:
    """Quick compression"""
    if isinstance(data, str):
        return _compressor.compress_string(data, method)
    return _compressor.compress(data, method)

def decompress(data: bytes, method: str = "gzip", as_string: bool = False) -> Union[bytes, str]:
    """Quick decompression"""
    if as_string:
        return _compressor.decompress_string(data, method)
    return _compressor.decompress(data, method)
