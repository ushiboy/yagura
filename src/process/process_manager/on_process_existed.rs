use uuid::Uuid;

use crate::process::ProcessManager;

impl ProcessManager {
    pub fn on_process_exited(&mut self, command_id: Uuid) {
        self.handlers.remove(&command_id);
    }
}
