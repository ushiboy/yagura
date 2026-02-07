use super::{App, AppMode, Form, UIState};

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            commands: vec![],
            should_quit: false,
            mode: AppMode::Normal,
            form: Form::new(),
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
        assert_eq!(app.should_quit(), false);
        assert_eq!(app.mode, AppMode::Normal);
        assert_eq!(app.ui_state, UIState::new());
    }
}
