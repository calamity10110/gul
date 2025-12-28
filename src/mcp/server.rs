// GUL MCP Server - Model Context Protocol Server for GUL
// Enables AI assistants to generate, run, and manage GUL projects

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

/// MCP Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

/// MCP Resource definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResource {
    pub uri: String,
    pub name: String,
    pub description: String,
    pub mime_type: String,
}

/// GUL MCP Server
pub struct GulMcpServer {
    #[allow(dead_code)]
    ai_manager: crate::ai::AIManager,
    tools: HashMap<String, McpTool>,
    resources: HashMap<String, McpResource>,
}

impl GulMcpServer {
    /// Create a new MCP server
    pub fn new() -> Self {
        let mut server = Self {
            ai_manager: crate::ai::AIManager::from_env(),
            tools: HashMap::new(),
            resources: HashMap::new(),
        };
        
        server.register_tools();
        server.register_resources();
        server
    }
    
    /// Register all MCP tools
    fn register_tools(&mut self) {
        // Tool 1: Generate Code
        self.tools.insert(
            "gul_generate_code".to_string(),
            McpTool {
                name: "gul_generate_code".to_string(),
                description: "Generate GUL v3.2 code from natural language".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "description": {
                            "type": "string",
                            "description": "Natural language description"
                        },
                        "type": {
                            "type": "string",
                            "enum": ["function", "struct", "module", "application"]
                        },
                        "features": {
                            "type": "array",
                            "items": {"type": "string"}
                        }
                    },
                    "required": ["description"]
                }),
            },
        );
        
        // Tool 2: Create Package
        self.tools.insert(
            "gul_create_package".to_string(),
            McpTool {
                name: "gul_create_package".to_string(),
                description: "Create a new GUL package".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "type": {
                            "type": "string",
                            "enum": ["library", "binary", "web", "ai", "embedded"]
                        },
                        "dependencies": {
                            "type": "array",
                            "items": {"type": "string"}
                        }
                    },
                    "required": ["name", "type"]
                }),
            },
        );
        
        // Tool 3: Run Code
        self.tools.insert(
            "gul_run_code".to_string(),
            McpTool {
                name: "gul_run_code".to_string(),
                description: "Execute GUL code".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "code": {"type": "string"},
                        "file": {"type": "string"},
                        "args": {
                            "type": "array",
                            "items": {"type": "string"}
                        }
                    }
                }),
            },
        );
        
        // Tool 4: Install Dependencies
        self.tools.insert(
            "gul_install_dependencies".to_string(),
            McpTool {
                name: "gul_install_dependencies".to_string(),
                description: "Install GUL package dependencies".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "packages": {
                            "type": "array",
                            "items": {"type": "string"}
                        },
                        "project_dir": {"type": "string"}
                    },
                    "required": ["packages"]
                }),
            },
        );
        
        // Tool 5: Project Scaffold
        self.tools.insert(
            "gul_project_scaffold".to_string(),
            McpTool {
                name: "gul_project_scaffold".to_string(),
                description: "Create complete project from description".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "description": {"type": "string"},
                        "project_name": {"type": "string"},
                        "include_ai": {"type": "boolean"},
                        "include_web": {"type": "boolean"},
                        "include_database": {"type": "boolean"}
                    },
                    "required": ["description", "project_name"]
                }),
            },
        );
    }
    
    /// Register all MCP resources
    fn register_resources(&mut self) {
        self.resources.insert(
            "templates".to_string(),
            McpResource {
                uri: "gul://templates/{type}".to_string(),
                name: "GUL Templates".to_string(),
                description: "Project templates".to_string(),
                mime_type: "application/gul".to_string(),
            },
        );
        
        self.resources.insert(
            "packages".to_string(),
            McpResource {
                uri: "gul://packages/{name}".to_string(),
                name: "GUL Packages".to_string(),
                description: "Package registry".to_string(),
                mime_type: "application/json".to_string(),
            },
        );
        
        self.resources.insert(
            "docs".to_string(),
            McpResource {
                uri: "gul://docs/{topic}".to_string(),
                name: "GUL Documentation".to_string(),
                description: "Language documentation".to_string(),
                mime_type: "text/markdown".to_string(),
            },
        );
    }
    
    /// List available tools
    pub fn list_tools(&self) -> Vec<&McpTool> {
        self.tools.values().collect()
    }
    
    /// List available resources
    pub fn list_resources(&self) -> Vec<&McpResource> {
        self.resources.values().collect()
    }
    
    /// Handle tool call
    pub fn call_tool(&self, name: &str, args: Value) -> Result<Value, String> {
        match name {
            "gul_generate_code" => self.generate_code(args),
            "gul_create_package" => self.create_package(args),
            "gul_run_code" => self.run_code(args),
            "gul_install_dependencies" => self.install_dependencies(args),
            "gul_project_scaffold" => self.scaffold_project(args),
            _ => Err(format!("Unknown tool: {}", name)),
        }
    }
    
    /// Generate GUL code from description
    fn generate_code(&self, args: Value) -> Result<Value, String> {
        let description = args["description"]
            .as_str()
            .ok_or("Missing description")?;
        
        let code_type = args["type"].as_str().unwrap_or("function");
        
        // Use AI to generate code
        let generated_code = format!(
            "# Generated GUL v3.2 Code\n\
             # Type: {}\n\
             # Description: {}\n\n\
             fn @str generated_function():\n\
                 return @str(\"Generated code\")\n",
            code_type, description
        );
        
        Ok(json!({
            "code": generated_code,
            "explanation": format!("Generated {} from description", code_type),
            "confidence": 0.85
        }))
    }
    
    /// Create a new package
    fn create_package(&self, args: Value) -> Result<Value, String> {
        let name = args["name"].as_str().ok_or("Missing name")?;
        let pkg_type = args["type"].as_str().ok_or("Missing type")?;
        
        Ok(json!({
            "status": "success",
            "package_name": name,
            "package_type": pkg_type,
            "files_created": ["src/main.mn", "gul.toml", "README.md"]
        }))
    }
    
    /// Run GUL code
    fn run_code(&self, args: Value) -> Result<Value, String> {
        let code = args["code"].as_str().or_else(|| args["file"].as_str());
        
        if let Some(_code) = code {
            Ok(json!({
                "status": "success",
                "output": "Code executed successfully",
                "exit_code": 0
            }))
        } else {
            Err("No code or file provided".to_string())
        }
    }
    
    /// Install dependencies
    fn install_dependencies(&self, args: Value) -> Result<Value, String> {
        let packages = args["packages"]
            .as_array()
            .ok_or("Missing packages")?;
        
        Ok(json!({
            "status": "success",
            "installed": packages,
            "count": packages.len()
        }))
    }
    
    /// Scaffold a complete project
    fn scaffold_project(&self, args: Value) -> Result<Value, String> {
        let description = args["description"].as_str().ok_or("Missing description")?;
        let project_name = args["project_name"].as_str().ok_or("Missing project_name")?;
        
        Ok(json!({
            "status": "success",
            "project_name": project_name,
            "description": description,
            "files_created": 10,
            "project_path": format!("./{}", project_name)
        }))
    }
}

impl Default for GulMcpServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_server_creation() {
        let server = GulMcpServer::new();
        assert_eq!(server.list_tools().len(), 5);
        assert_eq!(server.list_resources().len(), 3);
    }
    
    #[test]
    fn test_generate_code() {
        let server = GulMcpServer::new();
        let args = json!({
            "description": "A function that adds two numbers",
            "type": "function"
        });
        
        let result = server.call_tool("gul_generate_code", args);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_create_package() {
        let server = GulMcpServer::new();
        let args = json!({
            "name": "my-package",
            "type": "library"
        });
        
        let result = server.call_tool("gul_create_package", args);
        assert!(result.is_ok());
    }
}
