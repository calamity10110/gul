"""
GUL Rate Limiter
Advanced rate limiting with multiple algorithms.

Status: ✅ Implemented
Priority: Critical
Phase: 1
"""

from typing import Dict, Optional, Callable
from dataclasses import dataclass
from datetime import datetime, timedelta
import time
from collections import deque

__version__ = "0.1.0"
__all__ = ['RateLimiter', 'TokenBucket', 'SlidingWindow', 'FixedWindow', 'RateLimitExceeded']

class RateLimitExceeded(Exception):
    """Exception raised when rate limit is exceeded"""
    def __init__(self, retry_after: float):
        self.retry_after = retry_after
        super().__init__(f"Rate limit exceeded. Retry after {retry_after:.2f} seconds")

@dataclass
class RateLimitConfig:
    """Rate limit configuration"""
    max_requests: int = 100
    window_seconds: int = 60
    algorithm: str = "token_bucket"  # token_bucket, sliding_window, fixed_window

class TokenBucket:
    """
    Token bucket algorithm for rate limiting
    
    Allows bursts up to capacity, refills at steady rate.
    """
    
    def __init__(self, capacity: int, refill_rate: float):
        """
        Args:
            capacity: Maximum tokens in bucket
            refill_rate: Tokens added per second
        """
        self.capacity = capacity
        self.refill_rate = refill_rate
        self.tokens = float(capacity)
        self.last_update = time.time()
    
    def _refill(self):
        """Refill tokens based on time elapsed"""
        now = time.time()
        elapsed = now - self.last_update
        self.tokens = min(
            self.capacity,
            self.tokens + (elapsed * self.refill_rate)
        )
        self.last_update = now
    
    def consume(self, tokens: int = 1) -> bool:
        """
        Try to consume tokens
        
        Args:
            tokens: Number of tokens to consume
        
        Returns:
            True if tokens consumed, False if insufficient
        """
        self._refill()
        
        if self.tokens >= tokens:
            self.tokens -= tokens
            return True
        return False
    
    def get_wait_time(self, tokens: int = 1) -> float:
        """Get time to wait until tokens available"""
        self._refill()
        
        if self.tokens >= tokens:
            return 0.0
        
        needed = tokens - self.tokens
        return needed / self.refill_rate

class SlidingWindow:
    """
    Sliding window algorithm for rate limiting
    
    Tracks requests in a sliding time window.
    """
    
    def __init__(self, max_requests: int, window_seconds: int):
        self.max_requests = max_requests
        self.window_seconds = window_seconds
        self.requests = deque()
    
    def _clean_old(self):
        """Remove requests outside window"""
        cutoff = time.time() - self.window_seconds
        while self.requests and self.requests[0] < cutoff:
            self.requests.popleft()
    
    def allow_request(self) -> bool:
        """Check if request is allowed"""
        self._clean_old()
        
        if len(self.requests) < self.max_requests:
            self.requests.append(time.time())
            return True
        return False
    
    def get_wait_time(self) -> float:
        """Get time to wait until next request allowed"""
        self._clean_old()
        
        if len(self.requests) < self.max_requests:
            return 0.0
        
        oldest = self.requests[0]
        wait = self.window_seconds - (time.time() - oldest)
        return max(0.0, wait)

class FixedWindow:
    """
    Fixed window algorithm for rate limiting
    
    Resets counter at fixed intervals.
    """
    
    def __init__(self, max_requests: int, window_seconds: int):
        self.max_requests = max_requests
        self.window_seconds = window_seconds
        self.count = 0
        self.window_start = time.time()
    
    def _reset_if_needed(self):
        """Reset window if expired"""
        now = time.time()
        if now - self.window_start >= self.window_seconds:
            self.count = 0
            self.window_start = now
    
    def allow_request(self) -> bool:
        """Check if request is allowed"""
        self._reset_if_needed()
        
        if self.count < self.max_requests:
            self.count += 1
            return True
        return False
    
    def get_wait_time(self) -> float:
        """Get time to wait until window resets"""
        self._reset_if_needed()
        
        if self.count < self.max_requests:
            return 0.0
        
        elapsed = time.time() - self.window_start
        return max(0.0, self.window_seconds - elapsed)

class RateLimiter:
    """
    Advanced rate limiter with multiple algorithms
    
    Supports:
    - Token bucket (allows bursts)
    - Sliding window (smooth limiting)
    - Fixed window (simple counter)
    
    Example:
        # Per-user rate limiting
        limiter = RateLimiter(max_requests=100, window_seconds=60)
        
        if limiter.allow("user_123"):
            # Process request
            pass
        else:
            # Rate limit exceeded
            wait = limiter.get_wait_time("user_123")
            raise RateLimitExceeded(wait)
    """
    
    def __init__(
        self,
        max_requests: int = 100,
        window_seconds: int = 60,
        algorithm: str = "token_bucket"
    ):
        self.max_requests = max_requests
        self.window_seconds = window_seconds
        self.algorithm = algorithm
        self.limiters: Dict[str, any] = {}
    
    def _get_limiter(self, key: str):
        """Get or create limiter for key"""
        if key not in self.limiters:
            if self.algorithm == "token_bucket":
                rate = self.max_requests / self.window_seconds
                self.limiters[key] = TokenBucket(self.max_requests, rate)
            elif self.algorithm == "sliding_window":
                self.limiters[key] = SlidingWindow(self.max_requests, self.window_seconds)
            elif self.algorithm == "fixed_window":
                self.limiters[key] = FixedWindow(self.max_requests, self.window_seconds)
            else:
                raise ValueError(f"Unknown algorithm: {self.algorithm}")
        
        return self.limiters[key]
    
    def allow(self, key: str) -> bool:
        """
        Check if request from key is allowed
        
        Args:
            key: Identifier (user ID, IP address, etc.)
        
        Returns:
            True if allowed, False if rate limited
        """
        limiter = self._get_limiter(key)
        
        if self.algorithm == "token_bucket":
            return limiter.consume(1)
        else:
            return limiter.allow_request()
    
    def get_wait_time(self, key: str) -> float:
        """
        Get seconds to wait before retry
        
        Args:
            key: Identifier
        
        Returns:
            Seconds to wait (0 if allowed now)
        """
        limiter = self._get_limiter(key)
        
        if self.algorithm == "token_bucket":
            return limiter.get_wait_time(1)
        else:
            return limiter.get_wait_time()
    
    def check(self, key: str, raise_on_limit: bool = True) -> bool:
        """
        Check rate limit and optionally raise exception
        
        Args:
            key: Identifier
            raise_on_limit: Raise RateLimitExceeded if true
        
        Returns:
            True if allowed
        
        Raises:
            RateLimitExceeded: If rate limited and raise_on_limit=True
        """
        if self.allow(key):
            return True
        
        if raise_on_limit:
            wait_time = self.get_wait_time(key)
            raise RateLimitExceeded(wait_time)
        
        return False
    
    def reset(self, key: str):
        """Reset rate limit for key"""
        if key in self.limiters:
            del self.limiters[key]
    
    def middleware(self, get_key: Callable):
        """
        Create middleware that applies rate limiting
        
        Args:
            get_key: Function to extract key from request
        
        Returns:
            Middleware function
        """
        def wrapper(handler):
            def rate_limited_handler(request):
                key = get_key(request)
                self.check(key, raise_on_limit=True)
                return handler(request)
            return rate_limited_handler
        return wrapper

# Quick helpers
def rate_limit_by_ip(max_requests: int = 100, window_seconds: int = 60):
    """
    Create rate limiter that limits by IP address
    
    Args:
        max_requests: Maximum requests per window
        window_seconds: Window size in seconds
    
    Returns:
        Configured RateLimiter
    """
    limiter = RateLimiter(max_requests, window_seconds)
    
    def get_ip(request):
        return getattr(request, 'remote_addr', 'unknown')
    
    return limiter.middleware(get_ip)

def rate_limit_by_user(max_requests: int = 100, window_seconds: int = 60):
    """
    Create rate limiter that limits by user ID
    
    Args:
        max_requests: Maximum requests per window
        window_seconds: Window size in seconds
    
    Returns:
        Configured RateLimiter
    """
    limiter = RateLimiter(max_requests, window_seconds)
    
    def get_user(request):
        return getattr(request, 'user_id', 'anonymous')
    
    return limiter.middleware(get_user)
