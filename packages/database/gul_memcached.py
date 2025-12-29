"""
GUL Memcached
Memcached client (Text Protocol).

Status: ✅ Implemented
Priority: Medium
"""

import socket
from typing import Any, Optional
import pickle
import struct

__version__ = "0.1.0"
__all__ = ['MemcachedClient', 'Client']

class MemcachedClient:
    """
    Memcached Client (Text Protocol)
    
    Example:
        mc = MemcachedClient("localhost", 11211)
        mc.set("key", "value")
        val = mc.get("key")
    """
    
    def __init__(self, host: str, port: int = 11211):
        self.host = host
        self.port = port
        self.sock = None
        
    def connect(self):
        if not self.sock:
            self.sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            self.sock.connect((self.host, self.port))
            
    def close(self):
        if self.sock:
            self.sock.close()
            self.sock = None
            
    def set(self, key: str, value: Any, flags: int = 0, exptime: int = 0):
        self.connect()
        # Serialize logic could go here, for now assume str
        val_bytes = str(value).encode()
        cmd = f"set {key} {flags} {exptime} {len(val_bytes)}\r\n".encode()
        self.sock.sendall(cmd + val_bytes + b"\r\n")
        
        # Read response
        self.sock.recv(1024) # Expect "STORED"
        
    def get(self, key: str) -> Optional[str]:
        self.connect()
        cmd = f"get {key}\r\n".encode()
        self.sock.sendall(cmd)
        
        # Read response (simplified)
        response = b""
        while True:
            chunk = self.sock.recv(4096)
            response += chunk
            if b"END" in chunk:
                break
                
        # Parse VALUE <key> <flags> <bytes>\r\n<data>\r\nEND
        try:
            parts = response.split(b"\r\n")
            if parts[0].startswith(b"VALUE"):
                header = parts[0].split()
                # length = int(header[3])
                data = parts[1]
                return data.decode()
        except:
            pass
        return None
        
    def delete(self, key: str):
        self.connect()
        cmd = f"delete {key}\r\n".encode()
        self.sock.sendall(cmd)
        self.sock.recv(1024)

Client = MemcachedClient
