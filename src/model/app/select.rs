use uuid::Uuid;

use super::Command;

use super::App;

impl App {
    pub fn select_command_by_id(&mut self, command_id: Uuid) {
        if let Some(index) = self.commands.iter().position(|cmd| cmd.id() == command_id) {
            self.ui_state.command_list.selected_command_index = Some(index);
        }
    }

    pub fn get_selected_command(&self) -> Option<&Command> {
        if let Some(index) = self.ui_state.command_list.selected_command_index {
            self.commands.get(index)
        } else {
            None
        }
    }

    pub fn select_next_command(&mut self) {
        if self.commands.is_empty() {
            return;
        }

        self.ui_state.command_list.selected_command_index =
            Some(match self.ui_state.command_list.selected_command_index {
                Some(index) => (index + 1) % self.commands.len(),
                _ => 0,
            });
    }

    pub fn select_previous_command(&mut self) {
        if self.commands.is_empty() {
            return;
        }

        let len = self.commands.len();
        self.ui_state.command_list.selected_command_index =
            Some(match self.ui_state.command_list.selected_command_index {
                Some(index) => (index + len - 1) % len,
                _ => 0,
            });
    }
}

#[cfg(test)]
mod tests {
    use super::Command;
    use super::*;

    #[test]
    fn test_select_command_by_id() {
        let mut app = App::new();
        let command = Command::new("ls -la");
        app.add_command(command);

        assert!(app.ui_state.command_list.selected_command_index.is_none());
        app.select_command_by_id(app.commands()[0].id());
        assert_eq!(app.ui_state.command_list.selected_command_index, Some(0));
    }

    #[test]
    fn test_get_selected_command() {
        let mut app = App::new();
        let command = Command::new("ls -la");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);

        let selected_command = app.get_selected_command().unwrap();
        assert_eq!(selected_command.id(), command_id);
    }

    #[test]
    fn test_select_next_command_with_empty_commands() {
        let mut app = App::new();
        assert!(app.commands().is_empty());
        assert!(app.ui_state.command_list.selected_command_index.is_none());

        app.select_next_command();

        assert!(app.ui_state.command_list.selected_command_index.is_none());
    }

    #[test]
    fn test_select_next_command_with_none_selected() {
        let mut app = App::new();
        app.add_command(Command::new("ls -la"));
        app.add_command(Command::new("pwd"));
        app.add_command(Command::new("echo test"));
        assert!(app.ui_state.command_list.selected_command_index.is_none());

        app.select_next_command();

        assert_eq!(app.ui_state.command_list.selected_command_index, Some(0));
    }

    #[test]
    fn test_select_next_command_increments_index() {
        let mut app = App::new();
        app.add_command(Command::new("ls -la"));
        app.add_command(Command::new("pwd"));
        app.add_command(Command::new("echo test"));
        app.ui_state.command_list.selected_command_index = Some(0);

        app.select_next_command();

        assert_eq!(app.ui_state.command_list.selected_command_index, Some(1));

        app.select_next_command();

        assert_eq!(app.ui_state.command_list.selected_command_index, Some(2));
    }

    #[test]
    fn test_select_next_command_wraps_around() {
        let mut app = App::new();
        app.add_command(Command::new("ls -la"));
        app.add_command(Command::new("pwd"));
        app.add_command(Command::new("echo test"));
        app.ui_state.command_list.selected_command_index = Some(2);

        app.select_next_command();

        assert_eq!(app.ui_state.command_list.selected_command_index, Some(0));
    }

    #[test]
    fn test_select_previous_command_with_empty_commands() {
        let mut app = App::new();
        assert!(app.commands().is_empty());
        assert!(app.ui_state.command_list.selected_command_index.is_none());

        app.select_previous_command();

        assert!(app.ui_state.command_list.selected_command_index.is_none());
    }

    #[test]
    fn test_select_previous_command_with_none_selected() {
        let mut app = App::new();
        app.add_command(Command::new("ls -la"));
        app.add_command(Command::new("pwd"));
        app.add_command(Command::new("echo test"));
        assert!(app.ui_state.command_list.selected_command_index.is_none());

        app.select_previous_command();

        assert_eq!(app.ui_state.command_list.selected_command_index, Some(0));
    }

    #[test]
    fn test_select_previous_command_decrements_index() {
        let mut app = App::new();
        app.add_command(Command::new("ls -la"));
        app.add_command(Command::new("pwd"));
        app.add_command(Command::new("echo test"));
        app.ui_state.command_list.selected_command_index = Some(2);

        app.select_previous_command();

        assert_eq!(app.ui_state.command_list.selected_command_index, Some(1));

        app.select_previous_command();

        assert_eq!(app.ui_state.command_list.selected_command_index, Some(0));
    }

    #[test]
    fn test_select_previous_command_wraps_around() {
        let mut app = App::new();
        app.add_command(Command::new("ls -la"));
        app.add_command(Command::new("pwd"));
        app.add_command(Command::new("echo test"));
        app.ui_state.command_list.selected_command_index = Some(0);

        app.select_previous_command();

        assert_eq!(app.ui_state.command_list.selected_command_index, Some(2));
    }
}
