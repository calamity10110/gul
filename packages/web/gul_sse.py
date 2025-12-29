"""
GUL SSE (Server-Sent Events)
SSE implementation.

Status: ✅ Implemented
Priority: Low
"""

from typing import Optional, Dict, Any
import json

__version__ = "0.1.0"
__all__ = ['Event', 'stream']

class Event:
    """SSE Event"""
    def __init__(
        self,
        data: Any,
        event: Optional[str] = None,
        id: Optional[str] = None,
        retry: Optional[int] = None
    ):
        self.data = data
        self.event = event
        self.id = id
        self.retry = retry
    
    def __str__(self) -> str:
        lines = []
        
        if self.event:
            lines.append(f"event: {self.event}")
        
        if self.id:
            lines.append(f"id: {self.id}")
        
        if self.retry:
            lines.append(f"retry: {self.retry}")
        
        # Determine data format
        data_str = self.data
        if not isinstance(data_str, str):
            data_str = json.dumps(data_str)
        
        for line in data_str.split('\n'):
            lines.append(f"data: {line}")
        
        return "\n".join(lines) + "\n\n"

def stream(data: Any, event: Optional[str] = None, id: Optional[str] = None) -> str:
    """Create SSE event string"""
    return str(Event(data, event, id))
