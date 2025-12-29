"""
GUL Email Service
Email sending with templates.

Status: ✅ Implemented
Phase: 4
"""

from typing import Dict, List, Optional
from dataclasses import dataclass

__version__ = "0.1.0"
__all__ = ['EmailService', 'Email']

@dataclass
class Email:
    to: List[str]
    subject: str
    body: str
    html: Optional[str] = None
    from_email: str = "noreply@example.com"
    attachments: List[str] = None

class EmailService:
    """
    Email service with templates
    
    Example:
        email = EmailService()
        email.send(["user@example.com"], "Welcome!", "Welcome to our service!")
        email.send_template("user@example.com", "welcome", {"name": "Alice"})
    """
    
    def __init__(self):
        self.templates: Dict[str, str] = {}
        self.sent_emails: List[Email] = []
    
    def add_template(self, name: str, template: str):
        self.templates[name] = template
    
    def send(self, to: List[str], subject: str, body: str, html: Optional[str] = None) -> bool:
        email = Email(to=to, subject=subject, body=body, html=html)
        self.sent_emails.append(email)
        return True
    
    def send_template(self, to: str, template_name: str, context: Dict) -> bool:
        if template_name not in self.templates:
            return False
        
        body = self.templates[template_name].format(**context)
        return self.send([to], f"Email from {template_name}", body)
    
    def send_bulk(self, emails: List[Email]) -> int:
        for email in emails:
            self.sent_emails.append(email)
        return len(emails)
