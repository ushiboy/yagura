use uuid::Uuid;

use super::Command;

use super::App;

impl App {
    // Adds a new command to the application's command list.
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
    }

    // Removes a specific command from the application's command list.
    pub fn remove_command(&mut self, command: Command) {
        self.commands.retain(|c| c != &command);
    }

    // Retrieves a mutable reference to a command by its unique identifier.
    pub fn get_command_mut_by_id(&mut self, command_id: Uuid) -> Option<&mut Command> {
        self.commands.iter_mut().find(|cmd| cmd.id() == command_id)
    }

    // Removes the currently selected command from the application's command list.
    pub fn delete_selected_command(&mut self) {
        if let Some(index) = self.ui_state.selected_command_index() {
            self.commands.remove(index);
            self.ui_state.clear_selection();
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
    fn test_remove_command() {
        let mut app = App::new();
        let command1 = Command::new("ls -la");
        let command2 = Command::new("pwd");
        let command3 = Command::new("whoami");
        app.add_command(command1.clone());
        app.add_command(command2.clone());
        app.add_command(command3.clone());

        app.remove_command(command1);

        assert_eq!(app.commands().len(), 2);
        assert_eq!(app.commands()[0].command(), "pwd");
        assert_eq!(app.commands()[1].command(), "whoami");
    }

    #[test]
    fn test_get_command_mut_by_id() {
        let mut app = App::new();
        let command = Command::new("ls -la");
        let command_id = command.id();
        app.add_command(command);

        let cmd_mut = app.get_command_mut_by_id(command_id);
        assert!(cmd_mut.is_some());
        assert_eq!(cmd_mut.unwrap().command(), "ls -la");

        let non_existent_id = Uuid::now_v7();
        let cmd_none = app.get_command_mut_by_id(non_existent_id);
        assert!(cmd_none.is_none());
    }

    #[test]
    fn test_delete_selected_command() {
        let mut app = App::new();
        let command1 = Command::new("ls -la");
        let command2 = Command::new("pwd");
        app.add_command(command1);
        app.add_command(command2);
        app.ui_state.set_selected_index(0);

        app.delete_selected_command();

        assert_eq!(app.commands().len(), 1);
        assert_eq!(app.commands()[0].command(), "pwd");
        assert_eq!(app.ui_state.selected_command_index(), None);
    }

    #[test]
    fn test_delete_selected_command_no_selection() {
        let mut app = App::new();
        let command = Command::new("ls -la");
        app.add_command(command);

        app.delete_selected_command();
        assert_eq!(app.commands().len(), 1);
        assert_eq!(app.ui_state.selected_command_index(), None);
    }
}
