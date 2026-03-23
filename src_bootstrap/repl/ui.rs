use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
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
                        if state.eval_line(&line) {
                            running = false; // :quit was entered
                        }
                    }
                    (_, KeyCode::Up) => state.history_up(),
                    (_, KeyCode::Down) => state.history_down(),
                    (_, KeyCode::Left) => {
                        if state.cursor_col > 0 {
                            state.cursor_col -= 1;
                        }
                    }
                    (_, KeyCode::Right) => {
                        if state.cursor_col < state.input.len() {
                            state.cursor_col += 1;
                        }
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

    // Main vertical layout: banner (3) + content (rest)
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
        .style(
            Style::default()
                .fg(Color::White)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
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
    let output_items: Vec<ListItem> = state
        .output
        .iter()
        .map(|entry| match entry {
            OutputEntry::Input(s) => ListItem::new(Line::from(vec![
                Span::styled(
                    "> ",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(s),
            ])),
            OutputEntry::Continuation(s) => ListItem::new(Line::from(vec![
                Span::styled(". ", Style::default().fg(Color::DarkGray)),
                Span::raw(s),
            ])),
            OutputEntry::Result(s) => ListItem::new(Line::from(Span::styled(
                s.as_str(),
                Style::default().fg(Color::Cyan),
            ))),
            OutputEntry::Error(s) => ListItem::new(Line::from(Span::styled(
                s.as_str(),
                Style::default().fg(Color::Red),
            ))),
        })
        .collect();
    let output_list =
        List::new(output_items).block(Block::default().borders(Borders::ALL).title(" Output "));
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
    let (var_items, fn_items): (Vec<_>, Vec<_>) = vars
        .iter()
        .partition(|v| v.type_name != "fn" && v.type_name != "lambda");

    let mut sidebar_lines: Vec<ListItem> = Vec::new();

    // Variables section
    sidebar_lines.push(ListItem::new(Line::from(Span::styled(
        " Variables",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Yellow),
    ))));
    if var_items.is_empty() {
        sidebar_lines.push(ListItem::new(Span::styled(
            "  (none)",
            Style::default().fg(Color::DarkGray),
        )));
    }
    for v in &var_items {
        sidebar_lines.push(ListItem::new(Line::from(vec![
            Span::styled(format!("  {}", v.name), Style::default().fg(Color::White)),
            Span::styled(
                format!(": {} = {}", v.type_name, v.short_value),
                Style::default().fg(Color::DarkGray),
            ),
        ])));
    }

    sidebar_lines.push(ListItem::new("")); // spacer

    // Functions section
    sidebar_lines.push(ListItem::new(Line::from(Span::styled(
        " Functions",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .fg(Color::Yellow),
    ))));
    if fn_items.is_empty() {
        sidebar_lines.push(ListItem::new(Span::styled(
            "  (none)",
            Style::default().fg(Color::DarkGray),
        )));
    }
    for f in &fn_items {
        sidebar_lines.push(ListItem::new(Line::from(Span::styled(
            format!("  {} {}", f.name, f.short_value),
            Style::default().fg(Color::Green),
        ))));
    }

    let sidebar =
        List::new(sidebar_lines).block(Block::default().borders(Borders::ALL).title(" State "));
    frame.render_widget(sidebar, content_chunks[1]);
}
