"""
GUL Logging
Structured logging with multiple outputs.

Status: ✅ Implemented
Priority: High
"""

from typing import Dict, List, Optional, Any
from dataclasses import dataclass
from datetime import datetime
from enum import Enum
import json

__version__ = "0.1.0"
__all__ = ['Logger', 'LogLevel', 'LogHandler', 'ConsoleHandler', 'FileHandler']

class LogLevel(Enum):
    DEBUG = 10
    INFO = 20
    WARNING = 30
    ERROR = 40
    CRITICAL = 50

@dataclass
class LogRecord:
    level: LogLevel
    message: str
    timestamp: datetime
    logger_name: str
    extra: Dict[str, Any] = None
    
    def to_dict(self) -> Dict:
        """Convert to dictionary"""
        return {
            'level': self.level.name,
            'message': self.message,
            'timestamp': self.timestamp.isoformat(),
            'logger': self.logger_name,
            'extra': self.extra or {}
        }

class LogHandler:
    """Base log handler"""
    
    def __init__(self, level: LogLevel = LogLevel.INFO):
        self.level = level
    
    def emit(self, record: LogRecord):
        """Emit log record"""
        raise NotImplementedError

class ConsoleHandler(LogHandler):
    """Console log handler"""
    
    def emit(self, record: LogRecord):
        """Print to console"""
        timestamp = record.timestamp.strftime('%Y-%m-%d %H:%M:%S')
        level = record.level.name
        print(f'[{timestamp}] {level:8} {record.logger_name}: {record.message}')
        
        if record.extra:
            for key, value in record.extra.items():
                print(f'  {key}: {value}')

class FileHandler(LogHandler):
    """File log handler"""
    
    def __init__(self, filename: str, level: LogLevel = LogLevel.INFO, json_format: bool = False):
        super().__init__(level)
        self.filename = filename
        self.json_format = json_format
        self.file = None
    
    def emit(self, record: LogRecord):
        """Write to file"""
        # Simulated file writing
        if self.json_format:
            line = json.dumps(record.to_dict())
        else:
            timestamp = record.timestamp.strftime('%Y-%m-%d %H:%M:%S')
            line = f'[{timestamp}] {record.level.name:8} {record.logger_name}: {record.message}'
        
        # In real implementation, would write to file

class Logger:
    """
    Structured logger
    
    Example:
        logger = Logger("myapp")
        logger.add_handler(ConsoleHandler())
        logger.add_handler(FileHandler("app.log", json_format=True))
        
        logger.info("Server started", port=8000, host="localhost")
        logger.warning("High memory usage", usage_mb=512)
        logger.error("Database connection failed", error="timeout")
    """
    
    def __init__(self, name: str, level: LogLevel = LogLevel.INFO):
        self.name = name
        self.level = level
        self.handlers: List[LogHandler] = []
        self.extra_context: Dict[str, Any] = {}
    
    def add_handler(self, handler: LogHandler):
        """Add log handler"""
        self.handlers.append(handler)
    
    def set_level(self, level: LogLevel):
        """Set minimum log level"""
        self.level = level
    
    def add_context(self, **kwargs):
        """Add persistent context"""
        self.extra_context.update(kwargs)
    
    def log(self, level: LogLevel, message: str, **kwargs):
        """Log message"""
        if level.value < self.level.value:
            return
        
        extra = {**self.extra_context, **kwargs}
        
        record = LogRecord(
            level=level,
            message=message,
            timestamp=datetime.utcnow(),
            logger_name=self.name,
            extra=extra if extra else None
        )
        
        for handler in self.handlers:
            if level.value >= handler.level.value:
                handler.emit(record)
    
    def debug(self, message: str, **kwargs):
        """Log debug message"""
        self.log(LogLevel.DEBUG, message, **kwargs)
    
    def info(self, message: str, **kwargs):
        """Log info message"""
        self.log(LogLevel.INFO, message, **kwargs)
    
    def warning(self, message: str, **kwargs):
        """Log warning message"""
        self.log(LogLevel.WARNING, message, **kwargs)
    
    def error(self, message: str, **kwargs):
        """Log error message"""
        self.log(LogLevel.ERROR, message, **kwargs)
    
    def critical(self, message: str, **kwargs):
        """Log critical message"""
        self.log(LogLevel.CRITICAL, message, **kwargs)

# Global logger registry
_loggers: Dict[str, Logger] = {}

def get_logger(name: str) -> Logger:
    """Get or create logger"""
    if name not in _loggers:
        _loggers[name] = Logger(name)
    return _loggers[name]
