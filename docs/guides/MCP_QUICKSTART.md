# GUL MCP Quick Start Guide

## 🚀 Using GUL MCP Server with AI Assistants

The GUL MCP (Model Context Protocol) server allows AI assistants like Claude and GPT-4 to generate, run, and manage GUL projects automatically.

---

## 📦 Installation

```bash
# Install GUL with MCP support
cargo install gul --features mcp

# Or build from source
cargo build --release --features mcp
```

---

## ⚙️ Configuration

### For Claude Desktop

Add to `~/Library/Application Support/Claude/claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "gul": {
      "command": "gul",
      "args": ["mcp", "serve"],
      "env": {
        "GUL_AI_PROVIDER": "anthropic",
        "GUL_AI_MODEL": "claude-3-opus-20240229",
        "GUL_AI_API_KEY": "${ANTHROPIC_API_KEY}"
      }
    }
  }
}
```

### For Other AI Assistants

```json
{
  "mcpServers": {
    "gul-assistant": {
      "command": "gul-mcp-server",
      "args": ["--port", "3000"],
      "env": {
        "GUL_AI_PROVIDER": "openai",
        "GUL_AI_MODEL": "gpt-4",
        "GUL_AI_API_KEY": "${OPENAI_API_KEY}"
      }
    }
  }
}
```

---

## 💬 Example Conversations

### Example 1: Create a Web API

**You**: "Create a REST API in GUL that analyzes text sentiment"

**AI** (using MCP):

```
Using gul_project_scaffold...
Creating project structure...
Generating API endpoints...
Adding AI sentiment analysis...
Installing dependencies...
✅ Project created: sentiment-api/
```

**Result**: Complete working web API with sentiment analysis!

---

### Example 2: Generate and Test Code

**You**: "Write a function to process CSV data and calculate statistics"

**AI** (using MCP):

````
Using gul_generate_code...

Generated:
```gul
@imp python{pandas, numpy}

fn @dict process_csv(filepath):
    @python {
        df = pd.read_csv(filepath)
        stats = {
            "count": len(df),
            "mean": float(df.mean().mean()),
            "median": float(df.median().median())
        }
        return stats
    }
    return python.stats
````

Testing with gul_run_code...
✅ Tests passed!

```

---

### Example 3: Install Dependencies

**You**: "I need to work with machine learning. Install the required packages"

**AI** (using MCP):
```

Using gul_install_dependencies...
Installing:

- tensorflow
- numpy
- pandas
- scikit-learn

✅ All dependencies installed!

Created template: templates/ai_app/main.mn

````

---

## 🛠️ Available MCP Tools

### 1. `gul_generate_code`
Generate GUL code from natural language

```javascript
// AI uses this automatically
gul_generate_code({
  description: "function to fetch API data",
  type: "function",
  features: ["web", "async"]
})
````

### 2. `gul_create_package`

Create a new GUL package

```javascript
gul_create_package({
  name: "my-api",
  type: "web",
  dependencies: ["actix-web", "serde"],
});
```

### 3. `gul_run_code`

Execute GUL code

```javascript
gul_run_code({
  code: "print('Hello from GUL!')",
  args: [],
});
```

### 4. `gul_install_dependencies`

Install packages

```javascript
gul_install_dependencies({
  packages: ["numpy", "pandas", "tensorflow"],
});
```

### 5. `gul_project_scaffold`

Create entire projects

```javascript
gul_project_scaffold({
  description: "E-commerce platform with AI recommendations",
  project_name: "shop-ai",
  include_ai: true,
  include_web: true,
  include_database: true,
});
```

---

## 🎯 Use Cases

### 1. **Rapid Prototyping**

- "Create a chat application with WebSocket"
- AI generates complete app in seconds

### 2. **Learning GUL**

- "Show me how to use async functions"
- AI generates examples and explains

### 3. **Code Migration**

- "Convert this Python code to GUL"
- AI handles the translation

### 4. **Project Setup**

- "Start a new AI project with TensorFlow"
- AI scaffolds everything

### 5. **Debugging**

- "Why isn't this code working?"
- AI analyzes and fixes issues

---

## 📊 What AI Can Do

| Task                | AI Capability                       |
| ------------------- | ----------------------------------- |
| **Generate Code**   | ✅ Full functions, structs, modules |
| **Create Projects** | ✅ Complete scaffolding             |
| **Install Deps**    | ✅ Automatic dependency management  |
| **Run Tests**       | ✅ Execute and verify code          |
| **Debug**           | ✅ Find and fix errors              |
| **Optimize**        | ✅ Improve performance              |
| **Document**        | ✅ Add comments and docs            |
| **Migrate**         | ✅ Convert from other languages     |

---

## 🔥 Advanced Features

### Context Awareness

AI remembers your project structure and preferences:

```
You: "Add another endpoint"
AI: Adds to your existing API structure
```

### Multi-Step Operations

```
You: "Create a data science project and run analysis on this CSV"
AI:
1. Creates project
2. Generates analysis code
3. Installs pandas/numpy
4. Runs analysis
5. Shows results
```

### Code Enhancement

```
You: "Make this faster"
AI: Optimizes using Rust foreign blocks
```

---

## 🎓 Tips

1. **Be specific**: "Create a REST API with JWT auth" > "Make an API"
2. **Iterate**: AI can modify what it creates
3. **Ask for explanations**: "Why did you use this approach?"
4. **Request tests**: "Add tests for this code"
5. **Check results**: Always review generated code

---

## 🌟 Example Workflow

```
1. You: "I want to build a sentiment analysis API"

2. AI: [Using gul_project_scaffold]
   ✅ Created: sentiment-api/
   ✅ Added: Web server
   ✅ Added: AI integration
   ✅ Added: Tests

3. You: "Add caching to improve performance"

4. AI: [Using gul_generate_code]
   ✅ Added: Redis caching
   ✅ Updated: API endpoints
   ✅ Added: Cache tests

5. You: "Deploy to production"

6. AI: [Using gul_generate_code]
   ✅ Created: Dockerfile
   ✅ Created: docker-compose.yml
   ✅ Created: deploy.sh

7. Done! 🎉
```

---

## 🔗 Resources

- **MCP Specification**: `MCP_SERVER_SPEC.md`
- **Code**: `src/mcp/server.rs`
- **Examples**: `examples/` directory
- **Templates**: `templates/` directory

---

**Created**: 2025-12-28  
**Status**: ✅ Ready to Use  
**Protocol**: MCP 1.0
