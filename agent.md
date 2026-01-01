# AI Agents & Automation in GUL

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-30

---

## Overview

GUL is designed with **AI-first principles**, providing comprehensive support for AI agents, autonomous development, and intelligent automation through the **Model Context Protocol (MCP)** server.

### Key Features

- 🤖 **MCP Server Integration** - Full AI agent support
- 🔄 **Autonomous Workflows** - Automated development tasks
- 🛠️ **Code Generation** - Natural language to GUL v3.2 code
- 📦 **Package Automation** - Auto-create and maintain packages
- 🔍 **Smart Analysis** - Code quality and optimization
- 📚 **AI-Optimized Docs** - Metadata-rich documentation

---

## MCP Server

### What is MCP?

The **Model Context Protocol (MCP)** is a standardized protocol for AI agent integration. GUL's MCP server enables AI agents to:

- Generate GUL code from natural language descriptions
- Create complete projects and packages
- Run automated maintenance tasks
- Execute workflows and schedules
- Access documentation and examples

### Installation

The MCP server is included with GUL:

```bash
# Build GUL with MCP support
cargo build --release --features mcp

# Verify installation
gul-mcp --version
```

### Starting the Server

```bash
# Start MCP server
gul-mcp serve --port 3000

# With custom configuration
gul-mcp serve --config mcp_config.json

# Background mode
gul-mcp serve --daemon --log mcp.log
```

---

## MCP Tools

### 1. Code Generation

Generate GUL code from natural language descriptions:

```bash
# Via MCP protocol
{
  "tool": "gul_generate_code",
  "description": "Create a REST API with user authentication",
  "output_file": "api.mn"
}
```

**Example Output**:

```gul
@imp std.http
@imp std.crypto

struct User:
    id: int
    username: str
    password_hash: str

async authenticate(username: str, password: str) -> bool:
    let user = await db.query("SELECT * FROM users WHERE username = ?", username)
    let hash = crypto.hash(password)
    return user.password_hash == hash

async handle_login(req):
    let username = req.body["username"]
    let password = req.body["password"]
    
    if await authenticate(username, password):
        return @dict{status: 200, token: generate_token(username)}
    else:
        return @dict{status: 401, error: "Invalid credentials"}

mn:
    http.post("/login", handle_login)
    http.listen(8080)
```

### 2. Project Scaffolding

Create complete projects from descriptions:

```bash
{
  "tool": "gul_project_scaffold",
  "description": "Microservices application with API gateway and user service",
  "project_name": "my-microservices"
}
```

Generates:

```
my-microservices/
├── gateway/
│   └── main.mn
├── services/
│   └── users/
│       ├── main.mn
│       ├── models.mn
│       └── api.mn
├── shared/
│   └── auth.mn
└── Cargo.toml
```

### 3. Package Creation

Create new GUL packages:

```bash
{
  "tool": "gul_create_package",
  "name": "gul-auth",
  "category": "security",
  "description": "Authentication and authorization library"
}
```

### 4. Code Execution

Run GUL code through MCP:

```bash
{
  "tool": "gul_run_code",
  "code": "@imp std.io\n\nmn: print('Hello from MCP!')",
  "output": true
}
```

### 5. Dependency Installation

Install package dependencies:

```bash
{
  "tool": "gul_install_dependencies",
  "packages": ["pandas", "numpy", "tokio"]
}
```

---

## Automation & Workflows

### Auto-Maintenance

Automated code quality tasks:

```bash
# Format code
gul-mcp auto fmt

# Run linter
gul-mcp auto lint

# Type check
gul-mcp auto check

# Security audit
gul-mcp auto audit

# All at once
gul-mcp auto all
```

**Via MCP Protocol**:

```json
{
  "tool": "gul_auto_maintenance",
  "tasks": ["fmt", "lint", "check", "audit"]
}
```

### Workflows

Define and execute multi-step workflows:

**workflow.json**:

```json
{
  "name": "ci_workflow",
  "steps": [
    {"action": "fmt"},
    {"action": "lint"},
    {"action": "test"},
    {"action": "build", "args": ["--release"]}
  ]
}
```

**Execute**:

```bash
gul-mcp workflow run ci_workflow
```

### Scheduling

Schedule automated tasks:

```bash
# List schedules
gul-mcp schedule list

# Enable/disable schedules
gul-mcp schedule enable auto_lint
gul-mcp schedule disable weekly_deps

# Create new schedule
gul-mcp schedule create daily_test --task test --cron "0 0 * * *"
```

---

## Agent-Assisted Development

### Web Development

**Prompt**: "Create a web API for managing blog posts with CRUD operations"

**Generated Code**:

```gul
@imp std.http
@imp std.db

struct Post:
    id: int
    title: str
    content: str
    author: str
    created_at: str

async get_posts() -> list:
    return await db.query("SELECT * FROM posts ORDER BY created_at DESC")

async get_post(id: int) -> dict:
    return await db.query("SELECT * FROM posts WHERE id = ?", id)

async create_post(post: Post) -> dict:
    let id = await db.execute(
        "INSERT INTO posts (title, content, author) VALUES (?, ?, ?)",
        post.title, post.content, post.author
    )
    return @dict{id: id, message: "Post created"}

async update_post(id: int, post: Post) -> dict:
    await db.execute(
        "UPDATE posts SET title = ?, content = ? WHERE id = ?",
        post.title, post.content, id
    )
    return @dict{message: "Post updated"}

async delete_post(id: int) -> dict:
    await db.execute("DELETE FROM posts WHERE id = ?", id)
    return @dict{message: "Post deleted"}

mn:
    http.get("/posts", get_posts)
    http.get("/posts/:id", get_post)
    http.post("/posts", create_post)
    http.put("/posts/:id", update_post)
    http.delete("/posts/:id", delete_post)
    http.listen(3000)
```

### Data Analysis

**Prompt**: "Analyze CSV data and generate visualizations"

**Generated Code**:

```gul
@imp std.io
@imp python{pandas, matplotlib}

@python {
    import pandas as pd
    import matplotlib.pyplot as plt
    
    # Load data
    df = pd.read_csv("data.csv")
    
    # Analysis
    stats = {
        "mean": df.mean().to_dict(),
        "median": df.median().to_dict(),
        "std": df.std().to_dict()
    }
    
    # Visualizations
    df.plot(kind='bar', figsize=(10, 6))
    plt.savefig("analysis.png")
}

mn:
    print("Analysis complete!")
    print(python.stats)
```

### Embedded Development

**Prompt**: "Create LED blink program for ESP32-S3"

**Generated Code**:

```gul
@imp embedded.gpio
@imp embedded.time

let LED_PIN = 2

mn:
    let led = gpio.pin(LED_PIN, OUTPUT)
    
    loop:
        led.high()
        time.delay_ms(1000)
        led.low()
        time.delay_ms(1000)
```

---

## Best Practices

### 1. Code Generation

**✅ Good Prompts**:

- "Create a REST API with user authentication and rate limiting"
- "Build a data pipeline that processes CSV files and stores in PostgreSQL"
- "Implement a caching layer using Redis"

**❌ Vague Prompts**:

- "Make an app"
- "Add features"
- "Fix the code"

### 2. Security

**Secret Management**:

```gul
# Use .scrt files (never commit!)
@imp secrets

let API_KEY = secrets.get("API_KEY")  # From .scrt file
let DB_PASSWORD = secrets.get("DB_PASSWORD")

# Never hardcode secrets
# Bad: let API_KEY = "abc123xyz"
```

**Authentication**:

```gul
@imp std.crypto
@imp std.http

async verify_token(req):
    let token = req.headers["Authorization"]
    let valid = crypto.verify_jwt(token, SECRET_KEY)
    if not valid:
        return @dict{status: 401, error: "Unauthorized"}
```

### 3. Performance

**Use Rust for Performance-Critical Code**:

```gul
@rust {
    fn fast_compute(data: &[f64]) -> f64 {
        data.iter().map(|x| x * x).sum()
    }
}

mn:
    let data = @list[1.0, 2.0, 3.0, 4.0, 5.0]
    let result = rust.fast_compute(data)
```

### 4. Error Handling

**Always Handle Errors**:

```gul
async fetch_data(url: str) -> Result:
    try:
        let response = await http.get(url)
        return Ok(response.json())
    catch error:
        return Err(error)

mn:
    match await fetch_data("https://api.example.com"):
        Ok(data) => print(data)
        Err(error) => print("Error:", error)
```

### 5. Testing

**Generate Tests with Agent**:

```bash
# Prompt: "Generate unit tests for the authentication module"
```

**Generated**:

```gul
@imp testing

test "authenticate_valid_user":
    let result = await authenticate("alice", "password123")
    assert result == true

test "authenticate_invalid_password":
    let result = await authenticate("alice", "wrong")
    assert result == false

test "authenticate_nonexistent_user":
    let result = await authenticate("nobody", "password")
    assert result == false
```

---

## MCP Configuration

### Configuration File

**mcp_config.json**:

```json
{
  "server": {
    "host": "localhost",
    "port": 3000,
    "log_level": "info"
  },
  "tools": {
    "gul_generate_code": {
      "enabled": true,
      "max_tokens": 4096
    },
    "gul_create_package": {
      "enabled": true,
      "default_category": "utilities"
    }
  },
  "workflows": {
    "ci_workflow": {
      "steps": ["fmt", "lint", "test", "build"]
    }
  },
  "schedules": {
    "auto_lint": {
      "enabled": true,
      "trigger": "on_commit"
    }
  }
}
```

### Environment Variables

```bash
# MCP server configuration
export MCP_HOST=localhost
export MCP_PORT=3000
export MCP_LOG_LEVEL=debug

# API keys (for AI services)
export OPENAI_API_KEY=your_key_here
export ANTHROPIC_API_KEY=your_key_here
```

---

## Practical Examples

### Example 1: Generate Full-Stack App

**Agent Prompt**:

```
Create a full-stack web application with:
- React frontend
- GUL backend API
- PostgreSQL database
- User authentication
- CRUD operations for tasks
```

**Agent Actions**:

1. Scaffolds project structure
2. Generates backend API in GUL
3. Creates frontend React app
4. Sets up database schema
5. Implements authentication
6. Adds tests
7. Creates deployment config

### Example 2: Data Pipeline

**Agent Prompt**:

```
Build a data pipeline that:
- Reads CSV files from S3
- Processes data with pandas
- Stores results in PostgreSQL
- Sends alerts on errors
```

**Generated Pipeline**:

```gul
@imp std.io
@imp std.db
@imp python{pandas, boto3}

async process_file(bucket: str, key: str):
    @python {
        import boto3
        import pandas as pd
        
        # Download from S3
        s3 = boto3.client('s3')
        s3.download_file(bucket, key, 'temp.csv')
        
        # Process
        df = pd.read_csv('temp.csv')
        df['processed'] = df['value'] * 2
        
        # Convert to dict
        result = df.to_dict('records')
    }
    
    # Store in database
    for row in python.result:
        await db.execute(
            "INSERT INTO processed_data (id, value, processed) VALUES (?, ?, ?)",
            row['id'], row['value'], row['processed']
        )

mn:
    try:
        await process_file("my-bucket", "data.csv")
        print("Pipeline completed successfully")
    catch error:
        print("Error:", error)
        # Send alert
        await notify_error(error)
```

### Example 3: Microservices

**Agent Prompt**:

```
Create microservices architecture with:
- API Gateway
- User Service
- Order Service
- Payment Service
- Message Queue (RabbitMQ)
```

**Generated Structure**:

```
microservices/
├── gateway/
│   └── main.mn          # Routes requests to services
├── services/
│   ├── users/
│   │   ├── main.mn      # User CRUD
│   │   └── auth.mn      # Authentication
│   ├── orders/
│   │   └── main.mn      # Order management
│   └── payments/
│       └── main.mn      # Payment processing
└── shared/
    ├── queue.mn         # RabbitMQ integration
    └── models.mn        # Shared data models
```

---

## Troubleshooting

### MCP Server Won't Start

**Check port availability**:

```bash
lsof -i :3000
```

**Try different port**:

```bash
gul-mcp serve --port 3001
```

### Code Generation Fails

**Check syntax version**:

- Ensure prompts specify v3.2 syntax
- Use @ prefix for types
- Use `mn:` for entry point

**Increase timeout**:

```json
{
  "timeout": 60000
}
```

### Workflow Errors

**Check workflow definition**:

```bash
gul-mcp workflow validate ci_workflow
```

**View logs**:

```bash
tail -f mcp.log
```

---

## API Reference

### MCP Tools

| Tool | Description | Parameters |
|------|-------------|------------|
| `gul_generate_code` | Generate code from description | `description`, `output_file` |
| `gul_project_scaffold` | Create project structure | `description`, `project_name` |
| `gul_create_package` | Create new package | `name`, `category`, `description` |
| `gul_run_code` | Execute GUL code | `code`, `output` |
| `gul_install_dependencies` | Install packages | `packages` |

### MCP Resources

| Resource | Description | URI Pattern |
|----------|-------------|-------------|
| GUL Documentation | Language docs | `gul://docs/{topic}` |
| GUL Packages | Package registry | `gul://packages/{name}` |
| GUL Templates | Project templates | `gul://templates/{type}` |

### CLI Commands

```bash
# Server
gul-mcp serve [--port PORT] [--config FILE]

# Tools
gul-mcp generate DESCRIPTION
gul-mcp create PACKAGE_NAME
gul-mcp run CODE

# Maintenance
gul-mcp auto [fmt|lint|check|audit|all]

# Workflows
gul-mcp workflow [list|run|create|delete] NAME

# Scheduling
gul-mcp schedule [list|enable|disable|create] NAME
```

---

## Additional Resources

### Documentation

- [MCP Quickstart](docs/guides/MCP_QUICKSTART.md)
- [MCP Server API](docs/api/MCP_SERVER.md)
- [MCP Advanced](docs/api/MCP_ADVANCED.md)
- [Knowledgebase](docs/reference/knowledgebase.md)

### Examples

- [Web Development](examples/web_api_v32.mn)
- [Data Processing](examples/data_processing_v32.mn)
- [Embedded](examples/embedded_blink.mn)

### Community

- [GitHub](https://github.com/calamity10110/gul)
- [Discussions](https://github.com/calamity10110/gul/discussions)
- [Issues](https://github.com/calamity10110/gul/issues)

---

**Last Updated**: 2025-12-30  
**For**: AI Agents & Developers  
**Version**: v3.2 MCP Integration
