# GUL Security Headers

**Status**: ✅ Implemented  
**Version**: 0.1.0  
**Priority**: Critical (Phase 1)

## Features

- **CSP**: Content Security Policy
- **CORS**: Cross-Origin Resource Sharing
- **HSTS**: HTTP Strict Transport Security
- **XSS Protection**: X-XSS-Protection header
- **Frame Options**: Clickjacking protection
- **Presets**: Strict, Moderate, API, Development

## Installation

```python
@imp packages.security{SecurityHeaders, SecurityConfig, Presets}
```

## Quick Start

```gul
@imp packages.security{secure_headers}
@imp std.http

# Quick setup
let security = secure_headers(
    csp: "default-src 'self'",
    cors_origin: "https://example.com"
)

fn @dict handler(req):
    let response = @dict{
        status: 200,
        body: "Hello!"
    }

    # Apply security headers
    response.headers = security.get_headers()
    return response

mn:
    http.route("/", handler)
    http.listen(8080)
```

## API Reference

### SecurityConfig

Configuration for security headers.

**Parameters**:

- `csp` (str): Content Security Policy
- `cors_allow_origin` (str): CORS allowed origin (default: "\*")
- `cors_allow_methods` (str): Allowed HTTP methods
- `cors_allow_headers` (str): Allowed headers
- `hsts_max_age` (int): HSTS max age in seconds
- `hsts_include_subdomains` (bool): Include subdomains in HSTS
- `x_frame_options` (str): Frame options (DENY/SAMEORIGIN)
- `x_xss_protection` (str): XSS protection header
- `referrer_policy` (str): Referrer policy

### SecurityHeaders

Main class for applying security headers.

#### Methods

**`get_headers() -> Dict[str, str]`**

Returns dictionary of all security headers.

**`apply_to_response(response) -> response`**

Applies headers to response object.

**`middleware(handler) -> Callable`**

Creates middleware wrapper.

### Presets

Common security configurations.

**`Presets.strict()`** - Maximum security
**`Presets.moderate()`** - Balanced security
**`Presets.api()`** - API-focused (CORS enabled)
**`Presets.development()`** - Relaxed for development

## Examples

### Strict Security

```gul
@imp packages.security{SecurityHeaders, Presets}

let config = Presets.strict()
let security = SecurityHeaders(config)

fn @dict handler(req):
    let response = make_response()
    return security.apply_to_response(response)
```

### Custom Configuration

```gul
@imp packages.security{SecurityHeaders, SecurityConfig}

let config = SecurityConfig{
    csp: "default-src 'self'; script-src 'self' https://cdn.example.com",
    cors_allow_origin: "https://app.example.com",
    hsts_max_age: 63072000,
    x_frame_options: "SAMEORIGIN"
}

let security = SecurityHeaders(config)
```

### Middleware Integration

```gul
@imp std.http
@imp packages.security{secure_headers}

let security = secure_headers(
    csp: "default-src 'self'",
    cors_origin: "https://example.com"
)

# Wrap handler with middleware
let protected_handler = security.middleware(my_handler)

mn:
    http.route("/api", protected_handler)
    http.listen(8080)
```

### API Server

```gul
@imp packages.security{SecurityHeaders, Presets}

let security = SecurityHeaders(Presets.api())

fn @dict api_handler(req):
    let data = process_request(req)
    let response = @dict{
        status: 200,
        body: data,
        headers: security.get_headers()
    }
    return response
```

## Security Headers Explained

### Content Security Policy (CSP)

Prevents XSS and data injection attacks.

```gul
csp: "default-src 'self'; script-src 'self' 'unsafe-inline'"
```

### CORS (Cross-Origin Resource Sharing)

Controls which origins can access your API.

```gul
cors_allow_origin: "https://app.example.com"
cors_allow_methods: "GET, POST, PUT, DELETE"
```

### HSTS (HTTP Strict Transport Security)

Forces HTTPS connections.

```gul
hsts_max_age: 31536000  # 1 year
hsts_include_subdomains: true
```

### X-Frame-Options

Prevents clickjacking attacks.

```gul
x_frame_options: "DENY"  # or "SAMEORIGIN"
```

## Best Practices

1. **Use HTTPS**: Always use HTTPS in production
2. **Strict CSP**: Start with strict CSP, relax as needed
3. **HSTS**: Enable HSTS with long max-age
4. **Frame Protection**: Use DENY unless embedding needed
5. **CORS**: Be specific with allowed origins
6. **Testing**: Test headers in staging first

## Production Configuration

```gul
@imp packages.security{SecurityConfig, SecurityHeaders}

let config = SecurityConfig{
    csp: "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data: https:; font-src 'self'; connect-src 'self' https://api.example.com",
    cors_allow_origin: "https://app.example.com",
    cors_allow_methods: "GET, POST, PUT, DELETE",
    cors_allow_headers: "Content-Type, Authorization",
    hsts_max_age: 63072000,
    hsts_include_subdomains: true,
    hsts_preload: true,
    x_frame_options: "DENY",
    permissions_policy: "geolocation=(), microphone=(), camera=()"
}

let security = SecurityHeaders(config)
```

## Testing

```bash
# Run tests
python -m pytest packages/security/tests/

# Check headers
curl -I http://localhost:8080
```

## Integration

Works with:

- ✅ gul-http
- ✅ gul-api-gateway
- ✅ gul-auth
- ✅ Any HTTP framework

## License

MIT License - Part of GUL v0.13.0
