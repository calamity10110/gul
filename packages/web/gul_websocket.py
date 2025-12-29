"""
GUL WebSocket
WebSocket client and server messages.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Dict, Any, Optional, Callable, List
import json
import base64

__version__ = "0.1.0"
__all__ = ['Message', 'Frame', 'OpCode', 'WebSocket']

class OpCode:
    CONTINUATION = 0x0
    TEXT = 0x1
    BINARY = 0x2
    CLOSE = 0x8
    PING = 0x9
    PONG = 0xA

class Frame:
    """WebSocket frame"""
    def __init__(self, opcode: int, payload: bytes, fin: bool = True):
        self.opcode = opcode
        self.payload = payload
        self.fin = fin

class Message:
    """WS Message"""
    def __init__(self, type: str, data: Any):
        self.type = type
        self.data = data
    
    def to_json(self) -> str:
        return json.dumps({"type": self.type, "data": self.data})
    
    @classmethod
    def from_json(cls, data: str) -> 'Message':
        parsed = json.loads(data)
        return cls(parsed.get("type", "unknown"), parsed.get("data"))

class WebSocket:
    """
    WebSocket Helper
    
    Example:
        ws = WebSocket()
        frame = ws.create_frame("Hello")
    """
    
    def create_frame(self, data: Union[str, bytes], opcode: Optional[int] = None) -> bytes:
        """Create a WebSocket frame"""
        if isinstance(data, str):
            payload = data.encode('utf-8')
            opcode = opcode or OpCode.TEXT
        else:
            payload = data
            opcode = opcode or OpCode.BINARY
        
        header = bytearray()
        header.append(0x80 | opcode)  # FIN | OpCode
        
        length = len(payload)
        if length < 126:
            header.append(length)
        elif length < 65536:
            header.append(126)
            header.extend(length.to_bytes(2, byteorder='big'))
        else:
            header.append(127)
            header.extend(length.to_bytes(8, byteorder='big'))
        
        return header + payload
    
    def parse_frame(self, data: bytes) -> Optional[Frame]:
        """Parse a frame header (basic)"""
        if len(data) < 2:
            return None
        
        byte1 = data[0]
        fin = (byte1 & 0x80) != 0
        opcode = byte1 & 0x0F
        
        byte2 = data[1]
        mask = (byte2 & 0x80) != 0
        length = byte2 & 0x7F
        
        offset = 2
        if length == 126:
            offset += 2
        elif length == 127:
            offset += 8
        
        if mask:
            offset += 4
        
        return Frame(opcode, b"", fin=fin)

# Global helper
_ws = WebSocket()

def create_message(type: str, data: Any) -> str:
    return Message(type, data).to_json()
