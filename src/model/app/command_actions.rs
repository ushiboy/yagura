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

    pub fn remove_selected_command(&mut self) {
        if let Some(command) = self.get_selected_command() {
            let command_id = command.id();
            self.remove_command_by_id(command_id);
            self.ui_state.clear_selection();
            self.ui_state.remove_command_log_offset(command_id);
            self.change_normal_mode();
        }
    }

    pub fn yank_visible_command_output(&mut self, viewport_height: usize) {
        if let Some(output) = self.visible_output_as_plain_text(viewport_height)
            && let Some(clipboard) = &mut self.clipboard
        {
            clipboard
                .set_text(output)
                .expect("Failed to set clipboard text");
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
}
