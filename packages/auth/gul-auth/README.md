# GUL Auth Package

**Status**: ✅ Implemented  
**Version**: 0.1.0  
**Priority**: Critical (Phase 1)

## Features

- **Password Hashing**: PBKDF2-SHA256 with 100,000 iterations
- **JWT Tokens**: Access and refresh token support
- **Session Management**: Server-side session storage
- **Secure**: Constant-time password comparison
- **Type Safe**: Full type hints

## Installation

```python
# GUL imports
@imp packages.auth{AuthManager, AuthConfig, TokenPair}
```

## Quick Start

```python
@imp packages.auth{AuthManager, AuthConfig}

# Initialize
let config = AuthConfig{
    secret_key: "your-secret-key-here",
    token_expiry: 3600,      # 1 hour
    refresh_expiry: 86400    # 24 hours
}

let auth = AuthManager(config)

# Hash password
let (hash, salt) = auth.hash_password("secure_password")

# Verify password
let is_valid = auth.verify_password("secure_password", hash, salt)

# Create tokens
let tokens = auth.create_token_pair("user_123", {role: "admin"})
print(tokens.access_token)
print(tokens.refresh_token)

# Verify token
let payload = auth.verify_token(tokens.access_token)
if payload:
    print("User:", payload["sub"])
    print("Role:", payload["role"])

# Refresh token
let new_token = auth.refresh_access_token(tokens.refresh_token)

# Session management
let session_id = auth.create_session("user_123", {ip: "127.0.0.1"})
let session = auth.get_session(session_id)
auth.destroy_session(session_id)
```

## API Reference

### AuthConfig

Configuration for authentication system.

**Parameters**:

- `secret_key` (str): Secret key for JWT signing
- `algorithm` (str): JWT algorithm (default: "HS256")
- `token_expiry` (int): Access token expiry in seconds (default: 3600)
- `refresh_expiry` (int): Refresh token expiry in seconds (default: 86400)

### AuthManager

Main authentication manager.

**Methods**:

#### `hash_password(password: str, salt: Optional[str] = None) -> tuple[str, str]`

Hash password with PBKDF2-SHA256.

**Returns**: Tuple of (hash, salt)

#### `verify_password(password: str, password_hash: str, salt: str) -> bool`

Verify password against hash.

**Returns**: True if password matches

#### `create_token(user_id: str, claims: Optional[Dict] = None) -> str`

Create JWT access token.

**Returns**: JWT token string

#### `create_refresh_token(user_id: str) -> str`

Create refresh token.

**Returns**: JWT refresh token

#### `create_token_pair(user_id: str, claims: Optional[Dict] = None) -> TokenPair`

Create both access and refresh tokens.

**Returns**: TokenPair object

#### `verify_token(token: str) -> Optional[Dict]`

Verify and decode JWT token.

**Returns**: Decoded payload or None

#### `refresh_access_token(refresh_token: str) -> Optional[str]`

Get new access token using refresh token.

**Returns**: New access token or None

#### `create_session(user_id: str, metadata: Optional[Dict] = None) -> str`

Create server-side session.

**Returns**: Session ID

#### `get_session(session_id: str) -> Optional[Dict]`

Get session by ID.

**Returns**: Session data or None

#### `destroy_session(session_id: str) -> bool`

Destroy session.

**Returns**: True if session existed

#### `cleanup_expired_sessions(max_age_seconds: int = 86400) -> int`

Remove expired sessions.

**Returns**: Number of sessions removed

## Integration Example

### With GUL HTTP Server

```python
@imp std.http
@imp packages.auth{AuthManager, AuthConfig}

let auth = AuthManager(AuthConfig{secret_key: "secret"})

fn @dict login_handler(req):
    let username = req.json["username"]
    let password = req.json["password"]

    # Verify credentials (example)
    let user = get_user(username)
    if not user:
        return @dict{status: 401, body: "Invalid credentials"}

    if not auth.verify_password(password, user.hash, user.salt):
        return @dict{status: 401, body: "Invalid credentials"}

    # Create tokens
    let tokens = auth.create_token_pair(user.id, {role: user.role})

    return @dict{
        status: 200,
        body: tokens.to_dict()
    }

fn @dict protected_handler(req):
    let token = req.headers.get("Authorization", "").replace("Bearer ", "")
    let payload = auth.verify_token(token)

    if not payload:
        return @dict{status: 401, body: "Unauthorized"}

    return @dict{
        status: 200,
        body: @dict{message: "Hello " + payload["sub"]}
    }

mn:
    http.route("/login", login_handler)
    http.route("/protected", protected_handler)
    http.listen(8080)
```

## Security Best Practices

1. **Secret Key**: Use a strong, random secret key
2. **HTTPS**: Always use HTTPS in production
3. **Token Storage**: Store tokens securely (HTTP-only cookies recommended)
4. **Expiry**: Use short access token expiry (≤ 1 hour)
5. **Refresh Tokens**: Store refresh tokens securely, rotate on use
6. **Rate Limiting**: Implement rate limiting on login endpoints
7. **Password Policy**: Enforce strong password requirements

## Dependencies

- `PyJWT`: JWT token handling
- `hashlib`: Password hashing (standard library)
- `secrets`: Secure random generation (standard library)

## Testing

```bash
# Run tests
python -m pytest packages/auth/tests/

# Run with coverage
python -m pytest --cov=packages.auth packages/auth/tests/
```

## License

MIT License - Part of GUL v0.13.0
