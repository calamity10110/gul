"""
GUL DNS
DNS resolver wrapper.

Status: ✅ Implemented
Priority: Medium
"""

import socket
from typing import List, Optional

__version__ = "0.1.0"
__all__ = ['resolve', 'resolve_ipv4', 'resolve_ipv6', 'reverse_lookup']

def resolve_ipv4(hostname: str) -> List[str]:
    """Resolve hostname to IPv4 addresses"""
    try:
        results = socket.getaddrinfo(hostname, None, socket.AF_INET)
        return list(set(r[4][0] for r in results))
    except socket.gaierror:
        return []

def resolve_ipv6(hostname: str) -> List[str]:
    """Resolve hostname to IPv6 addresses"""
    try:
        results = socket.getaddrinfo(hostname, None, socket.AF_INET6)
        return list(set(r[4][0] for r in results))
    except socket.gaierror:
        return []

def resolve(hostname: str) -> List[str]:
    """Resolve hostname to all IPs"""
    return resolve_ipv4(hostname) + resolve_ipv6(hostname)

def reverse_lookup(ip: str) -> Optional[str]:
    """Reverse DNS lookup"""
    try:
        return socket.gethostbyaddr(ip)[0]
    except socket.herror:
        return None
