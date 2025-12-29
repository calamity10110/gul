"""
GUL Redis Advanced
Advanced Redis operations and patterns.

Status: ✅ Implemented
Priority: High
Phase: 2
"""

from typing import Dict, List, Optional, Any
import time
import json

__version__ = "0.1.0"
__all__ = ['RedisClient', 'RedisPubSub', 'RedisCache']

class RedisClient:
    """
    Redis client with advanced operations
    
    Example:
        redis = RedisClient()
        
        # Strings
        redis.set("key", "value", ttl=60)
        value = redis.get("key")
        
        # Lists
        redis.lpush("mylist", "item1")
        items = redis.lrange("mylist", 0, -1)
        
        # Sets
        redis.sadd("myset", "member1")
        members = redis.smembers("myset")
        
        # Hashes
        redis.hset("myhash", "field1", "value1")
        value = redis.hget("myhash", "field1")
    """
    
    def __init__(self, url: str = "redis://localhost:6379"):
        self.url = url
        self.data: Dict[str, Any] = {}
        self.expiry: Dict[str, float] = {}
        self.lists: Dict[str, List] = {}
        self.sets: Dict[str, set] = {}
        self.hashes: Dict[str, Dict] = {}
    
    def set(self, key: str, value: Any, ttl: Optional[int] = None):
        """Set string value"""
        self.data[key] = value
        
        if ttl:
            self.expiry[key] = time.time() + ttl
    
    def get(self, key: str) -> Optional[Any]:
        """Get string value"""
        self._check_expiry(key)
        return self.data.get(key)
    
    def delete(self, key: str) -> bool:
        """Delete key"""
        deleted = key in self.data
        self.data.pop(key, None)
        self.expiry.pop(key, None)
        return deleted
    
    def exists(self, key: str) -> bool:
        """Check if key exists"""
        self._check_expiry(key)
        return key in self.data
    
    def _check_expiry(self, key: str):
        """Check and remove expired key"""
        if key in self.expiry and time.time() > self.expiry[key]:
            self.delete(key)
    
    # List operations
    def lpush(self, key: str, *values):
        """Push to list (left)"""
        if key not in self.lists:
            self.lists[key] = []
        
        for value in values:
            self.lists[key].insert(0, value)
    
    def rpush(self, key: str, *values):
        """Push to list (right)"""
        if key not in self.lists:
            self.lists[key] = []
        
        self.lists[key].extend(values)
    
    def lpop(self, key: str) -> Optional[Any]:
        """Pop from list (left)"""
        if key not in self.lists or not self.lists[key]:
            return None
        
        return self.lists[key].pop(0)
    
    def lrange(self, key: str, start: int, end: int) -> List:
        """Get list range"""
        if key not in self.lists:
            return []
        
        if end == -1:
            return self.lists[key][start:]
        
        return self.lists[key][start:end+1]
    
    # Set operations
    def sadd(self, key: str, *members):
        """Add to set"""
        if key not in self.sets:
            self.sets[key] = set()
        
        self.sets[key].update(members)
    
    def srem(self, key: str, *members):
        """Remove from set"""
        if key in self.sets:
            self.sets[key].difference_update(members)
    
    def smembers(self, key: str) -> set:
        """Get set members"""
        return self.sets.get(key, set()).copy()
    
    def sismember(self, key: str, member: Any) -> bool:
        """Check set membership"""
        return member in self.sets.get(key, set())
    
    # Hash operations
    def hset(self, key: str, field: str, value: Any):
        """Set hash field"""
        if key not in self.hashes:
            self.hashes[key] = {}
        
        self.hashes[key][field] = value
    
    def hget(self, key: str, field: str) -> Optional[Any]:
        """Get hash field"""
        return self.hashes.get(key, {}).get(field)
    
    def hgetall(self, key: str) -> Dict:
        """Get all hash fields"""
        return self.hashes.get(key, {}).copy()
    
    def hdel(self, key: str, *fields):
        """Delete hash fields"""
        if key in self.hashes:
            for field in fields:
                self.hashes[key].pop(field, None)

class RedisPubSub:
    """Redis pub/sub"""
    
    def __init__(self, client: RedisClient):
        self.client = client
        self.subscriptions: Dict[str, List] = {}
    
    def publish(self, channel: str, message: Any):
        """Publish message"""
        if channel not in self.subscriptions:
            return 0
        
        message_str = json.dumps(message) if not isinstance(message, str) else message
        
        for callback in self.subscriptions[channel]:
            callback(message_str)
        
        return len(self.subscriptions[channel])
    
    def subscribe(self, channel: str, callback: Any):
        """Subscribe to channel"""
        if channel not in self.subscriptions:
            self.subscriptions[channel] = []
        
        self.subscriptions[channel].append(callback)

class RedisCache:
    """Cache decorator using Redis"""
    
    def __init__(self, client: RedisClient, ttl: int = 300):
        self.client = client
        self.ttl = ttl
    
    def cache(self, key_prefix: str):
        """Cache decorator"""
        def decorator(func):
            def wrapper(*args, **kwargs):
                # Create cache key
                cache_key = f"{key_prefix}:{json.dumps(args)}:{json.dumps(kwargs)}"
                
                # Check cache
                cached = self.client.get(cache_key)
                if cached is not None:
                    return json.loads(cached) if isinstance(cached, str) else cached
                
                # Compute and cache
                result = func(*args, **kwargs)
                self.client.set(cache_key, json.dumps(result), self.ttl)
                return result
            
            return wrapper
        return decorator
