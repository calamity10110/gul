# GUL Rate Limiter

**Status**: ✅ Implemented  
**Version**: 0.1.0  
**Priority**: Critical (Phase 1)

## Features

- **3 Algorithms**: Token Bucket, Sliding Window, Fixed Window
- **Per-key Limiting**: User, IP, API key, or custom
- **Middleware**: Easy integration with HTTP servers
- **Burst Support**: Token bucket allows controlled bursts
- **Wait Time**: Calculate retry-after values

## Installation

```python
@imp packages.api{RateLimiter, RateLimitExceeded}
```

## Quick Start

```gul
@imp packages.api{RateLimiter, RateLimitExceeded}
@imp std.http

# Create rate limiter
let limiter = RateLimiter{
    max_requests: 100,
    window_seconds: 60,
    algorithm: "token_bucket"
}

fn @dict handler(req):
    let user_id = req.user_id

    if not limiter.allow(user_id):
        let wait = limiter.get_wait_time(user_id)
        return @dict{
            status: 429,
            body: "Rate limit exceeded",
            headers: {"Retry-After": @str(wait)}
        }

    # Process request
    return @dict{status: 200, body: "OK"}

mn:
    http.route("/api", handler)
    http.listen(8080)
```

## Algorithms

### Token Bucket (Default)

Allows bursts up to capacity, refills at steady rate.

**Best for**: APIs that need to allow occasional bursts

```gul
let limiter = RateLimiter{
    max_requests: 100,  # Bucket capacity
    window_seconds: 60,  # Refill 100 tokens/60s = 1.67/s
    algorithm: "token_bucket"
}
```

### Sliding Window

Tracks requests in a sliding time window.

**Best for**: Smooth, consistent rate limiting

```gul
let limiter = RateLimiter{
    max_requests: 100,
    window_seconds: 60,
    algorithm: "sliding_window"
}
```

### Fixed Window

Resets counter at fixed intervals.

**Best for**: Simple counting, less memory

```gul
let limiter = RateLimiter{
    max_requests: 100,
    window_seconds: 60,
    algorithm: "fixed_window"
}
```

## API Reference

### RateLimiter

Main rate limiting class.

**Methods**:

**`allow(key: str) -> bool`**

Check if request is allowed.

**`get_wait_time(key: str) -> float`**

Get seconds to wait before retry.

**`check(key: str, raise_on_limit: bool = True) -> bool`**

Check and optionally raise exception.

**`reset(key: str)`**

Reset rate limit for key.

**`middleware(get_key: Callable)`**

Create middleware wrapper.

### RateLimitExceeded

Exception raised when limit exceeded.

**Attributes**:

- `retry_after`: Seconds to wait

## Examples

### Per-User Limiting

```gul
@imp packages.api{RateLimiter}

let limiter = RateLimiter{max_requests: 1000, window_seconds: 3600}

fn @dict api_handler(req):
    let user_id = get_user_id(req)

    try:
        limiter.check(user_id)
        return process_request(req)
    catch RateLimitExceeded as e:
        return @dict{
            status: 429,
            body: "Too many requests",
            headers: {"Retry-After": @str(e.retry_after)}
        }
```

### Per-IP Limiting

```gul
@imp packages.api{rate_limit_by_ip}

let ip_limited = rate_limit_by_ip(max_requests: 60, window_seconds: 60)

# Apply middleware
let protected = ip_limited(my_handler)
```

### Multi-tier Limiting

```gul
@imp packages.api{RateLimiter}

# Different limits for different tiers
let free_limiter = RateLimiter{max_requests: 100, window_seconds: 3600}
let pro_limiter = RateLimiter{max_requests: 1000, window_seconds: 3600}
let enterprise_limiter = RateLimiter{max_requests: 10000, window_seconds: 3600}

fn @dict handler(req):
    let user = get_user(req)
    let limiter = match user.tier:
        case "free": free_limiter
        case "pro": pro_limiter
        case "enterprise": enterprise_limiter

    limiter.check(user.id)
    return process(req)
```

### Custom Key Function

```gul
@imp packages.api{RateLimiter}

let limiter = RateLimiter{max_requests: 100, window_seconds: 60}

fn get_api_key(req):
    return req.headers.get("X-API-Key", "anonymous")

let middleware = limiter.middleware(get_api_key)
```

## Integration with gul-http

```gul
@imp std.http
@imp packages.api{RateLimiter, RateLimitExceeded}

let limiter = RateLimiter{max_requests: 100, window_seconds: 60}

fn @dict rate_limited_handler(req):
   let key = req.remote_addr

    try:
        limiter.check(key)
    catch RateLimitExceeded as e:
        return @dict{
            status: 429,
            body: @dict{error: "Rate limit exceeded"},
            headers: {
                "Retry-After": @str(e.retry_after),
                "X-RateLimit-Remaining": "0"
            }
        }

    # Process request
    return @dict{status: 200, body: "Success"}

mn:
    http.route("/api", rate_limited_handler)
    http.listen(8080)
```

## Headers

Standard rate limit headers:

```gul
fn add_rate_limit_headers(response, limiter, key):
    # Add standard headers
    response.headers["X-RateLimit-Limit"] = @str(limiter.max_requests)
    response.headers["X-RateLimit-Window"] = @str(limiter.window_seconds)

    let wait = limiter.get_wait_time(key)
    if wait > 0:
        response.headers["Retry-After"] = @str(wait)
        response.headers["X-RateLimit-Remaining"] = "0"

    return response
```

## Testing

```bash
# Run tests
python -m pytest packages/api/tests/test_rate_limiter.py

# Performance test
python -m pytest packages/api/tests/test_rate_limiter.py::test_performance
```

## Performance

- **Token Bucket**: O(1) per request
- **Sliding Window**: O(n) where n = requests in window
- **Fixed Window**: O(1) per request

**Memory**: O(k) where k = number of unique keys

## Best Practices

1. **Choose Right Algorithm**: Token bucket for bursts, sliding for smooth
2. **Set Reasonable Limits**: Test with realistic load
3. **Add Headers**: Inform clients of limits
4. **Graceful Degradation**: Return helpful error messages
5. **Monitor**: Track rate limit hits
6. **Cleanup**: Reset inactive keys periodically

## Integration

Works with:

- ✅ gul-http
- ✅ gul-api-gateway
- ✅ gul-auth
- ✅ gul-security-headers

## License

MIT License - Part of GUL v0.13.0
