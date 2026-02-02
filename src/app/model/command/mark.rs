use super::{Command, CommandStatus};
use crate::process::{ExitCode, Pid};
use nix::sys::signal::Signal;
use std::time::Instant;

impl Command {
    // Marks the command as running with the given PID.
    pub fn mark_running(&mut self, pid: Pid) {
        self.status = CommandStatus::Running;
        self.pid = Some(pid);
        self.start_time = Some(Instant::now());
        self.end_time = None;
    }

    // Marks the command as exited with the given exit code.
    pub fn mark_exit(&mut self, exit_code: ExitCode) {
        self.status = match exit_code {
            ExitCode::Signal(Signal::SIGINT | Signal::SIGKILL) => CommandStatus::Stopped,
            ExitCode::Code(0) => CommandStatus::Stopped,
            other => CommandStatus::Error(other),
        };
        self.pid = None;
        self.end_time = Some(Instant::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_running() {
        let mut command = Command::new("echo 'Hello, World!'");

        command.mark_running(Pid(1234));

        assert_eq!(command.status(), &CommandStatus::Running);
        assert_eq!(command.pid(), Some(Pid(1234)));
        assert!(command.start_time.is_some());
        assert!(command.end_time.is_none());
    }

    #[test]
    fn test_mark_exit_with_exit_code_zero() {
        let mut command = Command::new("echo 'Hello, World!'");
        command.mark_running(Pid(1234));

        command.mark_exit(ExitCode::Code(0));

        assert_eq!(command.status(), &CommandStatus::Stopped);
        assert_eq!(command.pid(), None);
        assert!(command.end_time.is_some());
    }
    #[test]
    fn test_mark_exit_with_non_zero_exit_code() {
        let mut command = Command::new("echo 'Hello, World!'");

        command.mark_exit(ExitCode::Code(1));

        assert_eq!(command.status(), &CommandStatus::Error(ExitCode::Code(1)));
        assert_eq!(command.pid(), None);
        assert!(command.end_time.is_some());
    }

    #[test]
    fn test_mark_exit_with_signal_interrupt() {
        let mut command = Command::new("echo 'Hello, World!'");

        command.mark_exit(ExitCode::Signal(Signal::SIGINT));

        assert_eq!(command.status(), &CommandStatus::Stopped);
        assert_eq!(command.pid(), None);
        assert!(command.end_time.is_some());
    }

    #[test]
    fn test_mark_exit_with_signal_kill() {
        let mut command = Command::new("echo 'Hello, World!'");

        command.mark_exit(ExitCode::Signal(Signal::SIGKILL));

        assert_eq!(command.status(), &CommandStatus::Stopped);
        assert_eq!(command.pid(), None);
        assert!(command.end_time.is_some());
    }
}
