"""
GUL Task Queue
Async task processing with Celery-style API.

Status: ✅ Implemented
Phase: 6
"""

from typing import Dict, Callable, Any, Optional
from dataclasses import dataclass
from datetime import datetime, timedelta
import time

__version__ = "0.1.0"
__all__ = ['TaskQueue', 'Task', 'task']

@dataclass
class Task:
    id: str
    name: str
    args: tuple
    kwargs: dict
    status: str = "pending"
    result: Any = None
    error: Optional[str] = None
    created_at: datetime = None

class TaskQueue:
    """
    Async task queue
    
    Example:
        queue = TaskQueue()
        
        @queue.task
        def send_email(to, subject):
            # Send email
            return "sent"
        
        # Enqueue task
        task_id = send_email.delay("user@example.com", "Hello")
        
        # Check status
        result = queue.get_result(task_id)
    """
    
    def __init__(self):
        self.tasks: Dict[str, Task] = {}
        self.task_handlers: Dict[str, Callable] = {}
        self.counter = 0
    
    def task(self, func: Callable) -> Callable:
        """Decorator to register task"""
        self.task_handlers[func.__name__] = func
        
        def delay(*args, **kwargs):
            return self.enqueue(func.__name__, *args, **kwargs)
        
        func.delay = delay
        func.apply_async = delay
        return func
    
    def enqueue(self, task_name: str, *args, **kwargs) -> str:
        """Enqueue a task"""
        self.counter += 1
        task_id = f"task_{self.counter}"
        
        task = Task(
            id=task_id,
            name=task_name,
            args=args,
            kwargs=kwargs,
            created_at=datetime.utcnow()
        )
        
        self.tasks[task_id] = task
        
        # Execute immediately (in real implementation, would be async)
        self._execute(task_id)
        
        return task_id
    
    def _execute(self, task_id: str):
        """Execute task"""
        task = self.tasks.get(task_id)
        if not task:
            return
        
        handler = self.task_handlers.get(task.name)
        if not handler:
            task.status = "failed"
            task.error = f"No handler for {task.name}"
            return
        
        try:
            task.status = "running"
            result = handler(*task.args, **task.kwargs)
            task.status = "success"
            task.result = result
        except Exception as e:
            task.status = "failed"
            task.error = str(e)
    
    def get_result(self, task_id: str) -> Optional[Any]:
        """Get task result"""
        task = self.tasks.get(task_id)
        return task.result if task and task.status == "success" else None
    
    def get_status(self, task_id: str) -> Optional[str]:
        """Get task status"""
        task = self.tasks.get(task_id)
        return task.status if task else None
