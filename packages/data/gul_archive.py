"""
GUL Archive
Archive manipulation (Zip/Tar).

Status: ✅ Implemented
Priority: Medium
"""

import zipfile
import tarfile
import os
import shutil
from typing import List, Optional

__version__ = "0.1.0"
__all__ = ['extract', 'make_archive', 'Zip', 'Tar']

class Zip:
    @staticmethod
    def extract(filename: str, path: str = "."):
        with zipfile.ZipFile(filename, 'r') as z:
            z.extractall(path)
            
    @staticmethod
    def create(filename: str, items: List[str]):
        with zipfile.ZipFile(filename, 'w', zipfile.ZIP_DEFLATED) as z:
            for item in items:
                if os.path.isdir(item):
                    for root, _, files in os.walk(item):
                        for file in files:
                            file_path = os.path.join(root, file)
                            arcname = os.path.relpath(file_path, os.path.dirname(item))
                            z.write(file_path, arcname)
                else:
                    z.write(item, os.path.basename(item))

class Tar:
    @staticmethod
    def extract(filename: str, path: str = "."):
        mode = "r:gz" if filename.endswith(".gz") else "r"
        with tarfile.open(filename, mode) as t:
            t.extractall(path, filter='data') # 'data' filter for safety (python 3.12+)
            
    @staticmethod
    def create(filename: str, items: List[str]):
        mode = "w:gz" if filename.endswith(".gz") else "w"
        with tarfile.open(filename, mode) as t:
            for item in items:
                t.add(item, arcname=os.path.basename(item))

def extract(filename: str, path: str = "."):
    """Auto-detect and extract"""
    if zipfile.is_zipfile(filename):
        Zip.extract(filename, path)
    elif tarfile.is_tarfile(filename):
        Tar.extract(filename, path)
    else:
        raise ValueError(f"Unknown archive format: {filename}")

def make_archive(base_name: str, format: str, root_dir: str):
    """Wrapper around shutil.make_archive"""
    shutil.make_archive(base_name, format, root_dir)
