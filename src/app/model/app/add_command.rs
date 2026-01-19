use super::Command;

use super::App;

impl App {
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_add_command() {
        let mut app = App::new();
        let command = Command::new("ls -la".to_string());

        app.add_command(command);

        assert_eq!(app.commands().len(), 1);
        assert_eq!(app.commands()[0].command(), "ls -la");
    }
}
