"""
GUL Authentication Framework
Provides JWT-based authentication and session management.

Status: ✅ Implemented
Priority: Critical
Phase: 1
"""

from typing import Optional, Dict, Any
import jwt
import hashlib
import secrets
from datetime import datetime, timedelta

__version__ = "0.1.0"
__all__ = ['AuthConfig', 'AuthManager', 'TokenPair']

class AuthConfig:
    """Authentication configuration"""
    def __init__(
        self,
        secret_key: str,
        algorithm: str = "HS256",
        token_expiry: int = 3600,
        refresh_expiry: int = 86400
    ):
        self.secret_key = secret_key
        self.algorithm = algorithm
        self.token_expiry = token_expiry
        self.refresh_expiry = refresh_expiry

class TokenPair:
    """Access and refresh token pair"""
    def __init__(self, access_token: str, refresh_token: str):
        self.access_token = access_token
        self.refresh_token = refresh_token
    
    def to_dict(self) -> Dict[str, str]:
        return {
            'access_token': self.access_token,
            'refresh_token': self.refresh_token
        }

class AuthManager:
    """
    Main authentication manager
    
    Features:
    - Password hashing with PBKDF2
    - JWT token creation and validation
    - Refresh token support
    - Session management
    
    Example:
        config = AuthConfig(secret_key="secret")
        auth = AuthManager(config)
        
        # Hash password
        hash, salt = auth.hash_password("password123")
        
        # Create tokens
        tokens = auth.create_token_pair("user_123")
        
        # Verify token
        payload = auth.verify_token(tokens.access_token)
    """
    
    def __init__(self, config: AuthConfig):
        self.config = config
        self.sessions: Dict[str, Dict[str, Any]] = {}
    
    def hash_password(self, password: str, salt: Optional[str] = None) -> tuple[str, str]:
        """
        Hash password with salt using PBKDF2-SHA256
        
        Args:
            password: Plain text password
            salt: Optional salt (generated if not provided)
        
        Returns:
            Tuple of (hash, salt)
        """
        if salt is None:
            salt = secrets.token_hex(32)
        
        hash_obj = hashlib.pbkdf2_hmac(
            'sha256',
            password.encode('utf-8'),
            salt.encode('utf-8'),
            100000  # iterations
        )
        return hash_obj.hex(), salt
    
    def verify_password(self, password: str, password_hash: str, salt: str) -> bool:
        """
        Verify password against hash
        
        Args:
            password: Plain text password to verify
            password_hash: Stored password hash
            salt: Stored salt
        
        Returns:
            True if password matches
        """
        computed_hash, _ = self.hash_password(password, salt)
        return secrets.compare_digest(computed_hash, password_hash)
    
    def create_token(self, user_id: str, claims: Optional[Dict] = None) -> str:
        """
        Create JWT access token
        
        Args:
            user_id: User identifier
            claims: Additional claims to include
        
        Returns:
            JWT token string
        """
        payload = {
            'sub': user_id,
            'iat': datetime.utcnow(),
            'exp': datetime.utcnow() + timedelta(seconds=self.config.token_expiry)
        }
        
        if claims:
            payload.update(claims)
        
        return jwt.encode(payload, self.config.secret_key, algorithm=self.config.algorithm)
    
    def create_refresh_token(self, user_id: str) -> str:
        """Create refresh token"""
        payload = {
            'sub': user_id,
            'type': 'refresh',
            'iat': datetime.utcnow(),
            'exp': datetime.utcnow() + timedelta(seconds=self.config.refresh_expiry)
        }
        
        return jwt.encode(payload, self.config.secret_key, algorithm=self.config.algorithm)
    
    def create_token_pair(self, user_id: str, claims: Optional[Dict] = None) -> TokenPair:
        """
        Create both access and refresh tokens
        
        Args:
            user_id: User identifier
            claims: Additional claims for access token
        
        Returns:
            TokenPair with both tokens
        """
        access_token = self.create_token(user_id, claims)
        refresh_token = self.create_refresh_token(user_id)
        return TokenPair(access_token, refresh_token)
    
    def verify_token(self, token: str) -> Optional[Dict]:
        """
        Verify and decode JWT token
        
        Args:
            token: JWT token string
        
        Returns:
            Decoded payload or None if invalid
        """
        try:
            payload = jwt.decode(
                token,
                self.config.secret_key,
                algorithms=[self.config.algorithm]
            )
            return payload
        except jwt.ExpiredSignatureError:
            return None
        except jwt.InvalidTokenError:
            return None
    
    def refresh_access_token(self, refresh_token: str) -> Optional[str]:
        """
        Refresh access token using refresh token
        
        Args:
            refresh_token: Valid refresh token
        
        Returns:
            New access token or None if invalid
        """
        payload = self.verify_token(refresh_token)
        
        if payload and payload.get('type') == 'refresh':
            return self.create_token(payload['sub'])
        
        return None
    
    def create_session(self, user_id: str, metadata: Optional[Dict] = None) -> str:
        """
        Create a server-side session
        
        Args:
            user_id: User identifier
            metadata: Optional session metadata
        
        Returns:
            Session ID
        """
        session_id = secrets.token_urlsafe(32)
        
        self.sessions[session_id] = {
            'user_id': user_id,
            'created_at': datetime.utcnow(),
            'last_accessed': datetime.utcnow(),
            'metadata': metadata or {}
        }
        
        return session_id
    
    def get_session(self, session_id: str) -> Optional[Dict]:
        """Get session by ID and update last accessed time"""
        session = self.sessions.get(session_id)
        if session:
            session['last_accessed'] = datetime.utcnow()
        return session
    
    def destroy_session(self, session_id: str) -> bool:
        """Destroy a session"""
        if session_id in self.sessions:
            del self.sessions[session_id]
            return True
        return False
    
    def cleanup_expired_sessions(self, max_age_seconds: int = 86400):
        """Remove sessions older than max_age_seconds"""
        now = datetime.utcnow()
        expired = [
            sid for sid, session in self.sessions.items()
            if (now - session['last_accessed']).total_seconds() > max_age_seconds
        ]
        
        for sid in expired:
            del self.sessions[sid]
        
        return len(expired)
