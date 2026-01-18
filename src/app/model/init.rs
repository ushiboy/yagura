use super::App;

impl App {
    pub fn new() -> Self {
        Self { should_quit: false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_new() {
        let app = App::new();
        assert_eq!(app.should_quit(), false);
    }
}
