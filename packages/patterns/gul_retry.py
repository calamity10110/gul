"""
GUL Retry Logic
Retry with exponential backoff.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Callable, Optional, Type, Tuple
from dataclasses import dataclass
import time

__version__ = "0.1.0"
__all__ = ['retry', 'RetryConfig', 'with_retry']

@dataclass
class RetryConfig:
    max_attempts: int = 3
    initial_delay: float = 1.0
    max_delay: float = 60.0
    exponential_base: float = 2.0
    exceptions: Tuple[Type[Exception], ...] = (Exception,)

def retry(
    max_attempts: int = 3,
    initial_delay: float = 1.0,
    max_delay: float = 60.0,
    exponential_base: float = 2.0,
    exceptions: Tuple[Type[Exception], ...] = (Exception,)
):
    """
    Retry decorator
    
    Example:
        @retry(max_attempts=5, initial_delay=0.5)
        def fetch_data():
            response = http.get("https://api.example.com/data")
            return response.json()
    """
    def decorator(func: Callable):
        def wrapper(*args, **kwargs):
            config = RetryConfig(max_attempts, initial_delay, max_delay, exponential_base, exceptions)
            return with_retry(func, config, *args, **kwargs)
        return wrapper
    return decorator

def with_retry(func: Callable, config: RetryConfig, *args, **kwargs):
    """Execute function with retry logic"""
    last_exception = None
    
    for attempt in range(config.max_attempts):
        try:
            return func(*args, **kwargs)
        except config.exceptions as e:
            last_exception = e
            
            if attempt < config.max_attempts - 1:
                delay = min(
                    config.initial_delay * (config.exponential_base ** attempt),
                    config.max_delay
                )
                time.sleep(delay)
    
    raise last_exception
