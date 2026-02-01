use super::Command;

use super::App;

impl App {
    // Adds a new command to the application's command list.
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    // Removes the currently selected command from the application's command list.
    pub fn remove_selected_command(&mut self) {
        if let Some(index) = self.selected_index {
            self.commands.remove(index);
            self.selected_index = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_command() {
        let mut app = App::new();
        let command = Command::new("ls -la");

        app.add_command(command);

        assert_eq!(app.commands().len(), 1);
        assert_eq!(app.commands()[0].command(), "ls -la");
    }

    #[test]
    fn test_remove_selected_command() {
        let mut app = App::new();
        let command1 = Command::new("ls -la");
        let command2 = Command::new("pwd");
        app.add_command(command1);
        app.add_command(command2);
        app.selected_index = Some(0);

        app.remove_selected_command();

        assert_eq!(app.commands().len(), 1);
        assert_eq!(app.commands()[0].command(), "pwd");
        assert_eq!(app.selected_index, None);
    }

    #[test]
    fn test_remove_selected_command_no_selection() {
        let mut app = App::new();
        let command = Command::new("ls -la");
        app.add_command(command);
        app.selected_index = None;

        app.remove_selected_command();
        assert_eq!(app.commands().len(), 1);
        assert_eq!(app.selected_index, None);
    }
}
