use super::App;
use super::Command;
use crate::model::OutputLine;
use crate::model::{calculate_physical_lines, filter_lines_by_physical_height, format_str};

impl App {
    // Returns the current log offset for the selected command, or None if no offset is set.
    pub fn get_command_log_offset(&self) -> Option<usize> {
        let index = self.ui_state.selected_command_index()?;
        let command = self.commands.get(index)?;
        self.ui_state.get_command_log_offset(command.id())
    }

    // Returns the maximum valid log offset for the selected command based on the current viewport size and line wrapping.
    pub fn get_command_log_max_offset(
        &self,
        viewport_height: usize,
        viewport_width: usize,
    ) -> Option<usize> {
        let index = self.ui_state.selected_command_index()?;
        let command = self.commands.get(index)?;
        let show_timestamp = self.command_log_timestamp_visibility();
        Some(self.calculate_max_offset(command, viewport_height, viewport_width, show_timestamp))
    }

    // Returns the visible output lines for the selected command, taking into account the current log offset, viewport height, and line wrapping.
    pub fn get_visible_output_lines(
        &self,
        viewport_height: usize,
        viewport_width: usize,
    ) -> Vec<&OutputLine> {
        if let Some(command) = self.get_selected_command() {
            let scroll_offset = self.get_command_log_offset().unwrap_or_else(|| {
                self.get_command_log_max_offset(viewport_height, viewport_width)
                    .unwrap_or(0)
            });
            let show_timestamp = self.command_log_timestamp_visibility();

            let sliced_lines = command
                .output_buffer()
                .slice_lines(scroll_offset, viewport_height);

            filter_lines_by_physical_height(
                sliced_lines,
                viewport_width,
                viewport_height,
                show_timestamp,
            )
        } else {
            vec![]
        }
    }

    // Returns the visible output lines for the selected command as plain text, taking into account the current log offset and viewport height.
    pub fn visible_output_as_plain_text(
        &self,
        viewport_height: usize,
        viewport_width: usize,
    ) -> Option<String> {
        let show_timestamp = self.command_log_timestamp_visibility();
        let filtered_lines = self.get_visible_output_lines(viewport_height, viewport_width);

        let formatted = filtered_lines
            .iter()
            .map(|line| format_str(line.timestamp(), &line.plain_text(), show_timestamp))
            .collect::<Vec<_>>()
            .join("\n");

        Some(formatted)
    }

    // Scrolls the command log down by one line, ensuring it doesn't exceed the maximum offset.
    pub fn line_down_command_log(&mut self, viewport_height: usize, viewport_width: usize) {
        self.scroll_down(viewport_height, viewport_width, 1);
    }

    // Scrolls the command log up by one line, ensuring it doesn't go below zero.
    pub fn line_up_command_log(&mut self, viewport_height: usize, viewport_width: usize) {
        self.scroll_up(viewport_height, viewport_width, 1);
    }

    // Scrolls the command log down by one page (viewport height), ensuring it doesn't exceed the maximum offset.
    pub fn page_down_command_log(&mut self, viewport_height: usize, viewport_width: usize) {
        self.scroll_down(viewport_height, viewport_width, viewport_height);
    }

    // Scrolls the command log up by one page (viewport height), ensuring it doesn't go below zero.
    pub fn page_up_command_log(&mut self, viewport_height: usize, viewport_width: usize) {
        self.scroll_up(viewport_height, viewport_width, viewport_height);
    }

    fn scroll_down(
        &mut self,
        viewport_height: usize,
        viewport_width: usize,
        physical_delta: usize,
    ) {
        if let Some(index) = self.ui_state.selected_command_index()
            && let Some(command) = self.commands.get(index)
            && let Some(current_offset) = self.ui_state.get_command_log_offset(command.id())
        {
            let id = command.id();
            let show_timestamp = self.command_log_timestamp_visibility();

            let new_offset = self.calculate_offset_for_scroll_down(
                command,
                current_offset,
                viewport_width,
                physical_delta,
                show_timestamp,
            );

            let max_offset =
                self.calculate_max_offset(command, viewport_height, viewport_width, show_timestamp);
            let new_offset = new_offset.min(max_offset);

            if current_offset == new_offset {
                self.ui_state.remove_command_log_offset(id);
            } else {
                self.ui_state.set_command_log_offset(id, new_offset);
            }
        }
    }

    fn scroll_up(&mut self, viewport_height: usize, viewport_width: usize, physical_delta: usize) {
        if let Some(index) = self.ui_state.selected_command_index()
            && let Some(command) = self.commands.get(index)
        {
            let id = command.id();
            let show_timestamp = self.command_log_timestamp_visibility();

            let max_offset =
                self.calculate_max_offset(command, viewport_height, viewport_width, show_timestamp);
            let current = self
                .ui_state
                .get_command_log_offset(id)
                .unwrap_or(max_offset);

            let new_offset = self.calculate_offset_for_scroll_up(
                command,
                current,
                viewport_width,
                physical_delta,
                show_timestamp,
            );

            self.ui_state.set_command_log_offset(id, new_offset);
        }
    }

    fn get_physical_line_count_at_offset(
        &self,
        command: &Command,
        logical_offset: usize,
        viewport_width: usize,
        show_timestamp: bool,
    ) -> Option<usize> {
        let lines = command.output_buffer().slice_lines(logical_offset, 1);
        let line = lines.first()?;
        let content = format_str(line.timestamp(), line.content(), show_timestamp);
        Some(calculate_physical_lines(&content, viewport_width))
    }

    fn calculate_offset_for_scroll_down(
        &self,
        command: &Command,
        current_offset: usize,
        viewport_width: usize,
        physical_delta: usize,
        show_timestamp: bool,
    ) -> usize {
        let total_lines = command.output_buffer().line_length();
        let mut accumulated_physical = 0;
        let mut logical_offset = current_offset;

        while logical_offset < total_lines && accumulated_physical < physical_delta {
            if let Some(physical_lines) = self.get_physical_line_count_at_offset(
                command,
                logical_offset,
                viewport_width,
                show_timestamp,
            ) {
                accumulated_physical += physical_lines;
                logical_offset += 1;
            } else {
                break;
            }
        }

        logical_offset
    }

    fn calculate_offset_for_scroll_up(
        &self,
        command: &Command,
        current_offset: usize,
        viewport_width: usize,
        physical_delta: usize,
        show_timestamp: bool,
    ) -> usize {
        if current_offset == 0 {
            return 0;
        }

        let mut accumulated_physical = 0;
        let mut logical_offset = current_offset;

        while logical_offset > 0 && accumulated_physical < physical_delta {
            logical_offset -= 1;
            if let Some(physical_lines) = self.get_physical_line_count_at_offset(
                command,
                logical_offset,
                viewport_width,
                show_timestamp,
            ) {
                accumulated_physical += physical_lines;
            } else {
                break;
            }
        }

        logical_offset
    }

    fn calculate_max_offset(
        &self,
        command: &Command,
        viewport_height: usize,
        viewport_width: usize,
        show_timestamp: bool,
    ) -> usize {
        let total_lines = command.output_buffer().line_length();
        if total_lines == 0 {
            return 0;
        }

        let mut accumulated_physical = 0;
        let mut logical_offset = total_lines;

        while logical_offset > 0 {
            logical_offset -= 1;
            if let Some(physical_lines) = self.get_physical_line_count_at_offset(
                command,
                logical_offset,
                viewport_width,
                show_timestamp,
            ) {
                if accumulated_physical + physical_lines > viewport_height {
                    logical_offset += 1;
                    break;
                }

                accumulated_physical += physical_lines;
            } else {
                break;
            }
        }

        logical_offset
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

        app.line_down_command_log(5, 80);

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
        app.line_down_command_log(10, 80);

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
        app.line_down_command_log(10, 80);

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

        app.line_up_command_log(10, 80);

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
        app.line_up_command_log(10, 80);

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
        app.line_up_command_log(10, 80);

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

        app.page_down_command_log(5, 80);

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
        app.page_down_command_log(10, 80);

        // With physical line scrolling, the exact offset depends on line wrapping
        // Since test lines are short (e.g., "Line 0"), they won't wrap at width 80
        // So behavior should be similar: scrolling down by 10 physical lines
        // means advancing by 10 logical lines
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
        app.page_down_command_log(10, 80);

        // Scrolling down by 10 physical lines from offset 8 should be clamped to max_offset (10)
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

        app.page_up_command_log(10, 80);

        // Defaults to max_offset (30 - 10 = 20), then subtracts 10 physical lines = 10
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
        app.page_up_command_log(10, 80);

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
        app.page_up_command_log(10, 80);

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
        app.line_down_command_log(10, 80);

        // max_offset = 0 - 10 = 0 (saturating_sub)
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
        app.line_down_command_log(10, 80);

        // current == new, so offset removed
        assert_eq!(app.get_command_log_offset(), None);
    }

    #[test]
    fn test_scrolling_with_wrapped_lines() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);

        // Add lines that will wrap to multiple physical lines at width 80
        // Each line is 200 characters, which wraps to 3 physical lines at width 80
        for i in 0..10 {
            app.add_output_line(
                command_id,
                OutputLine::new(format!("Line {}: {}", i, "x".repeat(200))),
            );
        }

        // viewport_height = 10 physical lines, viewport_width = 80
        // Each logical line wraps to ~3 physical lines
        // So we can fit approximately 3 logical lines in 10 physical lines

        // Test scrolling up from the bottom
        app.line_up_command_log(10, 80);
        let offset = app.get_command_log_offset();
        assert!(offset.is_some());
        assert!(offset.unwrap() < 10); // Should scroll back from the end
    }

    #[test]
    fn test_max_offset_with_wrapped_lines() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);

        // Add lines that will wrap
        for i in 0..20 {
            app.add_output_line(
                command_id,
                OutputLine::new(format!("Line {}: {}", i, "x".repeat(200))),
            );
        }

        // With wrapped lines, max_offset should be calculated based on physical lines
        let max_offset = app.get_command_log_max_offset(10, 80);
        assert!(max_offset.is_some());
        // The max offset should be greater than (20 - 10) because lines wrap
        assert!(max_offset.unwrap() > 10);
    }

    #[test]
    fn test_line_down_with_wrapped_lines() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);

        // Add lines that wrap
        for i in 0..20 {
            app.add_output_line(
                command_id,
                OutputLine::new(format!("Line {}: {}", i, "x".repeat(200))),
            );
        }

        app.ui_state.set_command_log_offset(command_id, 0);

        // Scroll down by 1 physical line
        app.line_down_command_log(10, 80);
        let after_scroll = app.get_command_log_offset();

        // Since we're scrolling down by 1 physical line but the first line wraps to ~3 physical lines,
        // the logical offset should still be 0 (we're still within the same wrapped line)
        // Actually, line_down scrolls by 1 physical line, so it might stay at 0 or move to 1
        // depending on whether we've consumed a full logical line
        assert!(after_scroll.is_some());
        assert!(after_scroll.unwrap() <= 1);
    }

    #[test]
    fn test_visible_output_with_wrapped_lines() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);

        // Add lines that wrap
        for i in 0..10 {
            app.add_output_line(
                command_id,
                OutputLine::new(format!("Line {}: {}", i, "x".repeat(200))),
            );
        }

        // Get visible output as plain text with physical line constraints
        let output = app.visible_output_as_plain_text(10, 80);
        assert!(output.is_some());

        let text = output.unwrap();
        // The output should contain fewer than 10 logical lines because they wrap
        let line_count = text.lines().count();
        assert!(line_count < 10);
        assert!(line_count > 0);
    }
}
