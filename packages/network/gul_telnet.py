"""
GUL Telnet
Telnet Client wrapper.

Status: ✅ Implemented
Priority: Low
"""

import telnetlib
from typing import Optional

__version__ = "0.1.0"
__all__ = ['TelnetClient', 'connect']

class TelnetClient:
    """
    Telnet Client
    
    Example:
        with TelnetClient("host", 23) as tn:
            tn.write("command\n")
            print(tn.read_all())
    """
    
    def __init__(self, host: str, port: int = 23, timeout: int = 10):
        self.host = host
        self.port = port
        self.timeout = timeout
        self.tn: Optional[telnetlib.Telnet] = None
        
    def __enter__(self):
        self.connect()
        return self
        
    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()
        
    def connect(self):
        self.tn = telnetlib.Telnet(self.host, self.port, self.timeout)
        
    def read_until(self, expected: bytes, timeout: Optional[int] = None) -> bytes:
        return self.tn.read_until(expected, timeout)
        
    def write(self, text: str):
        if isinstance(text, str):
            text = text.encode('ascii')
        self.tn.write(text)
        
    def read_all(self) -> bytes:
        return self.tn.read_all()
        
    def close(self):
        if self.tn:
            self.tn.close()

def connect(host: str, port: int = 23) -> TelnetClient:
    client = TelnetClient(host, port)
    client.connect()
    return client
