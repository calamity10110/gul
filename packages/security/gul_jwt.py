"""
GUL JWT
JSON Web Token implementation.

Status: ✅ Implemented
Priority: High
"""

from typing import Dict, Any, Optional, Union
import json
import base64
import hmac
import hashlib
import time

__version__ = "0.1.0"
__all__ = ['encode', 'decode', 'JWTError', 'ExpiredSignatureError', 'InvalidTokenError']

class JWTError(Exception): pass
class ExpiredSignatureError(JWTError): pass
class InvalidTokenError(JWTError): pass

def encode(
    payload: Dict[str, Any],
    key: str,
    algorithm: str = "HS256",
    headers: Optional[Dict[str, Any]] = None
) -> str:
    """Encode payload to JWT"""
    header = {"typ": "JWT", "alg": algorithm}
    if headers:
        header.update(headers)
    
    # 1. Header
    header_bytes = json.dumps(header, separators=(",", ":")).encode("utf-8")
    header_b64 = _base64url_encode(header_bytes)
    
    # 2. Payload
    # Add 'iat' (issued at) if not present
    if "iat" not in payload:
        payload["iat"] = int(time.time())
        
    payload_bytes = json.dumps(payload, separators=(",", ":")).encode("utf-8")
    payload_b64 = _base64url_encode(payload_bytes)
    
    # 3. Signature
    signing_input = f"{header_b64}.{payload_b64}".encode("utf-8")
    signature = _sign(signing_input, key, algorithm)
    signature_b64 = _base64url_encode(signature)
    
    return f"{header_b64}.{payload_b64}.{signature_b64}"

def decode(
    token: str,
    key: str,
    algorithms: Union[str, list] = "HS256",
    verify: bool = True,
    options: Optional[Dict[str, Any]] = None
) -> Dict[str, Any]:
    """Decode and verify JWT"""
    if isinstance(algorithms, str):
        algorithms = [algorithms]
        
    try:
        header_b64, payload_b64, signature_b64 = token.split(".")
    except ValueError:
        raise InvalidTokenError("Invalid token format")
    
    # helper for base64 padding
    def _pad(s): return s + "=" * (-len(s) % 4)
    
    try:
        header_json = base64.urlsafe_b64decode(_pad(header_b64)).decode("utf-8")
        header = json.loads(header_json)
        
        payload_json = base64.urlsafe_b64decode(_pad(payload_b64)).decode("utf-8")
        payload = json.loads(payload_json)
        
        signature = base64.urlsafe_b64decode(_pad(signature_b64))
    except Exception:
        raise InvalidTokenError("Invalid token encoding")
    
    if not verify:
        return payload
    
    # Verify algorithm
    if header.get("alg") not in algorithms:
        raise InvalidTokenError("Invalid algorithm")
        
    # Verify signature
    signing_input = f"{header_b64}.{payload_b64}".encode("utf-8")
    expected_signature = _sign(signing_input, key, header["alg"])
    
    if not hmac.compare_digest(signature, expected_signature):
        raise InvalidTokenError("Invalid signature")
    
    # Verify expiration
    now = int(time.time())
    if "exp" in payload and payload["exp"] < now:
        raise ExpiredSignatureError("Token expired")
        
    # Verify Not Before
    if "nbf" in payload and payload["nbf"] > now:
        raise InvalidTokenError("Token not yet valid")
        
    return payload

def _base64url_encode(data: bytes) -> str:
    return base64.urlsafe_b64encode(data).decode("utf-8").replace("=", "")

def _sign(msg: bytes, key: str, alg: str) -> bytes:
    if alg == "HS256":
        return hmac.new(key.encode("utf-8"), msg, hashlib.sha256).digest()
    elif alg == "HS384":
        return hmac.new(key.encode("utf-8"), msg, hashlib.sha384).digest()
    elif alg == "HS512":
        return hmac.new(key.encode("utf-8"), msg, hashlib.sha512).digest()
    else:
        raise NotImplementedError(f"Algorithm {alg} not supported")
