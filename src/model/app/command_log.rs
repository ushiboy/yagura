use super::App;
use super::Command;
use crate::model::OutputLine;
use crate::model::{calculate_physical_lines, format_str};

impl App {
    // Returns the current physical row offset for the selected command, or None if in auto-follow mode.
    pub fn get_command_log_offset(&self) -> Option<usize> {
        let index = self.ui_state.selected_command_index()?;
        let command = self.commands.get(index)?;
        self.ui_state.get_command_log_offset(command.id())
    }

    // Returns the maximum valid physical row offset for the selected command, or None if no command selected.
    pub fn get_command_log_max_offset(
        &self,
        viewport_height: usize,
        viewport_width: usize,
    ) -> Option<usize> {
        let index = self.ui_state.selected_command_index()?;
        let command = self.commands.get(index)?;
        let show_timestamp = self.command_log_timestamp_visibility();
        Some(self.calculate_max_physical_offset(
            command,
            viewport_height,
            viewport_width,
            show_timestamp,
        ))
    }

    // Returns the visible output lines for the selected command, along with the sub_offset for the first line.
    pub fn get_visible_lines_with_sub_offset(
        &self,
        viewport_height: usize,
        viewport_width: usize,
    ) -> (Vec<&OutputLine>, usize) {
        let Some(command) = self.get_selected_command() else {
            return (vec![], 0);
        };
        let show_timestamp = self.command_log_timestamp_visibility();

        match self.get_command_log_offset() {
            None => {
                let lines = self.collect_lines_from_bottom(
                    command,
                    viewport_height,
                    viewport_width,
                    show_timestamp,
                );
                (lines, 0)
            }
            Some(physical_offset) => self.collect_lines_from_physical_offset(
                command,
                physical_offset,
                viewport_height,
                viewport_width,
                show_timestamp,
            ),
        }
    }

    // Returns the visible output lines for the selected command, without sub_offset information.
    pub fn get_visible_output_lines(
        &self,
        viewport_height: usize,
        viewport_width: usize,
    ) -> Vec<&OutputLine> {
        self.get_visible_lines_with_sub_offset(viewport_height, viewport_width)
            .0
    }

    // Returns the visible output lines for the selected command as plain text.
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

    // Scrolls the command log down by one physical row.
    pub fn line_down_command_log(&mut self, viewport_height: usize, viewport_width: usize) {
        self.scroll_down(viewport_height, viewport_width, 1);
    }

    // Scrolls the command log up by one physical row.
    pub fn line_up_command_log(&mut self, viewport_height: usize, viewport_width: usize) {
        self.scroll_up(viewport_height, viewport_width, 1);
    }

    // Scrolls the command log down by one page (viewport_height physical rows).
    pub fn page_down_command_log(&mut self, viewport_height: usize, viewport_width: usize) {
        self.scroll_down(viewport_height, viewport_width, viewport_height);
    }

    // Scrolls the command log up by one page (viewport_height physical rows).
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
            let max_offset = self.calculate_max_physical_offset(
                command,
                viewport_height,
                viewport_width,
                show_timestamp,
            );
            let new_offset = (current_offset + physical_delta).min(max_offset);

            if new_offset >= max_offset {
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
            let max_offset = self.calculate_max_physical_offset(
                command,
                viewport_height,
                viewport_width,
                show_timestamp,
            );
            let current = self
                .ui_state
                .get_command_log_offset(id)
                .unwrap_or(max_offset);
            let new_offset = current.saturating_sub(physical_delta);
            self.ui_state.set_command_log_offset(id, new_offset);
        }
    }

    // Helper to calculate the total number of physical lines in a command's output, given the viewport width and timestamp visibility.
    fn calculate_total_physical_lines(
        &self,
        command: &Command,
        viewport_width: usize,
        show_timestamp: bool,
    ) -> usize {
        command
            .output_buffer()
            .lines()
            .iter()
            .map(|line| {
                let content = format_str(line.timestamp(), line.content(), show_timestamp);
                calculate_physical_lines(&content, viewport_width)
            })
            .sum()
    }

    // Calculates the maximum valid physical row offset for a command, given the viewport dimensions and timestamp visibility.
    fn calculate_max_physical_offset(
        &self,
        command: &Command,
        viewport_height: usize,
        viewport_width: usize,
        show_timestamp: bool,
    ) -> usize {
        let total = self.calculate_total_physical_lines(command, viewport_width, show_timestamp);
        total.saturating_sub(viewport_height)
    }

    // When in auto-follow (offset=None): walks backwards from the end to find lines that fill the viewport, returns them with sub_offset=0.
    fn collect_lines_from_bottom<'a>(
        &self,
        command: &'a Command,
        viewport_height: usize,
        viewport_width: usize,
        show_timestamp: bool,
    ) -> Vec<&'a OutputLine> {
        if viewport_height == 0 {
            return vec![];
        }
        let all_lines: Vec<&OutputLine> = command.output_buffer().lines().iter().collect();
        let mut accumulated = 0usize;
        let mut start = all_lines.len();

        for i in (0..all_lines.len()).rev() {
            let content = format_str(
                all_lines[i].timestamp(),
                all_lines[i].content(),
                show_timestamp,
            );
            let phys = calculate_physical_lines(&content, viewport_width);
            if accumulated + phys > viewport_height {
                break;
            }
            accumulated += phys;
            start = i;
        }

        all_lines[start..].to_vec()
    }

    // When at a specific physical_offset: finds the logical line containing that offset, then collects lines until filling the viewport after skipping sub_offset.
    fn collect_lines_from_physical_offset<'a>(
        &self,
        command: &'a Command,
        physical_offset: usize,
        viewport_height: usize,
        viewport_width: usize,
        show_timestamp: bool,
    ) -> (Vec<&'a OutputLine>, usize) {
        if viewport_height == 0 {
            return (vec![], 0);
        }
        let all_lines: Vec<&OutputLine> = command.output_buffer().lines().iter().collect();

        let mut accumulated = 0usize;
        let mut start_logical = all_lines.len();
        let mut sub_offset = 0usize;

        for (i, line) in all_lines.iter().enumerate() {
            let content = format_str(line.timestamp(), line.content(), show_timestamp);
            let phys = calculate_physical_lines(&content, viewport_width);

            if accumulated + phys > physical_offset {
                start_logical = i;
                sub_offset = physical_offset - accumulated;
                break;
            }
            accumulated += phys;
        }

        if start_logical == all_lines.len() {
            return (vec![], 0);
        }

        let needed = viewport_height + sub_offset;
        let mut collected_phys = 0usize;
        let mut result = Vec::new();

        for line in &all_lines[start_logical..] {
            let content = format_str(line.timestamp(), line.content(), show_timestamp);
            let phys = calculate_physical_lines(&content, viewport_width);
            result.push(*line);
            collected_phys += phys;
            if collected_phys >= needed {
                break;
            }
        }

        (result, sub_offset)
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

        // No offset set → no-op (auto-follow remains)
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

        // Short lines → 1 physical row each; physical offset == logical index numerically.
        app.ui_state.set_command_log_offset(command_id, 5);
        app.line_down_command_log(10, 80);

        // 5 + 1 = 6 physical rows
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

        // max_physical_offset = 20 - 10 = 10
        app.ui_state.set_command_log_offset(command_id, 10);
        app.line_down_command_log(10, 80);

        // Reached max → auto-follow (None)
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

        // max_physical_offset = 20 - 10 = 10; 10 - 1 = 9
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

        // saturating_sub(1) at 0 stays at 0
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

        // No offset set → no-op
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

        // 5 + 10 = 15 physical rows
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

        // max_physical_offset = 20 - 10 = 10
        app.ui_state.set_command_log_offset(command_id, 8);
        app.page_down_command_log(10, 80);

        // 8 + 10 = 18, clamped to max=10 → auto-follow (None)
        assert_eq!(app.get_command_log_offset(), None);
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

        // max = 30 - 10 = 20; 20 - 10 = 10
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

        // 5 - 10 saturates to 0
        assert_eq!(app.get_command_log_offset(), Some(0));
    }

    #[test]
    fn test_scrolling_with_empty_output() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);

        app.ui_state.set_command_log_offset(command_id, 0);
        app.line_down_command_log(10, 80);

        // max_physical_offset = 0; 0 + 1 clamped to 0 → auto-follow
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

        // total_physical=5, viewport=10 → max_physical_offset=0
        app.ui_state.set_command_log_offset(command_id, 0);
        app.line_down_command_log(10, 80);

        // 0 + 1 clamped to max=0 → auto-follow
        assert_eq!(app.get_command_log_offset(), None);
    }

    #[test]
    fn test_scrolling_with_wrapped_lines() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);

        for i in 0..10 {
            app.add_output_line(
                command_id,
                OutputLine::new(format!("Line {}: {}", i, "x".repeat(161))),
            );
        }

        app.line_up_command_log(10, 80);
        let offset = app.get_command_log_offset();
        assert!(offset.is_some());
        assert!(offset.unwrap() < 30);
    }

    #[test]
    fn test_max_offset_with_wrapped_lines() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);

        for i in 0..20 {
            app.add_output_line(
                command_id,
                OutputLine::new(format!("Line {}: {}", i, "x".repeat(161))),
            );
        }

        let max_offset = app.get_command_log_max_offset(10, 80);
        assert!(max_offset.is_some());
        assert!(
            max_offset.unwrap() > 10,
            "physical max offset must be much larger than logical max"
        );
    }

    #[test]
    fn test_visible_output_with_wrapped_lines() {
        let mut app = App::new();
        let command = Command::new("test");
        let command_id = command.id();
        app.add_command(command);
        app.select_command_by_id(command_id);

        for i in 0..10 {
            app.add_output_line(
                command_id,
                OutputLine::new(format!("Line {}: {}", i, "x".repeat(200))),
            );
        }

        let output = app.visible_output_as_plain_text(10, 80);
        assert!(output.is_some());
        let line_count = output.unwrap().lines().count();
        assert!(line_count < 10);
        assert!(line_count > 0);
    }
}
