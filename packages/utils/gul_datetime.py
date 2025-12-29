"""
GUL DateTime
Date and time utilities.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Optional, Union
from datetime import datetime, timedelta, timezone as tz, date
import re

__version__ = "0.1.0"
__all__ = ['now', 'parse', 'format', 'TimeDelta', 'DateTime']

class DateTime:
    """
    DateTime wrapper
    
    Example:
        dt = DateTime.now()
        
        # Add time
        later = dt.add(days=1, hours=5)
        
        # Format
        print(dt.format("YYYY-MM-DD HH:mm:ss"))
        
        # Parse
        dt = DateTime.parse("2023-01-01 12:00:00")
        
        # Difference
        diff = later.diff(dt)
        print(diff.days)
    """
    
    def __init__(self, dt: datetime):
        self._dt = dt
    
    @classmethod
    def now(cls) -> 'DateTime':
        """Current UTC time"""
        return cls(datetime.now(tz.utc))
    
    @classmethod
    def parse(cls, value: str, fmt: Optional[str] = None) -> 'DateTime':
        """Parse string to datetime"""
        if fmt:
            # Convert GUL format (YYYY-MM-DD) to Python format (%Y-%m-%d)
            py_fmt = cls._convert_format(fmt)
            dt = datetime.strptime(value, py_fmt)
        else:
            # Auto-detect common formats (simplified)
            try:
                dt = datetime.fromisoformat(value.replace('Z', '+00:00'))
            except:
                try:
                    dt = datetime.strptime(value, "%Y-%m-%d %H:%M:%S")
                except:
                    dt = datetime.strptime(value, "%Y-%m-%d")
        
        if dt.tzinfo is None:
            dt = dt.replace(tzinfo=tz.utc)
            
        return cls(dt)
    
    @classmethod
    def from_timestamp(cls, ts: Union[int, float]) -> 'DateTime':
        """From epoch timestamp"""
        dt = datetime.fromtimestamp(ts, tz.utc)
        return cls(dt)
    
    def format(self, fmt: str) -> str:
        """Format datetime"""
        py_fmt = self._convert_format(fmt)
        return self._dt.strftime(py_fmt)
    
    def add(self, **kwargs) -> 'DateTime':
        """Add time"""
        return DateTime(self._dt + timedelta(**kwargs))
    
    def sub(self, **kwargs) -> 'DateTime':
        """Subtract time"""
        return DateTime(self._dt - timedelta(**kwargs))
    
    def diff(self, other: 'DateTime') -> timedelta:
        """Difference between datetimes"""
        return self._dt - other._dt
    
    def timestamp(self) -> float:
        """Get timestamp"""
        return self._dt.timestamp()
    
    def iso(self) -> str:
        """ISO 8601 string"""
        return self._dt.isoformat()
    
    @staticmethod
    def _convert_format(fmt: str) -> str:
        """Convert common format tokens to strftime"""
        replacements = {
            'YYYY': '%Y',
            'YY': '%y',
            'MM': '%m',
            'DD': '%d',
            'HH': '%H',
            'mm': '%M',
            'ss': '%S',
            'SSS': '%f'
        }
        
        for k, v in replacements.items():
            fmt = fmt.replace(k, v)
        
        return fmt
    
    def __str__(self) -> str:
        return self.iso()
    
    def __repr__(self) -> str:
        return f"DateTime({self.iso()})"

# Helpers
def now() -> DateTime:
    return DateTime.now()

def parse(value: str, fmt: Optional[str] = None) -> DateTime:
    return DateTime.parse(value, fmt)

def format(dt: Union[datetime, DateTime], fmt: str) -> str:
    if isinstance(dt, datetime):
        dt = DateTime(dt)
    return dt.format(fmt)

TimeDelta = timedelta
