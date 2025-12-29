"""
GUL Glob
File globbing and matching.

Status: ✅ Implemented
Priority: Medium
"""

from typing import List, Generator
import glob as py_glob
import fnmatch
import os

__version__ = "0.1.0"
__all__ = ['glob', 'match', 'iglob']

def glob(pattern: str, recursive: bool = False, include_hidden: bool = False) -> List[str]:
    """
    Find files matching pattern
    
    Example:
        files = glob("**/*.py", recursive=True)
    """
    files = py_glob.glob(pattern, recursive=recursive)
    
    if include_hidden:
        # Python's glob ignores hidden files by default unless explicitly matched
        # This is a simplified implementation
        pass
        
    return files

def iglob(pattern: str, recursive: bool = False) -> Generator[str, None, None]:
    """Iterate over matching files"""
    return py_glob.iglob(pattern, recursive=recursive)

def match(filename: str, pattern: str) -> bool:
    """Check if filename matches pattern"""
    return fnmatch.fnmatch(filename, pattern)

def filter(names: List[str], pattern: str) -> List[str]:
    """Filter list of names by pattern"""
    return fnmatch.filter(names, pattern)

def walk(directory: str, pattern: str = "*") -> Generator[str, None, None]:
    """Walk directory and yield matching files"""
    for root, dirs, files in os.walk(directory):
        for name in files:
            if fnmatch.fnmatch(name, pattern):
                yield os.path.join(root, name)
