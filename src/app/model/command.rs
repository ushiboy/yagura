use std::time::{Duration, Instant};

use uuid::Uuid;

use crate::process::{ExitCode, Pid};

use super::OutputBuffer;

pub struct Command {
    id: Uuid,
    command: String,
    working_dir: Option<String>,
    output_buffer: OutputBuffer,
    status: CommandStatus,
    pid: Option<Pid>,
    start_time: Option<Instant>,
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

    pub fn pid(&self) -> Option<Pid> {
        self.pid
    }

    pub fn elapsed_time(&self) -> Option<Duration> {
        match (self.start_time, self.end_time) {
            (Some(start), Some(end)) => Some(end.duration_since(start)),
            (Some(start), None) => Some(Instant::now().duration_since(start)),
            _ => None,
        }
    }
}

pub enum CommandStatus {
    Stopped,
    Running,
    Error(ExitCode),
}

mod add_output_line;
mod init;
mod mark;
