use gul_tui::widgets::{Button, InputField};

fn main() {
    let btn = Button::new("Submit").selected(true);
    let inp = InputField::new("").placeholder("Username");
    println!("TUI Widgets instantiated (Requires ratatui generic backend to render)");
}
