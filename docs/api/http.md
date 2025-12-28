# HTTP Module API Reference

The `std.http` module provides comprehensive HTTP client and server capabilities for building web applications and services.

## 📚 Table of Contents

- [HTTP Server](#http-server)
- [HTTP Client](#http-client)
- [Request/Response](#requestresponse)
- [Middleware](#middleware)
- [WebSockets](#websockets)
- [Static Files](#static-files)

## 🌐 HTTP Server

### Server Class

```gul
import std.http

server = http.Server(
    host: str = "0.0.0.0",
    port: int = 8080,
    workers: int = 4,
    backlog: int = 128,
    keep_alive: bool = True,
    timeout: duration = duration.seconds(30)
)
```

**Methods:**

- `start()`: Start the HTTP server
- `stop()`: Stop the server gracefully
- `route(path, handler, methods=['GET'])`: Add route handler
- `get(path, handler)`: Add GET route
- `post(path, handler)`: Add POST route
- `put(path, handler)`: Add PUT route
- `delete(path, handler)`: Add DELETE route
- `patch(path, handler)`: Add PATCH route
- `use(middleware)`: Add middleware
- `group(prefix)`: Create route group
- `websocket(path, handler)`: Add WebSocket endpoint

**Example:**

```gul
server = http.Server(port=8080)

@server.get("/")
fn home(request):
    return "Welcome!"

@server.post("/api/users")
fn create_user(request):
    data = request.json()
    user = database.create_user(data)
    return http.json_response(user, status=201)

server.start()
```

## 🔗 HTTP Client

### Client Class

```gul
import std.http

client = http.Client(
    timeout: duration = duration.seconds(30),
    follow_redirects: bool = True,
    max_redirects: int = 10,
    verify_ssl: bool = True,
    user_agent: str = "GUL HTTP Client/0.13.0"
)
```

**Methods:**

- `get(url, headers={}, params={})`: GET request
- `post(url, body=None, json=None, headers={})`: POST request
- `put(url, body=None, json=None, headers={})`: PUT request
- `delete(url, headers={})`: DELETE request
- `patch(url, body=None, json=None, headers={})`: PATCH request
- `head(url, headers={})`: HEAD request
- `options(url, headers={})`: OPTIONS request
- `request(method, url, **kwargs)`: Generic request

**Example:**

```gul
client = http.Client()

# GET request
response = client.get("https://api.example.com/users")
users = response.json()

# POST request with JSON
response = client.post(
    "https://api.example.com/users",
    json={"name": "Alice", "email": "alice@example.com"}
)

# Custom headers
response = client.get(
    "https://api.example.com/protected",
    headers={"Authorization": f"Bearer {token}"}
)

# Query parameters
response = client.get(
    "https://api.example.com/search",
    params={"q": "gul", "limit": 10}
)
```

### Simple HTTP Functions

```gul
import std.http

# Quick HTTP methods
response = http.get("https://example.com")
response = http.post("https://api.example.com", json=data)
response = http.put("https://api.example.com/resource", json=updated_data)
response = http.delete("https://api.example.com/resource")
```

## 📨 Request/Response

### Request Object

```gul
struct Request:
    method: str              # HTTP method (GET, POST, etc.)
    path: str                # Request path
    query: QueryParams       # Query parameters
    headers: Headers         # HTTP headers
    body: bytes              # Raw request body
    client_ip: str           # Client IP address
    user_agent: str          # User agent string
    cookies: Cookies         # Request cookies
    files: Files             # Uploaded files
    user: User?              # Authenticated user (if any)
```

**Methods:**

- `json()`: Parse JSON body
- `form()`: Parse form data
- `text()`: Get body as text
- `bytes()`: Get body as bytes

**Example:**

```gul
@server.post("/api/submit")
fn handle_request(request):
    # Access request properties
    method = request.method
    path = request.path

    # Get query parameters
    page = request.query.get("page", type=int, default=1)

    # Get headers
    auth = request.headers.get("Authorization")

    # Parse JSON body
    data = request.json()

    # Get cookies
    session = request.cookies.get("session_id")

    return http.Response(body="Received!")
```

### Response Object

```gul
struct Response:
    status: int = 200
    body: str | bytes
    headers: map[str, str] = {}
    content_type: str = "text/plain"
```

**Helper Functions:**

```gul
# JSON response
http.json_response(data: Any, status: int = 200): Response

# HTML response
http.html_response(html: str, status: int = 200): Response

# File download
http.file_download(path: str, filename: str? = None): Response

# Redirect
http.redirect(url: str, permanent: bool = False): Response

# Stream response
http.stream_response(generator, content_type: str): Response

# Error responses
http.bad_request(message: str): Response          # 400
http.unauthorized(message: str): Response         # 401
http.forbidden(message: str): Response            # 403
http.not_found(message: str): Response            # 404
http.internal_error(message: str): Response       # 500
```

**Example:**

```gul
@server.get("/api/data")
fn get_data(request):
    data = {"users": [...], "posts": [...]}
    return http.json_response(data)

@server.get("/download/{filename}")
fn download_file(request, filename):
    return http.file_download(f"files/{filename}")

@server.get("/redirect")
fn redirect_page(request):
    return http.redirect("/new-location")
```

## 🔌 Middleware

### Built-in Middleware

```gul
import std.http.middleware

# Logger middleware
server.use(middleware.logger(
    format: str = "combined",  # "combined", "common", "short", "dev"
    output: str = "stdout"     # "stdout", "stderr", or file path
))

# CORS middleware
server.use(middleware.cors(
    allowed_origins: vec[str] = ["*"],
    allowed_methods: vec[str] = ["GET", "POST", "PUT", "DELETE"],
    allowed_headers: vec[str] = ["*"],
    expose_headers: vec[str] = [],
    max_age: duration = duration.hours(24),
    allow_credentials: bool = False
))

# Authentication middleware
server.use(middleware.auth(
    secret_key: str,
    algorithm: str = "HS256",
    exclude_paths: vec[str] = []
))

# Rate limiting middleware
server.use(middleware.rate_limit(
    requests_per_minute: int = 60,
    burst: int = 10,
    key_function: fn(Request): str = client_ip_key
))

# Compression middleware
server.use(middleware.compress(
    algorithms: vec[str] = ["gzip", "deflate", "br"],
    min_size: int = 1024  # Minimum response size to compress
))

# Security headers middleware
server.use(middleware.security_headers(
    hsts: bool = True,
    content_security_policy: str? = None,
    x_frame_options: str = "DENY",
    x_content_type_options: bool = True
))
```

### Custom Middleware

```gul
fn custom_middleware(next_handler):
    fn handler(request):
        # Pre-processing
        print(f"Request: {request.method} {request.path}")

        # Call next middleware/handler
        response = next_handler(request)

        # Post-processing
        response.headers["X-Custom-Header"] = "value"

        return response

    return handler

server.use(custom_middleware)
```

## 🔐 WebSockets

### WebSocket Server

```gul
import std.websocket

ws = websocket.Server()

@ws.on_connect
fn handle_connect(client):
    print(f"Client {client.id} connected")
    client.send({"type": "welcome"})

@ws.on_message
fn handle_message(client, message):
    print(f"Received: {message}")
    client.send({"type": "echo", "data": message})

    # Broadcast to all clients
    ws.broadcast({"type": "update", "data": message})

@ws.on_disconnect
fn handle_disconnect(client):
    print(f"Client {client.id} disconnected")

# Attach to HTTP server
server.websocket("/ws", ws)
```

### WebSocket Client

```gul
import std.websocket

ws = websocket.Client("ws://localhost:8080/ws")

@ws.on_open
fn on_open():
    print("Connected")
    ws.send({"type": "hello"})

@ws.on_message
fn on_message(message):
    print(f"Received: {message}")

@ws.on_close
fn on_close():
    print("Disconnected")

@ws.on_error
fn on_error(error):
    print(f"Error: {error}")

ws.connect()
```

## 📁 Static Files

### Serve Static Files

```gul
# Serve directory
server.static("/static", directory="public/")

# Or use a route
@server.get("/static/*filepath")
fn serve_static(request, filepath):
    return http.serve_file(f"public/{filepath}")

# With caching
server.static("/static", directory="public/", cache={
    "max_age": duration.days(7),
    "immutable": True
})
```

### Single File Response

```gul
@server.get("/download")
fn download(request):
    return http.serve_file(
        path="/path/to/file.pdf",
        content_type="application/pdf",
        download=True,  # Force download
        filename="document.pdf"  # Download filename
    )
```

## 🔧 Advanced Features

### Cookies

```gul
@server.get("/set-cookie")
fn set_cookie(request):
    response = http.Response(body="Cookie set!")
    response.set_cookie(
        name="session_id",
        value="abc123",
        max_age=duration.days(7),
        httponly=True,
        secure=True,
        samesite="Strict"
    )
    return response

@server.get("/read-cookie")
fn read_cookie(request):
    session_id = request.cookies.get("session_id")
    return f"Session: {session_id}"
```

### File Uploads

```gul
@server.post("/upload")
fn handle_upload(request):
    file = request.files.get("document")

    if file:
        file.save(f"uploads/{file.filename}")

        return http.json_response({
            "filename": file.filename,
            "size": file.size,
            "content_type": file.content_type
        })

    return http.bad_request("No file uploaded")
```

### Streaming Responses

```gul
@server.get("/stream")
fn stream_data(request):
    fn generate():
        for i in range(100):
            yield f"data: {i}\n\n"
            time.sleep(0.1)

    return http.stream_response(
        generator=generate(),
        content_type="text/event-stream"
    )
```

### Server-Sent Events (SSE)

```gul
@server.get("/events")
fn event_stream(request):
    fn send_events():
        while True:
            data = get_latest_data()
            yield http.sse_event(data, event="update")
            time.sleep(1)

    return http.sse_response(send_events())
```

## 📚 See Also

- [Web Development Guide](../guides/web-development.md)
- [Database Module](database.md)
- [Secret Management](../guides/secrets.md)

---

**Last Updated**: 2025-12-28  
**Version**: 1.0.0  
**License**: MIT
