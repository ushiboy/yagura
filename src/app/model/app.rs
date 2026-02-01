use super::{Command, Form};

pub struct App {
    commands: Vec<Command>,
    selected_index: Option<usize>,
    should_quit: bool,
    mode: AppMode,
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

#[derive(Debug, PartialEq)]
pub enum AppMode {
    Normal,
    AddingCommand,
}

mod add_output_line;
mod command_ops;
mod init;
mod mark_command;
mod mode;
mod quit;
mod select;
mod usecase;
