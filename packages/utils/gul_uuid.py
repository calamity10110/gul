"""
GUL UUID
UUID generation (v4, v7).

Status: ✅ Implemented
Priority: Medium
"""

import os
import time
import struct
import binascii

__version__ = "0.1.0"
__all__ = ['uuid4', 'uuid7', 'UUID']

class UUID:
    """UUID wrapper"""
    
    def __init__(self, value: str):
        self.value = value
    
    def __str__(self) -> str:
        return self.value
    
    def __repr__(self) -> str:
        return f"UUID('{self.value}')"

def uuid4() -> str:
    """
    Generate UUID v4 (Random)
    Format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
    """
    rand = os.urandom(16)
    
    # Set version (4) and variant
    # hex string manipulation is easier in pure python
    hex_str = binascii.hexlify(rand).decode()
    chars = list(hex_str)
    
    # Version 4
    chars[12] = '4'
    
    # Variant (8, 9, a, or b)
    chars[16] = hex(8 | (int(chars[16], 16) & 3))[2:]
    
    uuid = "".join(chars)
    return f"{uuid[:8]}-{uuid[8:12]}-{uuid[12:16]}-{uuid[16:20]}-{uuid[20:]}"

def uuid7() -> str:
    """
    Generate UUID v7 (Time-ordered)
    Format: unix_ts_ms-ver-rand
    """
    # Current timestamp in ms
    ts = int(time.time() * 1000)
    
    # 48 bits for timestamp
    ts_bytes = struct.pack(">Q", ts)[2:]
    
    # 74 bits of random
    rand = os.urandom(10)
    
    # Construct bytes
    uuid_bytes = bytearray(16)
    uuid_bytes[0:6] = ts_bytes
    uuid_bytes[6:16] = rand
    
    # Version 7
    uuid_bytes[6] = (uuid_bytes[6] & 0x0f) | 0x70
    
    # Variant
    uuid_bytes[8] = (uuid_bytes[8] & 0x3f) | 0x80
    
    hex_str = binascii.hexlify(uuid_bytes).decode()
    return f"{hex_str[:8]}-{hex_str[8:12]}-{hex_str[12:16]}-{hex_str[16:20]}-{hex_str[20:]}"
