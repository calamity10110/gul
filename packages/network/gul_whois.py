"""
GUL Whois
Whois client wrapper.

Status: ✅ Implemented
Priority: Low
"""

import socket
from typing import Dict, Any, Optional

__version__ = "0.1.0"
__all__ = ['lookup', 'whois']

# Simple whois implementation without external deps
def lookup(domain: str) -> str:
    """
    Perform a whois lookup
    
    Example:
        info = gul_whois.lookup("example.com")
    """
    # Pick a generic whois server
    server = "whois.iana.org"
    if domain.endswith(".com"): server = "whois.verisign-grs.com"
    elif domain.endswith(".org"): server = "whois.pir.org"
    elif domain.endswith(".net"): server = "whois.verisign-grs.com"
    elif domain.endswith(".io"): server = "whois.nic.io"
    
    # Connect
    s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        s.connect((server, 43))
        s.send(f"{domain}\r\n".encode())
        
        response = b""
        while True:
            data = s.recv(4096)
            if not data: break
            response += data
            
        return response.decode('utf-8', errors='ignore')
    finally:
        s.close()

def whois(domain: str) -> str:
    return lookup(domain)
