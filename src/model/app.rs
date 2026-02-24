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

    pub fn toggle_command_log_timestamp_visibility(&mut self) {
        self.ui_state.toggle_command_log_timestamp_visibility();
    }

    pub fn command_log_timestamp_visibility(&self) -> bool {
        self.ui_state.command_log_timestamp_visibility()
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

    pub fn toggle_help(&mut self) {
        self.ui_state.toggle_help();
    }

    pub fn help_visible(&self) -> bool {
        self.ui_state.help_visible()
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

mod command_actions;
mod command_log;
mod command_ops;
mod command_selection;
mod init;
mod mode;
mod quit;
