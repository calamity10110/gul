"""
GUL Security Headers Middleware
Provides security headers for HTTP responses.

Status: ✅ Implemented
Priority: Critical
Phase: 1
"""

from typing import Dict, Optional, Callable, Any
from dataclasses import dataclass

__version__ = "0.1.0"
__all__ = ['SecurityHeaders', 'SecurityConfig', 'secure_headers']

@dataclass
class SecurityConfig:
    """Security headers configuration"""
    
    # Content Security Policy
    csp: Optional[str] = None
    
    # Cross-Origin Resource Sharing
    cors_allow_origin: str = "*"
    cors_allow_methods: str = "GET, POST, PUT, DELETE, OPTIONS"
    cors_allow_headers: str = "Content-Type, Authorization"
    cors_max_age: int = 86400
    
    # HTTP Strict Transport Security
    hsts_max_age: int = 31536000  # 1 year
    hsts_include_subdomains: bool = True
    hsts_preload: bool = False
    
    # Other security headers
    x_frame_options: str = "DENY"
    x_content_type_options: str = "nosniff"
    x_xss_protection: str = "1; mode=block"
    referrer_policy: str = "strict-origin-when-cross-origin"
    permissions_policy: Optional[str] = None
    
    # Feature flags
    enable_csp: bool = True
    enable_cors: bool = True
    enable_hsts: bool = True
    enable_frame_options: bool = True

class SecurityHeaders:
    """
    Security headers middleware for HTTP responses
    
    Provides protection against common web vulnerabilities:
    - XSS (Cross-Site Scripting)
    - Clickjacking
    - MIME type sniffing
    - Protocol downgrade attacks
    
    Example:
        config = SecurityConfig(
            csp="default-src 'self'; script-src 'self' 'unsafe-inline'",
            cors_allow_origin="https://example.com"
        )
        security = SecurityHeaders(config)
        
        # Apply to response
        headers = security.get_headers()
        response.headers.update(headers)
    """
    
    def __init__(self, config: Optional[SecurityConfig] = None):
        self.config = config or SecurityConfig()
    
    def get_headers(self) -> Dict[str,str]:
        """
        Get all security headers as a dictionary
        
        Returns:
            Dictionary of header names to values
        """
        headers = {}
        
        # Content Security Policy
        if self.config.enable_csp and self.config.csp:
            headers['Content-Security-Policy'] = self.config.csp
        
        # CORS Headers
        if self.config.enable_cors:
            headers['Access-Control-Allow-Origin'] = self.config.cors_allow_origin
            headers['Access-Control-Allow-Methods'] = self.config.cors_allow_methods
            headers['Access-Control-Allow-Headers'] = self.config.cors_allow_headers
            headers['Access-Control-Max-Age'] = str(self.config.cors_max_age)
        
        # HSTS
        if self.config.enable_hsts:
            hsts_value = f"max-age={self.config.hsts_max_age}"
            if self.config.hsts_include_subdomains:
                hsts_value += "; includeSubDomains"
            if self.config.hsts_preload:
                hsts_value += "; preload"
            headers['Strict-Transport-Security'] = hsts_value
        
        # Frame Options
        if self.config.enable_frame_options:
            headers['X-Frame-Options'] = self.config.x_frame_options
        
        # Other security headers
        headers['X-Content-Type-Options'] = self.config.x_content_type_options
        headers['X-XSS-Protection'] = self.config.x_xss_protection
        headers['Referrer-Policy'] = self.config.referrer_policy
        
        # Permissions Policy
        if self.config.permissions_policy:
            headers['Permissions-Policy'] = self.config.permissions_policy
        
        return headers
    
    def apply_to_response(self, response: Any) -> Any:
        """
        Apply security headers to a response object
        
        Args:
            response: Response object with headers attribute
        
        Returns:
            Modified response object
        """
        if hasattr(response, 'headers'):
            response.headers.update(self.get_headers())
        return response
    
    def middleware(self, handler: Callable) -> Callable:
        """
        Create middleware function that applies security headers
        
        Args:
            handler: Request handler function
        
        Returns:
            Wrapped handler with security headers
        """
        def wrapper(request):
            response = handler(request)
            return self.apply_to_response(response)
        return wrapper

def secure_headers(
    csp: Optional[str] = None,
    cors_origin: str = "*",
    hsts: bool = True,
    frame_options: str = "DENY"
) -> SecurityHeaders:
    """
    Quick helper to create security headers with common settings
    
    Args:
        csp: Content Security Policy string
        cors_origin: CORS allowed origin
        hsts: Enable HSTS
        frame_options: X-Frame-Options value
    
    Returns:
        Configured SecurityHeaders instance
    
    Example:
        security = secure_headers(
            csp="default-src 'self'",
            cors_origin="https://example.com"
        )
    """
    config = SecurityConfig(
        csp=csp,
        cors_allow_origin=cors_origin,
        enable_hsts=hsts,
        x_frame_options=frame_options
    )
    return SecurityHeaders(config)

# Preset configurations
class Presets:
    """Common security header configurations"""
    
    @staticmethod
    def strict() -> SecurityConfig:
        """Strict security - maximum protection"""
        return SecurityConfig(
            csp="default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; frame-ancestors 'none'",
            cors_allow_origin="null",  # No CORS
            enable_cors=False,
            hsts_max_age=63072000,  # 2 years
            hsts_include_subdomains=True,
            hsts_preload=True,
            x_frame_options="DENY",
            permissions_policy="geolocation=(), microphone=(), camera=()"
        )
    
    @staticmethod
    def moderate() -> SecurityConfig:
        """Moderate security - balanced"""
        return SecurityConfig(
            csp="default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'",
            cors_allow_origin="*",
            hsts_max_age=31536000,  # 1 year
            hsts_include_subdomains=True,
            x_frame_options="SAMEORIGIN"
        )
    
    @staticmethod
    def api() -> SecurityConfig:
        """API-focused - allows CORS but strict otherwise"""
        return SecurityConfig(
            csp=None,  # Not needed for API
            cors_allow_origin="*",
            cors_allow_methods="GET, POST, PUT, DELETE, PATCH, OPTIONS",
            cors_allow_headers="Content-Type, Authorization, X-Requested-With",
            enable_csp=False,
            hsts_max_age=31536000,
            x_frame_options="DENY"
        )
    
    @staticmethod
    def development() -> SecurityConfig:
        """Development - relaxed for testing"""
        return SecurityConfig(
            csp="default-src 'self' 'unsafe-inline' 'unsafe-eval'",
            cors_allow_origin="*",
            enable_hsts=False,  # Don't use HSTS in dev
            x_frame_options="SAMEORIGIN"
        )
