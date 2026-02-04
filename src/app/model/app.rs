use super::{Command, Form};

// The main application state
pub struct App {
    // List of commands managed by the application
    commands: Vec<Command>,
    // Index of the currently selected command, if any
    selected_index: Option<usize>,
    // Flag indicating whether the application should quit
    should_quit: bool,
    // Current mode of the application
    mode: AppMode,
    // Form for adding or editing commands
    form: Form,
}

impl App {
    pub fn commands(&self) -> &Vec<Command> {
        &self.commands
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn selected_command_index(&self) -> Option<usize> {
        self.selected_index
    }

    pub fn mode(&self) -> &AppMode {
        &self.mode
    }

    pub fn form(&self) -> &Form {
        &self.form
    }

    pub fn form_mut(&mut self) -> &mut Form {
        &mut self.form
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
