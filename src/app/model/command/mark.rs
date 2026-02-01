use std::time::Instant;

use nix::sys::signal::Signal;

use crate::process::{ExitCode, Pid};

use super::{Command, CommandStatus};

impl Command {
    pub fn mark_running(&mut self, pid: Pid) {
        self.status = CommandStatus::Running;
        self.pid = Some(pid);
        self.start_time = Some(Instant::now());
        self.end_time = None;
    }

    pub fn mark_exit(&mut self, exit_code: ExitCode) {
        self.status = match exit_code {
            ExitCode::Signal(Signal::SIGINT | Signal::SIGKILL) => CommandStatus::Stopped,
            ExitCode::Code(0) => CommandStatus::Stopped,
            other => CommandStatus::Error(other),
        };
        self.pid = None;
        self.end_time = Some(Instant::now());
    }
}
