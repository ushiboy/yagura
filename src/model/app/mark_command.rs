use uuid::Uuid;

use crate::process::{ExitCode, ProcessId};

use super::App;

impl App {
    pub fn mark_command_run(&mut self, command_id: Uuid, pid: ProcessId) {
        if let Some(command) = self.get_command_mut_by_id(command_id) {
            command.mark_running(pid);
        }
    }

    pub fn mark_command_exit(&mut self, command_id: Uuid, exit_code: ExitCode) {
        if let Some(command) = self.get_command_mut_by_id(command_id) {
            command.mark_exit(exit_code);
        }
    }
}
