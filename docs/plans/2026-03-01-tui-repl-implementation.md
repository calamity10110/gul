# TUI REPL Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build an interactive TUI REPL for GUL with output pane, input editing, variables sidebar, and project banner, using the existing ratatui setup and bootstrap interpreter.

**Architecture:** New `src_bootstrap/repl/` module with 5 files. The REPL renders a split-pane TUI using ratatui (already a dependency), accepts user input via crossterm key events, evaluates GUL code through the existing Lexer → Parser → Interpreter pipeline, and displays results. The interpreter persists state between evaluations. A new `repl-tui` CLI subcommand launches it.

**Tech Stack:** Rust, ratatui 0.29, crossterm 0.28, existing `src_bootstrap` interpreter pipeline.

---

### Task 1: Make interpreter variables publicly accessible

**Files:**
- Modify: `src_bootstrap/backend/interpreter/mod.rs:14`

**Step 1: Change `pub(crate)` to `pub` on the variables field**

Find:
```rust
pub(crate) variables: HashMap<String, Value>,
```

Replace with:
```rust
pub variables: HashMap<String, Value>,
```

**Step 2: Run tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib`
Expected: 493 passed, 0 failed, 1 ignored

**Step 3: Commit**

```bash
git add src_bootstrap/backend/interpreter/mod.rs
git commit -m "refactor: make interpreter variables field public for REPL access"
```

---

### Task 2: Create REPL state module

**Files:**
- Create: `src_bootstrap/repl/state.rs`
- Create: `src_bootstrap/repl/mod.rs`

**Step 1: Create `src_bootstrap/repl/state.rs`**

```rust
use std::collections::HashMap;
use crate::backend::interpreter::{Interpreter, Value};
use crate::frontend::lexer::Lexer;
use crate::frontend::parser::Parser;

/// A single entry in the output history
#[derive(Clone, Debug)]
pub enum OutputEntry {
    Input(String),
    Continuation(String),
    Result(String),
    Error(String),
}

/// Variable info for sidebar display
#[derive(Clone, Debug)]
pub struct VarInfo {
    pub name: String,
    pub type_name: String,
    pub short_value: String,
}

/// Core REPL state
pub struct ReplState {
    pub interpreter: Interpreter,
    pub output: Vec<OutputEntry>,
    pub history: Vec<String>,
    pub history_pos: Option<usize>,
    pub input: String,
    pub cursor_col: usize,
    pub multiline_buf: Vec<String>,
    pub in_multiline: bool,
    pub loaded_file: Option<String>,
    pub project_name: String,
}

impl ReplState {
    pub fn new(project_name: String) -> Self {
        Self {
            interpreter: Interpreter::new(),
            output: Vec::new(),
            history: Vec::new(),
            history_pos: None,
            input: String::new(),
            cursor_col: 0,
            multiline_buf: Vec::new(),
            in_multiline: false,
            loaded_file: None,
            project_name,
        }
    }

    /// Evaluate a line of GUL code. Returns true if evaluation happened.
    pub fn eval_line(&mut self, line: &str) -> bool {
        // Handle special commands
        if line.starts_with(':') {
            return self.handle_command(line);
        }

        // Check for multi-line start (line ends with ':')
        let trimmed = line.trim();
        if trimmed.ends_with(':') && !self.in_multiline {
            self.in_multiline = true;
            self.multiline_buf.push(line.to_string());
            self.output.push(OutputEntry::Input(line.to_string()));
            return false;
        }

        // In multi-line mode
        if self.in_multiline {
            if trimmed.is_empty() {
                // Empty line submits the block
                self.in_multiline = false;
                let full_source = self.multiline_buf.join("\n");
                self.multiline_buf.clear();
                self.evaluate_source(&full_source);
                return true;
            } else {
                self.multiline_buf.push(line.to_string());
                self.output.push(OutputEntry::Continuation(line.to_string()));
                return false;
            }
        }

        // Single-line evaluation
        self.output.push(OutputEntry::Input(line.to_string()));
        self.evaluate_source(line);
        self.history.push(line.to_string());
        self.history_pos = None;
        true
    }

    fn evaluate_source(&mut self, source: &str) {
        // Wrap bare expressions in mn: block for evaluation
        let wrapped = if source.contains("mn:") || source.contains("@fn ")
            || source.contains("const ") || source.contains("var ")
            || source.contains("@imp ") || source.contains("@type ") {
            source.to_string()
        } else {
            format!("mn:\n    print({})", source)
        };

        let mut lexer = Lexer::new(&wrapped);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(program) => {
                match self.interpreter.run(&program) {
                    Ok(()) => {}
                    Err(e) => {
                        self.output.push(OutputEntry::Error(e));
                    }
                }
            }
            Err(e) => {
                self.output.push(OutputEntry::Error(format!("Parse error: {}", e)));
            }
        }
    }

    fn handle_command(&mut self, cmd: &str) -> bool {
        let parts: Vec<&str> = cmd.trim().splitn(2, ' ').collect();
        match parts[0] {
            ":help" => {
                self.output.push(OutputEntry::Result("Commands: :help :clear :load <file> :reset :quit".to_string()));
                self.output.push(OutputEntry::Result("Keys: Up/Down=history, Ctrl-C=cancel, Ctrl-L=clear, Ctrl-D=quit".to_string()));
            }
            ":clear" => {
                self.output.clear();
            }
            ":reset" => {
                self.interpreter = Interpreter::new();
                self.output.push(OutputEntry::Result("State reset.".to_string()));
            }
            ":load" => {
                if parts.len() > 1 {
                    let path = parts[1].trim();
                    match std::fs::read_to_string(path) {
                        Ok(source) => {
                            self.loaded_file = Some(path.to_string());
                            self.evaluate_source(&source);
                            self.output.push(OutputEntry::Result(format!("Loaded: {}", path)));
                        }
                        Err(e) => {
                            self.output.push(OutputEntry::Error(format!("Failed to load: {}", e)));
                        }
                    }
                } else {
                    self.output.push(OutputEntry::Error("Usage: :load <file.mn>".to_string()));
                }
            }
            ":quit" => return true, // Signal quit
            _ => {
                self.output.push(OutputEntry::Error(format!("Unknown command: {}", parts[0])));
            }
        }
        false
    }

    /// Get variables for sidebar display
    pub fn get_variables(&self) -> Vec<VarInfo> {
        self.interpreter.variables.iter()
            .filter(|(_, v)| !matches!(v, Value::NativeFunction(_)))
            .map(|(name, value)| {
                let type_name = match value {
                    Value::Integer(_) => "int",
                    Value::Float(_) => "float",
                    Value::String(_) => "str",
                    Value::Bool(_) => "bool",
                    Value::List(_) => "list",
                    Value::Dict(_) => "dict",
                    Value::Function(params, _, _) => "fn",
                    Value::Lambda(_, _) => "lambda",
                    Value::Null => "null",
                    _ => "any",
                }.to_string();
                let short_value = match value {
                    Value::Integer(n) => n.to_string(),
                    Value::Float(f) => format!("{:.4}", f),
                    Value::String(s) => {
                        if s.len() > 20 { format!("\"{}...\"", &s[..17]) }
                        else { format!("\"{}\"", s) }
                    }
                    Value::Bool(b) => b.to_string(),
                    Value::List(l) => format!("[..{}]", l.len()),
                    Value::Dict(d) => format!("{{..{}}}", d.len()),
                    Value::Function(params, _, _) => format!("fn({})", params.len()),
                    Value::Lambda(params, _) => format!("lambda({})", params.len()),
                    Value::Null => "null".to_string(),
                    _ => "...".to_string(),
                };
                VarInfo { name: name.clone(), type_name, short_value }
            })
            .collect()
    }

    pub fn history_up(&mut self) {
        if self.history.is_empty() { return; }
        let pos = match self.history_pos {
            Some(p) if p > 0 => p - 1,
            Some(p) => p,
            None => self.history.len() - 1,
        };
        self.history_pos = Some(pos);
        self.input = self.history[pos].clone();
        self.cursor_col = self.input.len();
    }

    pub fn history_down(&mut self) {
        match self.history_pos {
            Some(p) if p < self.history.len() - 1 => {
                self.history_pos = Some(p + 1);
                self.input = self.history[p + 1].clone();
                self.cursor_col = self.input.len();
            }
            _ => {
                self.history_pos = None;
                self.input.clear();
                self.cursor_col = 0;
            }
        }
    }
}
```

**Step 2: Create `src_bootstrap/repl/mod.rs`**

```rust
pub mod state;
pub mod ui;

pub use state::ReplState;
```

**Step 3: Register the module in `src_bootstrap/lib.rs`**

Add after the existing `pub mod tui;` line:
```rust
pub mod repl;
```

**Step 4: Run tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib`
Expected: 493 passed (may have compile errors — fix imports as needed)

**Step 5: Commit**

```bash
git add src_bootstrap/repl/
git add src_bootstrap/lib.rs
git commit -m "feat(repl): add REPL state module with eval, commands, history, variable tracking"
```

---

### Task 3: Create REPL TUI renderer

**Files:**
- Create: `src_bootstrap/repl/ui.rs`

**Step 1: Create `src_bootstrap/repl/ui.rs`**

```rust
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

use super::state::{OutputEntry, ReplState};

pub fn run_repl(project_name: String) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = ReplState::new(project_name);
    let mut running = true;
    let mut quit_requested = false;

    while running {
        terminal.draw(|frame| render(frame, &state))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match (key.modifiers, key.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                        state.input.clear();
                        state.cursor_col = 0;
                        state.in_multiline = false;
                        state.multiline_buf.clear();
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                        if state.input.is_empty() {
                            running = false;
                        }
                    }
                    (KeyModifiers::CONTROL, KeyCode::Char('l')) => {
                        state.output.clear();
                    }
                    (_, KeyCode::Enter) => {
                        let line = state.input.clone();
                        state.input.clear();
                        state.cursor_col = 0;
                        if line.trim() == ":quit" {
                            running = false;
                        } else {
                            state.eval_line(&line);
                        }
                    }
                    (_, KeyCode::Up) => state.history_up(),
                    (_, KeyCode::Down) => state.history_down(),
                    (_, KeyCode::Left) => {
                        if state.cursor_col > 0 { state.cursor_col -= 1; }
                    }
                    (_, KeyCode::Right) => {
                        if state.cursor_col < state.input.len() { state.cursor_col += 1; }
                    }
                    (_, KeyCode::Home) => state.cursor_col = 0,
                    (_, KeyCode::End) => state.cursor_col = state.input.len(),
                    (_, KeyCode::Backspace) => {
                        if state.cursor_col > 0 {
                            state.input.remove(state.cursor_col - 1);
                            state.cursor_col -= 1;
                        }
                    }
                    (_, KeyCode::Delete) => {
                        if state.cursor_col < state.input.len() {
                            state.input.remove(state.cursor_col);
                        }
                    }
                    (_, KeyCode::Char(c)) => {
                        state.input.insert(state.cursor_col, c);
                        state.cursor_col += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn render(frame: &mut Frame, state: &ReplState) {
    let area = frame.area();

    // Main vertical layout: banner (1) + content (rest)
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(5)])
        .split(area);

    // Banner
    let file_display = state.loaded_file.as_deref().unwrap_or("(none)");
    let banner_text = format!(
        " GUL REPL | project: {} | file: {} ",
        state.project_name, file_display
    );
    let banner = Paragraph::new(banner_text)
        .style(Style::default().fg(Color::White).bg(Color::Blue).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(banner, main_chunks[0]);

    // Content: left (output+input) + right sidebar
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(main_chunks[1]);

    // Left: output pane + input pane
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)])
        .split(content_chunks[0]);

    // Output pane
    let output_items: Vec<ListItem> = state.output.iter().map(|entry| {
        match entry {
            OutputEntry::Input(s) => ListItem::new(Line::from(vec![
                Span::styled("> ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(s),
            ])),
            OutputEntry::Continuation(s) => ListItem::new(Line::from(vec![
                Span::styled(". ", Style::default().fg(Color::DarkGray)),
                Span::raw(s),
            ])),
            OutputEntry::Result(s) => ListItem::new(Line::from(
                Span::styled(s.as_str(), Style::default().fg(Color::Cyan)),
            )),
            OutputEntry::Error(s) => ListItem::new(Line::from(
                Span::styled(s.as_str(), Style::default().fg(Color::Red)),
            )),
        }
    }).collect();
    let output_list = List::new(output_items)
        .block(Block::default().borders(Borders::ALL).title(" Output "));
    frame.render_widget(output_list, left_chunks[0]);

    // Input pane
    let prompt = if state.in_multiline { ". " } else { "gul> " };
    let input_text = format!("{}{}", prompt, state.input);
    let input_widget = Paragraph::new(input_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title(" Input "));
    frame.render_widget(input_widget, left_chunks[1]);

    // Sidebar: variables + functions
    let vars = state.get_variables();
    let (var_items, fn_items): (Vec<_>, Vec<_>) = vars.iter().partition(|v| v.type_name != "fn" && v.type_name != "lambda");

    let mut sidebar_lines: Vec<ListItem> = Vec::new();

    // Variables section
    sidebar_lines.push(ListItem::new(Line::from(
        Span::styled(" Variables", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
    )));
    if var_items.is_empty() {
        sidebar_lines.push(ListItem::new(Span::styled("  (none)", Style::default().fg(Color::DarkGray))));
    }
    for v in &var_items {
        sidebar_lines.push(ListItem::new(Line::from(vec![
            Span::styled(format!("  {}", v.name), Style::default().fg(Color::White)),
            Span::styled(format!(": {} = {}", v.type_name, v.short_value), Style::default().fg(Color::DarkGray)),
        ])));
    }

    sidebar_lines.push(ListItem::new("")); // spacer

    // Functions section
    sidebar_lines.push(ListItem::new(Line::from(
        Span::styled(" Functions", Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow)),
    )));
    if fn_items.is_empty() {
        sidebar_lines.push(ListItem::new(Span::styled("  (none)", Style::default().fg(Color::DarkGray))));
    }
    for f in &fn_items {
        sidebar_lines.push(ListItem::new(Line::from(
            Span::styled(format!("  {} {}", f.name, f.short_value), Style::default().fg(Color::Green)),
        )));
    }

    let sidebar = List::new(sidebar_lines)
        .block(Block::default().borders(Borders::ALL).title(" State "));
    frame.render_widget(sidebar, content_chunks[1]);
}
```

**Step 2: Update `src_bootstrap/repl/mod.rs`** to include ui:

```rust
pub mod state;
pub mod ui;

pub use state::ReplState;
pub use ui::run_repl;
```

**Step 3: Run tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib`
Expected: 493 passed, 0 failed, 1 ignored

**Step 4: Commit**

```bash
git add src_bootstrap/repl/ui.rs src_bootstrap/repl/mod.rs
git commit -m "feat(repl): add TUI renderer with output pane, input, sidebar, and banner"
```

---

### Task 4: Wire REPL to CLI

**Files:**
- Modify: `src_bootstrap/main.rs` (add ReplTui subcommand and handler)

**Step 1: Add `ReplTui` variant to the CLI enum**

Find the existing command variants (around line 130-148) and add after the `Tui` variant:

```rust
    /// Launch interactive TUI REPL
    ReplTui,
```

**Step 2: Add the match arm in main()**

Find the command dispatch section (around line 600+) and add:

```rust
        Commands::ReplTui => {
            let project_name = std::env::current_dir()
                .ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_else(|| "gul".to_string());
            gul_lang::repl::run_repl(project_name)
                .map_err(|e| format!("REPL error: {}", e))?;
        }
```

**Step 3: Run tests**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib`
Expected: 493 passed, 0 failed, 1 ignored

**Step 4: Build and verify CLI help**

Run: `cargo +stable-x86_64-pc-windows-msvc build 2>&1 | tail -3`
Expected: Clean build

**Step 5: Commit**

```bash
git add src_bootstrap/main.rs
git commit -m "feat(repl): wire TUI REPL to CLI as 'repl-tui' subcommand"
```

---

### Task 5: Manual test and final commit

**Step 1: Run full test suite**

Run: `cargo +stable-x86_64-pc-windows-msvc test --lib`
Expected: 493 passed, 0 failed, 1 ignored

**Step 2: Verify build**

Run: `cargo +stable-x86_64-pc-windows-msvc build 2>&1 | tail -3`
Expected: Clean build (warnings OK, 0 errors)

**Step 3: Update design doc status**

In `docs/plans/2026-03-01-tui-repl-design.md`, change:
```
**Status**: Approved
```
to:
```
**Status**: Implemented (2026-03-01)
```

**Step 4: Commit**

```bash
git add docs/plans/2026-03-01-tui-repl-design.md
git commit -m "docs: mark TUI REPL design as implemented"
```
