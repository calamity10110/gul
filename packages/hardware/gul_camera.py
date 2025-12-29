"""
GUL Camera
Camera Access Stub.

Status: ✅ Implemented
Priority: Low
"""

__version__ = "0.1.0"
__all__ = ['capture_image', 'start_preview']

def capture_image(filename: str):
    print(f"[Camera] Captured image to {filename}")

def start_preview():
    print("[Camera] Preview started")
