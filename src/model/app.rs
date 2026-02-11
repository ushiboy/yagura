use super::ui_state::adding_command_form::AddingCommandForm;
use super::{Command, UIState};

// The main application state
pub struct App {
    // List of commands managed by the application
    commands: Vec<Command>,
    // Flag indicating whether the application should quit
    should_quit: bool,
    // Current mode of the application
    mode: AppMode,
    // UI state of the application
    ui_state: UIState,
}

impl App {
    pub fn commands(&self) -> &Vec<Command> {
        &self.commands
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn selected_command_index(&self) -> Option<usize> {
        self.ui_state.selected_command_index()
    }

    pub fn command_list_scroll_offset(&self) -> usize {
        self.ui_state.command_list_scroll_offset()
    }

    pub fn mode(&self) -> &AppMode {
        &self.mode
    }

    pub fn form(&self) -> &AddingCommandForm {
        self.ui_state.adding_command_form()
    }

    pub fn form_mut(&mut self) -> &mut AddingCommandForm {
        self.ui_state.adding_command_form_mut()
    }

    pub fn get_command_log_offset(&self) -> Option<usize> {
        let index = self.ui_state.selected_command_index()?;
        let command = self.commands.get(index)?;
        self.ui_state.get_command_log_offset(command.id())
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

// Different modes the application can be in
#[derive(Debug, PartialEq)]
pub enum AppMode {
    // Default mode for normal operation
    Normal,
    // Mode for adding a new command
    AddingCommand,
    // Mode for deleting a command
    DeletingCommand,
}

mod add_output_line;
mod command_ops;
mod init;
mod mark_command;
mod mode;
mod quit;
mod select;
mod usecase;
