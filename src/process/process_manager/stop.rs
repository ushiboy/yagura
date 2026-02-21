use super::ProcessManager;
use anyhow::Result;
use uuid::Uuid;

impl ProcessManager {
    pub async fn stop(&mut self, command_id: Uuid) -> Result<()> {
        if let Some(handle) = self.handlers.get_mut(&command_id) {
            if let Some(kill_tx) = handle.kill_tx.take() {
                let _ = kill_tx.send(());
            }
            Ok(())
        } else {
            // ignore
            Ok(())
        }
    }
}
