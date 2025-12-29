"""
GUL Scheduler
Task scheduling with cron-like syntax.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Dict, Callable, Optional, List
from dataclasses import dataclass
from datetime import datetime, timedelta
import time

__version__ = "0.1.0"
__all__ = ['Scheduler', 'Job', 'every']

@dataclass
class Job:
    """Scheduled job"""
    name: str
    func: Callable
    interval: Optional[int] = None  # seconds
    cron: Optional[str] = None
    next_run: Optional[datetime] = None
    last_run: Optional[datetime] = None

class Scheduler:
    """
    Task scheduler
    
    Example:
        scheduler = Scheduler()
        
        # Run every 60 seconds
        @scheduler.every(60)
        def cleanup():
            print("Running cleanup")
        
        # Run at specific times (simplified cron)
        scheduler.cron("0 9 * * *", send_daily_report)
        
        # Start scheduler
        scheduler.run()
    """
    
    def __init__(self):
        self.jobs: List[Job] = []
        self.running = False
    
    def every(self, seconds: int):
        """Decorator to run function every N seconds"""
        def decorator(func: Callable):
            job = Job(
                name=func.__name__,
                func=func,
                interval=seconds,
                next_run=datetime.now() + timedelta(seconds=seconds)
            )
            self.jobs.append(job)
            return func
        return decorator
    
    def cron(self, expression: str, func: Callable):
        """Schedule with cron expression (simplified)"""
        job = Job(
            name=func.__name__,
            func=func,
            cron=expression,
            next_run=self._parse_cron(expression)
        )
        self.jobs.append(job)
    
    def run(self, max_iterations: Optional[int] = None):
        """Run scheduler"""
        self.running = True
        iteration = 0
        
        while self.running:
            if max_iterations and iteration >= max_iterations:
                break
            
            now = datetime.now()
            
            for job in self.jobs:
                if job.next_run and now >= job.next_run:
                    try:
                        job.func()
                        job.last_run = now
                        
                        if job.interval:
                            job.next_run = now + timedelta(seconds=job.interval)
                        elif job.cron:
                            job.next_run = self._parse_cron(job.cron)
                    
                    except Exception as e:
                        print(f"Error in job {job.name}: {e}")
            
            time.sleep(1)
            iteration += 1
    
    def stop(self):
        """Stop scheduler"""
        self.running = False
    
    def _parse_cron(self, expression: str) -> datetime:
        """Parse cron expression (simplified)"""
        # Format: "minute hour day month weekday"
        # Example: "0 9 * * *" = every day at 9:00
        
        parts = expression.split()
        if len(parts) != 5:
            return datetime.now() + timedelta(hours=1)
        
        minute, hour, day, month, weekday = parts
        
        now = datetime.now()
        next_run = now.replace(second=0, microsecond=0)
        
        # Set hour and minute
        if minute != '*':
            next_run = next_run.replace(minute=int(minute))
        if hour != '*':
            next_run = next_run.replace(hour=int(hour))
        
        # If time has passed today, schedule for tomorrow
        if next_run <= now:
            next_run += timedelta(days=1)
        
        return next_run

def every(seconds: int):
    """Create simple interval job"""
    def decorator(func: Callable):
        func._interval = seconds
        return func
    return decorator
