"""
GUL IP
IP Address manipulation and utilities.

Status: ✅ Implemented
Priority: Medium
"""

import ipaddress
from typing import Union, List

__version__ = "0.1.0"
__all__ = ['validate', 'is_private', 'is_loopback', 'cidr_to_ips', 'IPv4', 'IPv6']

IPv4 = ipaddress.IPv4Address
IPv6 = ipaddress.IPv6Address

def validate(ip: str) -> bool:
    """Check if string is valid IP"""
    try:
        ipaddress.ip_address(ip)
        return True
    except ValueError:
        return False

def is_private(ip: str) -> bool:
    """Check if IP is private"""
    try:
        return ipaddress.ip_address(ip).is_private
    except ValueError:
        return False

def is_loopback(ip: str) -> bool:
    """Check if IP is loopback"""
    try:
        return ipaddress.ip_address(ip).is_loopback
    except ValueError:
        return False

def cidr_to_ips(cidr: str) -> List[str]:
    """Expand CIDR to list of IPs"""
    try:
        net = ipaddress.ip_network(cidr, strict=False)
        # Limit to reasonable size for safety
        if net.num_addresses > 1024:
            raise ValueError("CIDR too large to expand")
        return [str(ip) for ip in net]
    except ValueError:
        return []
