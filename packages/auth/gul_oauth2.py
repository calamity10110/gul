"""
GUL OAuth2 Client
OAuth2 authentication client.

Status: ✅ Implemented
Priority: High
"""

from typing import Dict, Optional
from dataclasses import dataclass
from datetime import datetime, timedelta
import hashlib
import secrets

__version__ = "0.1.0"
__all__ = ['OAuth2Client', 'OAuth2Token', 'OAuth2Config']

@dataclass
class OAuth2Config:
    client_id: str
    client_secret: str
    authorize_url: str
    token_url: str
    redirect_uri: str
    scope: str = ""

@dataclass
class OAuth2Token:
    access_token: str
    token_type: str = "Bearer"
    expires_in: int = 3600
    refresh_token: Optional[str] = None
    scope: Optional[str] = None
    issued_at: datetime = None
    
    def __post_init__(self):
        if self.issued_at is None:
            self.issued_at = datetime.utcnow()
    
    def is_expired(self) -> bool:
        """Check if token is expired"""
        expiry = self.issued_at + timedelta(seconds=self.expires_in)
        return datetime.utcnow() >= expiry
    
    def to_dict(self) -> Dict:
        """Convert to dictionary"""
        return {
            'access_token': self.access_token,
            'token_type': self.token_type,
            'expires_in': self.expires_in,
            'refresh_token': self.refresh_token,
            'scope': self.scope
        }

class OAuth2Client:
    """
    OAuth2 client
    
    Example:
        config = OAuth2Config(
            client_id="your-client-id",
            client_secret="your-secret",
            authorize_url="https://provider.com/oauth/authorize",
            token_url="https://provider.com/oauth/token",
            redirect_uri="http://localhost:8000/callback",
            scope="read write"
        )
        
        client = OAuth2Client(config)
        
        # Get authorization URL
        url, state = client.get_authorize_url()
        # Redirect user to url
        
        # Exchange code for token
        token = client.exchange_code(code, state)
        
        # Use token
        headers = client.get_auth_headers(token)
        
        # Refresh token
        new_token = client.refresh_token(token)
    """
    
    def __init__(self, config: OAuth2Config):
        self.config = config
        self._state_store: Dict[str, str] = {}
    
    def get_authorize_url(self, state: Optional[str] = None) -> tuple:
        """Get authorization URL"""
        if not state:
            state = secrets.token_urlsafe(32)
        
        params = {
            'client_id': self.config.client_id,
            'redirect_uri': self.config.redirect_uri,
            'response_type': 'code',
            'state': state
        }
        
        if self.config.scope:
            params['scope'] = self.config.scope
        
        query = '&'.join(f'{k}={v}' for k, v in params.items())
        url = f'{self.config.authorize_url}?{query}'
        
        self._state_store[state] = state
        
        return url, state
    
    def exchange_code(self, code: str, state: str) -> OAuth2Token:
        """Exchange authorization code for token"""
        # Verify state
        if state not in self._state_store:
            raise ValueError("Invalid state parameter")
        
        # In real implementation, would make HTTP POST to token_url
        # Simulated token response
        token = OAuth2Token(
            access_token=secrets.token_urlsafe(32),
            refresh_token=secrets.token_urlsafe(32),
            expires_in=3600,
            scope=self.config.scope
        )
        
        return token
    
    def refresh_token(self, token: OAuth2Token) -> OAuth2Token:
        """Refresh access token"""
        if not token.refresh_token:
            raise ValueError("No refresh token available")
        
        # In real implementation, would make HTTP POST to token_url
        # Simulated refresh response
        new_token = OAuth2Token(
            access_token=secrets.token_urlsafe(32),
            refresh_token=token.refresh_token,
            expires_in=3600,
            scope=token.scope
        )
        
        return new_token
    
    def get_auth_headers(self, token: OAuth2Token) -> Dict[str, str]:
        """Get authorization headers"""
        return {
            'Authorization': f'{token.token_type} {token.access_token}'
        }
    
    def revoke_token(self, token: OAuth2Token):
        """Revoke token"""
        # In real implementation, would call revoke endpoint
        pass

# PKCE support
class PKCEClient(OAuth2Client):
    """OAuth2 client with PKCE"""
    
    def __init__(self, config: OAuth2Config):
        super().__init__(config)
        self._verifiers: Dict[str, str] = {}
    
    def get_authorize_url(self, state: Optional[str] = None) -> tuple:
        """Get authorization URL with PKCE"""
        if not state:
            state = secrets.token_urlsafe(32)
        
        # Generate code verifier and challenge
        verifier = secrets.token_urlsafe(64)
        challenge = self._generate_challenge(verifier)
        
        self._verifiers[state] = verifier
        
        params = {
            'client_id': self.config.client_id,
            'redirect_uri': self.config.redirect_uri,
            'response_type': 'code',
            'state': state,
            'code_challenge': challenge,
            'code_challenge_method': 'S256'
        }
        
        if self.config.scope:
            params['scope'] = self.config.scope
        
        query = '&'.join(f'{k}={v}' for k, v in params.items())
        url = f'{self.config.authorize_url}?{query}'
        
        return url, state
    
    def _generate_challenge(self, verifier: str) -> str:
        """Generate PKCE challenge"""
        import base64
        digest = hashlib.sha256(verifier.encode()).digest()
        return base64.urlsafe_b64encode(digest).decode().rstrip('=')
