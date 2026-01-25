use super::ProcessManager;
use anyhow::Result;
use uuid::Uuid;

impl ProcessManager {
    pub async fn stop(&mut self, command_id: Uuid) -> Result<()> {
        if let Some(handle) = self.handlers.remove(&command_id) {
            let _ = handle.kill_tx.send(());
            Ok(())
        } else {
            anyhow::bail!("No process found with the given command ID");
        }
    }
}
