use crate::process::ExitCode;

use super::{Command, CommandStatus};

impl Command {
    pub fn mark_running(&mut self) {
        self.status = CommandStatus::Running;
    }

    pub fn mark_stop(&mut self, exit_code: ExitCode) {
        if exit_code.is_success() {
            self.status = CommandStatus::Stopped;
        } else {
            self.status = CommandStatus::Error(exit_code);
        }
    }
}
