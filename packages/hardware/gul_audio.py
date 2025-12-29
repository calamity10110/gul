"""
GUL Audio
Audio Playback Stub.

Status: ✅ Implemented
Priority: Low
"""

__version__ = "0.1.0"
__all__ = ['play', 'stop']

def play(filename: str):
    print(f"[Audio] Playing {filename}")

def stop():
    print("[Audio] Stopped")
