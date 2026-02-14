use anyhow::{Context, Result};
use tokio::time::{Duration, timeout};

use super::{ProcessHandle, ProcessManager};
use crate::event::AppEvent;
use crate::model::{Command, OutputLine};
use crate::process::{ExitCode, ProcessId};
use nix::sys::signal::{Signal, killpg};
use std::io::Error;
use std::os::unix::process::ExitStatusExt;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as TokioCommand;
use tokio::sync::{mpsc, oneshot};

impl ProcessManager {
    pub async fn spawn(
        &mut self,
        command: &Command,
        event_tx: mpsc::UnboundedSender<AppEvent>,
    ) -> Result<ProcessId> {
        let command_id = command.id();
        let command_str = command.command();

        let parts = shlex::split(command_str).context("Failed to parse command string")?;
        if parts.is_empty() {
            anyhow::bail!("Command string is empty");
        }

        let program = &parts[0];
        let args = &parts[1..];

        let mut cmd_builder = TokioCommand::new(program);
        cmd_builder
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        unsafe {
            cmd_builder.pre_exec(|| {
                nix::unistd::setsid().map_err(Error::other)?;
                Ok(())
            });
        }

        if let Some(working_dir) = command.working_dir() {
            cmd_builder.current_dir(working_dir);
        }

        let mut child = cmd_builder.spawn().context("Failed to spawn process")?;

        let pid = child.id().context("Failed to get process ID")?;
        let (kill_tx, kill_rx) = oneshot::channel::<()>();

        let stdout = child.stdout.take().context("Failed to take stdout")?;
        let stdout_tx = event_tx.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = stdout_tx.send(AppEvent::ProcessOutput(command_id, OutputLine::new(line)));
            }
        });

        let stderr = child.stderr.take().context("Failed to take stderr")?;
        let stderr_tx = event_tx.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = stderr_tx.send(AppEvent::ProcessOutput(command_id, OutputLine::new(line)));
            }
        });

        let gpid = nix::unistd::Pid::from_raw(pid as i32);

        let exit_tx = event_tx;
        tokio::spawn(async move {
            let status = tokio::select! {
                status = child.wait() => status,
                _ = kill_rx => {
                    let _ = killpg(gpid, Signal::SIGINT);

                    match timeout(Duration::from_secs(5), child.wait()).await {
                        Ok(status) => status,
                        Err(_) => {
                            let _ = killpg(gpid, Signal::SIGKILL);
                            child.wait().await
                        }
                    }
                }
            };

            if let Ok(status) = status {
                let exit_code = if let Some(code) = status.code() {
                    ExitCode::Code(code)
                } else if let Some(signal) = status.signal() {
                    ExitCode::Signal(Signal::try_from(signal).unwrap())
                } else {
                    ExitCode::Code(-1) // Unknown exit status
                };
                let _ = exit_tx.send(AppEvent::ProcessExited(command_id, exit_code));
            }
        });

        let pid = ProcessId(pid);
        let handle = ProcessHandle {
            _pid: pid,
            kill_tx: Some(kill_tx),
        };

        self.handlers.insert(command_id, handle);

        Ok(pid)
    }
}
