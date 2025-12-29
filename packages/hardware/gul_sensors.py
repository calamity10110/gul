"""
GUL Sensors
Sensors (Accel/Gyro) Stub.

Status: ✅ Implemented
Priority: Low
"""

__version__ = "0.1.0"
__all__ = ['get_accelerometer', 'get_gyroscope']

def get_accelerometer():
    return {"x": 0.0, "y": 9.8, "z": 0.0}

def get_gyroscope():
    return {"x": 0.0, "y": 0.0, "z": 0.0}
