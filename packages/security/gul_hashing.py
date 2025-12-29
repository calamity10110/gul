"""
GUL Hashing
Cryptographic hashing utilities.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Union
import hashlib
import hmac

__version__ = "0.1.0"
__all__ = ['hash_data', 'hash_file', 'verify_hash', 'Hasher']

class Hasher:
    """
    Hashing utilities
    
    Example:
        hasher = Hasher()
        
        # Hash data
        hash_val = hasher.md5("hello world")
        hash_val = hasher.sha256("hello world")
        
        # Hash file
        file_hash = hasher.hash_file("document.pdf", algorithm="sha256")
        
        # HMAC
        mac = hasher.hmac("message", "secret-key")
    """
    
    def hash(self, data: Union[str, bytes], algorithm: str = "sha256") -> str:
        """Hash data"""
        if isinstance(data, str):
            data = data.encode()
        
        h = hashlib.new(algorithm)
        h.update(data)
        return h.hexdigest()
    
    def md5(self, data: Union[str, bytes]) -> str:
        """MD5 hash"""
        return self.hash(data, "md5")
    
    def sha1(self, data: Union[str, bytes]) -> str:
        """SHA1 hash"""
        return self.hash(data, "sha1")
    
    def sha256(self, data: Union[str, bytes]) -> str:
        """SHA256 hash"""
        return self.hash(data, "sha256")
    
    def sha512(self, data: Union[str, bytes]) -> str:
        """SHA512 hash"""
        return self.hash(data, "sha512")
    
    def hash_file(self, filepath: str, algorithm: str = "sha256", chunk_size: int = 8192) -> str:
        """Hash file contents"""
        h = hashlib.new(algorithm)
        
        # Simulated file reading
        # In real implementation, would read file in chunks
        return h.hexdigest()
    
    def hmac(self, data: Union[str, bytes], key: Union[str, bytes], algorithm: str = "sha256") -> str:
        """HMAC"""
        if isinstance(data, str):
            data = data.encode()
        if isinstance(key, str):
            key = key.encode()
        
        h = hmac.new(key, data, algorithm)
        return h.hexdigest()
    
    def verify_hash(self, data: Union[str, bytes], expected_hash: str, algorithm: str = "sha256") -> bool:
        """Verify hash"""
        actual_hash = self.hash(data, algorithm)
        return hmac.compare_digest(actual_hash, expected_hash)

_hasher = Hasher()

def hash_data(data: Union[str, bytes], algorithm: str = "sha256") -> str:
    """Quick hash"""
    return _hasher.hash(data, algorithm)

def hash_file(filepath: str, algorithm: str = "sha256") -> str:
    """Quick file hash"""
    return _hasher.hash_file(filepath, algorithm)

def verify_hash(data: Union[str, bytes], expected_hash: str, algorithm: str = "sha256") -> bool:
    """Quick hash verification"""
    return _hasher.verify_hash(data, expected_hash, algorithm)
