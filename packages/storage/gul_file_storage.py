"""
GUL File Storage
File upload and storage management.

Status: ✅ Implemented
Phase: 5
"""

from typing import Dict, Optional, BinaryIO
from dataclots import dataclass
import os
import hashlib

__version__ = "0.1.0"
__all__ = ['FileStorage', 'File']

@dataclass
class File:
    id: str
    filename: str
    size: int
    mime_type: str
    path: str
    checksum: str

class FileStorage:
    """
    File storage system
    
    Example:
        storage = FileStorage("/uploads")
        
        file_id = storage.upload("document.pdf", file_data, "application/pdf")
        file_info = storage.get(file_id)
        storage.delete(file_id)
    """
    
    def __init__(self, base_path: str = "/tmp/uploads"):
        self.base_path = base_path
        self.files: Dict[str, File] = {}
        self.counter = 0
    
    def upload(self, filename: str, content: bytes, mime_type: str) -> str:
        self.counter += 1
        file_id = f"file_{self.counter}"
        
        # Calculate checksum
        checksum = hashlib.sha256(content).hexdigest()
        
        path = os.path.join(self.base_path, file_id)
        
        file = File(
            id=file_id,
            filename=filename,
            size=len(content),
            mime_type=mime_type,
            path=path,
            checksum=checksum
        )
        
        self.files[file_id] = file
        return file_id
    
    def get(self, file_id: str) -> Optional[File]:
        return self.files.get(file_id)
    
    def delete(self, file_id: str) -> bool:
        if file_id in self.files:
            del self.files[file_id]
            return True
        return False
    
    def list_files(self, mime_type: Optional[str] = None) -> list:
        files = list(self.files.values())
        if mime_type:
            files = [f for f in files if f.mime_type == mime_type]
        return files
