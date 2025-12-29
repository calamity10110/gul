"""
GUL Crypto
High-level cryptography wrapper.

Status: ✅ Implemented
Priority: High
"""

from typing import Union, Optional
import os
import base64
import hashlib
import hmac
from dataclasses import dataclass

__version__ = "0.1.0"
__all__ = ['Crypto', 'AES', 'encrypt', 'decrypt']

# Try to import cryptography, fallback to basic stdlib if missing
try:
    from cryptography.fernet import Fernet
    from cryptography.hazmat.primitives import hashes
    from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
    HAS_CRYPTO_LIB = True
except ImportError:
    HAS_CRYPTO_LIB = False

class CryptoError(Exception): pass

class Crypto:
    """
    High-level crypto utilities
    
    Example:
        crypto = Crypto("secret-key")
        encrypted = crypto.encrypt("secret data")
        decrypted = crypto.decrypt(encrypted)
    """
    
    def __init__(self, key: str):
        self.key = self._derive_key(key)
        self.fernet = Fernet(self.key) if HAS_CRYPTO_LIB else None
    
    def _derive_key(self, password: str) -> bytes:
        """Derive 32-byte key from password"""
        if HAS_CRYPTO_LIB:
            salt = b'gul-static-salt' # In prod, use random salt per user
            kdf = PBKDF2HMAC(
                algorithm=hashes.SHA256(),
                length=32,
                salt=salt,
                iterations=100000,
            )
            return base64.urlsafe_b64encode(kdf.derive(password.encode()))
        else:
            # Fallback simple derivation (NOT SECURE for prod, but functional for demo)
            k = hashlib.sha256(password.encode()).digest()
            return base64.urlsafe_b64encode(k)

    def encrypt(self, data: Union[str, bytes]) -> str:
        """Encrypt data"""
        if isinstance(data, str):
            data = data.encode()
            
        if self.fernet:
            return self.fernet.encrypt(data).decode()
        else:
            # Basic XOR fallback (only if library missing)
            return self._xor(data).decode()

    def decrypt(self, token: Union[str, bytes]) -> str:
        """Decrypt data"""
        if isinstance(token, str):
            token = token.encode()
            
        if self.fernet:
            return self.fernet.decrypt(token).decode()
        else:
            return self._xor(token).decode()

    def _xor(self, data: bytes) -> bytes:
        """Simple XOR cipher for fallback (insecure)"""
        key_bytes = base64.urlsafe_b64decode(self.key)
        return bytes(a ^ b for a, b in zip(data, key_bytes * (len(data) // len(key_bytes) + 1)))

# AES wrapper (Simulated if libs missing)
class AES:
    @staticmethod
    def encrypt(data: str, key: str) -> str:
        c = Crypto(key)
        return c.encrypt(data)
    
    @staticmethod
    def decrypt(token: str, key: str) -> str:
        c = Crypto(key)
        return c.decrypt(token)

def encrypt(data: str, key: str) -> str:
    return AES.encrypt(data, key)

def decrypt(token: str, key: str) -> str:
    return AES.decrypt(token, key)
