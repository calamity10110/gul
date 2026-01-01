// GUL MCP Module
// Model Context Protocol server for AI-assisted development

pub mod auto_maintenance;
pub mod cli;
pub mod scheduler;
pub mod server;
pub mod tui;
pub mod webui;
pub mod workflow;

pub use auto_maintenance::AutoMaintenance;
pub use cli::{execute_cli, McpCli};
pub use scheduler::{Schedule, ScheduleInterval, ScheduledTask, TaskScheduler};
pub use server::{GulMcpServer, McpResource, McpTool};
pub use tui::McpDashboard;
pub use webui::McpWebUI;
pub use workflow::{Workflow, WorkflowExecutor, WorkflowStep, WorkflowTrigger};
