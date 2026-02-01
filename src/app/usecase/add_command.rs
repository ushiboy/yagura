use crate::app::{App, Command};

pub fn add_command(app: &mut App, command_text: String, working_dir: Option<String>) {
    let command = Command::new(command_text).with_working_dir(working_dir);
    let command_id = command.id();
    app.add_command(command);
    app.select_command_by_id(command_id);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_command_to_empty_app() {
        let mut app = App::new();
        assert_eq!(app.commands().len(), 0);
        assert!(app.selected_command_index().is_none());

        add_command(&mut app, "ls -la".to_string(), None);

        assert_eq!(app.commands().len(), 1);
        assert_eq!(app.commands()[0].command(), "ls -la");
        assert_eq!(app.selected_command_index(), Some(0));
    }

    #[test]
    fn test_add_command_to_app_with_existing_commands() {
        let mut app = App::new();
        add_command(&mut app, "first command".to_string(), None);
        add_command(&mut app, "second command".to_string(), None);

        assert_eq!(app.commands().len(), 2);
        assert_eq!(app.commands()[0].command(), "first command");
        assert_eq!(app.commands()[1].command(), "second command");
        assert_eq!(app.selected_command_index(), Some(1));
    }

    #[test]
    fn test_add_command_when_selection_is_not_at_end() {
        let mut app = App::new();
        add_command(&mut app, "first".to_string(), None);
        add_command(&mut app, "second".to_string(), None);
        add_command(&mut app, "third".to_string(), None);

        app.select_command_by_id(app.commands()[0].id());
        assert_eq!(app.selected_command_index(), Some(0));

        add_command(&mut app, "fourth".to_string(), None);

        assert_eq!(app.commands().len(), 4);
        assert_eq!(app.selected_command_index(), Some(3));
    }
}
