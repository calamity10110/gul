use std::{io, time::Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use tui_input::{backend::crossterm::EventHandler, Input};
use std::sync::{Arc, Mutex};
use std::thread;
use anyhow::Result;

struct App {
    input: Input,
    messages: Arc<Mutex<Vec<String>>>,
    port_name: String,
}

impl App {
    fn new(port: String) -> Self {
        Self {
            input: Input::default(),
            messages: Arc::new(Mutex::new(Vec::new())),
            port_name: port,
        }
    }
}

pub fn run(port: String) -> Result<()> {
    // 1. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Setup App & Serial
    let app = App::new(port);
    let messages = app.messages.clone();

    // Mock Serial Reader Thread
    thread::spawn(move || {
        let mut i = 0;
        loop {
            thread::sleep(Duration::from_millis(2000));
            messages.lock().unwrap().push(format!("MCU Output: Ping {}", i));
            i += 1;
        }
    });

    // 3. Run Loop
    let res = run_app(&mut terminal, app);

    // 4. Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => return Ok(()),
                    KeyCode::Enter => {
                        let cmd: String = app.input.value().into();
                        app.messages.lock().unwrap().push(format!("> {}", cmd));
                        app.input.reset();
                    }
                    _ => {
                        app.input.handle_event(&Event::Key(key));
                    }
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    let messages: Vec<ListItem> = app
        .messages
        .lock()
        .unwrap()
        .iter()
        .map(|m| ListItem::new(Line::from(vec![Span::raw(m.clone())])))
        .collect();

    let messages_list = List::new(messages)
        .block(Block::default().borders(Borders::ALL).title(format!("MCU Output ({})", app.port_name)))
        .start_corner(Corner::BottomLeft);

    f.render_widget(messages_list, chunks[0]);

    let width = chunks[1].width.max(3) - 3;
    let scroll = app.input.visual_scroll(width as usize);
    let input = Paragraph::new(app.input.value())
        .scroll((0, scroll as u16))
        .block(Block::default().borders(Borders::ALL).title("Input (ESC to quit)"));
    
    f.render_widget(input, chunks[1]);
    
    f.set_cursor(
        chunks[1].x + ((app.input.visual_cursor().max(scroll) - scroll) as u16) + 1,
        chunks[1].y + 1,
    )
}
