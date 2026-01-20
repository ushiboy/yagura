use uuid::Uuid;

use crate::app::OutputLine;

use super::OutputBuffer;

pub struct Command {
    id: Uuid,
    command: String,
    output_buffer: OutputBuffer,
}

impl Command {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn output_lines(&self) -> Vec<&OutputLine> {
        self.output_buffer.lines().iter().collect()
    }
}

mod add_output_line;
mod init;
