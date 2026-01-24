use uuid::Uuid;

use crate::process::{ExitCode, Pid};

use super::App;

impl App {
    pub fn mark_command_run(&mut self, command_id: Uuid, _pid: Pid) {
        if let Some(command) = self.commands.iter_mut().find(|cmd| cmd.id() == command_id) {
            command.mark_running();
        }
    }

    pub fn mark_command_exit(&mut self, command_id: Uuid, exit_code: ExitCode) {
        if let Some(command) = self.commands.iter_mut().find(|cmd| cmd.id() == command_id) {
            command.mark_stop(exit_code);
        }
    }
}
