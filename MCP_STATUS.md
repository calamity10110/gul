# MCP Code Generation Test

## Current Status

The `gul-mcp` binary is a **protocol server**, not a CLI tool. It registers MCP tools for AI agents to use via the Model Context Protocol, but doesn't provide direct command-line access.

## What `gul-mcp` Actually Does

```rust
// src/bin/gul-mcp.rs
fn main() {
    let server = GulMcpServer::new();
    
    // Lists available tools (display only)
    println!("Registered {} tools:", server.list_tools().len());
    for tool in server.list_tools() {
        println!("  - {}: {}", tool.name, tool.description);
    }
    
    // Server is ready for MCP protocol (not CLI)
    println!("Server ready. Use the MCP protocol to interact.");
}
```

## Available MCP Tools

The following tools are **registered** but require MCP protocol to invoke:

1. **gul_generate_code** - Generate GUL v3.2 code from natural language
2. **gul_project_scaffold** - Create complete project from description
3. **gul_run_code** - Execute GUL code
4. **gul_create_package** - Create a new GUL package
5. **gul_install_dependencies** - Install GUL package dependencies

## How MCP Tools Work

### Protocol-Based Invocation (Current)

AI agents use the MCP protocol to invoke tools:

```json
{
  "jsonrpc": "2.0",
  "method": "tools/call",
  "params": {
    "name": "gul_generate_code",
    "arguments": {
      "description": "Create a REST API for users",
      "output_file": "api.mn"
    }
  }
}
```

### CLI Invocation (Not Implemented Yet)

What users might expect (but doesn't work):

```bash
# These don't work - they all just start the server
gul-mcp generate "Create a REST API"
gul-mcp auto all
gul-mcp schedule enable auto_lint
```

## Workaround: Direct Code Generation

Since the CLI isn't wired up, you can use the `gul` compiler directly:

```bash
# Create a simple GUL file
cat > test_api.mn << 'EOF'
@imp std.http

async handle_request(req):
    return @dict{
        status: "ok",
        message: "Hello from GUL!"
    }

mn:
    http.get("/api/test", handle_request)
    http.listen(8080)
EOF

# Run it
gul run test_api.mn
```

## Testing MCP Functionality

### Option 1: Use AI Agent with MCP Protocol

An AI agent (like Claude, GPT-4, etc.) configured with MCP can invoke the tools.

### Option 2: Direct Tool Testing

Test the underlying code generation functionality:

```bash
# Check if GUL compiler can generate code
gul --help

# Create example from template
gul new my-project --template web-api

# Run examples
gul run examples/web_api_v32.mn
```

### Option 3: Manual Code Creation

Create GUL code manually following v3.2 syntax:

```gul
@imp std.io

fn greet(name: str) -> str:
    return "Hello, " + name

mn:
    let message = greet("World")
    print(message)
```

## Recommendation

The `gul-mcp` tool is designed for **AI agent integration**, not direct CLI usage. For actual code generation:

1. **Use AI agents** configured with MCP protocol
2. **Write GUL code** directly following v3.2 syntax
3. **Use templates** with `gul new --template`
4. **Run examples** from the examples/ directory

## Future Enhancement

To make this work as expected, the `gul-mcp.rs` binary would need:

```rust
// Enhanced CLI with subcommands
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Serve { port: u16 },
    Generate { description: String },
    Auto { task: String },
    Schedule { action: String, name: String },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Serve { port } => start_server(port),
        Commands::Generate { description } => generate_code(description),
        Commands::Auto { task } => run_auto_task(task),
        Commands::Schedule { action, name } => manage_schedule(action, name),
    }
}
```

---

**Status**: MCP server is working for protocol-based AI agent integration, but CLI subcommands are not implemented.

**Workaround**: Use `gul` compiler directly or write GUL code manually.
