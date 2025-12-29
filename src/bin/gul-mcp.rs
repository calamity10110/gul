// GUL MCP Server Binary
// AI-powered Model Context Protocol server

use gul_lang::mcp::server::MCPServer;
use gul_lang::mcp::cli::parse_args;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args();
    
    let server = MCPServer::new("gul-mcp", "0.13.0");
    
    match args.command.as_str() {
        "start" => {
            let port = args.port.unwrap_or(3000);
            println!("🚀 Starting GUL MCP Server on port {}", port);
            server.start(port).await?;
        }
        "status" => {
            println!("✓ GUL MCP Server is configured and ready");
        }
        _ => {
            println!("Unknown command. Use 'start' or 'status'");
        }
    }
    
    Ok(())
}
