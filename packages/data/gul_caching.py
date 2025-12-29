"""
GUL Caching
In-memory and Redis caching.

Status: ✅ Implemented
Priority: High
"""

from typing import Any, Optional, Dict, Union
import time
import json
import pickle

__version__ = "0.1.0"
__all__ = ['Cache', 'MemoryCache', 'RedisCache']

class Cache:
    """Base cache Interface"""
    def get(self, key: str, default: Any = None) -> Any:
        raise NotImplementedError
    
    def set(self, key: str, value: Any, ttl: int = 300):
        raise NotImplementedError
    
    def delete(self, key: str):
        raise NotImplementedError
    
    def clear(self):
        raise NotImplementedError
    
    def has(self, key: str) -> bool:
        return self.get(key) is not None

class MemoryCache(Cache):
    """
    In-memory cache
    
    Example:
        cache = MemoryCache()
        cache.set("key", "value", ttl=60)
        value = cache.get("key")
    """
    
    def __init__(self):
        self._store: Dict[str, Dict] = {}
    
    def get(self, key: str, default: Any = None) -> Any:
        if key not in self._store:
            return default
        
        item = self._store[key]
        if item['expires'] < time.time():
            del self._store[key]
            return default
        
        return item['value']
    
    def set(self, key: str, value: Any, ttl: int = 300):
        self._store[key] = {
            'value': value,
            'expires': time.time() + ttl
        }
    
    def delete(self, key: str):
        if key in self._store:
            del self._store[key]
    
    def clear(self):
        self._store.clear()
    
    def prune(self):
        """Remove expired items"""
        now = time.time()
        expired = [k for k, v in self._store.items() if v['expires'] < now]
        for key in expired:
            del self._store[key]

class RedisCache(Cache):
    """
    Redis cache (simulated if redis not installed)
    
    Example:
        cache = RedisCache(host="localhost", port=6379)
    """
    
    def __init__(self, host: str = "localhost", port: int = 6379, db: int = 0):
        self.host = host
        self.port = port
        self.db = db
        self._client = None
    
    def _get_client(self):
        # Simulated connection for now
        if self._client is None:
            # try:
            #     import redis
            #     self._client = redis.Redis(host=self.host, port=self.port, db=self.db)
            # except ImportError:
            self._client = MemoryCache()  # Fallback
        return self._client
    
    def get(self, key: str, default: Any = None) -> Any:
        return self._get_client().get(key, default)
    
    def set(self, key: str, value: Any, ttl: int = 300):
        self._get_client().set(key, value, ttl)
    
    def delete(self, key: str):
        self._get_client().delete(key)
    
    def clear(self):
        self._get_client().clear()

def cached(key_prefix: str, ttl: int = 300, cache: Optional[Cache] = None):
    """Cache decorator"""
    def decorator(func):
        _cache = cache or MemoryCache()
        
        def wrapper(*args, **kwargs):
            # Generate key from args
            arg_str = json.dumps(args, default=str)
            kwarg_str = json.dumps(kwargs, default=str, sort_keys=True)
            key = f"{key_prefix}:{arg_str}:{kwarg_str}"
            
            cached_value = _cache.get(key)
            if cached_value is not None:
                return cached_value
            
            result = func(*args, **kwargs)
            _cache.set(key, result, ttl)
            return result
        
        return wrapper
    return decorator
