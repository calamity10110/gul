"""
GUL IoT
IoT/GPIO Stub.

Status: ✅ Implemented
Priority: Low
"""

__version__ = "0.1.0"
__all__ = ['Pin', 'digital_write', 'digital_read']

class Pin:
    OUT = 1
    IN = 0
    HIGH = 1
    LOW = 0
    
    def __init__(self, number, mode):
        self.number = number
        self.mode = mode
        self.state = Pin.LOW
        
    def write(self, value):
        self.state = value
        print(f"[GPIO] Pin {self.number} -> {value}")
        
    def read(self):
        return self.state

def digital_write(pin, value):
    pin.write(value)
    
def digital_read(pin):
    return pin.read()
