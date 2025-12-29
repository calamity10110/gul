"""
GUL FTP
FTP Client wrapper.

Status: ✅ Implemented
Priority: Low
"""

import ftplib
import os
from typing import List, Optional

__version__ = "0.1.0"
__all__ = ['FtpClient', 'connect']

class FtpClient:
    """
    FTP Client
    
    Example:
        with FtpClient("ftp.example.com", "user", "pass") as ftp:
            ftp.upload("local.txt", "remote.txt")
            ftp.download("remote.txt", "downloaded.txt")
    """
    
    def __init__(self, host: str, username: str = "anonymous", password: str = "", port: int = 21):
        self.host = host
        self.username = username
        self.password = password
        self.port = port
        self.ftp = ftplib.FTP()
    
    def __enter__(self):
        self.connect()
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()

    def connect(self):
        self.ftp.connect(self.host, self.port)
        self.ftp.login(self.username, self.password)
        
    def list(self, path: str = ".") -> List[str]:
        return self.ftp.nlst(path)
        
    def upload(self, local_path: str, remote_path: Optional[str] = None):
        if remote_path is None:
            remote_path = os.path.basename(local_path)
            
        with open(local_path, "rb") as f:
            self.ftp.storbinary(f"STOR {remote_path}", f)
            
    def download(self, remote_path: str, local_path: Optional[str] = None):
        if local_path is None:
            local_path = os.path.basename(remote_path)
            
        with open(local_path, "wb") as f:
            self.ftp.retrbinary(f"RETR {remote_path}", f.write)
            
    def close(self):
        try:
            self.ftp.quit()
        except:
            self.ftp.close()

def connect(host: str, username: str = "anonymous", **kwargs) -> FtpClient:
    client = FtpClient(host, username, **kwargs)
    client.connect()
    return client
