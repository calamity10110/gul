// GUL New TUI IDE Demo
// Run the new Ratatui-based TUI IDE

use gul_lang::tui::GulTuiApp;

fn main() {
    println!("🚀 Launching GUL TUI IDE (Ratatui)...");

    let mut app = GulTuiApp::new();

    if let Err(e) = app.run() {
        eprintln!("TUI Error: {}", e);
        std::process::exit(1);
    }
}
