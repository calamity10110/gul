// GUL MCP Server Binary
// Runs the GUL MCP server

use gul_lang::mcp::server::GulMcpServer;

fn main() {
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
}
