use uuid::Uuid;

use super::Command;

use super::App;

impl App {
    pub fn select_command_by_id(&mut self, command_id: Uuid) {
        if let Some(index) = self.commands.iter().position(|cmd| cmd.id() == command_id) {
            self.ui_state.set_selected_index(index);
        }
    }

    pub fn get_selected_command(&self) -> Option<&Command> {
        if let Some(index) = self.ui_state.selected_command_index() {
            self.commands.get(index)
        } else {
            None
        }
    }

    pub fn select_next_command(&mut self, viewport_height: usize) {
        if self.commands.is_empty() {
            return;
        }

        let next_index = match self.ui_state.selected_command_index() {
            Some(index) => (index + 1) % self.commands.len(),
            None => 0,
        };
        self.ui_state.set_selected_index(next_index);
        self.adjust_scroll_offset(next_index, viewport_height);
    }

    pub fn select_previous_command(&mut self, viewport_height: usize) {
        if self.commands.is_empty() {
            return;
        }

        let len = self.commands.len();
        let previous_index = match self.ui_state.selected_command_index() {
            Some(index) => (index + len - 1) % len,
            None => 0,
        };
        self.ui_state.set_selected_index(previous_index);
        self.adjust_scroll_offset(previous_index, viewport_height);
    }

    fn adjust_scroll_offset(&mut self, selected_index: usize, viewport_height: usize) {
        let offset = self.ui_state.command_list_scroll_offset();
        let new_offset = if selected_index < offset {
            selected_index
        } else if viewport_height > 0 && selected_index >= offset + viewport_height {
            selected_index + 1 - viewport_height
        } else {
            offset
        };
        self.ui_state.set_command_list_scroll_offset(new_offset);
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

        assert!(app.ui_state.selected_command_index().is_none());
        app.select_command_by_id(app.commands()[0].id());
        assert_eq!(app.ui_state.selected_command_index(), Some(0));
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
        assert!(app.ui_state.selected_command_index().is_none());

        app.select_next_command(10);

        assert!(app.ui_state.selected_command_index().is_none());
    }

    #[test]
    fn test_select_next_command_with_none_selected() {
        let mut app = App::new();
        app.add_command(Command::new("ls -la"));
        app.add_command(Command::new("pwd"));
        app.add_command(Command::new("echo test"));
        assert!(app.ui_state.selected_command_index().is_none());

        app.select_next_command(10);

        assert_eq!(app.ui_state.selected_command_index(), Some(0));
    }

    #[test]
    fn test_select_next_command_increments_index() {
        let mut app = App::new();
        app.add_command(Command::new("ls -la"));
        app.add_command(Command::new("pwd"));
        app.add_command(Command::new("echo test"));
        app.ui_state.set_selected_index(0);

        app.select_next_command(10);

        assert_eq!(app.ui_state.selected_command_index(), Some(1));

        app.select_next_command(10);

        assert_eq!(app.ui_state.selected_command_index(), Some(2));
    }

    #[test]
    fn test_select_next_command_wraps_around() {
        let mut app = App::new();
        app.add_command(Command::new("ls -la"));
        app.add_command(Command::new("pwd"));
        app.add_command(Command::new("echo test"));
        app.ui_state.set_selected_index(2);

        app.select_next_command(10);

        assert_eq!(app.ui_state.selected_command_index(), Some(0));
    }

    #[test]
    fn test_select_previous_command_with_empty_commands() {
        let mut app = App::new();
        assert!(app.commands().is_empty());
        assert!(app.ui_state.selected_command_index().is_none());

        app.select_previous_command(10);

        assert!(app.ui_state.selected_command_index().is_none());
    }

    #[test]
    fn test_select_previous_command_with_none_selected() {
        let mut app = App::new();
        app.add_command(Command::new("ls -la"));
        app.add_command(Command::new("pwd"));
        app.add_command(Command::new("echo test"));
        assert!(app.ui_state.selected_command_index().is_none());

        app.select_previous_command(10);

        assert_eq!(app.ui_state.selected_command_index(), Some(0));
    }

    #[test]
    fn test_select_previous_command_decrements_index() {
        let mut app = App::new();
        app.add_command(Command::new("ls -la"));
        app.add_command(Command::new("pwd"));
        app.add_command(Command::new("echo test"));
        app.ui_state.set_selected_index(2);

        app.select_previous_command(10);

        assert_eq!(app.ui_state.selected_command_index(), Some(1));

        app.select_previous_command(10);

        assert_eq!(app.ui_state.selected_command_index(), Some(0));
    }

    #[test]
    fn test_select_previous_command_wraps_around() {
        let mut app = App::new();
        app.add_command(Command::new("ls -la"));
        app.add_command(Command::new("pwd"));
        app.add_command(Command::new("echo test"));
        app.ui_state.set_selected_index(0);

        app.select_previous_command(10);

        assert_eq!(app.ui_state.selected_command_index(), Some(2));
    }

    #[test]
    fn test_scroll_offset_advances_when_selection_exceeds_viewport() {
        let mut app = App::new();
        for i in 0..5 {
            app.add_command(Command::new(format!("cmd {}", i)));
        }
        app.ui_state.set_selected_index(1);

        // viewport_height=2: selecting index 2 should push offset to 1
        app.select_next_command(2);
        assert_eq!(app.ui_state.selected_command_index(), Some(2));
        assert_eq!(app.ui_state.command_list_scroll_offset(), 1);
    }

    #[test]
    fn test_scroll_offset_retreats_when_selection_goes_above_viewport() {
        let mut app = App::new();
        for i in 0..5 {
            app.add_command(Command::new(format!("cmd {}", i)));
        }
        app.ui_state.set_selected_index(2);
        app.ui_state.set_command_list_scroll_offset(2);

        // selecting index 1 (above offset 2) should move offset back to 1
        app.select_previous_command(2);
        assert_eq!(app.ui_state.selected_command_index(), Some(1));
        assert_eq!(app.ui_state.command_list_scroll_offset(), 1);
    }
}
