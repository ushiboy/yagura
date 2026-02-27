use super::{App, Command};

impl App {
    // Confirm adding a new command from the form inputs
    pub fn confirm_add_command(&mut self) {
        let form = self.form();
        let command_text = form.command_input().to_string();
        let working_dir = if form.working_dir_input().is_empty() {
            None
        } else {
            Some(form.working_dir_input().to_string())
        };

        let command = Command::new(command_text).with_working_dir(working_dir);
        let command_id = command.id();
        self.add_command(command);
        self.select_command_by_id(command_id);
        self.change_normal_mode();
    }

    // Remove the currently selected command and update selection and mode accordingly
    pub fn remove_selected_command(&mut self) {
        if let Some(command) = self.get_selected_command() {
            let current_selected_index = self.ui_state.selected_command_index().unwrap();
            let command_id = command.id();
            self.remove_command_by_id(command_id);

            let new_index = if !self.commands().is_empty() {
                Some(current_selected_index.min(self.commands().len() - 1))
            } else {
                None
            };

            match new_index {
                Some(idx) => self.ui_state.set_selected_index(idx),
                None => self.ui_state.clear_selection(),
            }

            self.ui_state.remove_command_log_offset(command_id);
            self.change_normal_mode();
        }
    }

    pub fn yank_visible_command_output(&mut self, viewport_height: usize) {
        if let Some(output) = self.visible_output_as_plain_text(viewport_height)
            && let Some(clipboard) = &mut self.clipboard
            && clipboard.set_text(output).is_err() {
                // ignore
            };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::AppMode;

    #[test]
    fn test_confirm_add_command() {
        let mut app = App::new();
        app.change_adding_mode();
        let form = app.form_mut();
        form.set_command_input("ls");
        form.set_working_dir_input("./tmp");

        app.confirm_add_command();

        assert_eq!(app.commands().len(), 1);
        assert_eq!(app.commands()[0].command(), "ls");
        assert_eq!(app.commands()[0].working_dir(), Some("./tmp"));
        assert_eq!(app.selected_command_index(), Some(0));
        assert_eq!(app.mode(), &AppMode::Normal);
    }

    #[test]
    fn test_confirm_add_command_without_working_dir() {
        let mut app = App::new();
        app.change_adding_mode();
        let form = app.form_mut();
        form.push_char('c');

        app.confirm_add_command();

        assert_eq!(app.commands().len(), 1);
        assert_eq!(app.commands()[0].command(), "c");
        assert_eq!(app.commands()[0].working_dir(), None);
    }

    #[test]
    fn test_confirm_add_command_with_existing_commands() {
        let mut app = App::new();
        app.add_command(Command::new("first"));
        app.add_command(Command::new("second"));
        app.change_adding_mode();
        let form = app.form_mut();
        form.set_command_input("third");

        app.confirm_add_command();

        assert_eq!(app.commands().len(), 3);
        assert_eq!(app.commands()[2].command(), "third");
        assert_eq!(app.selected_command_index(), Some(2));
    }

    #[test]
    fn test_remove_selected_command_from_middle() {
        let mut app = App::new();
        app.add_command(Command::new("first"));
        app.add_command(Command::new("second"));
        app.add_command(Command::new("third"));
        app.ui_state.set_selected_index(1);

        app.remove_selected_command();

        assert_eq!(app.commands().len(), 2);
        assert_eq!(app.commands()[0].command(), "first");
        assert_eq!(app.commands()[1].command(), "third");
        assert_eq!(app.selected_command_index(), Some(1));
        assert_eq!(app.mode(), &AppMode::Normal);
    }

    #[test]
    fn test_remove_selected_command_when_last_item() {
        let mut app = App::new();
        app.add_command(Command::new("first"));
        app.add_command(Command::new("second"));
        app.add_command(Command::new("third"));
        app.ui_state.set_selected_index(2);

        app.remove_selected_command();

        assert_eq!(app.commands().len(), 2);
        assert_eq!(app.commands()[0].command(), "first");
        assert_eq!(app.commands()[1].command(), "second");
        assert_eq!(app.selected_command_index(), Some(1));
        assert_eq!(app.mode(), &AppMode::Normal);
    }

    #[test]
    fn test_remove_selected_command_when_only_one() {
        let mut app = App::new();
        app.add_command(Command::new("only"));
        app.ui_state.set_selected_index(0);

        app.remove_selected_command();

        assert_eq!(app.commands().len(), 0);
        assert_eq!(app.selected_command_index(), None);
        assert_eq!(app.mode(), &AppMode::Normal);
    }

    #[test]
    fn test_remove_selected_command_when_first_item() {
        let mut app = App::new();
        app.add_command(Command::new("first"));
        app.add_command(Command::new("second"));
        app.add_command(Command::new("third"));
        app.ui_state.set_selected_index(0);

        app.remove_selected_command();

        assert_eq!(app.commands().len(), 2);
        assert_eq!(app.commands()[0].command(), "second");
        assert_eq!(app.commands()[1].command(), "third");
        assert_eq!(app.selected_command_index(), Some(0));
        assert_eq!(app.mode(), &AppMode::Normal);
    }

    #[test]
    fn test_remove_selected_command_when_no_selection() {
        let mut app = App::new();
        app.add_command(Command::new("first"));
        app.add_command(Command::new("second"));

        app.remove_selected_command();

        // Nothing should happen if no command is selected
        assert_eq!(app.commands().len(), 2);
    }
}
