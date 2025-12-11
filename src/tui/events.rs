// GUL TUI Events - Event handling for the TUI application

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};

/// Application events
#[derive(Debug, Clone)]
pub enum AppEvent {
    /// Key press event
    Key(KeyEvent),
    /// Mouse event
    Mouse(MouseEvent),
    /// Terminal resize
    Resize(u16, u16),
    /// Tick event for animations
    Tick,
    /// File system change
    FileChange(String),
    /// Build complete
    BuildComplete(BuildResult),
    /// Error occurred
    Error(String),
}

/// Build result
#[derive(Debug, Clone)]
pub struct BuildResult {
    pub success: bool,
    pub duration_ms: u64,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

/// Key bindings for the IDE
#[derive(Debug, Clone)]
pub struct KeyBindings {
    pub quit: Vec<KeyBinding>,
    pub save: Vec<KeyBinding>,
    pub open: Vec<KeyBinding>,
    pub build: Vec<KeyBinding>,
    pub run: Vec<KeyBinding>,
    pub format: Vec<KeyBinding>,
    pub command_palette: Vec<KeyBinding>,
    pub file_browser: Vec<KeyBinding>,
    pub terminal: Vec<KeyBinding>,
    pub next_tab: Vec<KeyBinding>,
    pub prev_tab: Vec<KeyBinding>,
    pub close_tab: Vec<KeyBinding>,
}

/// Single key binding
#[derive(Debug, Clone)]
pub struct KeyBinding {
    pub key: KeyCode,
    pub modifiers: KeyModifiers,
}

impl KeyBinding {
    pub fn new(key: KeyCode, modifiers: KeyModifiers) -> Self {
        KeyBinding { key, modifiers }
    }

    pub fn matches(&self, event: &KeyEvent) -> bool {
        event.code == self.key && event.modifiers == self.modifiers
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            quit: vec![KeyBinding::new(KeyCode::Char('q'), KeyModifiers::CONTROL)],
            save: vec![KeyBinding::new(KeyCode::Char('s'), KeyModifiers::CONTROL)],
            open: vec![KeyBinding::new(KeyCode::Char('o'), KeyModifiers::CONTROL)],
            build: vec![
                KeyBinding::new(KeyCode::Char('b'), KeyModifiers::CONTROL),
                KeyBinding::new(KeyCode::F(5), KeyModifiers::NONE),
            ],
            run: vec![
                KeyBinding::new(KeyCode::Char('r'), KeyModifiers::CONTROL),
                KeyBinding::new(KeyCode::F(6), KeyModifiers::NONE),
            ],
            format: vec![KeyBinding::new(
                KeyCode::Char('f'),
                KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            )],
            command_palette: vec![
                KeyBinding::new(KeyCode::Char('p'), KeyModifiers::CONTROL),
                KeyBinding::new(KeyCode::F(1), KeyModifiers::NONE),
            ],
            file_browser: vec![KeyBinding::new(KeyCode::Char('e'), KeyModifiers::CONTROL)],
            terminal: vec![KeyBinding::new(KeyCode::Char('`'), KeyModifiers::CONTROL)],
            next_tab: vec![KeyBinding::new(KeyCode::Tab, KeyModifiers::CONTROL)],
            prev_tab: vec![KeyBinding::new(
                KeyCode::Tab,
                KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            )],
            close_tab: vec![KeyBinding::new(KeyCode::Char('w'), KeyModifiers::CONTROL)],
        }
    }
}

impl KeyBindings {
    /// Check if any binding matches the event
    pub fn check(&self, event: &KeyEvent) -> Option<Action> {
        if self.quit.iter().any(|b| b.matches(event)) {
            return Some(Action::Quit);
        }
        if self.save.iter().any(|b| b.matches(event)) {
            return Some(Action::Save);
        }
        if self.open.iter().any(|b| b.matches(event)) {
            return Some(Action::Open);
        }
        if self.build.iter().any(|b| b.matches(event)) {
            return Some(Action::Build);
        }
        if self.run.iter().any(|b| b.matches(event)) {
            return Some(Action::Run);
        }
        if self.format.iter().any(|b| b.matches(event)) {
            return Some(Action::Format);
        }
        if self.command_palette.iter().any(|b| b.matches(event)) {
            return Some(Action::CommandPalette);
        }
        if self.file_browser.iter().any(|b| b.matches(event)) {
            return Some(Action::ToggleFileBrowser);
        }
        if self.terminal.iter().any(|b| b.matches(event)) {
            return Some(Action::ToggleTerminal);
        }
        if self.next_tab.iter().any(|b| b.matches(event)) {
            return Some(Action::NextTab);
        }
        if self.prev_tab.iter().any(|b| b.matches(event)) {
            return Some(Action::PrevTab);
        }
        if self.close_tab.iter().any(|b| b.matches(event)) {
            return Some(Action::CloseTab);
        }
        None
    }
}

/// IDE actions
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Quit,
    Save,
    Open,
    Build,
    Run,
    Format,
    CommandPalette,
    ToggleFileBrowser,
    ToggleTerminal,
    NextTab,
    PrevTab,
    CloseTab,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_binding_matches() {
        let binding = KeyBinding::new(KeyCode::Char('s'), KeyModifiers::CONTROL);

        let event = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL);
        assert!(binding.matches(&event));

        let wrong_event = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE);
        assert!(!binding.matches(&wrong_event));
    }

    #[test]
    fn test_key_bindings_check() {
        let bindings = KeyBindings::default();

        let save_event = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL);
        assert_eq!(bindings.check(&save_event), Some(Action::Save));

        let quit_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL);
        assert_eq!(bindings.check(&quit_event), Some(Action::Quit));
    }
}
