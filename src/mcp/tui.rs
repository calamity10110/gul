// GUL MCP TUI Dashboard - Terminal UI for MCP server management

use ratatui::prelude::*;
use ratatui::widgets::*;

/// MCP Dashboard state
pub struct McpDashboard {
    pub tools: Vec<String>,
    pub workflows: Vec<String>,
    pub schedules: Vec<String>,
    pub status: ServerStatus,
}

#[derive(Debug, Clone)]
pub struct ServerStatus {
    pub running: bool,
    pub tools_available: usize,
    pub workflows_active: usize,
    pub schedules_active: usize,
}

impl McpDashboard {
    pub fn new() -> Self {
        Self {
            tools: vec![
                "gul_generate_code".to_string(),
                "gul_create_package".to_string(),
                "gul_run_code".to_string(),
                "gul_install_dependencies".to_string(),
                "gul_test_code".to_string(),
                "gul_ai_enhance".to_string(),
                "gul_project_scaffold".to_string(),
            ],
            workflows: vec![
                "ci_workflow".to_string(),
                "ai_optimize_workflow".to_string(),
            ],
            schedules: vec![
                "auto_lint (on commit)".to_string(),
                "auto_format (on commit)".to_string(),
                "auto_test (on push)".to_string(),
                "daily_audit (daily)".to_string(),
            ],
            status: ServerStatus {
                running: true,
                tools_available: 7,
                workflows_active: 2,
                schedules_active: 4,
            },
        }
    }

    /// Get status widget
    pub fn status_widget(&self) -> Paragraph<'static> {
        let block = Block::default()
            .title("📊 MCP Server Status")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded);

        let status_text = [
            format!(
                "Server: {}",
                if self.status.running {
                    "✅ Running"
                } else {
                    "❌ Stopped"
                }
            ),
            format!("Tools Available: {}", self.status.tools_available),
            format!("Workflows Active: {}", self.status.workflows_active),
            format!("Schedules Active: {}", self.status.schedules_active),
        ];

        Paragraph::new(status_text.join("\n"))
            .block(block)
            .wrap(Wrap { trim: true })
    }

    /// Get tools list widget
    pub fn tools_widget(&self) -> List<'static> {
        let items: Vec<ListItem> = self
            .tools
            .iter()
            .map(|t| ListItem::new(format!("🛠️  {}", t)))
            .collect();

        List::new(items)
            .block(
                Block::default()
                    .title("MCP Tools")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(Style::default().fg(Color::Yellow))
    }

    /// Get workflows widget
    pub fn workflows_widget(&self) -> List<'static> {
        let items: Vec<ListItem> = self
            .workflows
            .iter()
            .map(|w| ListItem::new(format!("📋 {}", w)))
            .collect();

        List::new(items)
            .block(
                Block::default()
                    .title("Workflows")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(Style::default().fg(Color::Green))
    }

    /// Get schedules widget
    pub fn schedules_widget(&self) -> List<'static> {
        let items: Vec<ListItem> = self
            .schedules
            .iter()
            .map(|s| ListItem::new(format!("📅 {}", s)))
            .collect();

        List::new(items)
            .block(
                Block::default()
                    .title("Scheduled Tasks")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(Style::default().fg(Color::Cyan))
    }
}

impl Default for McpDashboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dashboard_creation() {
        let dashboard = McpDashboard::new();
        assert_eq!(dashboard.tools.len(), 7);
        assert_eq!(dashboard.workflows.len(), 2);
        assert_eq!(dashboard.schedules.len(), 4);
    }

    #[test]
    fn test_server_status() {
        let dashboard = McpDashboard::new();
        assert!(dashboard.status.running);
        assert_eq!(dashboard.status.tools_available, 7);
    }
}
