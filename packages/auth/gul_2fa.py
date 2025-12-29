"""
GUL 2FA (Two-Factor Authentication)
TOTP-based two-factor authentication.

Status: ✅ Implemented
Priority: High
"""

from typing import Optional
import hashlib
import hmac
import time
import base64
import secrets

__version__ = "0.1.0"
__all__ = ['TwoFactorAuth', 'generate_secret', 'verify_code']

class TwoFactorAuth:
    """
    TOTP-based 2FA
    
    Example:
        tfa = TwoFactorAuth()
        
        # Generate secret for user
        secret = tfa.generate_secret()
        
        # Get provisioning URI for QR code
        uri = tfa.get_provisioning_uri("user@example.com", "MyApp", secret)
        
        # Verify code
        is_valid = tfa.verify_code(secret, "123456")
    """
    
    def __init__(self, digits: int = 6, period: int = 30):
        self.digits = digits
        self.period = period
    
    def generate_secret(self, length: int = 32) -> str:
        """Generate random secret"""
        return base64.b32encode(secrets.token_bytes(length)).decode('utf-8')
    
    def get_code(self, secret: str, timestamp: Optional[int] = None) -> str:
        """Generate TOTP code"""
        if timestamp is None:
            timestamp = int(time.time())
        
        # Calculate time counter
        counter = timestamp // self.period
        
        return self._generate_hotp(secret, counter)
    
    def verify_code(
        self,
        secret: str,
        code: str,
        window: int = 1,
        timestamp: Optional[int] = None
    ) -> bool:
        """Verify TOTP code"""
        if timestamp is None:
            timestamp = int(time.time())
        
        # Check current and adjacent time windows
        for offset in range(-window, window + 1):
            check_time = timestamp + (offset * self.period)
            expected_code = self.get_code(secret, check_time)
            
            if self._constant_time_compare(code, expected_code):
                return True
        
        return False
    
    def get_provisioning_uri(
        self,
        email: str,
        issuer: str,
        secret: str
    ) -> str:
        """Get provisioning URI for QR code"""
        params = f"secret={secret}&issuer={issuer}&digits={self.digits}&period={self.period}"
        return f"otpauth://totp/{issuer}:{email}?{params}"
    
    def _generate_hotp(self, secret: str, counter: int) -> str:
        """Generate HOTP code"""
        # Decode secret
        try:
            key = base64.b32decode(secret, casefold=True)
        except:
            key = secret.encode()
        
        # Convert counter to bytes
        counter_bytes = counter.to_bytes(8, byteorder='big')
        
        # HMAC-SHA1
        hmac_hash = hmac.new(key, counter_bytes, hashlib.sha1).digest()
        
        # Dynamic truncation
        offset = hmac_hash[-1] & 0x0F
        code = (
            ((hmac_hash[offset] & 0x7F) << 24) |
            ((hmac_hash[offset + 1] & 0xFF) << 16) |
            ((hmac_hash[offset + 2] & 0xFF) << 8) |
            (hmac_hash[offset + 3] & 0xFF)
        )
        
        # Get last N digits
        code = code % (10 ** self.digits)
        
        return str(code).zfill(self.digits)
    
    def _constant_time_compare(self, a: str, b: str) -> bool:
        """Constant-time string comparison"""
        if len(a) != len(b):
            return False
        
        result = 0
        for x, y in zip(a, b):
            result |= ord(x) ^ ord(y)
        
        return result == 0

def generate_secret() -> str:
    """Quick secret generation"""
    tfa = TwoFactorAuth()
    return tfa.generate_secret()

def verify_code(secret: str, code: str) -> bool:
    """Quick code verification"""
    tfa = TwoFactorAuth()
    return tfa.verify_code(secret, code)
