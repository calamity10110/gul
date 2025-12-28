# Building a Web Server Tutorial

Learn to build a complete REST API web server with GUL, including routing, database integration, and authentication.

## 🎯 Project: Blog API

We'll create a RESTful API for a blog with users, posts, and comments.

## 📦 Step 1: Project Setup

```bash
mkdir blog-api
cd blog-api
gul init
```

Install dependencies (add to `gul.toml`):

```toml
[dependencies]
http = "*"
database = "*"
```

## 🗄️ Step 2: Database Schema

Create `schema.sql`:

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    published BOOLEAN DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    post_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (post_id) REFERENCES posts(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

## 🏗️ Step 3: Database Models

Create `src/models.mn`:

```gul
import std.database.orm

@orm.model
struct User:
    id: int @primary_key @auto_increment
    username: str @unique @required
    email: str @unique @required
    password_hash: str
    created_at: datetime @default(datetime.now)

    posts: vec[Post] @has_many
    comments: vec[Comment] @has_many

@orm.model
struct Post:
    id: int @primary_key @auto_increment
    title: str @required
    content: str
    user_id: int @foreign_key("users.id")
    published: bool @default(False)
    created_at: datetime @default(datetime.now)

    user: User @belongs_to
    comments: vec[Comment] @has_many

@orm.model
struct Comment:
    id: int @primary_key @auto_increment
    content: str @required
    post_id: int @foreign_key("posts.id")
    user_id: int @foreign_key("users.id")
    created_at: datetime @default(datetime.now)

    post: Post @belongs_to
    user: User @belongs_to
```

## 🔐 Step 4: Authentication

Create `src/auth.mn`:

```gul
import std.crypto
import std.crypto.jwt
import std.secrets

secret jwt_secret = env("JWT_SECRET", default="dev-secret-key")

fn hash_password(password: str): str:
    return crypto.bcrypt_hash(password)

fn verify_password(password: str, hash: str): bool:
    return crypto.bcrypt_verify(password, hash)

fn generate_token(user_id: int, username: str): str:
    payload = {
        "user_id": user_id,
        "username": username,
        "exp": time.now() + duration.days(7)
    }

    return jwt.encode(payload, jwt_secret)

fn verify_token(token: str): map?:
    try:
        return jwt.decode(token, jwt_secret)
   catch:
        return None

fn require_auth(next_handler):
    fn handler(request):
        auth_header = request.headers.get("Authorization")

        if auth_header is None:
            return http.unauthorized("Missing authorization header")

        parts = auth_header.split()
        if len(parts) != 2 or parts[0] != "Bearer":
            return http.unauthorized("Invalid authorization header")

        token = parts[1]
        payload = verify_token(token)

        if payload is None:
            return http.unauthorized("Invalid token")

        # Attach user to request
        request.user = payload

        return next_handler(request)

    return handler
```

## 🛣️ Step 5: API Routes

Create `src/routes/users.mn`:

```gul
import std.http

fn register_user_routes(server):
    @server.post("/api/register")
    fn register(request):
        data = request.json()

        # Validation
        if !data.get("username") or !data.get("email") or !data.get("password"):
            return http.bad_request("Missing required fields")

        # Check if user exists
        if User.where(username=data["username"]).first() is not None:
            return http.bad_request("Username already exists")

        # Create user
        user = User.create(
            username=data["username"],
            email=data["email"],
            password_hash=hash_password(data["password"])
        )

        # Generate token
        token = generate_token(user.id, user.username)

        return http.json_response({
            "user": {
                "id": user.id,
                "username": user.username,
                "email": user.email
            },
            "token": token
        }, status=201)

    @server.post("/api/login")
    fn login(request):
        data = request.json()

        user = User.where(username=data.get("username")).first()

        if user is None or !verify_password(data.get("password"), user.password_hash):
            return http.unauthorized("Invalid credentials")

        token = generate_token(user.id, user.username)

        return http.json_response({
            "user": {
                "id": user.id,
                "username": user.username,
                "email": user.email
            },
            "token": token
        })

    @server.get("/api/users/{id}")
    fn get_user(request, id: int):
        user = User.with("posts").find(id)

        if user is None:
            return http.not_found("User not found")

        return http.json_response({
            "id": user.id,
            "username": user.username,
            "email": user.email,
            "posts_count": len(user.posts),
            "created_at": user.created_at.to_string()
        })
```

Create `src/routes/posts.mn`:

```gul
@server.get("/api/posts")
fn list_posts(request):
    page = request.query.get("page", type=int, default=1)
    per_page = 10

    posts = Post.with("user")
        .where(published=True)
        .order_by("created_at", "DESC")
        .limit(per_page)
        .offset((page - 1) * per_page)
        .get()

    total = Post.where(published=True).count()

    return http.json_response({
        "posts": [
            {
                "id": post.id,
                "title": post.title,
                "content": post.content,
                "author": post.user.username,
                "created_at": post.created_at.to_string()
            }
            for post in posts
        ],
        "pagination": {
            "page": page,
            "per_page": per_page,
            "total": total,
            "pages": (total + per_page - 1) // per_page
        }
    })

@server.get("/api/posts/{id}")
fn get_post(request, id: int):
    post = Post.with("user", "comments.user").find(id)

    if post is None or !post.published:
        return http.not_found("Post not found")

    return http.json_response({
        "id": post.id,
        "title": post.title,
        "content": post.content,
        "author": {
            "id": post.user.id,
            "username": post.user.username
        },
        "comments": [
            {
                "id": comment.id,
                "content": comment.content,
                "author": comment.user.username,
                "created_at": comment.created_at.to_string()
            }
            for comment in post.comments
        ],
        "created_at": post.created_at.to_string()
    })

@server.post("/api/posts")
@require_auth
fn create_post(request):
    data = request.json()

    post = Post.create(
        title=data["title"],
        content=data["content"],
        user_id=request.user["user_id"],
        published=data.get("published", False)
    )

    return http.json_response({
        "id": post.id,
        "title": post.title,
        "created_at": post.created_at.to_string()
    }, status=201)

@server.put("/api/posts/{id}")
@require_auth
fn update_post(request, id: int):
    post = Post.find(id)

    if post is None:
        return http.not_found("Post not found")

    if post.user_id != request.user["user_id"]:
        return http.forbidden("Not authorized")

    data = request.json()

    post.title = data.get("title", post.title)
    post.content = data.get("content", post.content)
    post.published = data.get("published", post.published)
    post.save()

    return http.json_response({"status": "updated"})

@server.delete("/api/posts/{id}")
@require_auth
fn delete_post(request, id: int):
    post = Post.find(id)

    if post is None:
        return http.not_found("Post not found")

    if post.user_id != request.user["user_id"]:
        return http.forbidden("Not authorized")

    post.delete()

    return http.Response(status=204)
```

## 🚀 Step 6: Main Server

Create `src/main.mn`:

```gul
import std.http
import std.database
import std.secrets

# Initialize database
db = database.connect("sqlite:///blog.db")
db.execute_file("schema.sql")

# Create server
server = http.Server(port=8080)

# Middleware
server.use(middleware.logger())
server.use(middleware.cors())
server.use(middleware.json_parser())

# Register routes
register_user_routes(server)
register_post_routes(server)

# Health check
@server.get("/health")
fn health_check(request):
    return http.json_response({"status": "healthy"})

main:
    print("🚀 Blog API Server")
    print("📡 Running on http://localhost:8080")
    print("📚 Endpoints:")
    print("   POST /api/register")
    print("   POST /api/login")
    print("   GET  /api/posts")
    print("   POST /api/posts (auth required)")
    print("")

    server.start()
```

## 🧪 Step 7: Test the API

```bash
# Run the server
gul run src/main.mn

# In another terminal, test the endpoints:

# Register a user
curl -X POST http://localhost:8080/api/register \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com","password":"secret123"}'

# Login
curl -X POST http://localhost:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","password":"secret123"}'

# Create a post (use token from login)
curl -X POST http://localhost:8080/api/posts \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -d '{"title":"My First Post","content":"Hello World!","published":true}'

# List posts
curl http://localhost:8080/api/posts
```

## 📚 What You've Learned

✅ RESTful API design  
✅ Database models and relationships  
✅ JWT authentication  
✅ Password hashing  
✅ Middleware usage  
✅ CRUD operations  
✅ Pagination  
✅ Error handling

## 🎯 Next Steps

- Add comment endpoints
- Implement rate limiting
- Add input validation
- Deploy to production
- Add automated tests
- Build a frontend

---

**Last Updated**: 2025-12-28  
**Version**: 1.0.0  
**License**: MIT
