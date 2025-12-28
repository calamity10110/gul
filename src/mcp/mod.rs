// GUL MCP Module
// Model Context Protocol server for AI-assisted development

pub mod server;
pub mod scheduler;
pub mod workflow;
pub mod auto_maintenance;
pub mod cli;
pub mod tui;
pub mod webui;

pub use server::{GulMcpServer, McpResource, McpTool};
pub use scheduler::{TaskScheduler, Schedule, ScheduledTask, ScheduleInterval};
pub use workflow::{WorkflowExecutor, Workflow, WorkflowStep, WorkflowTrigger};
pub use auto_maintenance::AutoMaintenance;
pub use cli::{McpCli, execute_cli};
pub use tui::McpDashboard;
pub use webui::McpWebUI;
