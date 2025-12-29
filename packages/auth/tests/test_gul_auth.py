"""
Tests for GUL Authentication Framework
"""

import pytest
from datetime import datetime, timedelta
from packages.auth.gul_auth import AuthManager, AuthConfig, TokenPair

@pytest.fixture
def auth_manager():
    """Create AuthManager for testing"""
    config = AuthConfig(secret_key="test-secret-key")
    return AuthManager(config)

class TestPasswordHashing:
    def test_hash_password_creates_hash_and_salt(self, auth_manager):
        password = "test_password_123"
        hash_val, salt =auth_manager.hash_password(password)
        
        assert hash_val is not None
        assert salt is not None
        assert len(hash_val) > 0
        assert len(salt) > 0
    
    def test_verify_password_correct(self, auth_manager):
        password = "correct_password"
        hash_val, salt = auth_manager.hash_password(password)
        
        assert auth_manager.verify_password(password, hash_val, salt) is True
    
    def test_verify_password_incorrect(self, auth_manager):
        password = "correct_password"
        hash_val, salt = auth_manager.hash_password(password)
        
        assert auth_manager.verify_password("wrong_password", hash_val, salt) is False
    
    def test_same_password_different_salts(self, auth_manager):
        password = "same_password"
        hash1, salt1 = auth_manager.hash_password(password)
        hash2, salt2 = auth_manager.hash_password(password)
        
        assert hash1 != hash2  # Different hashes
        assert salt1 != salt2  # Different salts

class TestJWTTokens:
    def test_create_token(self, auth_manager):
        user_id = "user_123"
        token = auth_manager.create_token(user_id)
        
        assert token is not None
        assert isinstance(token, str)
        assert len(token) > 0
    
    def test_create_token_with_claims(self, auth_manager):
        user_id = "user_123"
        claims = {"role": "admin", "email": "test@example.com"}
        token = auth_manager.create_token(user_id, claims)
        
        payload = auth_manager.verify_token(token)
        assert payload is not None
        assert payload["sub"] == user_id
        assert payload["role"] == "admin"
        assert payload["email"] == "test@example.com"
    
    def test_verify_valid_token(self, auth_manager):
        user_id = "user_123"
        token = auth_manager.create_token(user_id)
        
        payload = auth_manager.verify_token(token)
        assert payload is not None
        assert payload["sub"] == user_id
    
    def test_verify_invalid_token(self, auth_manager):
        invalid_token = "invalid.token.here"
        
        payload = auth_manager.verify_token(invalid_token)
        assert payload is None
    
    def test_create_refresh_token(self, auth_manager):
        user_id = "user_123"
        refresh_token = auth_manager.create_refresh_token(user_id)
        
        assert refresh_token is not None
        payload = auth_manager.verify_token(refresh_token)
        assert payload["type"] == "refresh"
    
    def test_create_token_pair(self, auth_manager):
        user_id = "user_123"
        tokens = auth_manager.create_token_pair(user_id)
        
        assert isinstance(tokens, TokenPair)
        assert tokens.access_token is not None
        assert tokens.refresh_token is not None
        
        # Verify both tokens
        access_payload = auth_manager.verify_token(tokens.access_token)
        refresh_payload = auth_manager.verify_token(tokens.refresh_token)
        
        assert access_payload is not None
        assert refresh_payload is not None
        assert refresh_payload["type"] == "refresh"
    
    def test_refresh_access_token(self, auth_manager):
        user_id = "user_123"
        tokens = auth_manager.create_token_pair(user_id)
        
        new_access_token = auth_manager.refresh_access_token(tokens.refresh_token)
        assert new_access_token is not None
        
        payload = auth_manager.verify_token(new_access_token)
        assert payload["sub"] == user_id
    
    def test_refresh_with_access_token_fails(self, auth_manager):
        user_id = "user_123"
        access_token = auth_manager.create_token(user_id)
        
        new_token = auth_manager.refresh_access_token(access_token)
        assert new_token is None

class TestSessionManagement:
    def test_create_session(self, auth_manager):
        user_id = "user_123"
        session_id = auth_manager.create_session(user_id)
        
        assert session_id is not None
        assert isinstance(session_id, str)
        assert len(session_id) > 0
    
    def test_create_session_with_metadata(self, auth_manager):
        user_id = "user_123"
        metadata = {"ip": "127.0.0.1", "user_agent": "Test"}
        session_id = auth_manager.create_session(user_id, metadata)
        
        session = auth_manager.get_session(session_id)
        assert session is not None
        assert session["metadata"]["ip"] == "127.0.0.1"
    
    def test_get_session(self, auth_manager):
        user_id = "user_123"
        session_id = auth_manager.create_session(user_id)
        
        session = auth_manager.get_session(session_id)
        assert session is not None
        assert session["user_id"] == user_id
        assert "created_at" in session
        assert "last_accessed" in session
    
    def test_get_nonexistent_session(self, auth_manager):
        session = auth_manager.get_session("nonexistent_id")
        assert session is None
    
    def test_destroy_session(self, auth_manager):
        user_id = "user_123"
        session_id = auth_manager.create_session(user_id)
        
        # Verify session exists
        assert auth_manager.get_session(session_id) is not None
        
        # Destroy session
        result = auth_manager.destroy_session(session_id)
        assert result is True
        
        # Verify session gone
        assert auth_manager.get_session(session_id) is None
    
    def test_destroy_nonexistent_session(self, auth_manager):
        result = auth_manager.destroy_session("nonexistent_id")
        assert result is False
    
    def test_cleanup_expired_sessions(self, auth_manager):
        # Create sessions (they'll be recent)
        auth_manager.create_session("user1")
        auth_manager.create_session("user2")
        
        # No sessions should be cleaned  up with 24h max age
        removed = auth_manager.cleanup_expired_sessions(86400)
        assert removed == 0
        
        # Manually set old timestamp for testing
        for session_id in list(auth_manager.sessions.keys()):
            auth_manager.sessions[session_id]['last_accessed'] = (
                datetime.utcnow() - timedelta(days=2)
            )
        
        # Now cleanup should remove them
        removed = auth_manager.cleanup_expired_sessions(86400)
        assert removed == 2
        assert len(auth_manager.sessions) == 0

class TestTokenPair:
    def test_to_dict(self, auth_manager):
        user_id = "user_123"
        tokens = auth_manager.create_token_pair(user_id)
        
        dict_repr = tokens.to_dict()
        assert isinstance(dict_repr, dict)
        assert "access_token" in dict_repr
        assert "refresh_token" in dict_repr
        assert dict_repr["access_token"] == tokens.access_token
        assert dict_repr["refresh_token"] == tokens.refresh_token

if __name__ == "__main__":
    pytest.main([__file__, "-v"])
