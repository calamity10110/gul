"""
GUL JWT (JSON Web Tokens)
Standalone JWT implementation.

Status: ✅ Implemented
Priority: High
Phase: 1
"""

import json
import hmac
import hashlib
import base64
from typing import Dict, Optional, Any
from datetime import datetime, timedelta

__version__ = "0.1.0"
__all__ = ['JWT', 'JWTError', 'TokenExpired', 'InvalidToken']

class JWTError(Exception):
    """Base JWT exception"""
    pass

class TokenExpired(JWTError):
    """Token has expired"""
    pass

class InvalidToken(JWTError):
    """Token is invalid"""
    pass

class JWT:
    """
    Standalone JWT implementation
    
    Supports HS256, HS384, HS512 algorithms.
    
    Example:
        jwt = JWT(secret="my-secret-key")
        
        # Encode
        token = jwt.encode({'user_id': '123', 'role': 'admin'})
        
        # Decode
        payload = jwt.decode(token)
        print(payload['user_id'])
    """
    
    ALGORITHMS = {
        'HS256': hashlib.sha256,
        'HS384': hashlib.sha384,
        'HS512': hashlib.sha512
    }
    
    def __init__(self, secret: str, algorithm: str = 'HS256'):
        """
        Initialize JWT handler
        
        Args:
            secret: Secret key for signing
            algorithm: Algorithm to use (HS256, HS384, HS512)
        """
        if algorithm not in self.ALGORITHMS:
            raise ValueError(f"Unsupported algorithm: {algorithm}")
        
        self.secret = secret.encode('utf-8')
        self.algorithm = algorithm
    
    def encode(
        self,
        payload: Dict[str, Any],
        expires_in: Optional[int] = None
    ) -> str:
        """
        Encode payload as JWT
        
        Args:
            payload: Data to encode
            expires_in: Expiry time in seconds from now
        
        Returns:
            JWT token string
        """
        # Add standard claims
        now = datetime.utcnow()
        claims = payload.copy()
        claims['iat'] = int(now.timestamp())
        
        if expires_in:
            claims['exp'] = int((now + timedelta(seconds=expires_in)).timestamp())
        
        # Create header
        header = {
            'typ': 'JWT',
            'alg': self.algorithm
        }
        
        # Encode parts
        header_encoded = self._base64url_encode(json.dumps(header))
        payload_encoded = self._base64url_encode(json.dumps(claims))
        
        # Create signature
        message = f"{header_encoded}.{payload_encoded}"
        signature = self._sign(message)
        signature_encoded = self._base64url_encode(signature)
        
        return f"{message}.{signature_encoded}"
    
    def decode(
        self,
        token: str,
        verify: bool = True
    ) -> Dict[str, Any]:
        """
        Decode and verify JWT
        
        Args:
            token: JWT token string
            verify: Whether to verify signature
        
        Returns:
            Decoded payload
        
        Raises:
            InvalidToken: If token is malformed
            TokenExpired: If token has expired
        """
        try:
            parts = token.split('.')
            if len(parts) != 3:
                raise InvalidToken("Invalid token format")
            
            header_encoded, payload_encoded, signature_encoded = parts
            
            # Verify signature if requested
            if verify:
                message = f"{header_encoded}.{payload_encoded}"
                expected_signature = self._sign(message)
                actual_signature = self._base64url_decode(signature_encoded)
                
                if not hmac.compare_digest(expected_signature, actual_signature):
                    raise InvalidToken("Invalid signature")
            
            # Decode payload
            payload_json = self._base64url_decode(payload_encoded)
            payload = json.loads(payload_json)
            
            # Check expiry
            if 'exp' in payload:
                exp_time = datetime.fromtimestamp(payload['exp'])
                if datetime.utcnow() > exp_time:
                    raise TokenExpired("Token has expired")
            
            return payload
        
        except json.JSONDecodeError:
            raise InvalidToken("Invalid JSON in token")
        except Exception as e:
            if isinstance(e, (InvalidToken, TokenExpired)):
                raise
            raise InvalidToken(f"Token decode error: {str(e)}")
    
    def _sign(self, message: str) -> bytes:
        """Create HMAC signature"""
        hash_func = self.ALGORITHMS[self.algorithm]
        return hmac.new(
            self.secret,
            message.encode('utf-8'),
            hash_func
        ).digest()
    
    def _base64url_encode(self, data: Any) -> str:
        """Base64 URL-safe encoding"""
        if isinstance(data, str):
            data = data.encode('utf-8')
        elif isinstance(data, dict):
            data = json.dumps(data).encode('utf-8')
        
        encoded = base64.urlsafe_b64encode(data)
        return encoded.decode('utf-8').rstrip('=')
    
    def _base64url_decode(self, data: str) -> bytes:
        """Base64 URL-safe decoding"""
        # Add padding if needed
        padding = 4 - (len(data) % 4)
        if padding != 4:
            data += '=' * padding
        
        return base64.urlsafe_b64decode(data)
    
    @staticmethod
    def verify_token(token: str, secret: str, algorithm: str = 'HS256') -> Optional[Dict]:
        """
        Quick helper to verify a token
        
        Args:
            token: JWT token
            secret: Secret key
            algorithm: Algorithm used
        
        Returns:
            Payload if valid, None if invalid
        """
        try:
            jwt = JWT(secret, algorithm)
            return jwt.decode(token)
        except (InvalidToken, TokenExpired):
            return None
