"""
GUL Event Bus
Event-driven architecture with pub/sub pattern.

Status: ✅ Implemented
Priority: High
Phase: 2
"""

from typing import Dict, List, Callable, Optional, Any
from dataclasses import dataclass, field
from datetime import datetime

__version__ = "0.1.0"
__all__ = ['EventBus', 'Event', 'EventHandler']

@dataclass
class Event:
    """Event data"""
    name: str
    data: Any
    timestamp: datetime = field(default_factory=datetime.utcnow)
    metadata: Dict = field(default_factory=dict)

class EventHandler:
    """Event handler"""
    
    def __init__(self, callback: Callable, filters: Optional[Dict] = None):
        self.callback = callback
        self.filters = filters or {}
    
    def matches(self, event: Event) -> bool:
        """Check if handler matches event"""
        if not self.filters:
            return True
        
        for key, value in self.filters.items():
            if event.metadata.get(key) != value:
                return False
        
        return True
    
    def handle(self, event: Event):
        """Handle event"""
        if self.matches(event):
            self.callback(event)

class EventBus:
    """
    Event bus for event-driven architecture
    
    Example:
        bus = EventBus()
        
        # Subscribe to events
        @bus.on("user.created")
        def handle_user_created(event):
            print(f"User created: {event.data}")
        
        # Emit events
        bus.emit("user.created", {"id": "123", "name": "Alice"})
        
        # Filtered subscription
        bus.on("order.status", filters={"status": "completed"})(handle_completed)
    """
    
    def __init__(self):
        self.handlers: Dict[str, List[EventHandler]] = {}
        self.event_history: List[Event] = []
        self.max_history = 1000
    
    def on(
        self,
        event_name: str,
        filters: Optional[Dict] = None
    ) -> Callable:
        """
        Register event handler (decorator)
        
        Example:
            @bus.on("user.created")
            def handler(event):
                print(event.data)
        """
        def decorator(callback: Callable) -> Callable:
            self.subscribe(event_name, callback, filters)
            return callback
        
        return decorator
    
    def subscribe(
        self,
        event_name: str,
        callback: Callable,
        filters: Optional[Dict] = None
    ):
        """Subscribe to event"""
        if event_name not in self.handlers:
            self.handlers[event_name] = []
        
        handler = EventHandler(callback, filters)
        self.handlers[event_name].append(handler)
    
    def unsubscribe(self, event_name: str, callback: Callable):
        """Unsubscribe handler"""
        if event_name in self.handlers:
            self.handlers[event_name] = [
                h for h in self.handlers[event_name]
                if h.callback != callback
            ]
    
    def emit(
        self,
        event_name: str,
        data: Any = None,
        metadata: Optional[Dict] = None
    ):
        """Emit event"""
        event = Event(
            name=event_name,
            data=data,
            metadata=metadata or {}
        )
        
        # Store in history
        self.event_history.append(event)
        if len(self.event_history) > self.max_history:
            self.event_history.pop(0)
        
        # Dispatch to handlers
        if event_name in self.handlers:
            for handler in self.handlers[event_name]:
                try:
                    handler.handle(event)
                except Exception as e:
                    print(f"Error in event handler: {e}")
        
        # Handle wildcard subscribers
        if "*" in self.handlers:
            for handler in self.handlers["*"]:
                try:
                    handler.handle(event)
                except Exception as e:
                    print(f"Error in wildcard handler: {e}")
    
    def emit_async(self, event_name: str, data: Any = None):
        """Emit event asynchronously (simulated)"""
        # In real implementation, this would use threading or asyncio
        self.emit(event_name, data)
    
    def get_history(
        self,
        event_name: Optional[str] = None,
        limit: int = 100
    ) -> List[Event]:
        """Get event history"""
        if event_name:
            events = [e for e in self.event_history if e.name == event_name]
        else:
            events = self.event_history
        
        return events[-limit:]
    
    def clear_history(self):
        """Clear event history"""
        self.event_history.clear()
    
    def get_handlers(self, event_name: str) -> int:
        """Get number of handlers for event"""
        return len(self.handlers.get(event_name, []))

# Helper for creating event emitters
def create_emitter(bus: EventBus, prefix: str = ""):
    """Create event emitter with prefix"""
    def emit(event_name: str, data: Any = None):
        full_name = f"{prefix}.{event_name}" if prefix else event_name
        bus.emit(full_name, data)
    
    return emit
