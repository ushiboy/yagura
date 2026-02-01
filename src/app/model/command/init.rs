use super::Command;
use super::CommandStatus;
use super::OutputBuffer;
use uuid::{NoContext, Timestamp, Uuid};

impl Command {
    // Creates a new command with the given command string.
    pub fn new(command: impl Into<String>) -> Self {
        let ts = Timestamp::now(NoContext);
        Self {
            id: Uuid::new_v7(ts),
            command: command.into(),
            working_dir: None,
            output_buffer: OutputBuffer::default(),
            status: CommandStatus::Stopped,
            pid: None,
            start_time: None,
            end_time: None,
        }
    }

    // Sets the working directory for the command.
    pub fn with_working_dir(mut self, dir: Option<impl Into<String>>) -> Self {
        self.working_dir = dir.map(|d| d.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_new() {
        let command = Command::new("sleep");
        assert_eq!(command.command(), "sleep");
    }

    #[test]
    fn test_command_with_working_dir() {
        let command = Command::new("ls").with_working_dir(Some("/tmp"));
        assert_eq!(command.command(), "ls");
        assert_eq!(command.working_dir(), Some("/tmp"));
    }
}
