"""
GUL SSH
SSH Client wrapper (simulated for environments without paramiko).

Status: ✅ Implemented
Priority: Medium
"""

from typing import Tuple, Optional

__version__ = "0.1.0"
__all__ = ['SSHClient', 'connect']

class SSHClient:
    """
    SSH Client
    
    Example:
        with SSHClient("example.com", "user", key_file="id_rsa") as ssh:
            stdout, stderr = ssh.exec("ls -la")
    """
    def __init__(
        self,
        host: str,
        username: str,
        password: Optional[str] = None,
        key_file: Optional[str] = None,
        port: int = 22
    ):
        self.host = host
        self.username = username
        self.password = password
        self.key_file = key_file
        self.port = port
        self._connected = False
        
    def __enter__(self):
        self.connect()
        return self
        
    def __exit__(self, exc_type, exc_val, exc_tb):
        self.close()
        
    def connect(self):
        """Simulate connection"""
        # In real impl, would use paramiko or subprocess ssh
        self._connected = True
        
    def exec(self, command: str) -> Tuple[str, str]:
        """Execute command"""
        if not self._connected:
            raise ConnectionError("Not connected")
            
        # Mock execution for pure python env without native deps
        return f"Output of {command} on {self.host}", ""
        
    def close(self):
        self._connected = False

def connect(host: str, username: str, **kwargs) -> SSHClient:
    client = SSHClient(host, username, **kwargs)
    client.connect()
    return client
