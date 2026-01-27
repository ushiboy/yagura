use super::{App, AppMode, Form};

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            commands: vec![],
            selected_index: None,
            should_quit: false,
            mode: AppMode::Normal,
            form: Form::new(),
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
        assert_eq!(app.selected_index, None);
        assert_eq!(app.should_quit(), false);
        assert_eq!(app.mode, AppMode::Normal);
    }
}
