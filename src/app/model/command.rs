use uuid::Uuid;

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
}

mod add_output_line;
mod init;
