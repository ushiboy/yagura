use uuid::Uuid;

use crate::process::Command;

use super::App;

impl App {
    pub fn select_command_by_id(&mut self, command_id: Uuid) {
        if let Some(index) = self.commands.iter().position(|cmd| cmd.id() == command_id) {
            self.selected_index = Some(index);
        }
    }

    pub fn get_selected_command(&self) -> Option<&Command> {
        if let Some(index) = self.selected_index {
            self.commands.get(index)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::Command;

    #[test]
    fn test_select_command_by_id() {
        let mut app = App::new();
        let command = Command::new("ls -la".to_string());
        app.add_command(command);

        assert!(app.selected_index.is_none());
        app.select_command_by_id(app.commands()[0].id());
        assert_eq!(app.selected_index, Some(0));
    }

    #[test]
    fn test_get_selected_command() {
        let mut app = App::new();
        let command = Command::new("ls -la".to_string());
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);

        let selected_command = app.get_selected_command().unwrap();
        assert_eq!(selected_command.id(), command_id);
    }
}
