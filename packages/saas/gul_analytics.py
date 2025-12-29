"""
GUL Analytics
Event tracking and analytics.

Status: ✅ Implemented
Priority: High
Phase: 3
"""

from typing import Dict, List, Optional, Any
from dataclasses import dataclass, field
from datetime import datetime, timedelta
from collections import defaultdict

__version__ = "0.1.0"
__all__ = ['Analytics', 'Event', 'Aggregator']

@dataclass
class Event:
    """Analytics event"""
    name: str
    user_id: Optional[str]
    properties: Dict[str, Any]
    timestamp: datetime = field(default_factory=datetime.utcnow)
    session_id: Optional[str] = None

class Aggregator:
    """Aggregate analytics data"""
    
    @staticmethod
    def count_by_field(events: List[Event], field: str) -> Dict[Any, int]:
        """Count events by field"""
        counts = defaultdict(int)
        
        for event in events:
            value = event.properties.get(field)
            if value is not None:
                counts[value] += 1
        
        return dict(counts)
    
    @staticmethod
    def group_by_time(
        events: List[Event],
        interval: str = "day"
    ) -> Dict[str, int]:
        """Group events by time interval"""
        groups = defaultdict(int)
        
        for event in events:
            if interval == "hour":
                key = event.timestamp.strftime("%Y-%m-%d %H:00")
            elif interval == "day":
                key = event.timestamp.strftime("%Y-%m-%d")
            elif interval == "week":
                key = event.timestamp.strftime("%Y-W%W")
            elif interval == "month":
                key = event.timestamp.strftime("%Y-%m")
            else:
                key = event.timestamp.isoformat()
            
            groups[key] += 1
        
        return dict(groups)
    
    @staticmethod
    def funnel(events: List[Event], steps: List[str]) -> Dict[str, int]:
        """Calculate funnel conversion"""
        funnel_data = {}
        
        for i, step in enumerate(steps):
            step_events = [e for e in events if e.name == step]
            funnel_data[step] = len(step_events)
        
        return funnel_data

class Analytics:
    """
    Analytics and event tracking
    
    Example:
        analytics = Analytics()
        
        # Track events
        analytics.track("page_view", user_id="user_123", properties={
            "page": "/home",
            "referrer": "google.com"
        })
        
        analytics.track("button_click", properties={
            "button": "signup",
            "page": "/pricing"
        })
        
        # Query events
        events = analytics.query("page_view", days=7)
        
        # Get metrics
        metrics = analytics.get_metrics(days=30)
    """
    
    def __init__(self):
        self.events: List[Event] = []
        self.max_events = 100000
    
    def track(
        self,
        event_name: str,
        user_id: Optional[str] = None,
        properties: Optional[Dict] = None,
        session_id: Optional[str] = None
    ):
        """Track an event"""
        event = Event(
            name=event_name,
            user_id=user_id,
            properties=properties or {},
            session_id=session_id
        )
        
        self.events.append(event)
        
        # Trim old events
        if len(self.events) > self.max_events:
            self.events = self.events[-self.max_events:]
    
    def query(
        self,
        event_name: Optional[str] = None,
        user_id: Optional[str] = None,
        days: Optional[int] = None,
        limit: int = 1000
    ) -> List[Event]:
        """Query events"""
        results = self.events
        
        # Filter by event name
        if event_name:
            results = [e for e in results if e.name == event_name]
        
        # Filter by user
        if user_id:
            results = [e for e in results if e.user_id == user_id]
        
        # Filter by time
        if days:
            cutoff = datetime.utcnow() - timedelta(days=days)
            results = [e for e in results if e.timestamp >= cutoff]
        
        # Limit
        return results[-limit:]
    
    def get_metrics(self, days: int = 30) -> Dict[str, Any]:
        """Get aggregated metrics"""
        events = self.query(days=days)
        
        return {
            "total_events": len(events),
            "unique_users": len(set(e.user_id for e in events if e.user_id)),
            "events_by_name": Aggregator.count_by_field(events, "name"),
            "events_by_day": Aggregator.group_by_time(events, "day")
        }
    
    def get_user_journey(self, user_id: str, days: int = 7) -> List[Event]:
        """Get user journey"""
        return self.query(user_id=user_id, days=days)
    
    def get_retention(self, cohort_days: int = 30) -> Dict[str, float]:
        """Calculate retention rates"""
        cutoff = datetime.utcnow() - timedelta(days=cohort_days)
        
        # Get users from cohort period
        cohort_events = [e for e in self.events if e.timestamp >= cutoff and e.user_id]
        cohort_users = set(e.user_id for e in cohort_events)
        
        if not cohort_users:
            return {}
        
        # Calculate retention
        retention = {}
        for day in [1, 7, 14, 30]:
            day_cutoff = datetime.utcnow() - timedelta(days=day)
            active_users = set(
                e.user_id for e in self.events
                if e.timestamp >= day_cutoff and e.user_id in cohort_users
            )
            
            retention[f"day_{day}"] = len(active_users) / len(cohort_users) * 100
        
        return retention
    
    def get_funnel_analysis(self, steps: List[str]) -> Dict:
        """Analyze conversion funnel"""
        events = self.query()
        funnel = Aggregator.funnel(events, steps)
        
        # Calculate conversion rates
        conversions = {}
        prev_count = None
        
        for step, count in funnel.items():
            if prev_count is not None:
                conversions[f"{step}_conversion"] = (count / prev_count * 100) if prev_count > 0 else 0
            prev_count = count
        
        return {
            "funnel": funnel,
            "conversions": conversions
        }
    
    def export_events(
        self,
        event_name: Optional[str] = None,
        days: Optional[int] = None
    ) -> List[Dict]:
        """Export events as JSON"""
        events = self.query(event_name=event_name, days=days)
        
        return [
            {
                "name": e.name,
                "user_id": e.user_id,
                "properties": e.properties,
                "timestamp": e.timestamp.isoformat(),
                "session_id": e.session_id
            }
            for e in events
        ]
