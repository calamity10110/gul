# GUL MCP Server Specification

## 🎯 Overview

The **GUL Model Context Protocol (MCP) Server** enables AI assistants to:

- Generate GUL packages from natural language
- Create complete applications with AI
- Run and test GUL code
- Download and install dependencies
- Operate and manage GUL projects

---

## 🏗️ Architecture

```
┌─────────────────┐
│   AI Assistant  │ (Claude, GPT-4, etc.)
└────────┬────────┘
         │ MCP Protocol
         │
┌────────▼────────┐
│  GUL MCP Server │
└────────┬────────┘
         │
    ┌────┴────┬────────┬──────────┐
    │         │        │          │
┌───▼───┐ ┌──▼──┐ ┌──▼───┐ ┌────▼────┐
│ Gen   │ │ Run │ │ Pkg  │ │ Project │
│ Code  │ │ Code│ │ Mgmt │ │ Ops     │
└───────┘ └─────┘ └──────┘ └─────────┘
```

---

## 📋 MCP Tools

### 1. `gul_generate_code`

**Generate GUL code from natural language**

```json
{
  "name": "gul_generate_code",
  "description": "Generate GUL v3.2 code from natural language description",
  "inputSchema": {
    "type": "object",
    "properties": {
      "description": {
        "type": "string",
        "description": "Natural language description of what to build"
      },
      "type": {
        "type": "string",
        "enum": ["function", "struct", "module", "application"],
        "description": "Type of code to generate"
      },
      "features": {
        "type": "array",
        "items": { "type": "string" },
        "description": "Required features (e.g., 'database', 'web', 'ai')"
      }
    },
    "required": ["description"]
  }
}
```

**Example**:

```json
{
  "description": "Create a web API that analyzes sentiment using AI",
  "type": "application",
  "features": ["web", "ai", "nlp"]
}
```

**Output**: Complete GUL v3.2 code with @ prefix syntax

---

### 2. `gul_create_package`

**Create a complete GUL package structure**

```json
{
  "name": "gul_create_package",
  "description": "Create a new GUL package with standard structure",
  "inputSchema": {
    "type": "object",
    "properties": {
      "name": {
        "type": "string",
        "description": "Package name"
      },
      "type": {
        "type": "string",
        "enum": ["library", "binary", "web", "ai", "embedded"],
        "description": "Package type"
      },
      "dependencies": {
        "type": "array",
        "items": { "type": "string" },
        "description": "Required dependencies"
      },
      "ai_features": {
        "type": "object",
        "description": "AI-specific configuration"
      }
    },
    "required": ["name", "type"]
  }
}
```

**Creates**:

```
my-package/
├── gul.toml
├── src/
│   └── main.mn
├── tests/
│   └── test.mn
├── README.md
└── .gitignore
```

---

### 3. `gul_run_code`

**Execute GUL code and return results**

```json
{
  "name": "gul_run_code",
  "description": "Run GUL code and capture output",
  "inputSchema": {
    "type": "object",
    "properties": {
      "code": {
        "type": "string",
        "description": "GUL code to execute"
      },
      "file": {
        "type": "string",
        "description": "Path to GUL file (alternative to code)"
      },
      "args": {
        "type": "array",
        "items": { "type": "string" },
        "description": "Command-line arguments"
      },
      "timeout": {
        "type": "number",
        "description": "Execution timeout in seconds"
      }
    }
  }
}
```

---

### 4. `gul_install_dependencies`

**Download and install package dependencies**

```json
{
  "name": "gul_install_dependencies",
  "description": "Install GUL package dependencies",
  "inputSchema": {
    "type": "object",
    "properties": {
      "packages": {
        "type": "array",
        "items": { "type": "string" },
        "description": "Package names to install"
      },
      "project_dir": {
        "type": "string",
        "description": "Project directory path"
      },
      "registry": {
        "type": "string",
        "description": "Package registry URL"
      }
    },
    "required": ["packages"]
  }
}
```

---

### 5. `gul_test_code`

**Run tests on GUL code**

```json
{
  "name": "gul_test_code",
  "description": "Execute GUL tests",
  "inputSchema": {
    "type": "object",
    "properties": {
      "project_dir": {
        "type": "string",
        "description": "Project directory"
      },
      "test_pattern": {
        "type": "string",
        "description": "Test file pattern"
      },
      "coverage": {
        "type": "boolean",
        "description": "Generate coverage report"
      }
    }
  }
}
```

---

### 6. `gul_ai_enhance`

**Use AI to enhance/optimize GUL code**

```json
{
  "name": "gul_ai_enhance",
  "description": "Enhance GUL code using AI",
  "inputSchema": {
    "type": "object",
    "properties": {
      "code": {
        "type": "string",
        "description": "Code to enhance"
      },
      "goals": {
        "type": "array",
        "items": {
          "type": "string",
          "enum": ["performance", "readability", "safety", "features"]
        },
        "description": "Enhancement goals"
      },
      "ai_provider": {
        "type": "string",
        "enum": ["openai", "anthropic", "google", "local"]
      }
    },
    "required": ["code"]
  }
}
```

---

### 7. `gul_project_scaffold`

**Scaffold a complete project from AI description**

```json
{
  "name": "gul_project_scaffold",
  "description": "Create entire project structure from description",
  "inputSchema": {
    "type": "object",
    "properties": {
      "description": {
        "type": "string",
        "description": "Project description in natural language"
      },
      "project_name": {
        "type": "string"
      },
      "include_ai": {
        "type": "boolean",
        "description": "Include AI capabilities"
      },
      "include_web": {
        "type": "boolean",
        "description": "Include web server"
      },
      "include_database": {
        "type": "boolean",
        "description": "Include database"
      }
    },
    "required": ["description", "project_name"]
  }
}
```

---

## 🔄 MCP Resources

### 1. `gul://templates`

**Access GUL project templates**

```json
{
  "uri": "gul://templates/{type}",
  "name": "GUL Templates",
  "description": "Project templates for different use cases",
  "mimeType": "application/gul"
}
```

Types: `basic`, `web`, `ai`, `embedded`, `data-science`

---

### 2. `gul://packages`

**Browse available packages**

```json
{
  "uri": "gul://packages/{name}",
  "name": "GUL Packages",
  "description": "Package registry access",
  "mimeType": "application/json"
}
```

---

### 3. `gul://docs`

**Access GUL documentation**

```json
{
  "uri": "gul://docs/{topic}",
  "name": "GUL Documentation",
  "description": "Language documentation and guides",
  "mimeType": "text/markdown"
}
```

---

## 🎬 Usage Examples

### Example 1: Create Web API with AI

```
AI: Create a sentiment analysis API
↓
MCP: gul_project_scaffold({
  description: "REST API that analyzes text sentiment using AI",
  project_name: "sentiment-api",
  include_ai: true,
  include_web: true
})
↓
Result: Complete project with:
- Web server setup
- AI sentiment analysis
- API endpoints
- Tests
- Documentation
```

### Example 2: Generate and Run Code

```
AI: Generate a function to process CSV data
↓
MCP: gul_generate_code({
  description: "Load CSV, filter rows, calculate stats",
  type: "function",
  features: ["data-science"]
})
↓
MCP: gul_run_code({
  code: <generated_code>,
  args: ["data.csv"]
})
↓
Result: Executed with output
```

### Example 3: Install and Test

```
AI: Install pandas and test the data processor
↓
MCP: gul_install_dependencies({
  packages: ["pandas", "numpy"]
})
↓
MCP: gul_test_code({
  project_dir: "./",
  coverage: true
})
↓
Result: Tests pass, 95% coverage
```

---

## 🛠️ Implementation

### Server Structure

```rust
// src/mcp/server.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct GulMcpServer {
    ai_manager: crate::ai::AIManager,
    code_generator: crate::autonomous::AiCodeGenerator,
    project_manager: ProjectManager,
}

impl GulMcpServer {
    pub fn new() -> Self {
        Self {
            ai_manager: AIManager::from_env(),
            code_generator: AiCodeGenerator::new(AiProvider::from_env()),
            project_manager: ProjectManager::new(),
        }
    }

    pub async fn handle_tool(&self, tool: &str, args: Value) -> Result<Value> {
        match tool {
            "gul_generate_code" => self.generate_code(args).await,
            "gul_create_package" => self.create_package(args).await,
            "gul_run_code" => self.run_code(args).await,
            "gul_install_dependencies" => self.install_deps(args).await,
            "gul_test_code" => self.test_code(args).await,
            "gul_ai_enhance" => self.ai_enhance(args).await,
            "gul_project_scaffold" => self.scaffold_project(args).await,
            _ => Err("Unknown tool".into())
        }
    }

    async fn generate_code(&self, args: Value) -> Result<Value> {
        let desc = args["description"].as_str()?;
        let code_type = args["type"].as_str().unwrap_or("function");
        let features = args["features"].as_array().unwrap_or(&vec![]);

        let request = CodeGenRequest {
            prompt: desc.to_string(),
            language: "gul".to_string(),
            context: features.iter().map(|f| f.to_string()).collect(),
            max_tokens: 2048,
            temperature: 0.7,
        };

        let response = self.code_generator.generate(request)?;

        Ok(json!({
            "code": response.code,
            "explanation": response.explanation,
            "confidence": response.confidence
        }))
    }

    async fn scaffold_project(&self, args: Value) -> Result<Value> {
        let desc = args["description"].as_str()?;
        let name = args["project_name"].as_str()?;
        let include_ai = args["include_ai"].as_bool().unwrap_or(false);
        let include_web = args["include_web"].as_bool().unwrap_or(false);

        // Generate project structure
        self.project_manager.scaffold(name, desc, include_ai, include_web)?;

        Ok(json!({
            "status": "success",
            "project_path": format!("./{}", name),
            "files_created": 10
        }))
    }
}
```

---

## 🚀 Deployment

### As MCP Server

```json
{
  "mcpServers": {
    "gul": {
      "command": "gul",
      "args": ["mcp", "serve"],
      "env": {
        "GUL_AI_PROVIDER": "openai",
        "GUL_AI_API_KEY": "${OPENAI_API_KEY}"
      }
    }
  }
}
```

### Usage with Claude Desktop

```json
{
  "mcpServers": {
    "gul-assistant": {
      "command": "gul-mcp-server",
      "args": ["--port", "3000"],
      "env": {
        "GUL_AI_PROVIDER": "anthropic",
        "GUL_AI_MODEL": "claude-3-opus-20240229"
      }
    }
  }
}
```

---

## 📊 Benefits

| Feature                     | Benefit                                      |
| --------------------------- | -------------------------------------------- |
| **Natural Language → Code** | Describe what you want, get working GUL code |
| **Complete Projects**       | Full project scaffolding from description    |
| **Auto Dependencies**       | AI determines and installs needed packages   |
| **Test Generation**         | Auto-generate tests for code                 |
| **Code Enhancement**        | AI improves existing code                    |
| **Multi-Provider**          | Works with any AI (OpenAI, Anthropic, etc.)  |
| **Standardized**            | MCP protocol standard across all AIs         |

---

## 🎯 Next Steps

1. ✅ Create `src/mcp/` module
2. ✅ Implement MCP server
3. ✅ Add tool handlers
4. ✅ Create resource providers
5. ✅ Test with Claude/GPT-4
6. ✅ Publish MCP server
7. ✅ Documentation

---

**Generated**: 2025-12-27  
**Status**: Design Complete - Ready for Implementation  
**Protocol**: MCP (Model Context Protocol)
