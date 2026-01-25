use uuid::Uuid;

use crate::process::ProcessManager;

impl ProcessManager {
    pub fn on_process_existed(&mut self, command_id: Uuid) {
        self.handlers.remove(&command_id);
    }
}
