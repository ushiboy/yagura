use crate::app::CommandStatus;

pub fn display_status(status: &CommandStatus) -> String {
    match status {
        CommandStatus::Stopped => "Stopped".to_string(),
        CommandStatus::Running => "Running".to_string(),
        CommandStatus::Error(code) => format!("ERROR:{}", code.0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::ExitCode;

    #[test]
    fn test_display_status_stopped() {
        let status = CommandStatus::Stopped;
        assert_eq!(display_status(&status), "Stopped");
    }

    #[test]
    fn test_display_status_running() {
        let status = CommandStatus::Running;
        assert_eq!(display_status(&status), "Running");
    }

    #[test]
    fn test_display_status_error_with_zero() {
        let status = CommandStatus::Error(ExitCode(1));
        assert_eq!(display_status(&status), "ERROR:1");
    }
}
