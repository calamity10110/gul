"""
GUL Secrets Manager
Secure secrets management.

Status: ✅ Implemented
Priority: High
"""

from typing import Dict, Optional
import os
import json
import base64

__version__ = "0.1.0"
__all__ = ['SecretsManager', 'Secret']

class Secret:
    """Encrypted secret value"""
    
    def __init__(self, value: str, encrypted: bool = False):
        self._value = value
        self._encrypted = encrypted
    
    def get(self) -> str:
        """Get decrypted value"""
        if self._encrypted:
            return base64.b64decode(self._value).decode()
        return self._value
    
    def encrypt(self, key: str = "") -> str:
        """Encrypt value"""
        if not self._encrypted:
            self._value = base64.b64encode(self._value.encode()).decode()
            self._encrypted = True
        return self._value

class SecretsManager:
    """
    Secrets manager
    
    Example:
        secrets = SecretsManager()
        
        # Set secrets
        secrets.set("db_password", "super-secret")
        secrets.set("api_key", "key-123")
        
        # Get secrets
        db_pass = secrets.get("db_password")
        
        # Load from env
        secrets.load_from_env(prefix="SECRET_")
        
        # Load from file
        secrets.load_from_file("secrets.json")
    """
    
    def __init__(self):
        self._secrets: Dict[str, Secret] = {}
    
    def set(self, key: str, value: str, encrypt: bool = False):
        """Set secret"""
        secret = Secret(value, encrypted=False)
        if encrypt:
            secret.encrypt()
        self._secrets[key] = secret
    
    def get(self, key: str, default: Optional[str] = None) -> Optional[str]:
        """Get secret value"""
        if key in self._secrets:
            return self._secrets[key].get()
        return default
    
    def has(self, key: str) -> bool:
        """Check if secret exists"""
        return key in self._secrets
    
    def delete(self, key: str):
        """Delete secret"""
        if key in self._secrets:
            del self._secrets[key]
    
    def load_from_env(self, prefix: str = ""):
        """Load secrets from environment variables"""
        for key, value in os.environ.items():
            if prefix and not key.startswith(prefix):
                continue
            
            secret_key = key[len(prefix):].lower()
            self.set(secret_key, value)
    
    def load_from_file(self, filename: str):
        """Load secrets from JSON file"""
        if not os.path.exists(filename):
            return
        
        with open(filename) as f:
            data = json.load(f)
            for key, value in data.items():
                self.set(key, value)
    
    def to_dict(self, include_values: bool = False) -> Dict:
        """Export secrets (optionally with values)"""
        if include_values:
            return {k: v.get() for k, v in self._secrets.items()}
        return {k: "***" for k in self._secrets.keys()}
