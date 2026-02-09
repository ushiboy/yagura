use uuid::Uuid;

use super::super::OutputLine;
use super::App;

impl App {
    pub fn add_output_line(&mut self, command_id: Uuid, line: OutputLine) {
        if let Some(command) = self.commands.iter_mut().find(|cmd| cmd.id() == command_id) {
            command.add_output_line(line);
        }
    }
}
