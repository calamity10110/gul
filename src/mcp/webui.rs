// GUL MCP WebUI - Web interface for MCP server management

use serde::{Deserialize, Serialize};

/// WebUI state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpWebUI {
    pub server_running: bool,
    pub tools: Vec<ToolInfo>,
    pub workflows: Vec<WorkflowInfo>,
    pub schedules: Vec<ScheduleInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowInfo {
    pub name: String,
    pub description: String,
    pub steps: usize,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleInfo {
    pub name: String,
    pub interval: String,
    pub last_run: Option<String>,
    pub enabled: bool,
}

impl McpWebUI {
    pub fn new() -> Self {
        Self {
            server_running: true,
            tools: vec![
                ToolInfo {
                    name: "gul_generate_code".to_string(),
                    description: "Generate code from description".to_string(),
                    enabled: true,
                },
                ToolInfo {
                    name: "gul_create_package".to_string(),
                    description: "Create new package".to_string(),
                    enabled: true,
                },
                ToolInfo {
                    name: "gul_run_code".to_string(),
                    description: "Execute GUL code".to_string(),
                    enabled: true,
                },
                ToolInfo {
                    name: "gul_install_dependencies".to_string(),
                    description: "Install packages".to_string(),
                    enabled: true,
                },
                ToolInfo {
                    name: "gul_test_code".to_string(),
                    description: "Run tests".to_string(),
                    enabled: true,
                },
                ToolInfo {
                    name: "gul_ai_enhance".to_string(),
                    description: "AI code optimization".to_string(),
                    enabled: true,
                },
                ToolInfo {
                    name: "gul_project_scaffold".to_string(),
                    description: "Create complete project".to_string(),
                    enabled: true,
                },
            ],
            workflows: vec![
                WorkflowInfo {
                    name: "ci_workflow".to_string(),
                    description: "Complete CI workflow".to_string(),
                    steps: 4,
                    status: "ready".to_string(),
                },
                WorkflowInfo {
                    name: "ai_optimize_workflow".to_string(),
                    description: "AI optimization pipeline".to_string(),
                    steps: 3,
                    status: "ready".to_string(),
                },
            ],
            schedules: vec![
                ScheduleInfo {
                    name: "auto_lint".to_string(),
                    interval: "on_commit".to_string(),
                    last_run: None,
                    enabled: true,
                },
                ScheduleInfo {
                    name: "auto_format".to_string(),
                    interval: "on_commit".to_string(),
                    last_run: None,
                    enabled: true,
                },
                ScheduleInfo {
                    name: "auto_test".to_string(),
                    interval: "on_push".to_string(),
                    last_run: None,
                    enabled: true,
                },
                ScheduleInfo {
                    name: "daily_audit".to_string(),
                    interval: "daily".to_string(),
                    last_run: None,
                    enabled: true,
                },
            ],
        }
    }
    
    /// Generate HTML dashboard
    pub fn generate_html(&self) -> String {
        format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>GUL MCP Dashboard</title>
    <style>
        body {{
            font-family: 'Inter', sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: #fff;
            margin: 0;
            padding: 20px;
        }}
        .container {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        h1 {{
            text-align: center;
            font-size: 3rem;
            margin-bottom: 2rem;
        }}
        .dashboard {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
            gap: 20px;
        }}
        .card {{
            background: rgba(255, 255, 255, 0.1);
            border-radius: 15px;
            padding: 20px;
            backdrop-filter: blur(10px);
        }}
        .card h2 {{
            margin-top: 0;
            border-bottom: 2px solid rgba(255, 255, 255, 0.3);
            padding-bottom: 10px;
        }}
        .status {{
            display: flex;
            align-items: center;
            gap: 10px;
            margin: 10px 0;
        }}
        .status-dot {{
            width: 12px;
            height: 12px;
            border-radius: 50%;
            background: #4ade80;
        }}
        .tool-item, .workflow-item, .schedule-item {{
            padding: 10px;
            margin: 5px 0;
            background: rgba(255, 255, 255, 0.05);
            border-radius: 8px;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }}
        .badge {{
            background: #4ade80;
            padding: 4px 12px;
            border-radius: 12px;
            font-size: 0.9rem;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>🤖 GUL MCP Dashboard</h1>
        
        <div class="dashboard">
            <div class="card">
                <h2>📊 Server Status</h2>
                <div class="status">
                    <div class="status-dot"></div>
                    <span>Server Running</span>
                </div>
                <div>Tools: {} available</div>
                <div>Workflows: {} active</div>
                <div>Schedules: {} active</div>
            </div>
            
            <div class="card">
                <h2>🛠️ MCP Tools</h2>
                {}
            </div>
            
            <div class="card">
                <h2>📋 Workflows</h2>
                {}
            </div>
            
            <div class="card">
                <h2>📅 Scheduled Tasks</h2>
                {}
            </div>
        </div>
    </div>
</body>
</html>
        "#,
            self.tools.len(),
            self.workflows.len(),
            self.schedules.len(),
            self.tools.iter()
                .map(|t| format!(r#"<div class="tool-item"><span>{}</span><span class="badge">✓</span></div>"#, t.name))
                .collect::<Vec<_>>()
                .join("\n"),
            self.workflows.iter()
                .map(|w| format!(r#"<div class="workflow-item"><span>{}</span><span>{} steps</span></div>"#, w.name, w.steps))
                .collect::<Vec<_>>()
                .join("\n"),
            self.schedules.iter()
                .map(|s| format!(r#"<div class="schedule-item"><span>{}</span><span>{}</span></div>"#, s.name, s.interval))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
    
    /// Get JSON status
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "server_running": self.server_running,
            "tools": self.tools,
            "workflows": self.workflows,
            "schedules": self.schedules
        })
    }
}

impl Default for McpWebUI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_webui_creation() {
        let webui = McpWebUI::new();
        assert!(webui.server_running);
        assert_eq!(webui.tools.len(), 7);
    }
    
    #[test]
    fn test_generate_html() {
        let webui = McpWebUI::new();
        let html = webui.generate_html();
        assert!(html.contains("GUL MCP Dashboard"));
        assert!(html.contains("gul_generate_code"));
    }
}
