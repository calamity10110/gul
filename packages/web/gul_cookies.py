"""
GUL Cookies
HTTP cookie management.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Dict, Optional
from datetime import datetime, timedelta

__version__ = "0.1.0"
__all__ = ['Cookie', 'CookieJar']

class Cookie:
    """HTTP cookie"""
    
    def __init__(
        self,
        name: str,
        value: str,
        domain: Optional[str] = None,
        path: str = "/",
        expires: Optional[datetime] = None,
        max_age: Optional[int] = None,
        secure: bool = False,
        http_only: bool = False,
        same_site: Optional[str] = None
    ):
        self.name = name
        self.value = value
        self.domain = domain
        self.path = path
        self.expires = expires
        self.max_age = max_age
        self.secure = secure
        self.http_only = http_only
        self.same_site = same_site
    
    def to_header(self) -> str:
        """Convert to Set-Cookie header"""
        parts = [f"{self.name}={self.value}"]
        
        if self.domain:
            parts.append(f"Domain={self.domain}")
        if self.path:
            parts.append(f"Path={self.path}")
        if self.expires:
            parts.append(f"Expires={self.expires.strftime('%a, %d %b %Y %H:%M:%S GMT')}")
        if self.max_age is not None:
            parts.append(f"Max-Age={self.max_age}")
        if self.secure:
            parts.append("Secure")
        if self.http_only:
            parts.append("HttpOnly")
        if self.same_site:
            parts.append(f"SameSite={self.same_site}")
        
        return "; ".join(parts)
    
    def is_expired(self) -> bool:
        """Check if cookie is expired"""
        if self.expires and datetime.utcnow() > self.expires:
            return True
        return False

class CookieJar:
    """
    Cookie jar for managing HTTP cookies
    
    Example:
        jar = CookieJar()
        
        # Set cookie
        jar.set("session_id", "abc123", max_age=3600, http_only=True)
        
        # Get cookie
        session = jar.get("session_id")
        
        # Delete cookie
        jar.delete("session_id")
        
        # Generate headers
        headers = jar.get_headers()
    """
    
    def __init__(self):
        self.cookies: Dict[str, Cookie] = {}
    
    def set(
        self,
        name: str,
        value: str,
        **kwargs
    ):
        """Set cookie"""
        cookie = Cookie(name, value, **kwargs)
        self.cookies[name] = cookie
    
    def get(self, name: str) -> Optional[str]:
        """Get cookie value"""
        cookie = self.cookies.get(name)
        if cookie and not cookie.is_expired():
            return cookie.value
        return None
    
    def delete(self, name: str):
        """Delete cookie"""
        if name in self.cookies:
            # Set expired cookie to delete it
            self.cookies[name].expires = datetime.utcnow() - timedelta(days=1)
            self.cookies[name].max_age = 0
    
    def clear(self):
        """Clear all cookies"""
        for name in list(self.cookies.keys()):
            self.delete(name)
    
    def get_headers(self) -> Dict[str, str]:
        """Get Set-Cookie headers"""
        headers = {}
        for i, cookie in enumerate(self.cookies.values()):
            header_key = "Set-Cookie" if i == 0 else f"Set-Cookie-{i}"
            headers[header_key] = cookie.to_header()
        return headers
    
    def parse_cookie_header(self, header: str):
        """Parse Cookie header"""
        for part in header.split(';'):
            part = part.strip()
            if '=' in part:
                name, value = part.split('=', 1)
                self.set(name.strip(), value.strip())
