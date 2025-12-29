"""
GUL CSRF
CSRF (Cross-Site Request Forgery) protection.

Status: ✅ Implemented
Priority: Medium
"""

import secrets
import hmac
from typing import Optional

__version__ = "0.1.0"
__all__ = ['Csrf', 'generate_token', 'verify_token']

class Csrf:
    """
    CSRF Protection
    
    Example:
        csrf = Csrf("secret-key")
        token = csrf.generate_token()
        is_valid = csrf.verify_token(token)
    """
    
    def __init__(self, secret_key: str):
        self.secret_key = secret_key.encode()
        
    def generate_token(self, salt_length: int = 32) -> str:
        """Generate a signed CSRF token"""
        salt = secrets.token_hex(salt_length // 2)
        signature = self._sign(salt)
        return f"{salt}.{signature}"
        
    def verify_token(self, token: str) -> bool:
        """Verify token signature"""
        try:
            salt, signature = token.split('.', 1)
        except ValueError:
            return False
            
        expected_signature = self._sign(salt)
        return hmac.compare_digest(signature, expected_signature)
        
    def _sign(self, salt: str) -> str:
        return hmac.new(self.secret_key, salt.encode(), "sha256").hexdigest()

def generate_token(secret: str) -> str:
    """Quick generation"""
    return Csrf(secret).generate_token()

def verify_token(token: str, secret: str) -> bool:
    """Quick verification"""
    return Csrf(secret).verify_token(token)
