# GUL Web IDE & Web Development Guide

Complete guide to using the GUL Web IDE and developing web applications.

## 📋 Table of Contents

1. [Using the Web IDE](#using-the-web-ide)
2. [Developing Web Applications](#developing-web-applications)
3. [Hosting & Deployment](#hosting--deployment)
4. [Web IDE Features](#web-ide-features)

---

## 🌐 Using the Web IDE

### Starting the Web IDE Locally

```bash
# Navigate to the web directory
cd web

# Install dependencies (first time only)
npm install

# Start development server
dx serve
```

The IDE will be available at `http://localhost:8080`

### Web IDE Interface

```
┌─────────────────────────────────────────────────────────────┐
│  GUL Web IDE                                    [⚙️] [👤]   │
├──────────┬──────────────────────────────────────────────────┤
│ FILES    │  main.mn                                    [×]  │
│          │                                                   │
│ 📁 src   │  1  mn main():                                   │
│  📄 main │  2      print("Hello, Web!")                     │
│  📄 utils│  3                                               │
│ 📁 tests │  4                                               │
│          │                                                   │
├──────────┼──────────────────────────────────────────────────┤
│ CONSOLE  │  OUTPUT                                          │
│          │  > gul run main.mn                               │
│          │  Hello, Web!                                     │
│          │  ✓ Completed in 0.5s                             │
└──────────┴──────────────────────────────────────────────────┘
```

### Key Features

- **Real-time Collaboration**: Share your workspace URL
- **Cloud Sync**: Auto-save to cloud storage
- **Syntax Highlighting**: Full GUL language support
- **Integrated Terminal**: Run commands in-browser
- **Git Integration**: Commit and push directly from IDE
- **Live Preview**: See output instantly

---

## 🚀 Developing Web Applications

### Basic Web App Structure

```gul
imp web

mn main():
    # Create a web application
    app = web.App.new()

    # Define routes
    app.route("/", handle_home)
    app.route("/api/data", handle_api)

    # Start server
    app.run(host="0.0.0.0", port=8080)

@fn handle_home(request):
    return web.html("""
        <!DOCTYPE html>
        <html>
        <head>
            <title>GUL Web App</title>
        </head>
        <body>
            <h1>Welcome to GUL!</h1>
        </body>
        </html>
    """)

@asy handle_api(request):
    data = await fetch_data_from_db()
    return web.json(data)
```

### Using Dioxus for Interactive UIs

```gul
imp dioxus

@fn App():
    ?count = 0

    return html!(
        div {
            h1 { "Counter: {?count}" }
            button {
                onclick: |_| ?count = ?count + 1,
                "Increment"
            }
            button {
                onclick: |_| ?count = ?count - 1,
                "Decrement"
            }
        }
    )

mn main():
    dioxus.launch(App)
```

### REST API Example

```gul
imp web, json

# Define data model
@map User = {
    id: @int,
    name: @str,
    email: @str
}

# In-memory storage
@global ?users = []

@fn create_user(request):
    user_data = json.parse(request.body)
    user = User{
        id: len(@global ?users) + 1,
        name: user_data.name,
        email: user_data.email
    }
    @global ?users.append(user)
    return web.json(user, status=201)

@fn get_users(request):
    return web.json(@global ?users)

@fn get_user(request):
    user_id = int(request.params.id)
    user = @global ?users.find(|u| u.id == user_id)

    @if user:
        return web.json(user)
    @else:
        return web.json({error: "User not found"}, status=404)

mn main():
    app = web.App.new()

    app.post("/api/users", create_user)
    app.get("/api/users", get_users)
    app.get("/api/users/:id", get_user)

    app.run(port=8080)
```

### Database Integration

```gul
imp web, db

@cs sql:
    CREATE TABLE IF NOT EXISTS posts (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT NOT NULL,
        content TEXT NOT NULL,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

@asy get_posts(request):
    posts = await db.query("SELECT * FROM posts ORDER BY created_at DESC")
    return web.json(posts)

@asy create_post(request):
    data = json.parse(request.body)

    result = await db.execute(
        "INSERT INTO posts (title, content) VALUES (?, ?)",
        [data.title, data.content]
    )

    return web.json({id: result.last_insert_id}, status=201)

mn main():
    # Initialize database
    db.init("blog.db")
    db.execute(sql.create_table)

    # Start web server
    app = web.App.new()
    app.get("/api/posts", get_posts)
    app.post("/api/posts", create_post)
    app.run(port=8080)
```

---

## 📦 Hosting & Deployment

### Building for Production

```bash
# Build optimized web app
cd web
dx build --release

# Output will be in dist/
```

### Deploying to Vercel

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy
cd web
vercel --prod
```

### Deploying to Netlify

```bash
# Install Netlify CLI
npm i -g netlify-cli

# Deploy
cd web
netlify deploy --prod --dir=dist
```

### Self-Hosting with Docker

Create `Dockerfile`:

```dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/gul /usr/local/bin/
COPY web/dist /var/www/html

EXPOSE 8080
CMD ["gul", "serve", "/var/www/html"]
```

Build and run:

```bash
docker build -t gul-web .
docker run -p 8080:8080 gul-web
```

### Deploying to AWS

```bash
# Build for production
dx build --release

# Upload to S3
aws s3 sync dist/ s3://my-gul-app

# Configure CloudFront for SPA routing
aws cloudfront create-distribution \
    --origin-domain-name my-gul-app.s3.amazonaws.com
```

---

## 🎨 Web IDE Features

### Customization

```javascript
// .mn/config.json
{
  "theme": "dark",
  "fontSize": 14,
  "fontFamily": "Fira Code",
  "tabSize": 4,
  "autoSave": true,
  "autoSaveDelay": 1000,
  "formatOnSave": true,
  "linting": true
}
```

### Extensions

```bash
# Install extensions
gul ext install syntax-highlighter
gul ext install git-integration
gul ext install ai-assistant

# List installed extensions
gul ext list

# Remove extension
gul ext remove ai-assistant
```

### Keyboard Shortcuts

| Shortcut       | Action           |
| -------------- | ---------------- |
| `Ctrl+S`       | Save file        |
| `Ctrl+Shift+S` | Save all         |
| `Ctrl+P`       | Quick open file  |
| `Ctrl+Shift+P` | Command palette  |
| `Ctrl+F`       | Find             |
| `Ctrl+H`       | Find and replace |
| `Ctrl+/`       | Toggle comment   |
| `Ctrl+B`       | Toggle sidebar   |
| `Ctrl+J`       | Toggle console   |
| `Ctrl+`        | Zoom in          |
| `Ctrl-`        | Zoom out         |
| `F11`          | Fullscreen       |

### Collaboration Features

```gul
# Share your workspace
workspace_url = ide.share()
print(f"Share this URL: {workspace_url}")

# Join a shared workspace
ide.join("https://gul-ide.dev/workspace/abc123")

# Real-time cursor tracking
ide.enable_cursor_tracking()

# Voice chat
ide.start_voice_chat()
```

---

## 🔧 Advanced Web Development

### WebAssembly Compilation

```bash
# Compile GUL to WASM
gul build --target wasm main.mn

# Output: main.wasm
```

Use in web page:

```html
<!DOCTYPE html>
<html>
  <head>
    <title>GUL WASM App</title>
  </head>
  <body>
    <script type="module">
      import init from "./main.wasm";

      async function run() {
        await init();
        // Your GUL code runs here
      }

      run();
    </script>
  </body>
</html>
```

### Progressive Web App (PWA)

Add `manifest.json`:

```json
{
  "name": "GUL App",
  "short_name": "GUL",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#000000",
  "theme_color": "#00ff00",
  "icons": [
    {
      "src": "/icon-192.png",
      "sizes": "192x192",
      "type": "image/png"
    },
    {
      "src": "/icon-512.png",
      "sizes": "512x512",
      "type": "image/png"
    }
  ]
}
```

Add service worker:

```javascript
// sw.js
self.addEventListener("install", (event) => {
  event.waitUntil(
    caches.open("gul-v1").then((cache) => {
      return cache.addAll(["/", "/index.html", "/main.wasm", "/styles.css"]);
    })
  );
});

self.addEventListener("fetch", (event) => {
  event.respondWith(
    caches.match(event.request).then((response) => {
      return response || fetch(event.request);
    })
  );
});
```

### Server-Side Rendering (SSR)

```gul
imp dioxus.ssr

@fn render_to_string(component):
    return dioxus.ssr.render(component)

@fn handle_request(request):
    html = render_to_string(App())
    return web.html(f"""
        <!DOCTYPE html>
        <html>
        <head>
            <title>GUL SSR</title>
        </head>
        <body>
            <div id="app">{html}</div>
            <script src="/hydrate.js"></script>
        </body>
        </html>
    """)
```

---

## 📊 Performance Optimization

### Code Splitting

```gul
# Lazy load components
@lazy import HeavyComponent from "./heavy.mn"

@fn App():
    return html!(
        div {
            Suspense {
                fallback: html!(div { "Loading..." }),
                HeavyComponent {}
            }
        }
    )
```

### Caching Strategy

```gul
imp web.cache

# Cache API responses
@cache(ttl=3600)  # Cache for 1 hour
@asy get_data():
    return await fetch_from_api()

# Cache static assets
app.static("/assets", cache_control="public, max-age=31536000")
```

### Compression

```gul
# Enable gzip compression
app.use_compression()

# Enable Brotli for better compression
app.use_brotli()
```

---

## 🔒 Security Best Practices

### CORS Configuration

```gul
app.cors({
    origins: ["https://example.com"],
    methods: ["GET", "POST"],
    headers: ["Content-Type", "Authorization"],
    credentials: true
})
```

### CSRF Protection

```gul
imp web.csrf

app.use_csrf_protection()

@fn handle_form(request):
    @if not csrf.verify(request):
        return web.error(403, "CSRF validation failed")

    # Process form
```

### Rate Limiting

```gul
imp web.ratelimit

@ratelimit(max_requests=100, window=60)  # 100 requests per minute
@fn api_endpoint(request):
    return web.json({status: "ok"})
```

---

## 📚 Additional Resources

- [Web Examples](examples/web/)
- [Dioxus Documentation](https://dioxuslabs.com)
- [Web API Reference](docs/api/web.md)
- [Deployment Guide](docs/deployment.md)

**Build amazing web applications with GUL!** 🚀
