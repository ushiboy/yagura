use crate::process::Pid;

pub fn format_pid(pid: Option<Pid>) -> String {
    if let Some(pid_value) = pid {
        format!("{}", pid_value)
    } else {
        " -- ".to_string()
    }
}
