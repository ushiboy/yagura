use super::App;

impl App {
    pub fn get_command_log_offset(&self) -> Option<usize> {
        let index = self.ui_state.selected_command_index()?;
        let command = self.commands.get(index)?;
        self.ui_state.get_command_log_offset(command.id())
    }

    pub fn line_down_command_log(&mut self, viewport_height: usize) {
        if let Some(index) = self.ui_state.selected_command_index()
            && let Some(command) = self.commands.get(index)
            && let Some(current_offset) = self.ui_state.get_command_log_offset(command.id())
        {
            let id = command.id();
            let total_lines = command.output_buffer().line_length();
            let max_offset = total_lines.saturating_sub(viewport_height);
            let new_offset = (current_offset + 1).min(max_offset);

            if current_offset == new_offset {
                self.ui_state.remove_command_log_offset(id);
            } else {
                self.ui_state.set_command_log_offset(id, new_offset);
            }
        };
    }

    pub fn line_up_command_log(&mut self, viewport_height: usize) {
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
            let new_offset = current.saturating_sub(1);
            self.ui_state.set_command_log_offset(id, new_offset);
        }
    }

    pub fn page_down_command_log(&mut self, viewport_height: usize) {
        if let Some(index) = self.ui_state.selected_command_index()
            && let Some(command) = self.commands.get(index)
            && let Some(current_offset) = self.ui_state.get_command_log_offset(command.id())
        {
            let id = command.id();
            let total_lines = command.output_buffer().line_length();
            let max_offset = total_lines.saturating_sub(viewport_height);
            let new_offset = (current_offset + viewport_height).min(max_offset);

            if current_offset == new_offset {
                self.ui_state.remove_command_log_offset(id);
            } else {
                self.ui_state.set_command_log_offset(id, new_offset);
            }
        };
    }

    pub fn page_up_command_log(&mut self, viewport_height: usize) {
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
            let new_offset = current.saturating_sub(viewport_height);
            self.ui_state.set_command_log_offset(id, new_offset);
        }
    }
}
