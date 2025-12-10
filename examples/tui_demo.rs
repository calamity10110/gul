use gul_lang::runtime::ui_runtime::{UiComponent, UiRuntime};
use std::{thread, time::Duration};

fn main() {
    let mut runtime = UiRuntime::new();

    println!("Starting TUI Demo...");
    thread::sleep(Duration::from_secs(1));
    runtime.clear();

    // 1. Text Components
    println!("--- Text Components ---");
    runtime.print(UiComponent::Text {
        content: "Welcome to GUL TUI Demo".to_string(),
        fg: Some("green".to_string()),
        bg: None,
        bold: true,
    });

    runtime.print(UiComponent::Text {
        content: "This is a demonstration of the built-in TUI capabilities.".to_string(),
        fg: Some("blue".to_string()),
        bg: None,
        bold: false,
    });

    // 2. Button
    println!("\n--- Button ---");
    runtime.print(UiComponent::Button {
        text: "Click Me!".to_string(),
    });

    // 3. Slider
    println!("\n--- Slider ---");
    runtime.print(UiComponent::Slider {
        min: 0,
        max: 100,
        value: 75,
        label: Some("Volume".to_string()),
    });

    // 4. Progress Bar
    println!("\n--- Progress Bar ---");
    runtime.print(UiComponent::Progress {
        value: 45,
        max: 100,
        label: Some("Downloading".to_string()),
    });

    // 5. Table
    println!("\n--- Table ---");
    runtime.print(UiComponent::Table {
        headers: vec!["ID".to_string(), "Name".to_string(), "Role".to_string()],
        rows: vec![
            vec!["1".to_string(), "Alice".to_string(), "Admin".to_string()],
            vec!["2".to_string(), "Bob".to_string(), "User".to_string()],
            vec!["3".to_string(), "Charlie".to_string(), "Guest".to_string()],
        ],
    });

    // 6. Canvas (ASCII Art)
    println!("\n--- Canvas ---");
    let mut canvas = UiComponent::new_canvas(40, 10);
    canvas.draw_rect(5, 1, 35, 8);
    canvas.draw_line(5, 1, 35, 8);
    canvas.draw_line(5, 8, 35, 1);
    runtime.print(canvas);

    println!("\nDemo completed successfully!");
}
