use anyhow::{Context, Result};

use crate::app::{Command, OutputLine};
use crate::event::AppEvent;
use crate::process::Pid;
use std::{process::Stdio, sync::Arc};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio::sync::{Mutex, mpsc};

use super::{ProcessHandle, ProcessManager};

impl ProcessManager {
    pub async fn spawn(
        &mut self,
        command: &Command,
        event_tx: mpsc::UnboundedSender<AppEvent>,
    ) -> Result<()> {
        let command_id = command.id();
        let command_str = command.command();

        let parts: Vec<&str> = command_str.split_whitespace().collect();
        if parts.is_empty() {
            anyhow::bail!("Command string is empty");
        }

        let program = parts[0];
        let args = &parts[1..];

        let mut child = TokioCommand::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .context("Failed to spawn process")?;

        let pid = child.id().context("Failed to get process ID")?;

        let stdout = child.stdout.take().context("Failed to take stdout")?;
        let stdout_tx = event_tx.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = stdout_tx.send(AppEvent::ProcessOutput(command_id, OutputLine::new(line)));
            }
        });

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
