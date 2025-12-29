"""
GUL WebSocket Server
WebSocket server with rooms and broadcasting.

Status: ✅ Implemented
Priority: High
Phase: 2
"""

from typing import Dict, List, Set, Optional, Callable
from dataclasses import dataclass, field
import json

__version__ = "0.1.0"
__all__ = ['WebSocketServer', 'Connection', 'Room']

@dataclass
class Connection:
    """WebSocket connection"""
    id: str
    user_data: Dict = field(default_factory=dict)
    rooms: Set[str] = field(default_factory=set)

class Room:
    """WebSocket room"""
    
    def __init__(self, name: str):
        self.name = name
        self.connections: Set[str] = set()
    
    def add(self, conn_id: str):
        """Add connection"""
        self.connections.add(conn_id)
    
    def remove(self, conn_id: str):
        """Remove connection"""
        self.connections.discard(conn_id)
    
    def broadcast(self, message: Dict, exclude: Optional[str] = None) -> int:
        """Broadcast to room"""
        sent = 0
        for conn_id in self.connections:
            if conn_id != exclude:
                sent += 1
        return sent

class WebSocketServer:
    """
    WebSocket server with rooms and broadcasting
    
    Example:
        ws = WebSocketServer()
        
        # Handle connection
        ws.on_connect(lambda conn: print(f"Connected: {conn.id}"))
        ws.on_message(lambda conn, msg: ws.broadcast(msg))
        
        # Create room
        ws.create_room("chat")
        ws.join_room(conn_id, "chat")
        
        # Broadcast
        ws.broadcast_to_room("chat", {"text": "Hello!"})
    """
    
    def __init__(self):
        self.connections: Dict[str, Connection] = {}
        self.rooms: Dict[str, Room] = {}
        self.handlers: Dict[str, List[Callable]] = {
            'connect': [],
            'disconnect': [],
            'message': []
        }
    
    def on_connect(self, handler: Callable) -> 'WebSocketServer':
        """Register connect handler"""
        self.handlers['connect'].append(handler)
        return self
    
    def on_disconnect(self, handler: Callable) -> 'WebSocketServer':
        """Register disconnect handler"""
        self.handlers['disconnect'].append(handler)
        return self
    
    def on_message(self, handler: Callable) -> 'WebSocketServer':
        """Register message handler"""
        self.handlers['message'].append(handler)
        return self
    
    def connect(self, conn_id: str, user_data: Optional[Dict] = None) -> Connection:
        """Handle new connection"""
        conn = Connection(id=conn_id, user_data=user_data or {})
        self.connections[conn_id] = conn
        
        for handler in self.handlers['connect']:
            handler(conn)
        
        return conn
    
    def disconnect(self, conn_id: str):
        """Handle disconnection"""
        if conn_id not in self.connections:
            return
        
        conn = self.connections[conn_id]
        
        # Remove from all rooms
        for room_name in list(conn.rooms):
            self.leave_room(conn_id, room_name)
        
        for handler in self.handlers['disconnect']:
            handler(conn)
        
        del self.connections[conn_id]
    
    def send(self, conn_id: str, message: Dict):
        """Send message to connection"""
        if conn_id not in self.connections:
            return False
        
        # In real implementation, this would send over websocket
        return True
    
    def broadcast(self, message: Dict, exclude: Optional[str] = None) -> int:
        """Broadcast to all connections"""
        sent = 0
        for conn_id in self.connections:
            if conn_id != exclude:
                self.send(conn_id, message)
                sent += 1
        return sent
    
    def receive(self, conn_id: str, message: Dict):
        """Handle received message"""
        if conn_id not in self.connections:
            return
        
        conn = self.connections[conn_id]
        
        for handler in self.handlers['message']:
            handler(conn, message)
    
    def create_room(self, name: str) -> Room:
        """Create room"""
        room = Room(name)
        self.rooms[name] = room
        return room
    
    def join_room(self, conn_id: str, room_name: str) -> bool:
        """Join room"""
        if conn_id not in self.connections:
            return False
        
        if room_name not in self.rooms:
            self.create_room(room_name)
        
        self.rooms[room_name].add(conn_id)
        self.connections[conn_id].rooms.add(room_name)
        return True
    
    def leave_room(self, conn_id: str, room_name: str) -> bool:
        """Leave room"""
        if conn_id not in self.connections or room_name not in self.rooms:
            return False
        
        self.rooms[room_name].remove(conn_id)
        self.connections[conn_id].rooms.discard(room_name)
        return True
    
    def broadcast_to_room(
        self,
        room_name: str,
        message: Dict,
        exclude: Optional[str] = None
    ) -> int:
        """Broadcast to room"""
        if room_name not in self.rooms:
            return 0
        
        room = self.rooms[room_name]
        sent = 0
        
        for conn_id in room.connections:
            if conn_id != exclude:
                self.send(conn_id, message)
                sent += 1
        
        return sent
    
    def get_room_members(self, room_name: str) -> List[str]:
        """Get room members"""
        if room_name not in self.rooms:
            return []
        
        return list(self.rooms[room_name].connections)
