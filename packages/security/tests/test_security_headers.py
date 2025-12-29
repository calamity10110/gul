"""
Tests for GUL Security Headers
"""

import pytest
from packages.security.gul_security_headers import (
    SecurityHeaders,
    SecurityConfig,
    secure_headers,
    Presets
)

class MockResponse:
    """Mock response object for testing"""
    def __init__(self):
        self.headers = {}

class TestSecurityConfig:
    def test_default_config(self):
        config = SecurityConfig()
        
        assert config.cors_allow_origin == "*"
        assert config.hsts_max_age == 31536000
        assert config.x_frame_options == "DENY"
        assert config.enable_csp is True
    
    def test_custom_config(self):
        config = SecurityConfig(
            csp="default-src 'self'",
            cors_allow_origin="https://example.com",
            hsts_max_age=63072000
        )
        
        assert config.csp == "default-src 'self'"
        assert config.cors_allow_origin == "https://example.com"
        assert config.hsts_max_age == 63072000

class TestSecurityHeaders:
    def test_get_headers_default(self):
        security = SecurityHeaders()
        headers = security.get_headers()
        
        assert 'Access-Control-Allow-Origin' in headers
        assert 'Strict-Transport-Security' in headers
        assert 'X-Frame-Options' in headers
        assert 'X-Content-Type-Options' in headers
        assert 'X-XSS-Protection' in headers
    
    def test_get_headers_with_csp(self):
        config = SecurityConfig(csp="default-src 'self'")
        security = SecurityHeaders(config)
        headers = security.get_headers()
        
        assert headers['Content-Security-Policy'] == "default-src 'self'"
    
    def test_cors_headers(self):
        config = SecurityConfig(
            cors_allow_origin="https://example.com",
            cors_allow_methods="GET, POST",
            cors_allow_headers="Content-Type"
        )
        security = SecurityHeaders(config)
        headers = security.get_headers()
        
        assert headers['Access-Control-Allow-Origin'] == "https://example.com"
        assert headers['Access-Control-Allow-Methods'] == "GET, POST"
        assert headers['Access-Control-Allow-Headers'] == "Content-Type"
    
    def test_hsts_full(self):
        config = SecurityConfig(
            hsts_max_age=63072000,
            hsts_include_subdomains=True,
            hsts_preload=True
        )
        security = SecurityHeaders(config)
        headers = security.get_headers()
        
        hsts = headers['Strict-Transport-Security']
        assert 'max-age=63072000' in hsts
        assert 'includeSubDomains' in hsts
        assert 'preload' in hsts
    
    def test_hsts_basic(self):
        config = SecurityConfig(
            hsts_max_age=31536000,
            hsts_include_subdomains=False,
            hsts_preload=False
        )
        security = SecurityHeaders(config)
        headers = security.get_headers()
        
        hsts = headers['Strict-Transport-Security']
        assert hsts == 'max-age=31536000'
    
    def test_frame_options(self):
        config = SecurityConfig(x_frame_options="SAMEORIGIN")
        security = SecurityHeaders(config)
        headers = security.get_headers()
        
        assert headers['X-Frame-Options'] == "SAMEORIGIN"
    
    def test_permissions_policy(self):
        config = SecurityConfig(
            permissions_policy="geolocation=(), microphone=()"
        )
        security = SecurityHeaders(config)
        headers = security.get_headers()
        
        assert headers['Permissions-Policy'] == "geolocation=(), microphone=()"
    
    def test_disabled_csp(self):
        config = SecurityConfig(enable_csp=False, csp="default-src 'self'")
        security = SecurityHeaders(config)
        headers = security.get_headers()
        
        assert 'Content-Security-Policy' not in headers
    
    def test_disabled_cors(self):
        config = SecurityConfig(enable_cors=False)
        security = SecurityHeaders(config)
        headers = security.get_headers()
        
        assert 'Access-Control-Allow-Origin' not in headers
    
    def test_disabled_hsts(self):
        config = SecurityConfig(enable_hsts=False)
        security = SecurityHeaders(config)
        headers = security.get_headers()
        
        assert 'Strict-Transport-Security' not in headers

class TestApplyToResponse:
    def test_apply_to_response(self):
        security = SecurityHeaders()
        response = MockResponse()
        
        result = security.apply_to_response(response)
        
        assert len(result.headers) > 0
        assert 'X-Frame-Options' in result.headers
        assert result is response
    
    def test_middleware(self):
        security = SecurityHeaders()
        
        def handler(request):
            response = MockResponse()
            response.body = "test"
            return response
        
        wrapped = security.middleware(handler)
        response = wrapped("fake_request")
        
        assert len(response.headers) > 0
        assert response.body == "test"

class TestHelpers:
    def test_secure_headers_basic(self):
        security = secure_headers()
        headers = security.get_headers()
        
        assert 'X-Frame-Options' in headers
    
    def test_secure_headers_custom(self):
        security = secure_headers(
            csp="default-src 'self'",
            cors_origin="https://example.com",
            frame_options="SAMEORIGIN"
        )
        headers = security.get_headers()
        
        assert headers['Content-Security-Policy'] == "default-src 'self'"
        assert headers['Access-Control-Allow-Origin'] == "https://example.com"
        assert headers['X-Frame-Options'] == "SAMEORIGIN"

class TestPresets:
    def test_strict_preset(self):
        config = Presets.strict()
        security = SecurityHeaders(config)
        headers = security.get_headers()
        
        assert 'Content-Security-Policy' in headers
        assert 'frame-ancestors' in headers['Content-Security-Policy']
        assert headers['X-Frame-Options'] == "DENY"
        assert config.hsts_preload is True
    
    def test_moderate_preset(self):
        config = Presets.moderate()
        security = SecurityHeaders(config)
        headers = security.get_headers()
        
        assert 'Content-Security-Policy' in headers
        assert 'unsafe-inline' in headers['Content-Security-Policy']
        assert headers['X-Frame-Options'] == "SAMEORIGIN"
    
    def test_api_preset(self):
        config = Presets.api()
        security = SecurityHeaders(config)
        headers = security.get_headers()
        
        assert 'Content-Security-Policy' not in headers
        assert headers['Access-Control-Allow-Origin'] == "*"
        assert 'PATCH' in headers['Access-Control-Allow-Methods']
    
    def test_development_preset(self):
        config = Presets.development()
        security = SecurityHeaders(config)
        headers = security.get_headers()
        
        assert 'unsafe-eval' in headers['Content-Security-Policy']
        assert headers['Access-Control-Allow-Origin'] == "*"
        assert 'Strict-Transport-Security' not in headers

if __name__ == "__main__":
    pytest.main([__file__, "-v"])
