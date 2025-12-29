"""
GUL URL Parser
URL parsing and building.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Dict, Optional
from urllib.parse import urlparse, parse_qs, urlencode, urlunparse

__version__ = "0.1.0"
__all__ = ['URL', 'parse_url']

class URL:
    """
    URL parser and builder
    
    Example:
        url = URL("https://example.com:8080/path?key=value#fragment")
        
        print(url.scheme)  # "https"
        print(url.host)    # "example.com"  
        print(url.port)    # 8080
        print(url.path)    # "/path"
        print(url.query)   # {"key": ["value"]}
        
        # Build URL
        url = URL(scheme="https", host="api.example.com")
        url.set_path("/users/123")
        url.add_query("format", "json")
        print(url.build())  # "https://api.example.com/users/123?format=json"
    """
    
    def __init__(self, url: Optional[str] = None, **kwargs):
        if url:
            parsed = urlparse(url)
            self.scheme = parsed.scheme
            self.netloc = parsed.netloc
            self.path = parsed.path
            self.params = parsed.params
            self.query_string = parsed.query
            self.fragment = parsed.fragment
            self.query = parse_qs(parsed.query)
            
            # Parse host and port
            if ':' in self.netloc:
                self.host, port_str = self.netloc.rsplit(':', 1)
                self.port = int(port_str) if port_str.isdigit() else None
            else:
                self.host = self.netloc
                self.port = None
        else:
            self.scheme = kwargs.get('scheme', 'https')
            self.host = kwargs.get('host', '')
            self.port = kwargs.get('port')
            self.path = kwargs.get('path', '/')
            self.params = kwargs.get('params', '')
            self.query = kwargs.get('query', {})
            self.query_string = ''
            self.fragment = kwargs.get('fragment', '')
            self.netloc = self._build_netloc()
    
    def _build_netloc(self) -> str:
        """Build netloc from host and port"""
        if self.port:
            return f"{self.host}:{self.port}"
        return self.host
    
    def set_path(self, path: str):
        """Set path"""
        self.path = path if path.startswith('/') else f'/{path}'
    
    def add_query(self, key: str, value: str):
        """Add query parameter"""
        if key not in self.query:
            self.query[key] = []
        self.query[key].append(value)
    
    def set_query(self, key: str, value: str):
        """Set query parameter (replaces existing)"""
        self.query[key] = [value]
    
    def remove_query(self, key: str):
        """Remove query parameter"""
        if key in self.query:
            del self.query[key]
    
    def build(self) -> str:
        """Build URL string"""
        self.netloc = self._build_netloc()
        
        # Build query string
        if self.query:
            self.query_string = urlencode(
                [(k, v) for k, values in self.query.items() for v in values]
            )
        
        return urlunparse((
            self.scheme,
            self.netloc,
            self.path,
            self.params,
            self.query_string,
            self.fragment
        ))
    
    def __str__(self) -> str:
        return self.build()

def parse_url(url: str) -> URL:
    """Quick URL parsing"""
    return URL(url)
