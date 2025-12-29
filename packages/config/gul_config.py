"""
GUL Configuration Manager
Application configuration with env vars and files.

Status: ✅ Implemented
Priority: High
"""

from typing import Dict, Any, Optional, List
import os
import json

__version__ = "0.1.0"
__all__ = ['Config', 'load_config', 'get_env']

class Config:
    """
    Configuration manager
    
    Example:
        config = Config()
        config.load_from_dict({"database": {"host": "localhost", "port": 5432}})
        config.load_from_env(prefix="APP_")
        
        db_host = config.get("database.host")
        db_port = config.get("database.port", default=5432)
        
        # Type conversion
        port = config.get_int("database.port")
        debug = config.get_bool("debug", default=False)
    """
    
    def __init__(self):
        self._data: Dict[str, Any] = {}
    
    def load_from_dict(self, data: Dict):
        """Load from dictionary"""
        self._merge_dict(self._data, data)
    
    def load_from_json(self, filename: str):
        """Load from JSON file"""
        with open(filename) as f:
            data = json.load(f)
            self.load_from_dict(data)
    
    def load_from_env(self, prefix: str = ""):
        """Load from environment variables"""
        for key, value in os.environ.items():
            if prefix and not key.startswith(prefix):
                continue
            
            config_key = key[len(prefix):].lower().replace('_', '.')
            self.set(config_key, value)
    
    def get(self, key: str, default: Any = None) -> Any:
        """Get configuration value"""
        parts = key.split('.')
        current = self._data
        
        for part in parts:
            if not isinstance(current, dict) or part not in current:
                return default
            current = current[part]
        
        return current
    
    def get_int(self, key: str, default: int = 0) -> int:
        """Get integer value"""
        value = self.get(key, default)
        return int(value) if value is not None else default
    
    def get_float(self, key: str, default: float = 0.0) -> float:
        """Get float value"""
        value = self.get(key, default)
        return float(value) if value is not None else default
    
    def get_bool(self, key: str, default: bool = False) -> bool:
        """Get boolean value"""
        value = self.get(key, default)
        
        if isinstance(value, bool):
            return value
        
        if isinstance(value, str):
            return value.lower() in ('true', '1', 'yes', 'on')
        
        return bool(value) if value is not None else default
    
    def get_list(self, key: str, default: Optional[List] = None) -> List:
        """Get list value"""
        value = self.get(key, default)
        
        if isinstance(value, list):
            return value
        
        if isinstance(value, str):
            return [v.strip() for v in value.split(',')]
        
        return default or []
    
    def set(self, key: str, value: Any):
        """Set configuration value"""
        parts = key.split('.')
        current = self._data
        
        for i, part in enumerate(parts[:-1]):
            if part not in current:
                current[part] = {}
            elif not isinstance(current[part], dict):
                current[part] = {}
            current = current[part]
        
        current[parts[-1]] = value
    
    def has(self, key: str) -> bool:
        """Check if key exists"""
        return self.get(key) is not None
    
    def to_dict(self) -> Dict:
        """Export as dictionary"""
        return self._data.copy()
    
    def _merge_dict(self, target: Dict, source: Dict):
        """Merge source dict into target"""
        for key, value in source.items():
            if key in target and isinstance(target[key], dict) and isinstance(value, dict):
                self._merge_dict(target[key], value)
            else:
                target[key] = value

def load_config(
    json_file: Optional[str] = None,
    env_prefix: str = "",
    defaults: Optional[Dict] = None
) -> Config:
    """Quick config loading"""
    config = Config()
    
    if defaults:
        config.load_from_dict(defaults)
    
    if json_file and os.path.exists(json_file):
        config.load_from_json(json_file)
    
    if env_prefix:
        config.load_from_env(env_prefix)
    
    return config

def get_env(key: str, default: str = "") -> str:
    """Get environment variable"""
    return os.environ.get(key, default)
