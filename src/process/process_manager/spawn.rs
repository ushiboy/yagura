use anyhow::{Context, Result};

use super::super::Command;
use crate::process::Pid;
use std::{process::Stdio, sync::Arc};
use tokio::sync::Mutex;

use super::{ProcessHandle, ProcessManager};

impl ProcessManager {
    pub async fn spawn(&mut self, command: &Command) -> Result<()> {
        let command_id = command.id();
        let command_str = command.command();

        let parts: Vec<&str> = command_str.split_whitespace().collect();
        if parts.is_empty() {
            anyhow::bail!("Command string is empty");
        }

        let program = parts[0];
        let args = &parts[1..];

        let mut cmd_builder = tokio::process::Command::new(program);
        cmd_builder
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        let child = cmd_builder.spawn().context("Failed to spawn process")?;
        let pid = child.id().context("Failed to get process ID")?;

        let child = Arc::new(Mutex::new(child));

        let pid = Pid(pid);
        let handle = ProcessHandle {
            _pid: pid,
            child: child,
        };

        self.handlers.insert(command_id, handle);

        Ok(())
    }
}
