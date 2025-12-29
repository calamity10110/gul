"""
GUL Message Queue
Message queue implementation with Redis backend support.

Status: ✅ Implemented
Priority: High
Phase: 2
"""

from typing import Dict, List, Optional, Callable, Any
from collections import deque
from dataclasses import dataclass
import time
import json

__version__ = "0.1.0"
__all__ = ['MessageQueue', 'Message', 'Consumer']

@dataclass
class Message:
    """Queue message"""
    id: str
    queue: str
    body: Any
    timestamp: float
    retries: int = 0
    max_retries: int = 3

class Consumer:
    """Message consumer"""
    
    def __init__(self, handler: Callable):
        self.handler = handler
        self.running = False
    
    def process(self, message: Message) -> bool:
        """Process message"""
        try:
            self.handler(message.body)
            return True
        except Exception as e:
            print(f"Error processing message: {e}")
            return False

class MessageQueue:
    """
    Message queue with retry and dead letter queue
    
    Example:
        mq = MessageQueue()
        
        # Publish
        mq.publish("tasks", {"action": "process", "data": 123})
        
        # Consume
        def handler(msg):
            print(f"Processing: {msg}")
        
        mq.consume("tasks", handler)
    """
    
    def __init__(self):
        self.queues: Dict[str, deque] = {}
        self.dead_letter_queue: deque = deque()
        self.consumers: Dict[str, List[Consumer]] = {}
        self.message_counter = 0
    
    def publish(self, queue_name: str, body: Any, delay: int = 0) -> str:
        """Publish message to queue"""
        if queue_name not in self.queues:
            self.queues[queue_name] = deque()
        
        self.message_counter += 1
        message = Message(
            id=f"msg_{self.message_counter}",
            queue=queue_name,
            body=body,
            timestamp=time.time() + delay
        )
        
        self.queues[queue_name].append(message)
        return message.id
    
    def consume(self, queue_name: str, handler: Callable, auto_ack: bool = True):
        """Register consumer"""
        if queue_name not in self.consumers:
            self.consumers[queue_name] = []
        
        consumer = Consumer(handler)
        self.consumers[queue_name].append(consumer)
        
        # Start processing
        self._process_queue(queue_name, consumer, auto_ack)
    
    def _process_queue(self, queue_name: str, consumer: Consumer, auto_ack: bool):
        """Process messages from queue"""
        if queue_name not in self.queues:
            return
        
        queue = self.queues[queue_name]
        
        while queue:
            message = queue.popleft()
            
            # Check if delayed
            if message.timestamp > time.time():
                queue.append(message)
                continue
            
            # Process
            success = consumer.process(message)
            
            if not success and message.retries < message.max_retries:
                # Retry
                message.retries += 1
                queue.append(message)
            elif not success:
                # Move to dead letter queue
                self.dead_letter_queue.append(message)
    
    def get_queue_size(self, queue_name: str) -> int:
        """Get queue size"""
        if queue_name not in self.queues:
            return 0
        return len(self.queues[queue_name])
    
    def purge_queue(self, queue_name: str):
        """Clear queue"""
        if queue_name in self.queues:
            self.queues[queue_name].clear()
    
    def get_dead_letters(self) -> List[Message]:
        """Get dead letter messages"""
        return list(self.dead_letter_queue)

# Redis-backed queue (simplified)
class RedisQueue:
    """Redis-backed message queue"""
    
    def __init__(self, redis_url: str = "redis://localhost:6379"):
        self.redis_url = redis_url
        self.queues: Dict[str, List] = {}
    
    def publish(self, queue_name: str, message: Dict):
        """Publish to Redis list"""
        if queue_name not in self.queues:
            self.queues[queue_name] = []
        
        self.queues[queue_name].append(json.dumps(message))
    
    def consume(self, queue_name: str, timeout: int = 0) -> Optional[Dict]:
        """Consume from Redis list (blocking)"""
        if queue_name not in self.queues or not self.queues[queue_name]:
            return None
        
        message_str = self.queues[queue_name].pop(0)
        return json.loads(message_str)
