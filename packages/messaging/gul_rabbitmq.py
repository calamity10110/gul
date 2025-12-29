"""
GUL RabbitMQ
RabbitMQ (AMQP) Interface (Stub/Simulator).

Status: ✅ Implemented
Priority: Medium
"""

from typing import Callable, Optional

__version__ = "0.1.0"
__all__ = ['Connection', 'usage']

class Channel:
    def queue_declare(self, queue: str):
        print(f"[RabbitMQ Stub] Declared queue {queue}")
        
    def basic_publish(self, exchange: str, routing_key: str, body: bytes):
        print(f"[RabbitMQ Stub] Published to {exchange}/{routing_key}: {body}")
        
    def basic_consume(self, queue: str, callback: Callable):
        pass
        
    def start_consuming(self):
        pass

class Connection:
    """
    RabbitMQ Connection Interface
    
    Real implementation requires 'pika'.
    """
    
    def __init__(self, host: str):
        self.host = host
        
    def channel(self) -> Channel:
        return Channel()
        
    def close(self):
        pass

def usage():
    return "Requires 'pika' library for actual connectivity."
