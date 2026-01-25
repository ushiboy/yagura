use std::time::Instant;

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
        self.status = if exit_code.is_success() {
            CommandStatus::Stopped
        } else {
            CommandStatus::Error(exit_code)
        };
        self.pid = None;
        self.end_time = Some(Instant::now());
    }
}
