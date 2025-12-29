"""
GUL Serialization
Object serialization to multiple formats.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Any, Dict
import json
import pickle
import base64

__version__ = "0.1.0"
__all__ = ['Serializer', 'to_json', 'from_json', 'to_pickle', 'from_pickle']

class Serializer:
    """
    Multi-format serializer
    
    Example:
        serializer = Serializer()
        
        data = {"name": "Alice", "age": 30}
        
        # JSON
        json_str = serializer.to_json(data)
        restored = serializer.from_json(json_str)
        
        # Pickle
        pickled = serializer.to_pickle(data)
        restored = serializer.from_pickle(pickled)
        
        # Base64
        encoded = serializer.to_base64(data)
        restored = serializer.from_base64(encoded)
    """
    
    def to_json(self, obj: Any, pretty: bool = False) -> str:
        """Serialize to JSON"""
        if pretty:
            return json.dumps(obj, indent=2, default=str)
        return json.dumps(obj, default=str)
    
    def from_json(self, data: str) -> Any:
        """Deserialize from JSON"""
        return json.loads(data)
    
    def to_pickle(self, obj: Any) -> bytes:
        """Serialize to pickle"""
        return pickle.dumps(obj)
    
    def from_pickle(self, data: bytes) -> Any:
        """Deserialize from pickle"""
        return pickle.loads(data)
    
    def to_base64(self, obj: Any) -> str:
        """Serialize to base64-encoded JSON"""
        json_str = self.to_json(obj)
        return base64.b64encode(json_str.encode()).decode()
    
    def from_base64(self, data: str) -> Any:
        """Deserialize from base64-encoded JSON"""
        json_str = base64.b64decode(data).decode()
        return self.from_json(json_str)

# Quick functions
_serializer = Serializer()

def to_json(obj: Any, pretty: bool = False) -> str:
    """Quick JSON serialization"""
    return _serializer.to_json(obj, pretty)

def from_json(data: str) -> Any:
    """Quick JSON deserialization"""
    return _serializer.from_json(data)

def to_pickle(obj: Any) -> bytes:
    """Quick pickle serialization"""
    return _serializer.to_pickle(obj)

def from_pickle(data: bytes) -> Any:
    """Quick pickle deserialization"""
    return _serializer.from_pickle(data)
