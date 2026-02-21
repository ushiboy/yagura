use super::App;

impl App {
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_quit() {
        let mut app = App::new();
        assert!(!app.should_quit());

        app.quit();
        assert!(app.should_quit());
    }
}
