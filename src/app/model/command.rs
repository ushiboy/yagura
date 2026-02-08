use super::OutputBuffer;
use crate::process::{ExitCode, ProcessId};
use std::time::{Duration, Instant};
use uuid::Uuid;

// Represents a command being executed in the system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    // Unique identifier for the command
    id: Uuid,
    // The command string to be executed
    command: String,
    // Optional working directory for the command
    working_dir: Option<String>,
    // Buffer to store the output of the command
    output_buffer: OutputBuffer,
    // Current status of the command
    status: CommandStatus,
    // Process ID of the running command, if applicable
    pid: Option<ProcessId>,
    // Timestamps for tracking execution duration
    start_time: Option<Instant>,
    // End time of the command execution
    end_time: Option<Instant>,
}

impl Command {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn working_dir(&self) -> Option<&str> {
        self.working_dir.as_deref()
    }

    pub fn output_buffer(&self) -> &OutputBuffer {
        &self.output_buffer
    }

    pub fn status(&self) -> &CommandStatus {
        &self.status
    }

    pub fn pid(&self) -> Option<ProcessId> {
        self.pid
    }

    // Calculates the elapsed time of the command execution.
    pub fn elapsed_time(&self) -> Option<Duration> {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            (Some(start), None) => Some(Instant::now().duration_since(start)),
            _ => None,
        }
    }
}

// Represents the status of a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandStatus {
    Stopped,
    Running,
    Error(ExitCode),
}

mod add_output_line;
mod init;
mod mark;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elapsed_time() {
        let mut command = Command::new("echo 'Hello, World!'");

        assert_eq!(command.elapsed_time(), None);

        command.start_time = Some(Instant::now());
        assert!(command.elapsed_time().is_some());

        command.end_time = Some(Instant::now() + Duration::from_millis(100));
        let elapsed = command.elapsed_time().unwrap();
        assert!(elapsed.as_millis() == 100);
    }
}
