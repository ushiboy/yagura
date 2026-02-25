use super::App;

impl App {
    // Returns the current log offset for the selected command, or None if no offset is set.
    pub fn get_command_log_offset(&self) -> Option<usize> {
        let index = self.ui_state.selected_command_index()?;
        let command = self.commands.get(index)?;
        self.ui_state.get_command_log_offset(command.id())
    }

    pub fn visible_output_as_plain_text(&self, viewport_height: usize) -> Option<String> {
        let cmd = self.get_selected_command()?;
        let total_lines = cmd.output_buffer().line_length();
        let scroll_offset = self
            .get_command_log_offset()
            .unwrap_or_else(|| total_lines.saturating_sub(viewport_height));
        let lines = cmd
            .output_buffer()
            .slice_lines(scroll_offset, viewport_height);

        Some(
            lines
                .iter()
                .map(|line| line.content())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    // Scrolls the command log down by one line, ensuring it doesn't exceed the maximum offset.
    pub fn line_down_command_log(&mut self, viewport_height: usize) {
        self.scroll_down(viewport_height, 1);
    }

    // Scrolls the command log up by one line, ensuring it doesn't go below zero.
    pub fn line_up_command_log(&mut self, viewport_height: usize) {
        self.scroll_up(viewport_height, 1);
    }

    // Scrolls the command log down by one page (viewport height), ensuring it doesn't exceed the maximum offset.
    pub fn page_down_command_log(&mut self, viewport_height: usize) {
        self.scroll_down(viewport_height, viewport_height);
    }

    // Scrolls the command log up by one page (viewport height), ensuring it doesn't go below zero.
    pub fn page_up_command_log(&mut self, viewport_height: usize) {
        self.scroll_up(viewport_height, viewport_height);
    }

    fn scroll_down(&mut self, viewport_height: usize, delta: usize) {
        if let Some(index) = self.ui_state.selected_command_index()
            && let Some(command) = self.commands.get(index)
            && let Some(current_offset) = self.ui_state.get_command_log_offset(command.id())
        {
            let id = command.id();
            let total_lines = command.output_buffer().line_length();
            let max_offset = total_lines.saturating_sub(viewport_height);
            let new_offset = (current_offset + delta).min(max_offset);

            if current_offset == new_offset {
                self.ui_state.remove_command_log_offset(id);
            } else {
                self.ui_state.set_command_log_offset(id, new_offset);
            }
        }
    }

    fn scroll_up(&mut self, viewport_height: usize, delta: usize) {
        if let Some(index) = self.ui_state.selected_command_index()
            && let Some(command) = self.commands.get(index)
        {
            let id = command.id();
            let total_lines = command.output_buffer().line_length();
            let max_offset = total_lines.saturating_sub(viewport_height);
            let current = self
                .ui_state
                .get_command_log_offset(id)
                .unwrap_or(max_offset);
            let new_offset = current.saturating_sub(delta);
            self.ui_state.set_command_log_offset(id, new_offset);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Command, OutputLine};

    fn add_output_lines(app: &mut App, command_id: uuid::Uuid, count: usize) {
        for i in 0..count {
            app.add_output_line(command_id, OutputLine::new(format!("Line {}", i)));
        }
    }

    #[test]
    fn test_get_command_log_offset_with_no_offset_set() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);

        assert_eq!(app.get_command_log_offset(), None);
    }

    #[test]
    fn test_get_command_log_offset_with_offset_set() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 10);

        app.ui_state.set_command_log_offset(command_id, 5);

        assert_eq!(app.get_command_log_offset(), Some(5));
    }

    #[test]
    fn test_line_down_command_log_with_no_offset() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 10);

        app.line_down_command_log(5);

        // No offset was set, so nothing should happen
        assert_eq!(app.get_command_log_offset(), None);
    }

    #[test]
    fn test_line_down_command_log_increments_offset() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 20);

        app.ui_state.set_command_log_offset(command_id, 5);
        app.line_down_command_log(10);

        assert_eq!(app.get_command_log_offset(), Some(6));
    }

    #[test]
    fn test_line_down_command_log_at_max_offset() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 20);

        // max_offset = 20 - 10 = 10
        app.ui_state.set_command_log_offset(command_id, 10);
        app.line_down_command_log(10);

        // Should remove offset when at max
        assert_eq!(app.get_command_log_offset(), None);
    }

    #[test]
    fn test_line_up_command_log_with_no_offset() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 20);

        app.line_up_command_log(10);

        // Defaults to max_offset (20 - 10 = 10), then decrements to 9
        assert_eq!(app.get_command_log_offset(), Some(9));
    }

    #[test]
    fn test_line_up_command_log_decrements_offset() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 20);

        app.ui_state.set_command_log_offset(command_id, 5);
        app.line_up_command_log(10);

        assert_eq!(app.get_command_log_offset(), Some(4));
    }

    #[test]
    fn test_line_up_command_log_at_zero() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 20);

        app.ui_state.set_command_log_offset(command_id, 0);
        app.line_up_command_log(10);

        // saturating_sub ensures it stays at 0
        assert_eq!(app.get_command_log_offset(), Some(0));
    }

    #[test]
    fn test_page_down_command_log_with_no_offset() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 10);

        app.page_down_command_log(5);

        // No offset was set, so nothing should happen
        assert_eq!(app.get_command_log_offset(), None);
    }

    #[test]
    fn test_page_down_command_log_increments_by_viewport() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 50);

        app.ui_state.set_command_log_offset(command_id, 5);
        app.page_down_command_log(10);

        // 5 + 10 = 15
        assert_eq!(app.get_command_log_offset(), Some(15));
    }

    #[test]
    fn test_page_down_command_log_clamped_to_max() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 20);

        // max_offset = 20 - 10 = 10
        app.ui_state.set_command_log_offset(command_id, 8);
        app.page_down_command_log(10);

        // 8 + 10 = 18, but max is 10, so clamped to 10
        assert_eq!(app.get_command_log_offset(), Some(10));
    }

    #[test]
    fn test_page_up_command_log_with_no_offset() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 30);

        app.page_up_command_log(10);

        // Defaults to max_offset (30 - 10 = 20), then subtracts 10 = 10
        assert_eq!(app.get_command_log_offset(), Some(10));
    }

    #[test]
    fn test_page_up_command_log_decrements_by_viewport() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 50);

        app.ui_state.set_command_log_offset(command_id, 25);
        app.page_up_command_log(10);

        // 25 - 10 = 15
        assert_eq!(app.get_command_log_offset(), Some(15));
    }

    #[test]
    fn test_page_up_command_log_at_zero() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 20);

        app.ui_state.set_command_log_offset(command_id, 5);
        app.page_up_command_log(10);

        // 5 - 10 = saturating to 0
        assert_eq!(app.get_command_log_offset(), Some(0));
    }

    #[test]
    fn test_scrolling_with_empty_output() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);

        // No output lines added
        app.ui_state.set_command_log_offset(command_id, 0);
        app.line_down_command_log(10);

        // max_offset = 0 - 10 = 0 (saturating_sub)
        // 0 + 1 = 1, min(1, 0) = 0
        // current_offset (0) == new_offset (0), so offset is removed
        assert_eq!(app.get_command_log_offset(), None);
    }

    #[test]
    fn test_scrolling_with_output_smaller_than_viewport() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);
        add_output_lines(&mut app, command_id, 5);

        // viewport_height = 10, total_lines = 5
        // max_offset = 5 - 10 = 0 (saturating_sub)
        app.ui_state.set_command_log_offset(command_id, 0);
        app.line_down_command_log(10);

        // new_offset = min(0 + 1, 0) = 0
        // current == new, so offset removed
        assert_eq!(app.get_command_log_offset(), None);
    }
}
