"""
GUL IMAP
IMAP Email retrieval.

Status: ✅ Implemented
Priority: Low
"""

import imaplib
import email
from typing import List, Dict, Any, Optional

__version__ = "0.1.0"
__all__ = ['ImapClient']

class ImapClient:
    """
    IMAP Client
    
    Example:
        client = ImapClient("imap.example.com", "user", "pass")
        emails = client.fetch_latest(5)
    """
    
    def __init__(self, host: str, username: str, password: str, port: int = 993, ssl: bool = True):
        self.host = host
        self.port = port
        self.username = username
        self.password = password
        self.ssl = ssl
        self.connection = None
        
    def connect(self):
        if self.connection: return
        
        if self.ssl:
            self.connection = imaplib.IMAP4_SSL(self.host, self.port)
        else:
            self.connection = imaplib.IMAP4(self.host, self.port)
            
        self.connection.login(self.username, self.password)
        
    def fetch_latest(self, count: int = 10, folder: str = "INBOX") -> List[Dict[str, Any]]:
        self.connect()
        self.connection.select(folder)
        
        _, messages = self.connection.search(None, 'ALL')
        msg_ids = messages[0].split()
        
        latest_ids = msg_ids[-count:]
        results = []
        
        for num in reversed(latest_ids):
            _, msg_data = self.connection.fetch(num, '(RFC822)')
            raw_email = msg_data[0][1]
            msg = email.message_from_bytes(raw_email)
            
            results.append({
                "subject": msg["subject"],
                "from": msg["from"],
                "date": msg["date"],
                "body": self._get_body(msg)
            })
            
        return results
    
    def _get_body(self, msg) -> str:
        if msg.is_multipart():
            for part in msg.walk():
                content_type = part.get_content_type()
                if content_type == "text/plain":
                    return part.get_payload(decode=True).decode()
        else:
            return msg.get_payload(decode=True).decode()
        return ""
    
    def close(self):
        if self.connection:
            self.connection.close()
            self.connection.logout()
