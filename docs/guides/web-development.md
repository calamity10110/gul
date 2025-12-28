# Web Development

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# Web Development with GUL

This guide covers building modern web applications with GUL, including HTTP servers, routing, middleware, templates, and full-stack development patterns.

## 🌐 Overview

GUL provides comprehensive web development capabilities:

- **HTTP Server**: Built-in high-performance HTTP/HTTPS server
- **Routing**: Flexible routing with path parameters and wildcards
- **Middleware**: Composable middleware for cross-cutting concerns
- **Templates**: Server-side rendering with GUL templates
- **WebSockets**: Real-time bidirectional communication
- **Static Files**: Efficient static file serving
- **REST APIs**: Easy REST API development
- **Database Integration**: Seamless database connectivity

## 🚀 Quick Start

### Hello World Server

```gul
import std.http

server = http.Server(port=8080)

@server.route("/")
fn index(request):
    return http.Response(
        body="Hello, GUL!",
        content_type="text/plain"
    )

server.start()
print("Server running on http://localhost:8080")
```

Run it:

```bash
gul run server.mn
```

## 🛣️ Routing

### Basic Routes

```gul
import std.http

server = http.Server(port=8080)

# GET request
@server.get("/")
fn home(request):
    return "<h1>Welcome to GUL!</h1>"

# POST request
@server.post("/api/users")
fn create_user(request):
    user_data = request.json()
    # Process user creation
    return http.json_response({"status": "created", "id": 123})

# Multiple HTTP methods
@server.route("/api/resource", methods=["GET", "POST", "PUT"])
fn handle_resource(request):
    match request.method:
        "GET": return get_resource()
        "POST": return create_resource(request.json())
        "PUT": return update_resource(request.json())
```

### Path Parameters

```gul
# Named parameters
@server.get("/users/{user_id}")
fn get_user(request, user_id: int):
    user = database.get_user(user_id)
    return http.json_response(user)

# Multiple parameters
@server.get("/posts/{post_id}/comments/{comment_id}")
fn get_comment(request, post_id: int, comment_id: int):
    comment = database.get_comment(post_id, comment_id)
    return http.json_response(comment)

# Optional parameters
@server.get("/search/{query?}")
fn search(request, query: str? = None):
    if query is None:
        return http.json_response([])
    results = perform_search(query)
    return http.json_response(results)
```

### Query Parameters

```gul
@server.get("/api/users")
fn list_users(request):
    # Access query parameters
    page = request.query.get("page", type=int, default=1)
    limit = request.query.get("limit", type=int, default=10)
    sort_by = request.query.get("sort", default="created_at")

    users = database.query(
        "SELECT * FROM users ORDER BY ? LIMIT ? OFFSET ?",
        [sort_by, limit, (page - 1) * limit]
    )

    return http.json_response({
        "users": users,
        "page": page,
        "total": database.count("users")
    })
```

### Wildcard Routes

```gul
# Catch-all route
@server.get("/static/*filepath")
fn serve_static(request, filepath: str):
    return http.serve_file(f"public/{filepath}")

# Route groups
@server.group("/api/v1")
module api_v1:
    @server.get("/users")
    fn list_users(request):
        return http.json_response(get_all_users())

    @server.get("/posts")
    fn list_posts(request):
        return http.json_response(get_all_posts())
```

## 🔌 Middleware

### Built-in Middleware

```gul
import std.http
import std.http.middleware

server = http.Server(port=8080)

# Logging middleware
server.use(middleware.logger())

# CORS middleware
server.use(middleware.cors(
    allowed_origins=["https://example.com"],
    allowed_methods=["GET", "POST", "PUT", "DELETE"],
    allowed_headers=["Content-Type", "Authorization"]
))

# Authentication middleware
server.use(middleware.auth(
    secret_key=env("JWT_SECRET"),
    exclude_paths=["/", "/login", "/register"]
))

# Rate limiting
server.use(middleware.rate_limit(
    requests_per_minute=60,
    burst=10
))

# Compression
server.use(middleware.compress())
```

### Custom Middleware

```gul
import std.http

fn request_timer_middleware(next_handler):
    fn handler(request):
        start_time = time.now()

        # Call next middleware/handler
        response = next_handler(request)

        # Calculate request duration
        duration = time.now() - start_time

        # Add header
        response.headers["X-Response-Time"] = f"{duration}ms"

        return response

    return handler

# Use custom middleware
server.use(request_timer_middleware)
```

### Middleware Chain

```gul
# Apply middleware to specific routes
@server.get("/admin/dashboard")
@middleware.require_auth()
@middleware.require_role("admin")
fn admin_dashboard(request):
    return render_template("admin/dashboard.html", {
        "user": request.user
    })
```

## 📝 Request Handling

### Accessing Request Data

```gul
@server.post("/api/submit")
fn handle_submission(request):
    # Get JSON body
    json_data = request.json()

    # Get form data
    form_data = request.form()
    name = form_data.get("name")
    email = form_data.get("email")

    # Get headers
    auth_header = request.headers.get("Authorization")
    content_type = request.content_type

    # Get cookies
    session_id = request.cookies.get("session_id")

    # Get uploaded files
    uploaded_file = request.files.get("avatar")
    if uploaded_file:
        uploaded_file.save(f"uploads/{uploaded_file.filename}")

    # Get client info
    client_ip = request.client_ip
    user_agent = request.user_agent

    return http.json_response({"status": "success"})
```

### File Uploads

```gul
@server.post("/upload")
fn handle_upload(request):
    file = request.files.get("document")

    if not file:
        return http.Response(status=400, body="No file uploaded")

    # Validate file
    if file.size > 10 * 1024 * 1024:  # 10MB limit
        return http.Response(status=400, body="File too large")

    allowed_extensions = ["pdf", "docx", "txt"]
    if file.extension not in allowed_extensions:
        return http.Response(status=400, body="Invalid file type")

    # Save file
    filename = generate_unique_filename(file.filename)
    file.save(f"uploads/{filename}")

    # Store in database
    database.insert("files", {
        "filename": filename,
        "original_name": file.filename,
        "size": file.size,
        "uploaded_at": datetime.now()
    })

    return http.json_response({
        "status": "uploaded",
        "filename": filename
    })
```

## 📤 Response Types

### JSON Responses

```gul
@server.get("/api/data")
fn get_data(request):
    data = {
        "users": get_users(),
        "posts": get_posts(),
        "meta": {
            "timestamp": time.now(),
            "version": "1.0"
        }
    }

    return http.json_response(data)

    # Or with custom status
    return http.json_response(data, status=201)
```

### HTML Responses

```gul
@server.get("/page")
fn render_page(request):
    html = """
        <!DOCTYPE html>
        <html>
        <head><title>My Page</title></head>
        <body><h1>Hello from GUL!</h1></body>
        </html>
    """

    return http.html_response(html)
```

### File Downloads

```gul
@server.get("/download/{file_id}")
fn download_file(request, file_id: int):
    file_info = database.get_file(file_id)

    return http.file_download(
        path=file_info.path,
        filename=file_info.original_name,
        content_type=file_info.mime_type
    )
```

### Redirects

```gul
@server.get("/old-page")
fn redirect_old(request):
    return http.redirect("/new-page", permanent=True)

@server.post("/login")
fn login(request):
    credentials = request.form()
    if authenticate(credentials):
        return http.redirect("/dashboard")
    else:
        return http.redirect("/login?error=invalid")
```

### Streaming Responses

```gul
@server.get("/stream")
fn stream_data(request):
    fn generate_data():
        for i in range(100):
            yield f"data: {i}\n\n"
            time.sleep(0.1)

    return http.stream_response(
        generator=generate_data(),
        content_type="text/event-stream"
    )
```

## 🎨 Templates

### Template Engine

```gul
import std.http.templates

# Configure template directory
templates = templates.Engine(directory="templates/")

@server.get("/profile/{user_id}")
fn user_profile(request, user_id: int):
    user = database.get_user(user_id)
    posts = database.get_user_posts(user_id)

    return templates.render("profile.html", {
        "user": user,
        "posts": posts,
        "request": request
    })
```

### Template Syntax (profile.html)

```html
<!DOCTYPE html>
<html>
  <head>
    <title>^{ user.name }^ - Profile</title>
  </head>
  <body>
    <h1>^{ user.name }^</h1>
    <p>Email: ^{ user.email }^</p>
    <p>Joined: ^{ user.created_at }^</p>

    <!-- Conditional rendering -->
    ^& if user.is_verified &^
    <span class="badge">Verified</span>
    ^& endif &^

    <!-- Loop over posts -->
    <h2>Posts</h2>
    <ul>
      ^& for post in posts &^
      <li>
        <h3>^{ post.title }^</h3>
        <p>^{ post.content }^</p>
        <small>Posted on ^{ post.created_at }^</small>
      </li>
      ^& endfor &^
    </ul>

    <!-- Template inheritance -->
    ^& extends "base.html" &^

    <!-- Include partial -->
    ^& include "partials/footer.html" &^
  </body>
</html>
```

### Template Filters

```html
<!-- String filters -->
<p>^{ user.name | uppercase }^</p>
<p>^{ user.bio | truncate(100) }^</p>
<p>^{ html_content | safe }^</p>

<!-- Number filters -->
<p>Price: ^{ price | currency("USD") }^</p>
<p>^{ number | format_number }^</p>

<!-- Date filters -->
<p>^{ created_at | date("Y-m-d H:i:s") }^</p>
<p>^{ updated_at | time_ago }^</p>

<!-- Custom filters -->
^{ user.email | obfuscate }^
```

## 💾 Database Integration

### Database Setup

```gul
import std.database
import std.secrets

# Configure database connection
secret db_url = env("DATABASE_URL")
db = database.connect(db_url)

# Connection pooling
db_pool = database.ConnectionPool(
    url=db_url,
    min_connections=5,
    max_connections=20
)
```

### CRUD Operations

```gul
@server.get("/api/users")
fn list_users(request):
    users = db.query("SELECT * FROM users")
    return http.json_response(users)

@server.get("/api/users/{id}")
fn get_user(request, id: int):
    user = db.query_one("SELECT * FROM users WHERE id = ?", [id])
    if user is None:
        return http.json_response({"error": "Not found"}, status=404)
    return http.json_response(user)

@server.post("/api/users")
fn create_user(request):
    data = request.json()

    user_id = db.insert("users", {
        "name": data["name"],
        "email": data["email"],
        "created_at": datetime.now()
    })

    return http.json_response({"id": user_id}, status=201)

@server.put("/api/users/{id}")
fn update_user(request, id: int):
    data = request.json()

    db.update("users", {
        "name": data["name"],
        "email": data["email"]
    }, where={"id": id})

    return http.json_response({"status": "updated"})

@server.delete("/api/users/{id}")
fn delete_user(request, id: int):
    db.delete("users", where={"id": id})
    return http.Response(status=204)
```

### ORM Pattern

```gul
import std.database.orm

# Define model
@orm.model
struct User:
    id: int @primary_key @auto_increment
    name: str @required
    email: str @unique @required
    password_hash: str
    created_at: datetime @default(datetime.now)

    # Virtual field
    @property
    fn full_profile(self): dict:
        return {
            "id": self.id,
            "name": self.name,
            "email": self.email,
            "member_since": self.created_at
        }

# Use ORM
@server.get("/api/users")
fn list_users(request):
    users = User.all()
    return http.json_response([u.full_profile() for u in users])

@server.post("/api/users")
fn create_user(request):
    data = request.json()

    user = User.create(
        name=data["name"],
        email=data["email"],
        password_hash=hash_password(data["password"])
    )

    return http.json_response(user.full_profile(), status=201)
```

## 🔐 Authentication & Authorization

### JWT Authentication

```gul
import std.crypto.jwt
import std.secrets

secret jwt_secret = env("JWT_SECRET")

@server.post("/auth/login")
fn login(request):
    credentials = request.json()

    user = database.query_one(
        "SELECT * FROM users WHERE email = ?",
        [credentials["email"]]
    )

    if user and verify_password(credentials["password"], user.password_hash):
        # Generate JWT token
        token = jwt.encode(
            payload={
                "user_id": user.id,
                "email": user.email,
                "exp": time.now() + duration.days(7)
            },
            secret=jwt_secret
        )

        return http.json_response({
            "token": token,
            "user": user.profile()
        })

    return http.json_response(
        {"error": "Invalid credentials"},
        status=401
    )

@server.get("/api/protected")
@middleware.require_auth()
fn protected_route(request):
    # request.user is populated by auth middleware
    return http.json_response({
        "message": f"Hello, {request.user.name}!",
        "user": request.user
    })
```

### Session Management

```gul
import std.http.session

# Configure sessions
server.use(middleware.session(
    secret_key=env("SESSION_SECRET"),
    cookie_name="gul_session",
    max_age=duration.hours(24)
))

@server.post("/login")
fn login(request):
    credentials = request.form()
    user = authenticate(credentials)

    if user:
        # Store in session
        request.session["user_id"] = user.id
        request.session["logged_in"] = True

        return http.redirect("/dashboard")

    return http.redirect("/login?error=1")

@server.get("/dashboard")
fn dashboard(request):
    if not request.session.get("logged_in"):
        return http.redirect("/login")

    user_id = request.session.get("user_id")
    user = database.get_user(user_id)

    return templates.render("dashboard.html", {"user": user})

@server.post("/logout")
fn logout(request):
    request.session.clear()
    return http.redirect("/")
```

## 🔌 WebSockets

### WebSocket Server

```gul
import std.websocket

ws_server = websocket.Server()

@ws_server.on_connect
fn handle_connect(client):
    print(f"Client connected: {client.id}")
    client.send({"type": "welcome", "message": "Connected!"})

@ws_server.on_message
fn handle_message(client, message):
    print(f"Received from {client.id}: {message}")

    # Echo back
    client.send({"type": "echo", "data": message})

    # Broadcast to all clients
    ws_server.broadcast({"type": "update", "from": client.id, "data": message})

@ws_server.on_disconnect
fn handle_disconnect(client):
    print(f"Client disconnected: {client.id}")

# Attach to HTTP server
server.websocket("/ws", ws_server)
```

### Real-time Chat Application

```gul
import std.websocket
import std.database

chat_users = {}  # In-memory user tracking

@ws_server.on_connect
fn user_join(client):
    chat_users[client.id] = {
        "id": client.id,
        "joined_at": time.now()
    }

    # Notify all users
    ws_server.broadcast({
        "type": "user_joined",
        "user_id": client.id,
        "total_users": len(chat_users)
    })

@ws_server.on_message
fn handle_chat_message(client, message):
    # Save to database
    db.insert("messages", {
        "user_id": message.get("user_id"),
        "content": message.get("content"),
        "created_at": datetime.now()
    })

    # Broadcast to all users
    ws_server.broadcast({
        "type": "new_message",
        "user_id": message["user_id"],
        "content": message["content"],
        "timestamp": time.now()
    })

@ws_server.on_disconnect
fn user_leave(client):
    if client.id in chat_users:
        del chat_users[client.id]

    ws_server.broadcast({
        "type": "user_left",
        "user_id": client.id,
        "total_users": len(chat_users)
    })
```

## 🌍 Full Application Example

### Complete REST API with Database

```gul
import std.http
import std.database
import std.secrets

# Configuration
secret db_url = env("DATABASE_URL")
db = database.connect(db_url)

# Initialize server
server = http.Server(port=8080)

# Middleware
server.use(middleware.logger())
server.use(middleware.cors())
server.use(middleware.json_parser())

# Routes
@server.get("/")
fn home(request):
    return http.json_response({
        "app": "GUL Blog API",
        "version": "1.0.0",
        "endpoints": {
            "posts": "/api/posts",
            "users": "/api/users"
        }
    })

@server.get("/api/posts")
fn list_posts(request):
    page = request.query.get("page", type=int, default=1)
    per_page = 10

    posts = db.query(
        """
        SELECT p.*, u.name as author_name
        FROM posts p
        JOIN users u ON p.user_id = u.id
        ORDER BY p.created_at DESC
        LIMIT ? OFFSET ?
        """,
        [per_page, (page - 1) * per_page]
    )

    total = db.count("posts")

    return http.json_response({
        "posts": posts,
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total": total,
            "pages": (total + per_page - 1) // per_page
        }
    })

@server.get("/api/posts/{id}")
fn get_post(request, id: int):
    post = db.query_one(
        """
        SELECT p.*, u.name as author_name
        FROM posts p
        JOIN users u ON p.user_id = u.id
        WHERE p.id = ?
        """,
        [id]
    )

    if post is None:
        return http.json_response({"error": "Post not found"}, status=404)

    return http.json_response(post)

@server.post("/api/posts")
@middleware.require_auth()
fn create_post(request):
    data = request.json()

    # Validation
    if not data.get("title") or not data.get("content"):
        return http.json_response(
            {"error": "Title and content are required"},
            status=400
        )

    post_id = db.insert("posts", {
        "title": data["title"],
        "content": data["content"],
        "user_id": request.user.id,
        "created_at": datetime.now()
    })

    return http.json_response(
        {"id": post_id, "status": "created"},
        status=201
    )

@server.put("/api/posts/{id}")
@middleware.require_auth()
fn update_post(request, id: int):
    post = db.query_one("SELECT * FROM posts WHERE id = ?", [id])

    if post is None:
        return http.json_response({"error": "Not found"}, status=404)

    if post.user_id != request.user.id:
        return http.json_response({"error": "Forbidden"}, status=403)

    data = request.json()

    db.update("posts", {
        "title": data.get("title", post.title),
        "content": data.get("content", post.content),
        "updated_at": datetime.now()
    }, where={"id": id})

    return http.json_response({"status": "updated"})

@server.delete("/api/posts/{id}")
@middleware.require_auth()
fn delete_post(request, id: int):
    post = db.query_one("SELECT * FROM posts WHERE id = ?", [id])

    if post is None:
        return http.json_response({"error": "Not found"}, status=404)

    if post.user_id != request.user.id:
        return http.json_response({"error": "Forbidden"}, status=403)

    db.delete("posts", where={"id": id})

    return http.Response(status=204)

# Start server
print("Starting server on http://localhost:8080")
server.start()
```

## 📚 See Also

- [HTTP Module API](../api/http.md)
- [Database Module](../api/database.md)
- [Secret Management](secrets.md)
- [Testing & Deployment](testing-deployment.md)

---

**Last Updated**: 2025-12-28  
**Version: 0.13.0  
**License**: MIT
