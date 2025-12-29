"""
GUL Rate Limit
Rate limiting for APIs and services.

Status: ✅ Implemented
Priority: High
"""

from typing import Dict, Tuple, Optional
import time
from collections import deque

__version__ = "0.1.0"
__all__ = ['RateLimiter', 'TokenBucket', 'SlidingWindow']

class RateLimiter:
    """Base rate limiter"""
    def allow(self, key: str) -> bool:
        raise NotImplementedError
    
    def reset(self, key: str):
        raise NotImplementedError

class TokenBucket(RateLimiter):
    """
    Token Bucket rate limiter
    
    Example:
        # 10 requests per second
        limiter = TokenBucket(rate=10, capacity=10)
        
        if limiter.allow("user_123"):
            process_request()
    """
    
    def __init__(self, rate: float, capacity: float):
        self.rate = rate  # tokens per second
        self.capacity = capacity
        self.buckets: Dict[str, Dict] = {}
    
    def allow(self, key: str) -> bool:
        now = time.time()
        
        if key not in self.buckets:
            self.buckets[key] = {
                'tokens': self.capacity,
                'last_update': now
            }
        
        bucket = self.buckets[key]
        
        # Add tokens based on time passed
        elapsed = now - bucket['last_update']
        bucket['tokens'] = min(
            self.capacity,
            bucket['tokens'] + elapsed * self.rate
        )
        bucket['last_update'] = now
        
        # Consume token
        if bucket['tokens'] >= 1:
            bucket['tokens'] -= 1
            return True
        
        return False
    
    def reset(self, key: str):
        if key in self.buckets:
            del self.buckets[key]

class SlidingWindow(RateLimiter):
    """
    Sliding Window rate limiter
    
    Example:
        # 100 requests per minute
        limiter = SlidingWindow(limit=100, window=60)
    """
    
    def __init__(self, limit: int, window: int):
        self.limit = limit
        self.window = window
        self.requests: Dict[str, deque] = {}
    
    def allow(self, key: str) -> bool:
        now = time.time()
        
        if key not in self.requests:
            self.requests[key] = deque()
        
        timestamps = self.requests[key]
        
        # Remove expired timestamps
        while timestamps and timestamps[0] <= now - self.window:
            timestamps.popleft()
        
        # Check limit
        if len(timestamps) < self.limit:
            timestamps.append(now)
            return True
        
        return False
    
    def reset(self, key: str):
        if key in self.requests:
            del self.requests[key]

def limit(limiter: RateLimiter, key_func: Optional[Callable] = None):
    """Rate limit decorator"""
    def decorator(func):
        def wrapper(*args, **kwargs):
            key = key_func(*args, **kwargs) if key_func else "default"
            
            if not limiter.allow(key):
                raise Exception("Rate limit exceeded")
            
            return func(*args, **kwargs)
        return wrapper
    return decorator
