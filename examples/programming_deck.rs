use gul_lang::runtime::ui_runtime::{TreeNode, UiComponent, UiRuntime};
use std::{thread, time::Duration};

fn main() {
    let mut runtime = UiRuntime::new();
    runtime.clear();

    // Simulate a "Programming Deck" / IDE Interface

    // 1. Header / Toolbar
    let header = UiComponent::Text {
        content: " GLOB IDE v0.11.0 - [Programming Deck] ".to_string(),
        fg: Some("white".to_string()),
        bg: Some("blue".to_string()),
        bold: true,
    };

    // 2. Left Panel: File Tree
    let file_tree = UiComponent::Tree {
        nodes: vec![
            TreeNode {
                name: "src".to_string(),
                children: vec![
                    TreeNode {
                        name: "main.mn".to_string(),
                        children: vec![],
                    },
                    TreeNode {
                        name: "utils.mn".to_string(),
                        children: vec![],
                    },
                ],
            },
            TreeNode {
                name: "tests".to_string(),
                children: vec![TreeNode {
                    name: "test_api.mn".to_string(),
                    children: vec![],
                }],
            },
            TreeNode {
                name: "package.toml".to_string(),
                children: vec![],
            },
        ],
    };

    // 3. Center Panel: Code Editor (simulated with Text)
    let editor_content = UiComponent::VBox {
        children: vec![
            UiComponent::Text {
                content: "mn main():".to_string(),
                fg: Some("magenta".to_string()),
                bg: None,
                bold: true,
            },
            UiComponent::Text {
                content: "    print(\"Hello, Programming Deck!\")".to_string(),
                fg: Some("green".to_string()),
                bg: None,
                bold: false,
            },
            UiComponent::Text {
                content: "    ?count = 0".to_string(),
                fg: Some("cyan".to_string()),
                bg: None,
                bold: false,
            },
            UiComponent::Text {
                content: "    @while ?count < 10:".to_string(),
                fg: Some("magenta".to_string()),
                bg: None,
                bold: true,
            },
            UiComponent::Text {
                content: "        ?count = ?count + 1".to_string(),
                fg: Some("white".to_string()),
                bg: None,
                bold: false,
            },
        ],
    };

    // 4. Right Panel: System Stats
    let stats_panel = UiComponent::VBox {
        children: vec![
            UiComponent::Text {
                content: "CPU Usage".to_string(),
                fg: Some("yellow".to_string()),
                bg: None,
                bold: true,
            },
            UiComponent::Progress {
                value: 45,
                max: 100,
                label: None,
            },
            UiComponent::Text {
                content: "Memory".to_string(),
                fg: Some("yellow".to_string()),
                bg: None,
                bold: true,
            },
            UiComponent::Progress {
                value: 60,
                max: 100,
                label: None,
            },
            UiComponent::Text {
                content: "Disk".to_string(),
                fg: Some("yellow".to_string()),
                bg: None,
                bold: true,
            },
            UiComponent::Progress {
                value: 20,
                max: 100,
                label: None,
            },
        ],
    };

    // Combine Left, Center, Right into Main Area
    // Note: HBox implementation in ui_runtime.rs is simple sequential rendering for now,
    // but conceptually this represents the layout.
    let main_area = UiComponent::HBox {
        children: vec![
            UiComponent::VBox {
                children: vec![
                    UiComponent::Text {
                        content: "[EXPLORER]".to_string(),
                        fg: Some("cyan".to_string()),
                        bg: None,
                        bold: true,
                    },
                    file_tree,
                ],
            },
            UiComponent::VBox {
                children: vec![
                    UiComponent::Text {
                        content: "[EDITOR - main.mn]".to_string(),
                        fg: Some("cyan".to_string()),
                        bg: None,
                        bold: true,
                    },
                    editor_content,
                ],
            },
            UiComponent::VBox {
                children: vec![
                    UiComponent::Text {
                        content: "[SYSTEM]".to_string(),
                        fg: Some("cyan".to_string()),
                        bg: None,
                        bold: true,
                    },
                    stats_panel,
                ],
            },
        ],
    };

    // 5. Bottom Panel: Terminal / Output
    let terminal = UiComponent::VBox {
        children: vec![
            UiComponent::Text {
                content: "TERMINAL".to_string(),
                fg: Some("white".to_string()),
                bg: Some("black".to_string()),
                bold: true,
            },
            UiComponent::Text {
                content: "$ cargo run".to_string(),
                fg: Some("green".to_string()),
                bg: None,
                bold: false,
            },
            UiComponent::Text {
                content: "Compiling gul v0.11.0...".to_string(),
                fg: Some("white".to_string()),
                bg: None,
                bold: false,
            },
            UiComponent::Text {
                content: "Finished dev [unoptimized + debuginfo] target(s) in 0.5s".to_string(),
                fg: Some("white".to_string()),
                bg: None,
                bold: false,
            },
            UiComponent::Text {
                content: "Running `target/debug/glob`".to_string(),
                fg: Some("white".to_string()),
                bg: None,
                bold: false,
            },
        ],
    };

    // 6. Status Bar
    let status_bar = UiComponent::Text {
        content: " READY  |  Ln 5, Col 20  |  UTF-8  |  GLOB".to_string(),
        fg: Some("black".to_string()),
        bg: Some("green".to_string()),
        bold: true,
    };

    // Render Everything
    runtime.print(header);
    println!("\n"); // Spacer
    runtime.print(main_area);
    println!("\n"); // Spacer
    runtime.print(terminal);
    println!("\n"); // Spacer
    runtime.print(status_bar);

    // Interactive Loop (Simulation)
    println!("\n(Press Ctrl+C to exit)");
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
