// Example: TUI Application

use gul_tui::{
    widgets::{Block, Text},
    Color, Frame, Rect, Style, Terminal,
};

fn main() -> Result<(), std::io::Error> {
    println!("GUL TUI Example\n");

    let mut terminal = Terminal::new()?;
    let (width, height) = terminal.size();

    println!("Terminal size: {}x{}", width, height);

    terminal.draw(|frame| {
        let area = Rect::new(0, 0, width, height);

        // Create styled text
        let text = Text::new("Hello from GUL TUI!");

        // Create block with title
        let block = Block::new().title("GUL TUI Demo");

        frame.render_widget(block, area);
        frame.render_widget(text, area);
    })?;

    println!("TUI rendered successfully!");

    Ok(())
}
