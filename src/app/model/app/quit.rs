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
        assert_eq!(app.should_quit(), false);

        app.quit();
        assert_eq!(app.should_quit(), true);
    }
}
