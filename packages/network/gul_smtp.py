"""
GUL SMTP
SMTP Email client.

Status: ✅ Implemented
Priority: Medium
"""

import smtplib
from email.mime.text import MIMEText
from email.mime.multipart import MIMEMultipart
from typing import List, Optional, Union

__version__ = "0.1.0"
__all__ = ['SmtpClient', 'send_email']

class SmtpClient:
    """
    SMTP Client
    
    Example:
        client = SmtpClient("smtp.example.com", 587, "user", "pass")
        client.send("to@example.com", "Subject", "Body")
    """
    
    def __init__(self, host: str, port: int = 587, username: str = "", password: str = "", use_tls: bool = True):
        self.host = host
        self.port = port
        self.username = username
        self.password = password
        self.use_tls = use_tls
        
    def send(
        self,
        to_addrs: Union[str, List[str]],
        subject: str,
        body: str,
        from_addr: Optional[str] = None,
        html: bool = False
    ):
        """Send email"""
        if isinstance(to_addrs, str):
            to_addrs = [to_addrs]
            
        from_addr = from_addr or self.username
        
        msg = MIMEMultipart()
        msg['From'] = from_addr
        msg['To'] = ", ".join(to_addrs)
        msg['Subject'] = subject
        
        msg.attach(MIMEText(body, 'html' if html else 'plain'))
        
        with smtplib.SMTP(self.host, self.port) as server:
            if self.use_tls:
                server.starttls()
            
            if self.username and self.password:
                server.login(self.username, self.password)
                
            server.send_message(msg)

def send_email(
    host: str,
    port: int,
    username: str,
    password: str,
    to: str,
    subject: str,
    body: str
):
    """Quick email send"""
    client = SmtpClient(host, port, username, password)
    client.send(to, subject, body)
