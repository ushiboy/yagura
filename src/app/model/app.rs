use super::Command;

pub struct App {
    commands: Vec<Command>,
    selected_index: Option<usize>,
    should_quit: bool,
}

impl App {
    pub fn commands(&self) -> &Vec<Command> {
        &self.commands
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }
}

mod add_command;
mod add_output_line;
mod init;
mod quit;
mod select;
