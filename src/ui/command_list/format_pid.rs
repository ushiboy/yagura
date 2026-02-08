use crate::process::ProcessId;

pub fn format_pid(pid: Option<ProcessId>) -> String {
    if let Some(pid_value) = pid {
        format!("{}", pid_value)
    } else {
        " -- ".to_string()
    }
}
