"""
GUL Environment Manager
Environment and deployment configuration.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Dict, Optional, List
import os

__version__ = "0.1.0"
__all__ = ['Environment', 'is_production', 'is_development']

class Environment:
    """
    Environment manager
    
    Example:
        env = Environment()
        
        # Detect environment
        if env.is_production():
            enable_caching()
        
        # Environment-specific config
        db_host = env.get("DATABASE_HOST", 
            production="prod-db.example.com",
            staging="staging-db.example.com",
            development="localhost"
        )
    """
    
    PRODUCTION = "production"
    STAGING = "staging"
    DEVELOPMENT = "development"
    TEST = "test"
    
    def __init__(self, env: Optional[str] = None):
        self.env = env or os.environ.get("ENV", self.DEVELOPMENT)
    
    def is_production(self) -> bool:
        """Check if production"""
        return self.env == self.PRODUCTION
    
    def is_staging(self) -> bool:
        """Check if staging"""
        return self.env == self.STAGING
    
    def is_development(self) -> bool:
        """Check if development"""
        return self.env == self.DEVELOPMENT
    
    def is_test(self) -> bool:
        """Check if test"""
        return self.env == self.TEST
    
    def get(self, key: str, default: Optional[str] = None, **env_defaults) -> str:
        """Get environment-specific value"""
        # Check environment variable first
        value = os.environ.get(key)
        if value:
            return value
        
        # Check environment-specific defaults
        if self.env in env_defaults:
            return env_defaults[self.env]
        
        # Return default
        return default or ""
    
    def require(self, *keys: str):
        """Require environment variables"""
        missing = []
        for key in keys:
            if key not in os.environ:
                missing.append(key)
        
        if missing:
            raise ValueError(f"Missing required environment variables: {', '.join(missing)}")
    
    def set_default(self, key: str, value: str):
        """Set default environment variable"""
        if key not in os.environ:
            os.environ[key] = value

_current_env = Environment()

def is_production() -> bool:
    """Check if production environment"""
    return _current_env.is_production()

def is_development() -> bool:
    """Check if development environment"""
    return _current_env.is_development()
