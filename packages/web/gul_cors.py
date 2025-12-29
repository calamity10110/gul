"""
GUL CORS
CORS (Cross-Origin Resource Sharing) utilities.

Status: ✅ Implemented
Priority: Medium
"""

from typing import List, Dict, Optional, Union

__version__ = "0.1.0"
__all__ = ['Cors', 'cors_headers']

class Cors:
    """
    CORS Configuration
    
    Example:
        cors = Cors(
            allow_origins=["https://example.com"],
            allow_methods=["GET", "POST"],
            allow_headers=["Authorization"],
            max_age=3600
        )
        
        headers = cors.get_headers(request_origin="https://example.com")
    """
    
    def __init__(
        self,
        allow_origins: Union[List[str], str] = "*",
        allow_methods: List[str] = ["GET", "POST", "PUT", "DELETE", "OPTIONS"],
        allow_headers: List[str] = ["Content-Type", "Authorization"],
        allow_credentials: bool = True,
        expose_headers: Optional[List[str]] = None,
        max_age: int = 600
    ):
        self.allow_origins = allow_origins
        self.allow_methods = allow_methods
        self.allow_headers = allow_headers
        self.allow_credentials = allow_credentials
        self.expose_headers = expose_headers or []
        self.max_age = max_age

    def is_allowed(self, origin: str) -> bool:
        if self.allow_origins == "*":
            return True
        return origin in self.allow_origins

    def get_headers(self, request_origin: Optional[str] = None) -> Dict[str, str]:
        headers = {}
        
        # Access-Control-Allow-Origin
        if self.allow_origins == "*":
            headers["Access-Control-Allow-Origin"] = "*"
        elif request_origin and self.is_allowed(request_origin):
            headers["Access-Control-Allow-Origin"] = request_origin
            headers["Vary"] = "Origin"
            
        # Access-Control-Allow-Methods
        headers["Access-Control-Allow-Methods"] = ", ".join(self.allow_methods)
        
        # Access-Control-Allow-Headers
        headers["Access-Control-Allow-Headers"] = ", ".join(self.allow_headers)
        
        # Access-Control-Allow-Credentials
        if self.allow_credentials:
            headers["Access-Control-Allow-Credentials"] = "true"
            
        # Access-Control-Max-Age
        headers["Access-Control-Max-Age"] = str(self.max_age)
        
        # Access-Control-Expose-Headers
        if self.expose_headers:
            headers["Access-Control-Expose-Headers"] = ", ".join(self.expose_headers)
            
        return headers

def cors_headers(origin: str = "*") -> Dict[str, str]:
    """Quick defaults"""
    return Cors(allow_origins=origin).get_headers(request_origin=origin if origin != "*" else None)
