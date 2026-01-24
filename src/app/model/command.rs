use uuid::Uuid;

use crate::process::ExitCode;

use super::OutputBuffer;

pub struct Command {
    id: Uuid,
    command: String,
    output_buffer: OutputBuffer,
    status: CommandStatus,
}

impl Command {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn output_buffer(&self) -> &OutputBuffer {
        &self.output_buffer
    }

    pub fn status(&self) -> &CommandStatus {
        &self.status
    }
}

pub enum CommandStatus {
    Stopped,
    Running,
    Error(ExitCode),
}

mod add_output_line;
mod init;
