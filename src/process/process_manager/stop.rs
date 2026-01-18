use super::ProcessManager;
use anyhow::{Context, Result};
use uuid::Uuid;

impl ProcessManager {
    pub async fn stop(&mut self, command_id: Uuid) -> Result<()> {
        if let Some(handle) = self.handlers.remove(&command_id) {
            let mut child = handle.child.lock().await;
            child.kill().await.context("Failed to kill process")?;
        } else {
            anyhow::bail!("No process found with the given command ID");
        }
        Ok(())
    }
}
