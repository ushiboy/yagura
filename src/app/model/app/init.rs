use super::App;
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
    }
}
