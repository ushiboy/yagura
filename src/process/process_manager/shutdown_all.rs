use super::ProcessManager;
use anyhow::Result;
use uuid::Uuid;

impl ProcessManager {
    pub async fn shutdown_all(&mut self) -> Result<()> {
        let command_ids: Vec<Uuid> = self.handlers.keys().cloned().collect();
        for command_id in command_ids {
            self.stop(command_id).await?;
        }
        Ok(())
    }
}
