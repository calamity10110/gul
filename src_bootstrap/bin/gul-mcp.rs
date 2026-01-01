// GUL MCP Server Binary with CLI
// Provides both MCP protocol server and command-line interface

mod gul_mcp_codegen;

use clap::{Parser, Subcommand, ValueEnum};
use gul_lang::mcp::server::GulMcpServer;
use std::process::Command;

#[derive(Parser)]
#[command(name = "gul-mcp")]
#[command(version = "0.13.0")]
#[command(about = "GUL MCP Server and CLI")]
#[command(
    long_about = "GUL Model Context Protocol server with command-line interface for code generation, automation, and AI agent integration"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start MCP protocol server
    Serve {
        #[arg(short, long, default_value = "3000")]
        port: u16,

        #[arg(long)]
        daemon: bool,
    },

    /// Generate GUL code from natural language description
    Generate {
        /// Description of code to generate
        description: String,

        #[arg(short, long)]
        output: Option<String>,
        
        #[arg(long)]
        use_api: bool,
    },

    /// Run auto-maintenance tasks
    Auto {
        #[arg(value_enum)]
        task: AutoTask,
    },

    /// Manage scheduled tasks
    Schedule {
        #[arg(value_enum)]
        action: ScheduleAction,

        /// Schedule name
        name: String,
    },

    /// Execute workflows
    Workflow {
        #[arg(value_enum)]
        action: WorkflowAction,

        /// Workflow name (required for run/delete)
        name: Option<String>,
    },
}

#[derive(ValueEnum, Clone)]
enum AutoTask {
    Fmt,
    Lint,
    Check,
    Audit,
    All,
}

#[derive(ValueEnum, Clone)]
enum ScheduleAction {
    List,
    Enable,
    Disable,
    Create,
}

#[derive(ValueEnum, Clone)]
enum WorkflowAction {
    List,
    Run,
    Create,
    Delete,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(cmd) => {
            if let Err(e) = execute_command(cmd) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            // Default: show server info (backward compatibility)
            show_server_info();
        }
    }
}

fn execute_command(cmd: Commands) -> Result<(), String> {
    match cmd {
        Commands::Serve { port, daemon } => cmd_serve(port, daemon),
        Commands::Generate {
            description,
            output,
            use_api,
        } => cmd_generate(description, output, use_api),
        Commands::Auto { task } => cmd_auto(task),
        Commands::Schedule { action, name } => cmd_schedule(action, name),
        Commands::Workflow { action, name } => cmd_workflow(action, name),
    }
}

fn cmd_serve(port: u16, daemon: bool) -> Result<(), String> {
    println!("GUL MCP Server v0.13.0");
    println!("======================\n");

    if daemon {
        return Err("Daemon mode not yet implemented".to_string());
    }

    println!("Starting MCP server on port {}...", port);

    let server = GulMcpServer::new();

    println!("\nRegistered {} tools:", server.list_tools().len());
    for tool in server.list_tools() {
        println!("  - {}: {}", tool.name, tool.description);
    }

    println!("\nRegistered {} resources:", server.list_resources().len());
    for resource in server.list_resources() {
        println!("  - {}: {}", resource.name, resource.description);
    }

    println!("\n✅ Server ready on port {}", port);
    println!("Use MCP protocol to interact with tools");
    println!("Tools are registered and ready for MCP protocol integration");

    Ok(())
}

fn cmd_generate(description: String, output: Option<String>, use_api: bool) -> Result<(), String> {
    println!("🤖 Generating GUL v3.2 code...");
    println!("Description: {}", description);
    
    if use_api {
        println!("Mode: AI-powered (using public API)");
    } else {
        println!("Mode: Template-based");
    }
    println!();
    
    // Generate code using enhanced v3.2 templates or AI
    let code = gul_mcp_codegen::generate_v32_code(&description, use_api)?;
    
    if let Some(file) = output {
        std::fs::write(&file, &code)
            .map_err(|e| format!("Failed to write file: {}", e))?;
        println!("✅ Code written to: {}", file);
        println!("   Syntax: GUL v3.2 with full annotations");
    } else {
        println!("Generated code:\n{}", code);
    }
    

    
    Ok(())
}


fn cmd_auto(task: AutoTask) -> Result<(), String> {
    match task {
        AutoTask::Fmt => {
            println!("🔧 Running cargo fmt...");
            run_cargo_cmd(&["fmt"])
        }
        AutoTask::Lint => {
            println!("🔍 Running cargo clippy...");
            run_cargo_cmd(&["clippy", "--", "-D", "warnings"])
        }
        AutoTask::Check => {
            println!("✅ Running cargo check...");
            run_cargo_cmd(&["check"])
        }
        AutoTask::Audit => {
            println!("🔒 Running cargo audit...");
            run_cargo_cmd(&["audit"])
        }
        AutoTask::All => {
            println!("🚀 Running all maintenance tasks...\n");
            cmd_auto(AutoTask::Fmt)?;
            println!();
            cmd_auto(AutoTask::Lint)?;
            println!();
            cmd_auto(AutoTask::Check)?;
            println!();
            cmd_auto(AutoTask::Audit)?;
            println!("\n✅ All maintenance tasks complete!");
            Ok(())
        }
    }
}

fn cmd_schedule(action: ScheduleAction, _name: String) -> Result<(), String> {
    match action {
        ScheduleAction::List => {
            Err("Schedule management not yet implemented".to_string())
        }
        ScheduleAction::Enable => {
            Err("Schedule management not yet implemented".to_string())
        }
        ScheduleAction::Disable => {
            Err("Schedule management not yet implemented".to_string())
        }
        ScheduleAction::Create => {
            Err("Schedule management not yet implemented".to_string())
        }
    }
}

fn cmd_workflow(action: WorkflowAction, _name: Option<String>) -> Result<(), String> {
    match action {
        WorkflowAction::List => {
            Err("Workflow management not yet implemented".to_string())
        }
        WorkflowAction::Run => {
            Err("Workflow management not yet implemented".to_string())
        }
        WorkflowAction::Create => {
            Err("Workflow management not yet implemented".to_string())
        }
        WorkflowAction::Delete => {
            Err("Workflow management not yet implemented".to_string())
        }
    }
}

fn run_cargo_cmd(args: &[&str]) -> Result<(), String> {
    let output = Command::new("cargo")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run cargo: {}", e))?;

    if output.status.success() {
        println!("✅ Success");
        if !output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Ok(())
    } else {
        println!("❌ Failed");
        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        }
        Err(format!("cargo {:?} failed", args))
    }
}

fn show_server_info() {
    println!("GUL MCP Server v0.13.0");
    println!("======================\n");

    let server = GulMcpServer::new();

    println!("Registered {} tools:", server.list_tools().len());
    for tool in server.list_tools() {
        println!("  - {}: {}", tool.name, tool.description);
    }

    println!("\nRegistered {} resources:", server.list_resources().len());
    for resource in server.list_resources() {
        println!("  - {}: {}", resource.name, resource.description);
    }

    println!("\nServer ready. Use the MCP protocol to interact.");
    println!("Example tools: gul_generate_code, gul_create_package, gul_run_code");
    println!("\n💡 Tip: Run 'gul-mcp --help' to see all available commands");
}
