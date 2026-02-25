use super::{App, AppMode, UIState};
use arboard::Clipboard;

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            clipboard: Clipboard::new().ok(),
            commands: vec![],
            should_quit: false,
            mode: AppMode::Normal,
            ui_state: UIState::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_new() {
        let app = App::new();
        assert!(app.commands.is_empty());
        assert!(!app.should_quit());
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.ui_state, UIState::new());
    }
}
