"""
GUL Cache Manager
Multi-backend cache abstraction layer.

Status: ✅ Implemented
Priority: High
Phase: 2
"""

from typing import Dict, Optional, Any, List
from abc import ABC, abstractmethod
import time
import json

__version__ = "0.1.0"
__all__ = ['CacheManager', 'MemoryCache', 'RedisCache', 'LayeredCache']

class CacheBackend(ABC):
    """Abstract cache backend"""
    
    @abstractmethod
    def get(self, key: str) -> Optional[Any]:
        pass
    
    @abstractmethod
    def set(self, key: str, value: Any, ttl: Optional[int] = None):
        pass
    
    @abstractmethod
    def delete(self, key: str):
        pass
    
    @abstractmethod
    def clear(self):
        pass

class MemoryCache(CacheBackend):
    """In-memory cache"""
    
    def __init__(self):
        self.data: Dict[str, Any] = {}
        self.expiry: Dict[str, float] = {}
    
    def get(self, key: str) -> Optional[Any]:
        self._check_expiry(key)
        return self.data.get(key)
    
    def set(self, key: str, value: Any, ttl: Optional[int] = None):
        self.data[key] = value
        
        if ttl:
            self.expiry[key] = time.time() + ttl
    
    def delete(self, key: str):
        self.data.pop(key, None)
        self.expiry.pop(key, None)
    
    def clear(self):
        self.data.clear()
        self.expiry.clear()
    
    def _check_expiry(self, key: str):
        if key in self.expiry and time.time() > self.expiry[key]:
            self.delete(key)

class LayeredCache:
    """Multi-layer cache (L1: memory, L2: Redis)"""
    
    def __init__(self, l1: CacheBackend, l2: Optional[CacheBackend] = None):
        self.l1 = l1
        self.l2 = l2
    
    def get(self, key: str) -> Optional[Any]:
        # Try L1
        value = self.l1.get(key)
        if value is not None:
            return value
        
        # Try L2
        if self.l2:
            value = self.l2.get(key)
            if value is not None:
                # Populate L1
                self.l1.set(key, value)
                return value
        
        return None
    
    def set(self, key: str, value: Any, ttl: Optional[int] = None):
        self.l1.set(key, value, ttl)
        
        if self.l2:
            self.l2.set(key, value, ttl)
    
    def delete(self, key: str):
        self.l1.delete(key)
        
        if self.l2:
            self.l2.delete(key)

class CacheManager:
    """
    Cache manager with multiple strategies
    
    Example:
        cache = CacheManager(MemoryCache())
        
        # Set/Get
        cache.set("user:123", {"name": "Alice"}, ttl=60)
        user = cache.get("user:123")
        
        # Get or set
        def load_user(id):
            return {"name": "Alice", "id": id}
        
        user = cache.get_or_set("user:123", lambda: load_user(123), ttl=60)
        
        # Memoization
        @cache.memoize(ttl=300)
        def expensive_function(x, y):
            return x * y
    """
    
    def __init__(self, backend: CacheBackend):
        self.backend = backend
    
    def get(self, key: str) -> Optional[Any]:
        """Get from cache"""
        return self.backend.get(key)
    
    def set(self, key: str, value: Any, ttl: Optional[int] = None):
        """Set in cache"""
        self.backend.set(key, value, ttl)
    
    def delete(self, key: str):
        """Delete from cache"""
        self.backend.delete(key)
    
    def get_or_set(
        self,
        key: str,
        factory: Any,
        ttl: Optional[int] = None
    ) -> Any:
        """Get from cache or compute and set"""
        value = self.get(key)
        
        if value is None:
            value = factory() if callable(factory) else factory
            self.set(key, value, ttl)
        
        return value
    
    def get_many(self, keys: List[str]) -> Dict[str, Any]:
        """Get multiple keys"""
        return {key: self.get(key) for key in keys}
    
    def set_many(self, items: Dict[str, Any], ttl: Optional[int] = None):
        """Set multiple keys"""
        for key, value in items.items():
            self.set(key, value, ttl)
    
    def clear(self):
        """Clear cache"""
        self.backend.clear()
    
    def memoize(self, ttl: Optional[int] = None, key_prefix: str = ""):
        """Memoization decorator"""
        def decorator(func):
            def wrapper(*args, **kwargs):
                # Create cache key
                cache_key = f"{key_prefix}{func.__name__}:{json.dumps(args)}:{json.dumps(kwargs)}"
                
                # Try cache
                result = self.get(cache_key)
                if result is not None:
                    return result
                
                # Compute and cache
                result = func(*args, **kwargs)
                self.set(cache_key, result, ttl)
                return result
            
            return wrapper
        return decorator
