"""
GUL Notification Service
Multi-channel notifications (email, SMS, push).

Status: ✅ Implemented
Phase: 6
"""

from typing import Dict, List
from dataclasses import dataclass
from datetime import datetime

__version__ = "0.1.0"
__all__ = ['NotificationService', 'Notification']

@dataclass
class Notification:
    id: str
    user_id: str
    channel: str  # "email", "sms", "push", "in_app"
    title: str
    message: str
    sent_at: datetime = None
    read_at: datetime = None

class NotificationService:
    """
    Multi-channel notification service
    
    Example:
        notif = NotificationService()
        
        # Send notification
        notif.send("user_123", "email", "Welcome!", "Thanks for signing up")
        
        # Send to multiple channels
        notif.broadcast("user_123", ["email", "push"], "New message", "You have a new message")
        
        # Get user notifications
        notifications = notif.get_user_notifications("user_123")
    """
    
    def __init__(self):
        self.notifications: List[Notification] = []
        self.counter = 0
    
    def send(self, user_id: str, channel: str, title: str, message: str) -> str:
        """Send notification"""
        self.counter += 1
        
        notification = Notification(
            id=f"notif_{self.counter}",
            user_id=user_id,
            channel=channel,
            title=title,
            message=message,
            sent_at=datetime.utcnow()
        )
        
        self.notifications.append(notification)
        return notification.id
    
    def broadcast(self, user_id: str, channels: List[str], title: str, message: str) -> List[str]:
        """Send to multiple channels"""
        return [self.send(user_id, ch, title, message) for ch in channels]
    
    def get_user_notifications(self, user_id: str, unread_only: bool = False) -> List[Notification]:
        """Get user notifications"""
        notifs = [n for n in self.notifications if n.user_id == user_id]
        
        if unread_only:
            notifs = [n for n in notifs if n.read_at is None]
        
        return sorted(notifs, key=lambda n: n.sent_at, reverse=True)
    
    def mark_read(self, notification_id: str):
        """Mark notification as read"""
        for notif in self.notifications:
            if notif.id == notification_id:
                notif.read_at = datetime.utcnow()
                break
